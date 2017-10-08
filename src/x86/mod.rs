// Licensed under the 2-Clause BSD license <LICENSE or
// https://opensource.org/licenses/BSD-2-Clause>. This
// file may not be copied, modified, or distributed
// except according to those terms.

//! Disassemble x86 and x86_64 code.
//!
//! This is based on a C library, asmx86.

mod instruction_operations;
mod operand_types;

pub use self::instruction_operations::*;
pub use self::operand_types::*;

use std::cmp;
use std::fmt;
use std::ptr;

/// A segment register
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(i32)]
pub enum SegmentRegister {
    /// Extra data segment register.
    ES = 0,
    /// Code segment register.
    CS = 1,
    /// Stack segment register.
    SS = 2,
    /// Data segment register.
    DS = 3,
    /// Extra data segment register. Used as a thread register on some platforms.
    FS = 4,
    /// Extra data segment register. Used as a thread register on some platforms.
    GS = 5,
    /// Default segment register.
    DEFAULT = 7,
}

impl SegmentRegister {
    /// Lookup a `SegmentRegister` given the `i32` value.
    fn from_i32(i: i32) -> Self {
        match i {
            0 => SegmentRegister::ES,
            1 => SegmentRegister::CS,
            2 => SegmentRegister::SS,
            3 => SegmentRegister::DS,
            4 => SegmentRegister::FS,
            5 => SegmentRegister::GS,
            7 => SegmentRegister::DEFAULT,
            _ => panic!("Unknown segment register {}", i),
        }
    }
}

impl Default for SegmentRegister {
    fn default() -> Self {
        SegmentRegister::DEFAULT
    }
}

/// An operand for an `Instruction`.
///
/// The type of operand is given by the `operand` member. If the type is
/// `OperandType::NONE`, none of the other members are defined.
///
/// If the operand type is not `OperandType::NONE`, then the `size` field
/// is valid.
///
/// If the type is `OperandType::IMM`, then the operand is a constant
/// integer contained in the `immediate` member.
///
/// If the type is `OperandType::MEM`, then the operand is a memory reference
/// and the other members are used as defined.
///
/// The address of a memory reference is effectively the following formula,
/// where references to the component array should look up the current value
/// should look up the current value of the register or substitute zero if
/// the value is `OperandType::NONE`:
///
/// ```math
/// address = components[0] + components[1] * scale + immediate
/// ```
///
/// TODO: Perhaps this should be an enumeration with separate values
/// for invalid, immediate value, a memory reference or a register.
#[derive(Debug)]
#[repr(C)]
pub struct InstructionOperand {
    /// The type of the operand, a register or one of a set of special values.
    pub operand: OperandType,
    /// The address components of a memory operand.
    pub components: [OperandType; 2],
    /// If this is a memory operand, then this value is used to scale
    /// the second component.
    pub scale: u8,
    /// The size of the operand in bytes. For register operands, this
    /// is the size of the register. For memory operands, this is the
    /// size of the memory access.
    pub size: u16,
    /// The value of a constant integer when the operand is the `IMM`
    /// type. When the operand is a memory reference, this contains
    /// a constant offset for the memory reference.
    pub immediate: isize,
    /// The segment register that will be used for a memory access. This
    /// will always contain a segment register, as `SegmentRegister::DEFAULT`
    /// is resolved to the default register.
    pub segment: SegmentRegister,
}

impl Default for InstructionOperand {
    fn default() -> Self {
        InstructionOperand {
            operand: OperandType::NONE,
            components: [OperandType::NONE, OperandType::NONE],
            scale: 1,
            size: 0,
            immediate: 0,
            segment: SegmentRegister::DEFAULT,
        }
    }
}

/// An instruction.
///
/// An instruction represents the full amount of information that
/// we have about the instruction that has been disassembled from
/// the binary opcode data.
#[derive(Default, Debug)]
#[repr(C)]
pub struct Instruction {
    /// Which `InstructionOperation` this instruction is.
    pub operation: InstructionOperation,
    /// The operands for this instruction.
    pub operands: [InstructionOperand; 3],
    /// A bit field that may contain the flags described by [`X86Flag`].
    ///
    /// [`X86Flag`]: struct.X86Flag.html
    pub flags: u32,
    /// The segment prefix. This will be either `SegmentPrefix::DEFAULT`
    /// or a segment register (like `SegmentPrefix::ES`).
    pub segment: SegmentRegister,
    /// How many bytes in the binary opcode data are used by this
    /// instruction.
    ///
    /// This can be used to continue disassembling at the next
    /// instruction. An invalid instruction may have a value of
    /// `0` here.
    pub length: usize,
}

/// Flags used by `Instruction`.
pub struct X86Flag;

impl X86Flag {
    /// The lock prefix was provided to this instruction.
    pub const LOCK: u32 = 1;
    /// This instruction is an unconditional repeated string instruction.
    pub const REP: u32 = 2;
    /// The repeated string instruction is conditional and uses the `REPNE`
    /// prefix.
    pub const REPNE: u32 = 4;
    /// The repeated string instruction is conditional and uses the `REPE`
    /// prefix.
    pub const REPE: u32 = 8;
    /// The operand size prefix was used.
    pub const OPSIZE: u32 = 16;
    /// The address size prefix was used.
    pub const ADDRSIZE: u32 = 32;

    /// The instruction may be valid, but an insufficient number of bytes
    /// were provided. When this flag is set, the disassembly should not
    /// be considered a success.
    pub const INSUFFICIENT_LENGTH: u32 = 0x8000_0000;

    /// The instruction is any repeated string instruction.
    pub const ANY_REP: u32 = X86Flag::REP | X86Flag::REPE | X86Flag::REPNE;
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(i32)]
enum RepPrefix {
    NONE = 0i32,
    REPNE,
    REPE,
}

#[derive(Debug)]
#[repr(C)]
struct DecodeState {
    result: Instruction,
    operand0: *mut InstructionOperand,
    operand1: *mut InstructionOperand,
    opcode_start: *const u8,
    opcode: *const u8,
    addr: usize,
    len: usize,
    original_length: usize,
    op_size: u16,
    final_op_size: u16,
    addr_size: u16,
    flags: u32,
    invalid: bool,
    insufficient_length: bool,
    op_prefix: bool,
    rep: RepPrefix,
    using64: bool,
    rex: bool,
    rex_rm_1: bool,
    rex_rm_2: bool,
    rex_reg: bool,
    rip_rel_fixup: *mut isize,
}

impl Default for DecodeState {
    fn default() -> Self {
        DecodeState {
            result: Instruction::default(),
            operand0: ptr::null_mut(),
            operand1: ptr::null_mut(),
            opcode_start: ptr::null(),
            opcode: ptr::null(),
            addr: 0usize,
            len: 0usize,
            original_length: 0usize,
            op_size: 0u16,
            final_op_size: 0u16,
            addr_size: 0u16,
            flags: 0u32,
            invalid: false,
            insufficient_length: false,
            op_prefix: false,
            rep: RepPrefix::NONE,
            using64: false,
            rex: false,
            rex_rm_1: false,
            rex_rm_2: false,
            rex_reg: false,
            rip_rel_fixup: ptr::null_mut(),
        }
    }
}

struct DecodeFlags;

impl DecodeFlags {
    const LOCK: u32 = 0x0020;
    const REP: u32 = 0x0040;
    const REP_COND: u32 = 0x0080;
    const BYTE: u32 = 0x0100;
    const FLIP_OPERANDS: u32 = 0x0200;
    const IMM_SX: u32 = 0x0400;
    const INC_OPERATION_FOR_64: u32 = 0x0800;
    const OPERATION_OP_SIZE: u32 = 0x1000;
    const FORCE_16BIT: u32 = 0x2000;
    const INVALID_IN_64BIT: u32 = 0x400;
    const DEFAULT_TO_64BIT: u32 = 0x8000;

    const REG_RM_SIZE_MASK: u32 = 0x03;

    const REG_RM_2X_SIZE: u32 = 0x01;
    const REG_RM_FAR_SIZE: u32 = 0x02;
    const REG_RM_NO_SIZE: u32 = 0x03;
}

#[repr(C)]
struct InstructionEncoding {
    pub operation: u16,
    pub flags: u16,
    pub func: fn(&mut DecodeState),
}

static MAIN_OPCODE_MAP: [InstructionEncoding; 256] = [
    InstructionEncoding {
        operation: InstructionOperation::ADD as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::FLIP_OPERANDS | DecodeFlags::LOCK) as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::ADD as (u16),
        flags: (DecodeFlags::FLIP_OPERANDS | DecodeFlags::LOCK) as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::ADD as (u16),
        flags: DecodeFlags::BYTE as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::ADD as (u16),
        flags: 0,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::ADD as (u16),
        flags: DecodeFlags::BYTE as u16,
        func: decode_eax_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::ADD as (u16),
        flags: 0,
        func: decode_eax_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUSH as (u16),
        flags: 0,
        func: decode_push_pop_seg,
    },
    InstructionEncoding {
        operation: InstructionOperation::POP as (u16),
        flags: 0,
        func: decode_push_pop_seg,
    },
    InstructionEncoding {
        operation: InstructionOperation::OR as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::FLIP_OPERANDS | DecodeFlags::LOCK) as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::OR as (u16),
        flags: (DecodeFlags::FLIP_OPERANDS | DecodeFlags::LOCK) as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::OR as (u16),
        flags: DecodeFlags::BYTE as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::OR as (u16),
        flags: 0,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::OR as (u16),
        flags: DecodeFlags::BYTE as u16,
        func: decode_eax_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::OR as (u16),
        flags: 0,
        func: decode_eax_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUSH as (u16),
        flags: 0,
        func: decode_push_pop_seg,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0,
        func: decode_two_byte,
    },
    InstructionEncoding {
        operation: InstructionOperation::ADC as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::FLIP_OPERANDS | DecodeFlags::LOCK) as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::ADC as (u16),
        flags: (DecodeFlags::FLIP_OPERANDS | DecodeFlags::LOCK) as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::ADC as (u16),
        flags: DecodeFlags::BYTE as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::ADC as (u16),
        flags: 0,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::ADC as (u16),
        flags: DecodeFlags::BYTE as u16,
        func: decode_eax_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::ADC as (u16),
        flags: 0,
        func: decode_eax_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUSH as (u16),
        flags: 0,
        func: decode_push_pop_seg,
    },
    InstructionEncoding {
        operation: InstructionOperation::POP as (u16),
        flags: 0,
        func: decode_push_pop_seg,
    },
    InstructionEncoding {
        operation: InstructionOperation::SBB as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::FLIP_OPERANDS | DecodeFlags::LOCK) as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::SBB as (u16),
        flags: (DecodeFlags::FLIP_OPERANDS | DecodeFlags::LOCK) as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::SBB as (u16),
        flags: DecodeFlags::BYTE as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::SBB as (u16),
        flags: 0,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::SBB as (u16),
        flags: DecodeFlags::BYTE as u16,
        func: decode_eax_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::SBB as (u16),
        flags: 0,
        func: decode_eax_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUSH as (u16),
        flags: 0,
        func: decode_push_pop_seg,
    },
    InstructionEncoding {
        operation: InstructionOperation::POP as (u16),
        flags: 0,
        func: decode_push_pop_seg,
    },
    InstructionEncoding {
        operation: InstructionOperation::AND as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::FLIP_OPERANDS | DecodeFlags::LOCK) as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::AND as (u16),
        flags: (DecodeFlags::FLIP_OPERANDS | DecodeFlags::LOCK) as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::AND as (u16),
        flags: DecodeFlags::BYTE as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::AND as (u16),
        flags: 0,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::AND as (u16),
        flags: DecodeFlags::BYTE as u16,
        func: decode_eax_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::AND as (u16),
        flags: 0,
        func: decode_eax_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0,
        func: invalid_decode,
    },
    InstructionEncoding {
        operation: InstructionOperation::DAA as (u16),
        flags: 0,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: InstructionOperation::SUB as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::FLIP_OPERANDS | DecodeFlags::LOCK) as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::SUB as (u16),
        flags: (DecodeFlags::FLIP_OPERANDS | DecodeFlags::LOCK) as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::SUB as (u16),
        flags: DecodeFlags::BYTE as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::SUB as (u16),
        flags: 0,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::SUB as (u16),
        flags: DecodeFlags::BYTE as u16,
        func: decode_eax_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::SUB as (u16),
        flags: 0,
        func: decode_eax_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0,
        func: invalid_decode,
    },
    InstructionEncoding {
        operation: InstructionOperation::DAS as (u16),
        flags: 0,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: InstructionOperation::XOR as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::FLIP_OPERANDS | DecodeFlags::LOCK) as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::XOR as (u16),
        flags: (DecodeFlags::FLIP_OPERANDS | DecodeFlags::LOCK) as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::XOR as (u16),
        flags: DecodeFlags::BYTE as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::XOR as (u16),
        flags: 0,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::XOR as (u16),
        flags: DecodeFlags::BYTE as u16,
        func: decode_eax_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::XOR as (u16),
        flags: 0,
        func: decode_eax_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0,
        func: invalid_decode,
    },
    InstructionEncoding {
        operation: InstructionOperation::AAA as (u16),
        flags: 0,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMP as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::FLIP_OPERANDS) as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMP as (u16),
        flags: DecodeFlags::FLIP_OPERANDS as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMP as (u16),
        flags: DecodeFlags::BYTE as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMP as (u16),
        flags: 0,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMP as (u16),
        flags: DecodeFlags::BYTE as u16,
        func: decode_eax_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMP as (u16),
        flags: 0,
        func: decode_eax_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0,
        func: invalid_decode,
    },
    InstructionEncoding {
        operation: InstructionOperation::AAS as (u16),
        flags: 0,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: InstructionOperation::INC as (u16),
        flags: 0,
        func: decode_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::INC as (u16),
        flags: 0,
        func: decode_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::INC as (u16),
        flags: 0,
        func: decode_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::INC as (u16),
        flags: 0,
        func: decode_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::INC as (u16),
        flags: 0,
        func: decode_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::INC as (u16),
        flags: 0,
        func: decode_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::INC as (u16),
        flags: 0,
        func: decode_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::INC as (u16),
        flags: 0,
        func: decode_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::DEC as (u16),
        flags: 0,
        func: decode_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::DEC as (u16),
        flags: 0,
        func: decode_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::DEC as (u16),
        flags: 0,
        func: decode_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::DEC as (u16),
        flags: 0,
        func: decode_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::DEC as (u16),
        flags: 0,
        func: decode_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::DEC as (u16),
        flags: 0,
        func: decode_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::DEC as (u16),
        flags: 0,
        func: decode_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::DEC as (u16),
        flags: 0,
        func: decode_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUSH as (u16),
        flags: DecodeFlags::DEFAULT_TO_64BIT as u16,
        func: decode_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUSH as (u16),
        flags: DecodeFlags::DEFAULT_TO_64BIT as u16,
        func: decode_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUSH as (u16),
        flags: DecodeFlags::DEFAULT_TO_64BIT as u16,
        func: decode_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUSH as (u16),
        flags: DecodeFlags::DEFAULT_TO_64BIT as u16,
        func: decode_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUSH as (u16),
        flags: DecodeFlags::DEFAULT_TO_64BIT as u16,
        func: decode_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUSH as (u16),
        flags: DecodeFlags::DEFAULT_TO_64BIT as u16,
        func: decode_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUSH as (u16),
        flags: DecodeFlags::DEFAULT_TO_64BIT as u16,
        func: decode_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUSH as (u16),
        flags: DecodeFlags::DEFAULT_TO_64BIT as u16,
        func: decode_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::POP as (u16),
        flags: DecodeFlags::DEFAULT_TO_64BIT as u16,
        func: decode_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::POP as (u16),
        flags: DecodeFlags::DEFAULT_TO_64BIT as u16,
        func: decode_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::POP as (u16),
        flags: DecodeFlags::DEFAULT_TO_64BIT as u16,
        func: decode_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::POP as (u16),
        flags: DecodeFlags::DEFAULT_TO_64BIT as u16,
        func: decode_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::POP as (u16),
        flags: DecodeFlags::DEFAULT_TO_64BIT as u16,
        func: decode_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::POP as (u16),
        flags: DecodeFlags::DEFAULT_TO_64BIT as u16,
        func: decode_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::POP as (u16),
        flags: DecodeFlags::DEFAULT_TO_64BIT as u16,
        func: decode_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::POP as (u16),
        flags: DecodeFlags::DEFAULT_TO_64BIT as u16,
        func: decode_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUSHA as (u16),
        flags: (DecodeFlags::INVALID_IN_64BIT | DecodeFlags::OPERATION_OP_SIZE) as u16,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: InstructionOperation::POPA as (u16),
        flags: (DecodeFlags::INVALID_IN_64BIT | DecodeFlags::OPERATION_OP_SIZE) as u16,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: InstructionOperation::BOUND as (u16),
        flags: DecodeFlags::REG_RM_2X_SIZE as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::ARPL as (u16),
        flags: 0,
        func: decode_arpl,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0,
        func: invalid_decode,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0,
        func: invalid_decode,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0,
        func: invalid_decode,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0,
        func: invalid_decode,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUSH as (u16),
        flags: DecodeFlags::DEFAULT_TO_64BIT as u16,
        func: decode_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::IMUL as (u16),
        flags: 0,
        func: decode_reg_rm_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUSH as (u16),
        flags: (DecodeFlags::IMM_SX | DecodeFlags::DEFAULT_TO_64BIT) as u16,
        func: decode_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::IMUL as (u16),
        flags: DecodeFlags::IMM_SX as u16,
        func: decode_reg_rm_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::INSB as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::OPERATION_OP_SIZE | DecodeFlags::REP) as u16,
        func: decode_edi_dx,
    },
    InstructionEncoding {
        operation: InstructionOperation::INSW as (u16),
        flags: (DecodeFlags::OPERATION_OP_SIZE | DecodeFlags::REP) as u16,
        func: decode_edi_dx,
    },
    InstructionEncoding {
        operation: InstructionOperation::OUTSB as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::OPERATION_OP_SIZE | DecodeFlags::REP) as u16,
        func: decode_dx_esi,
    },
    InstructionEncoding {
        operation: InstructionOperation::OUTSW as (u16),
        flags: (DecodeFlags::OPERATION_OP_SIZE | DecodeFlags::REP) as u16,
        func: decode_dx_esi,
    },
    InstructionEncoding {
        operation: InstructionOperation::JO as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::DEFAULT_TO_64BIT) as u16,
        func: decode_rel_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JNO as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::DEFAULT_TO_64BIT) as u16,
        func: decode_rel_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JB as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::DEFAULT_TO_64BIT) as u16,
        func: decode_rel_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JAE as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::DEFAULT_TO_64BIT) as u16,
        func: decode_rel_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JE as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::DEFAULT_TO_64BIT) as u16,
        func: decode_rel_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JNE as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::DEFAULT_TO_64BIT) as u16,
        func: decode_rel_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JBE as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::DEFAULT_TO_64BIT) as u16,
        func: decode_rel_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JA as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::DEFAULT_TO_64BIT) as u16,
        func: decode_rel_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JS as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::DEFAULT_TO_64BIT) as u16,
        func: decode_rel_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JNS as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::DEFAULT_TO_64BIT) as u16,
        func: decode_rel_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JPE as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::DEFAULT_TO_64BIT) as u16,
        func: decode_rel_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JPO as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::DEFAULT_TO_64BIT) as u16,
        func: decode_rel_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JL as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::DEFAULT_TO_64BIT) as u16,
        func: decode_rel_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JGE as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::DEFAULT_TO_64BIT) as u16,
        func: decode_rel_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JLE as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::DEFAULT_TO_64BIT) as u16,
        func: decode_rel_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JG as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::DEFAULT_TO_64BIT) as u16,
        func: decode_rel_imm,
    },
    InstructionEncoding {
        operation: 0u16,
        flags: (DecodeFlags::BYTE | DecodeFlags::LOCK) as u16,
        func: decode_group_rm_imm,
    },
    InstructionEncoding {
        operation: 0u16,
        flags: DecodeFlags::LOCK as u16,
        func: decode_group_rm_imm,
    },
    InstructionEncoding {
        operation: 0u16,
        flags: (DecodeFlags::BYTE | DecodeFlags::INVALID_IN_64BIT | DecodeFlags::LOCK) as u16,
        func: decode_group_rm_imm,
    },
    InstructionEncoding {
        operation: 0u16,
        flags: (DecodeFlags::IMM_SX | DecodeFlags::LOCK) as u16,
        func: decode_group_rm_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::TEST as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::FLIP_OPERANDS) as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::TEST as (u16),
        flags: DecodeFlags::FLIP_OPERANDS as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::XCHG as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::FLIP_OPERANDS | DecodeFlags::LOCK) as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::XCHG as (u16),
        flags: (DecodeFlags::FLIP_OPERANDS | DecodeFlags::LOCK) as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::FLIP_OPERANDS) as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: DecodeFlags::FLIP_OPERANDS as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: DecodeFlags::BYTE as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: 0,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: 0,
        func: decode_rms_reg_v,
    },
    InstructionEncoding {
        operation: InstructionOperation::LEA as (u16),
        flags: DecodeFlags::REG_RM_NO_SIZE as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: DecodeFlags::FLIP_OPERANDS as u16,
        func: decode_rms_reg_v,
    },
    InstructionEncoding {
        operation: InstructionOperation::POP as (u16),
        flags: DecodeFlags::DEFAULT_TO_64BIT as u16,
        func: decode_rmv,
    },
    InstructionEncoding {
        operation: InstructionOperation::NOP as (u16),
        flags: 0,
        func: decode_nop,
    },
    InstructionEncoding {
        operation: InstructionOperation::XCHG as (u16),
        flags: 0,
        func: decode_eax_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::XCHG as (u16),
        flags: 0,
        func: decode_eax_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::XCHG as (u16),
        flags: 0,
        func: decode_eax_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::XCHG as (u16),
        flags: 0,
        func: decode_eax_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::XCHG as (u16),
        flags: 0,
        func: decode_eax_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::XCHG as (u16),
        flags: 0,
        func: decode_eax_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::XCHG as (u16),
        flags: 0,
        func: decode_eax_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::CBW as (u16),
        flags: DecodeFlags::OPERATION_OP_SIZE as u16,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: InstructionOperation::CWD as (u16),
        flags: DecodeFlags::OPERATION_OP_SIZE as u16,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: InstructionOperation::CALLF as (u16),
        flags: DecodeFlags::INVALID_IN_64BIT as u16,
        func: decode_far_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::FWAIT as (u16),
        flags: 0,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUSHF as (u16),
        flags: (DecodeFlags::DEFAULT_TO_64BIT | DecodeFlags::OPERATION_OP_SIZE) as u16,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: InstructionOperation::POPF as (u16),
        flags: (DecodeFlags::DEFAULT_TO_64BIT | DecodeFlags::OPERATION_OP_SIZE) as u16,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: InstructionOperation::SAHF as (u16),
        flags: 0,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: InstructionOperation::LAHF as (u16),
        flags: 0,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: DecodeFlags::BYTE as u16,
        func: decode_eax_addr,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: 0,
        func: decode_eax_addr,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::FLIP_OPERANDS) as u16,
        func: decode_eax_addr,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: DecodeFlags::FLIP_OPERANDS as u16,
        func: decode_eax_addr,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOVSB as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::OPERATION_OP_SIZE | DecodeFlags::REP) as u16,
        func: decode_edi_esi,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOVSW as (u16),
        flags: (DecodeFlags::OPERATION_OP_SIZE | DecodeFlags::REP) as u16,
        func: decode_edi_esi,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMPSB as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::FLIP_OPERANDS |
                    DecodeFlags::OPERATION_OP_SIZE | DecodeFlags::REP_COND) as u16,
        func: decode_edi_esi,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMPSW as (u16),
        flags: (DecodeFlags::FLIP_OPERANDS | DecodeFlags::OPERATION_OP_SIZE |
                    DecodeFlags::REP_COND) as u16,
        func: decode_edi_esi,
    },
    InstructionEncoding {
        operation: InstructionOperation::TEST as (u16),
        flags: DecodeFlags::BYTE as u16,
        func: decode_eax_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::TEST as (u16),
        flags: 0,
        func: decode_eax_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::STOSB as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::OPERATION_OP_SIZE | DecodeFlags::REP) as u16,
        func: decode_edi_eax,
    },
    InstructionEncoding {
        operation: InstructionOperation::STOSW as (u16),
        flags: (DecodeFlags::OPERATION_OP_SIZE | DecodeFlags::REP) as u16,
        func: decode_edi_eax,
    },
    InstructionEncoding {
        operation: InstructionOperation::LODSB as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::OPERATION_OP_SIZE | DecodeFlags::REP) as u16,
        func: decode_eax_esi,
    },
    InstructionEncoding {
        operation: InstructionOperation::LODSW as (u16),
        flags: (DecodeFlags::OPERATION_OP_SIZE | DecodeFlags::REP) as u16,
        func: decode_eax_esi,
    },
    InstructionEncoding {
        operation: InstructionOperation::SCASB as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::FLIP_OPERANDS |
                    DecodeFlags::OPERATION_OP_SIZE | DecodeFlags::REP_COND) as u16,
        func: decode_edi_eax,
    },
    InstructionEncoding {
        operation: InstructionOperation::SCASW as (u16),
        flags: (DecodeFlags::FLIP_OPERANDS | DecodeFlags::OPERATION_OP_SIZE |
                    DecodeFlags::REP_COND) as u16,
        func: decode_edi_eax,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: DecodeFlags::BYTE as u16,
        func: decode_op_reg_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: DecodeFlags::BYTE as u16,
        func: decode_op_reg_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: DecodeFlags::BYTE as u16,
        func: decode_op_reg_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: DecodeFlags::BYTE as u16,
        func: decode_op_reg_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: DecodeFlags::BYTE as u16,
        func: decode_op_reg_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: DecodeFlags::BYTE as u16,
        func: decode_op_reg_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: DecodeFlags::BYTE as u16,
        func: decode_op_reg_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: DecodeFlags::BYTE as u16,
        func: decode_op_reg_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: 0,
        func: decode_op_reg_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: 0,
        func: decode_op_reg_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: 0,
        func: decode_op_reg_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: 0,
        func: decode_op_reg_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: 0,
        func: decode_op_reg_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: 0,
        func: decode_op_reg_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: 0,
        func: decode_op_reg_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: 0,
        func: decode_op_reg_imm,
    },
    InstructionEncoding {
        operation: 1u16,
        flags: DecodeFlags::BYTE as u16,
        func: decode_group_rm_imm,
    },
    InstructionEncoding {
        operation: 1u16,
        flags: 0,
        func: decode_group_rm_imm_8v,
    },
    InstructionEncoding {
        operation: InstructionOperation::RETN as (u16),
        flags: DecodeFlags::FORCE_16BIT as u16,
        func: decode_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::RETN as (u16),
        flags: 0,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: InstructionOperation::LES as (u16),
        flags: DecodeFlags::REG_RM_FAR_SIZE as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::LDS as (u16),
        flags: DecodeFlags::REG_RM_FAR_SIZE as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: 2u16,
        flags: DecodeFlags::BYTE as u16,
        func: decode_group_rm_imm,
    },
    InstructionEncoding {
        operation: 2u16,
        flags: 0,
        func: decode_group_rm_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::ENTER as (u16),
        flags: 0,
        func: decode_imm_16_imm_8,
    },
    InstructionEncoding {
        operation: InstructionOperation::LEAVE as (u16),
        flags: 0,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: InstructionOperation::RETF as (u16),
        flags: DecodeFlags::FORCE_16BIT as u16,
        func: decode_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::RETF as (u16),
        flags: 0,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: InstructionOperation::INT3 as (u16),
        flags: 0,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: InstructionOperation::INT as (u16),
        flags: DecodeFlags::BYTE as u16,
        func: decode_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::INTO as (u16),
        flags: 0,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: InstructionOperation::IRET as (u16),
        flags: 0,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: 1u16,
        flags: DecodeFlags::BYTE as u16,
        func: decode_group_rm_one,
    },
    InstructionEncoding {
        operation: 1u16,
        flags: 0,
        func: decode_group_rm_one,
    },
    InstructionEncoding {
        operation: 1u16,
        flags: DecodeFlags::BYTE as u16,
        func: decode_group_rm_cl,
    },
    InstructionEncoding {
        operation: 1u16,
        flags: 0,
        func: decode_group_rm_cl,
    },
    InstructionEncoding {
        operation: InstructionOperation::AAM as (u16),
        flags: DecodeFlags::BYTE as u16,
        func: decode_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::AAD as (u16),
        flags: DecodeFlags::BYTE as u16,
        func: decode_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0,
        func: invalid_decode,
    },
    InstructionEncoding {
        operation: InstructionOperation::XLAT as (u16),
        flags: 0,
        func: decode_al_ebx_al,
    },
    InstructionEncoding {
        operation: 0u16,
        flags: 0,
        func: decode_fpu,
    },
    InstructionEncoding {
        operation: 1u16,
        flags: 0,
        func: decode_fpu,
    },
    InstructionEncoding {
        operation: 2u16,
        flags: 0,
        func: decode_fpu,
    },
    InstructionEncoding {
        operation: 3u16,
        flags: 0,
        func: decode_fpu,
    },
    InstructionEncoding {
        operation: 4u16,
        flags: 0,
        func: decode_fpu,
    },
    InstructionEncoding {
        operation: 5u16,
        flags: 0,
        func: decode_fpu,
    },
    InstructionEncoding {
        operation: 6u16,
        flags: 0,
        func: decode_fpu,
    },
    InstructionEncoding {
        operation: 7u16,
        flags: 0,
        func: decode_fpu,
    },
    InstructionEncoding {
        operation: InstructionOperation::LOOPNE as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::DEFAULT_TO_64BIT) as u16,
        func: decode_rel_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::LOOPE as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::DEFAULT_TO_64BIT) as u16,
        func: decode_rel_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::LOOP as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::DEFAULT_TO_64BIT) as u16,
        func: decode_rel_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JCXZ as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::DEFAULT_TO_64BIT) as u16,
        func: decode_rel_imm_addr_size,
    },
    InstructionEncoding {
        operation: InstructionOperation::IN as (u16),
        flags: DecodeFlags::BYTE as u16,
        func: decode_eax_imm_8,
    },
    InstructionEncoding {
        operation: InstructionOperation::IN as (u16),
        flags: 0,
        func: decode_eax_imm_8,
    },
    InstructionEncoding {
        operation: InstructionOperation::OUT as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::FLIP_OPERANDS) as u16,
        func: decode_eax_imm_8,
    },
    InstructionEncoding {
        operation: InstructionOperation::OUT as (u16),
        flags: DecodeFlags::FLIP_OPERANDS as u16,
        func: decode_eax_imm_8,
    },
    InstructionEncoding {
        operation: InstructionOperation::CALL as (u16),
        flags: DecodeFlags::DEFAULT_TO_64BIT as u16,
        func: decode_rel_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JMP as (u16),
        flags: DecodeFlags::DEFAULT_TO_64BIT as u16,
        func: decode_rel_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JMPF as (u16),
        flags: DecodeFlags::INVALID_IN_64BIT as u16,
        func: decode_far_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JMP as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::DEFAULT_TO_64BIT) as u16,
        func: decode_rel_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::IN as (u16),
        flags: DecodeFlags::BYTE as u16,
        func: decode_eax_dx,
    },
    InstructionEncoding {
        operation: InstructionOperation::IN as (u16),
        flags: 0,
        func: decode_eax_dx,
    },
    InstructionEncoding {
        operation: InstructionOperation::OUT as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::FLIP_OPERANDS) as u16,
        func: decode_eax_dx,
    },
    InstructionEncoding {
        operation: InstructionOperation::OUT as (u16),
        flags: DecodeFlags::FLIP_OPERANDS as u16,
        func: decode_eax_dx,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0,
        func: invalid_decode,
    },
    InstructionEncoding {
        operation: InstructionOperation::INT1 as (u16),
        flags: 0,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0,
        func: invalid_decode,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0,
        func: invalid_decode,
    },
    InstructionEncoding {
        operation: InstructionOperation::HLT as (u16),
        flags: 0,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMC as (u16),
        flags: 0,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: 3u16,
        flags: (DecodeFlags::BYTE | DecodeFlags::LOCK) as u16,
        func: decode_group_f6f7,
    },
    InstructionEncoding {
        operation: 3u16,
        flags: DecodeFlags::LOCK as u16,
        func: decode_group_f6f7,
    },
    InstructionEncoding {
        operation: InstructionOperation::CLC as (u16),
        flags: 0,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: InstructionOperation::STC as (u16),
        flags: 0,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: InstructionOperation::CLI as (u16),
        flags: 0,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: InstructionOperation::STI as (u16),
        flags: 0,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: InstructionOperation::CLD as (u16),
        flags: 0,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: InstructionOperation::STD as (u16),
        flags: 0,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: 4u16,
        flags: (DecodeFlags::BYTE | DecodeFlags::LOCK) as u16,
        func: decode_group_rm,
    },
    InstructionEncoding {
        operation: 5u16,
        flags: DecodeFlags::LOCK as u16,
        func: decode_group_ff,
    },
];

static TWO_BYTE_OPCODE_MAP: [InstructionEncoding; 256] = [
    InstructionEncoding {
        operation: 6u16,
        flags: 0,
        func: decode_group_0f00,
    },
    InstructionEncoding {
        operation: 7u16,
        flags: 0,
        func: decode_group_0f01,
    },
    InstructionEncoding {
        operation: InstructionOperation::LAR as (u16),
        flags: 0,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::LSL as (u16),
        flags: 0,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0,
        func: invalid_decode,
    },
    InstructionEncoding {
        operation: InstructionOperation::SYSCALL as (u16),
        flags: 0,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: InstructionOperation::CLTS as (u16),
        flags: 0,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: InstructionOperation::SYSRET as (u16),
        flags: 0,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVD as (u16),
        flags: 0,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: InstructionOperation::WBINVD as (u16),
        flags: 0,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0,
        func: invalid_decode,
    },
    InstructionEncoding {
        operation: InstructionOperation::UD2 as (u16),
        flags: 0,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0,
        func: invalid_decode,
    },
    InstructionEncoding {
        operation: 8u16,
        flags: DecodeFlags::REG_RM_NO_SIZE as u16,
        func: decode_group_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::FEMMS as (u16),
        flags: 0,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: 0u16,
        flags: 0,
        func: decode_3dnow,
    },
    InstructionEncoding {
        operation: 0u16,
        flags: 0,
        func: decode_sse_table,
    },
    InstructionEncoding {
        operation: 0u16,
        flags: DecodeFlags::FLIP_OPERANDS as u16,
        func: decode_sse_table,
    },
    InstructionEncoding {
        operation: 1u16,
        flags: 0,
        func: decode_sse_table,
    },
    InstructionEncoding {
        operation: 2u16,
        flags: DecodeFlags::FLIP_OPERANDS as u16,
        func: decode_sse_table,
    },
    InstructionEncoding {
        operation: 3u16,
        flags: 0,
        func: decode_sse_table,
    },
    InstructionEncoding {
        operation: 4u16,
        flags: 0,
        func: decode_sse_table,
    },
    InstructionEncoding {
        operation: 5u16,
        flags: 0,
        func: decode_sse_table,
    },
    InstructionEncoding {
        operation: 6u16,
        flags: DecodeFlags::FLIP_OPERANDS as u16,
        func: decode_sse_table,
    },
    InstructionEncoding {
        operation: 9u16,
        flags: DecodeFlags::REG_RM_NO_SIZE as u16,
        func: decode_group_rm,
    },
    InstructionEncoding {
        operation: 10u16,
        flags: DecodeFlags::REG_RM_NO_SIZE as u16,
        func: decode_group_rm,
    },
    InstructionEncoding {
        operation: 10u16,
        flags: DecodeFlags::REG_RM_NO_SIZE as u16,
        func: decode_group_rm,
    },
    InstructionEncoding {
        operation: 10u16,
        flags: DecodeFlags::REG_RM_NO_SIZE as u16,
        func: decode_group_rm,
    },
    InstructionEncoding {
        operation: 10u16,
        flags: DecodeFlags::REG_RM_NO_SIZE as u16,
        func: decode_group_rm,
    },
    InstructionEncoding {
        operation: 10u16,
        flags: DecodeFlags::REG_RM_NO_SIZE as u16,
        func: decode_group_rm,
    },
    InstructionEncoding {
        operation: 10u16,
        flags: DecodeFlags::REG_RM_NO_SIZE as u16,
        func: decode_group_rm,
    },
    InstructionEncoding {
        operation: 10u16,
        flags: DecodeFlags::REG_RM_NO_SIZE as u16,
        func: decode_group_rm,
    },
    InstructionEncoding {
        operation: OperandType::REG_CR0 as (u16),
        flags: (DecodeFlags::DEFAULT_TO_64BIT | DecodeFlags::LOCK) as u16,
        func: decode_reg_cr,
    },
    InstructionEncoding {
        operation: OperandType::REG_DR0 as (u16),
        flags: (DecodeFlags::DEFAULT_TO_64BIT | DecodeFlags::LOCK) as u16,
        func: decode_reg_cr,
    },
    InstructionEncoding {
        operation: OperandType::REG_CR0 as (u16),
        flags: (DecodeFlags::FLIP_OPERANDS | DecodeFlags::DEFAULT_TO_64BIT | DecodeFlags::LOCK) as
            u16,
        func: decode_reg_cr,
    },
    InstructionEncoding {
        operation: OperandType::REG_DR0 as (u16),
        flags: (DecodeFlags::FLIP_OPERANDS | DecodeFlags::DEFAULT_TO_64BIT | DecodeFlags::LOCK) as
            u16,
        func: decode_reg_cr,
    },
    InstructionEncoding {
        operation: OperandType::REG_TR0 as (u16),
        flags: (DecodeFlags::DEFAULT_TO_64BIT | DecodeFlags::LOCK) as u16,
        func: decode_reg_cr,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0,
        func: invalid_decode,
    },
    InstructionEncoding {
        operation: OperandType::REG_TR0 as (u16),
        flags: (DecodeFlags::FLIP_OPERANDS | DecodeFlags::DEFAULT_TO_64BIT | DecodeFlags::LOCK) as
            u16,
        func: decode_reg_cr,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0,
        func: invalid_decode,
    },
    InstructionEncoding {
        operation: 7u16,
        flags: 0,
        func: decode_sse_table,
    },
    InstructionEncoding {
        operation: 7u16,
        flags: DecodeFlags::FLIP_OPERANDS as u16,
        func: decode_sse_table,
    },
    InstructionEncoding {
        operation: 8u16,
        flags: 0,
        func: decode_sse_table,
    },
    InstructionEncoding {
        operation: 9u16,
        flags: DecodeFlags::FLIP_OPERANDS as u16,
        func: decode_sse_table,
    },
    InstructionEncoding {
        operation: 10u16,
        flags: 0,
        func: decode_sse_table,
    },
    InstructionEncoding {
        operation: 11u16,
        flags: 0,
        func: decode_sse_table,
    },
    InstructionEncoding {
        operation: 12u16,
        flags: 0,
        func: decode_sse_table,
    },
    InstructionEncoding {
        operation: 13u16,
        flags: 0,
        func: decode_sse_table,
    },
    InstructionEncoding {
        operation: InstructionOperation::WRMSR as (u16),
        flags: 0,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: InstructionOperation::RDTSC as (u16),
        flags: 0,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: InstructionOperation::RDMSR as (u16),
        flags: 0,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: InstructionOperation::RDPMC as (u16),
        flags: 0,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: InstructionOperation::SYSENTER as (u16),
        flags: 0,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: InstructionOperation::SYSEXIT as (u16),
        flags: 0,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0,
        func: invalid_decode,
    },
    InstructionEncoding {
        operation: InstructionOperation::GETSEC as (u16),
        flags: 0,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0,
        func: invalid_decode,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0,
        func: invalid_decode,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0,
        func: invalid_decode,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0,
        func: invalid_decode,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0,
        func: invalid_decode,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0,
        func: invalid_decode,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0,
        func: invalid_decode,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0,
        func: invalid_decode,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMOVO as (u16),
        flags: 0,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMOVNO as (u16),
        flags: 0,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMOVB as (u16),
        flags: 0,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMOVAE as (u16),
        flags: 0,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMOVE as (u16),
        flags: 0,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMOVNE as (u16),
        flags: 0,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMOVBE as (u16),
        flags: 0,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMOVA as (u16),
        flags: 0,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMOVS as (u16),
        flags: 0,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMOVNS as (u16),
        flags: 0,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMOVPE as (u16),
        flags: 0,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMOVPO as (u16),
        flags: 0,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMOVL as (u16),
        flags: 0,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMOVGE as (u16),
        flags: 0,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMOVLE as (u16),
        flags: 0,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMOVG as (u16),
        flags: 0,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: 14u16,
        flags: 0,
        func: decode_sse_table,
    },
    InstructionEncoding {
        operation: InstructionOperation::SQRTPS as (u16),
        flags: 0,
        func: decode_sse,
    },
    InstructionEncoding {
        operation: InstructionOperation::RSQRTPS as (u16),
        flags: 0,
        func: decode_sse_single,
    },
    InstructionEncoding {
        operation: InstructionOperation::RCPPS as (u16),
        flags: 0,
        func: decode_sse_single,
    },
    InstructionEncoding {
        operation: InstructionOperation::ANDPS as (u16),
        flags: 0,
        func: decode_sse_packed,
    },
    InstructionEncoding {
        operation: InstructionOperation::ANDNPS as (u16),
        flags: 0,
        func: decode_sse_packed,
    },
    InstructionEncoding {
        operation: InstructionOperation::ORPS as (u16),
        flags: 0,
        func: decode_sse_packed,
    },
    InstructionEncoding {
        operation: InstructionOperation::XORPS as (u16),
        flags: 0,
        func: decode_sse_packed,
    },
    InstructionEncoding {
        operation: InstructionOperation::ADDPS as (u16),
        flags: 0,
        func: decode_sse,
    },
    InstructionEncoding {
        operation: InstructionOperation::MULPS as (u16),
        flags: 0,
        func: decode_sse,
    },
    InstructionEncoding {
        operation: 15u16,
        flags: 0,
        func: decode_sse_table,
    },
    InstructionEncoding {
        operation: 16u16,
        flags: 0,
        func: decode_sse_table,
    },
    InstructionEncoding {
        operation: InstructionOperation::SUBPS as (u16),
        flags: 0,
        func: decode_sse,
    },
    InstructionEncoding {
        operation: InstructionOperation::MINPS as (u16),
        flags: 0,
        func: decode_sse,
    },
    InstructionEncoding {
        operation: InstructionOperation::DIVPS as (u16),
        flags: 0,
        func: decode_sse,
    },
    InstructionEncoding {
        operation: InstructionOperation::MAXPS as (u16),
        flags: 0,
        func: decode_sse,
    },
    InstructionEncoding {
        operation: 17u16,
        flags: 0,
        func: decode_sse_table,
    },
    InstructionEncoding {
        operation: 18u16,
        flags: 0,
        func: decode_sse_table,
    },
    InstructionEncoding {
        operation: 19u16,
        flags: 0,
        func: decode_sse_table,
    },
    InstructionEncoding {
        operation: InstructionOperation::PACKSSWB as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PCMPGTB as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PCMPGTW as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PCMPGTD as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PACKUSWB as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUNPCKHBW as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUNPCKHWD as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUNPCKHDQ as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PACKSSDW as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUNPCKLQDQ as (u16),
        flags: 0,
        func: decode_mmx_sse_only,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUNPCKHQDQ as (u16),
        flags: 0,
        func: decode_mmx_sse_only,
    },
    InstructionEncoding {
        operation: 20u16,
        flags: DecodeFlags::INC_OPERATION_FOR_64 as u16,
        func: decode_sse_table,
    },
    InstructionEncoding {
        operation: 21u16,
        flags: 0,
        func: decode_sse_table,
    },
    InstructionEncoding {
        operation: 22u16,
        flags: 0,
        func: decode_see_table_imm_8,
    },
    InstructionEncoding {
        operation: 0u16,
        flags: 0,
        func: decode_mmx_group,
    },
    InstructionEncoding {
        operation: 1u16,
        flags: 0,
        func: decode_mmx_group,
    },
    InstructionEncoding {
        operation: 2u16,
        flags: 0,
        func: decode_mmx_group,
    },
    InstructionEncoding {
        operation: InstructionOperation::PCMPEQB as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PCMPEQW as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PCMPEQD as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::EMMS as (u16),
        flags: 0,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: InstructionOperation::VMREAD as (u16),
        flags: (DecodeFlags::FLIP_OPERANDS | DecodeFlags::DEFAULT_TO_64BIT) as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::VMWRITE as (u16),
        flags: (DecodeFlags::FLIP_OPERANDS | DecodeFlags::DEFAULT_TO_64BIT) as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0,
        func: invalid_decode,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0,
        func: invalid_decode,
    },
    InstructionEncoding {
        operation: 23u16,
        flags: 0,
        func: decode_sse_table,
    },
    InstructionEncoding {
        operation: 24u16,
        flags: 0,
        func: decode_sse_table,
    },
    InstructionEncoding {
        operation: 25u16,
        flags: (DecodeFlags::INC_OPERATION_FOR_64 | DecodeFlags::FLIP_OPERANDS) as u16,
        func: decode_sse_table,
    },
    InstructionEncoding {
        operation: 21u16,
        flags: DecodeFlags::FLIP_OPERANDS as u16,
        func: decode_sse_table,
    },
    InstructionEncoding {
        operation: InstructionOperation::JO as (u16),
        flags: DecodeFlags::DEFAULT_TO_64BIT as u16,
        func: decode_rel_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JNO as (u16),
        flags: DecodeFlags::DEFAULT_TO_64BIT as u16,
        func: decode_rel_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JB as (u16),
        flags: DecodeFlags::DEFAULT_TO_64BIT as u16,
        func: decode_rel_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JAE as (u16),
        flags: DecodeFlags::DEFAULT_TO_64BIT as u16,
        func: decode_rel_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JE as (u16),
        flags: DecodeFlags::DEFAULT_TO_64BIT as u16,
        func: decode_rel_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JNE as (u16),
        flags: DecodeFlags::DEFAULT_TO_64BIT as u16,
        func: decode_rel_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JBE as (u16),
        flags: DecodeFlags::DEFAULT_TO_64BIT as u16,
        func: decode_rel_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JA as (u16),
        flags: DecodeFlags::DEFAULT_TO_64BIT as u16,
        func: decode_rel_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JS as (u16),
        flags: DecodeFlags::DEFAULT_TO_64BIT as u16,
        func: decode_rel_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JNS as (u16),
        flags: DecodeFlags::DEFAULT_TO_64BIT as u16,
        func: decode_rel_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JPE as (u16),
        flags: DecodeFlags::DEFAULT_TO_64BIT as u16,
        func: decode_rel_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JPO as (u16),
        flags: DecodeFlags::DEFAULT_TO_64BIT as u16,
        func: decode_rel_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JL as (u16),
        flags: DecodeFlags::DEFAULT_TO_64BIT as u16,
        func: decode_rel_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JGE as (u16),
        flags: DecodeFlags::DEFAULT_TO_64BIT as u16,
        func: decode_rel_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JLE as (u16),
        flags: DecodeFlags::DEFAULT_TO_64BIT as u16,
        func: decode_rel_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JG as (u16),
        flags: DecodeFlags::DEFAULT_TO_64BIT as u16,
        func: decode_rel_imm,
    },
    InstructionEncoding {
        operation: InstructionOperation::SETO as (u16),
        flags: 0,
        func: decode_rm8,
    },
    InstructionEncoding {
        operation: InstructionOperation::SETNO as (u16),
        flags: 0,
        func: decode_rm8,
    },
    InstructionEncoding {
        operation: InstructionOperation::SETB as (u16),
        flags: 0,
        func: decode_rm8,
    },
    InstructionEncoding {
        operation: InstructionOperation::SETAE as (u16),
        flags: 0,
        func: decode_rm8,
    },
    InstructionEncoding {
        operation: InstructionOperation::SETE as (u16),
        flags: 0,
        func: decode_rm8,
    },
    InstructionEncoding {
        operation: InstructionOperation::SETNE as (u16),
        flags: 0,
        func: decode_rm8,
    },
    InstructionEncoding {
        operation: InstructionOperation::SETBE as (u16),
        flags: 0,
        func: decode_rm8,
    },
    InstructionEncoding {
        operation: InstructionOperation::SETA as (u16),
        flags: 0,
        func: decode_rm8,
    },
    InstructionEncoding {
        operation: InstructionOperation::SETS as (u16),
        flags: 0,
        func: decode_rm8,
    },
    InstructionEncoding {
        operation: InstructionOperation::SETNS as (u16),
        flags: 0,
        func: decode_rm8,
    },
    InstructionEncoding {
        operation: InstructionOperation::SETPE as (u16),
        flags: 0,
        func: decode_rm8,
    },
    InstructionEncoding {
        operation: InstructionOperation::SETPO as (u16),
        flags: 0,
        func: decode_rm8,
    },
    InstructionEncoding {
        operation: InstructionOperation::SETL as (u16),
        flags: 0,
        func: decode_rm8,
    },
    InstructionEncoding {
        operation: InstructionOperation::SETGE as (u16),
        flags: 0,
        func: decode_rm8,
    },
    InstructionEncoding {
        operation: InstructionOperation::SETLE as (u16),
        flags: 0,
        func: decode_rm8,
    },
    InstructionEncoding {
        operation: InstructionOperation::SETG as (u16),
        flags: 0,
        func: decode_rm8,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUSH as (u16),
        flags: 0,
        func: decode_push_pop_seg,
    },
    InstructionEncoding {
        operation: InstructionOperation::POP as (u16),
        flags: 0,
        func: decode_push_pop_seg,
    },
    InstructionEncoding {
        operation: InstructionOperation::CPUID as (u16),
        flags: 0,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: InstructionOperation::BT as (u16),
        flags: DecodeFlags::FLIP_OPERANDS as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::SHLD as (u16),
        flags: 0,
        func: decode_rm_reg_imm_8,
    },
    InstructionEncoding {
        operation: InstructionOperation::SHLD as (u16),
        flags: 0,
        func: decode_rm_reg_cl,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0,
        func: invalid_decode,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0,
        func: invalid_decode,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUSH as (u16),
        flags: 0,
        func: decode_push_pop_seg,
    },
    InstructionEncoding {
        operation: InstructionOperation::POP as (u16),
        flags: 0,
        func: decode_push_pop_seg,
    },
    InstructionEncoding {
        operation: InstructionOperation::RSM as (u16),
        flags: 0,
        func: decode_no_operands,
    },
    InstructionEncoding {
        operation: InstructionOperation::BTS as (u16),
        flags: DecodeFlags::FLIP_OPERANDS as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::SHRD as (u16),
        flags: 0,
        func: decode_rm_reg_imm_8,
    },
    InstructionEncoding {
        operation: InstructionOperation::SHRD as (u16),
        flags: 0,
        func: decode_rm_reg_cl,
    },
    InstructionEncoding {
        operation: 24u16,
        flags: 0,
        func: decode_group_0fae,
    },
    InstructionEncoding {
        operation: InstructionOperation::IMUL as (u16),
        flags: 0,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMPXCHG as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::FLIP_OPERANDS | DecodeFlags::LOCK) as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMPXCHG as (u16),
        flags: (DecodeFlags::FLIP_OPERANDS | DecodeFlags::LOCK) as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::LSS as (u16),
        flags: DecodeFlags::REG_RM_FAR_SIZE as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::BTR as (u16),
        flags: DecodeFlags::FLIP_OPERANDS as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::LFS as (u16),
        flags: DecodeFlags::REG_RM_FAR_SIZE as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::LGS as (u16),
        flags: DecodeFlags::REG_RM_FAR_SIZE as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOVZX as (u16),
        flags: 0,
        func: decode_mov_sxzx8,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOVZX as (u16),
        flags: 0,
        func: decode_mov_sxzx16,
    },
    InstructionEncoding {
        operation: InstructionOperation::POPCNT as (u16),
        flags: 0,
        func: decode_0fb8,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0,
        func: invalid_decode,
    },
    InstructionEncoding {
        operation: 11u16,
        flags: 0,
        func: decode_group_rm_imm_8v,
    },
    InstructionEncoding {
        operation: InstructionOperation::BTC as (u16),
        flags: DecodeFlags::FLIP_OPERANDS as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::BSF as (u16),
        flags: 0,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::BSR as (u16),
        flags: 0,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOVSX as (u16),
        flags: 0,
        func: decode_mov_sxzx8,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOVSX as (u16),
        flags: 0,
        func: decode_mov_sxzx16,
    },
    InstructionEncoding {
        operation: InstructionOperation::XADD as (u16),
        flags: (DecodeFlags::BYTE | DecodeFlags::FLIP_OPERANDS) as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: InstructionOperation::XADD as (u16),
        flags: DecodeFlags::FLIP_OPERANDS as u16,
        func: decode_reg_rm,
    },
    InstructionEncoding {
        operation: 26u16,
        flags: 0,
        func: decode_see_table_imm_8,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOVNTI as (u16),
        flags: 0,
        func: decode_mov_nti,
    },
    InstructionEncoding {
        operation: 27u16,
        flags: 0,
        func: decode_pinsrw,
    },
    InstructionEncoding {
        operation: 28u16,
        flags: DecodeFlags::FLIP_OPERANDS as u16,
        func: decode_see_table_imm_8,
    },
    InstructionEncoding {
        operation: 29u16,
        flags: 0,
        func: decode_see_table_imm_8,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMPXCH8B as (u16),
        flags: 0,
        func: decode_cmp_xch_8b,
    },
    InstructionEncoding {
        operation: InstructionOperation::BSWAP as (u16),
        flags: 0,
        func: decode_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::BSWAP as (u16),
        flags: 0,
        func: decode_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::BSWAP as (u16),
        flags: 0,
        func: decode_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::BSWAP as (u16),
        flags: 0,
        func: decode_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::BSWAP as (u16),
        flags: 0,
        func: decode_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::BSWAP as (u16),
        flags: 0,
        func: decode_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::BSWAP as (u16),
        flags: 0,
        func: decode_op_reg,
    },
    InstructionEncoding {
        operation: InstructionOperation::BSWAP as (u16),
        flags: 0,
        func: decode_op_reg,
    },
    InstructionEncoding {
        operation: 30u16,
        flags: 0,
        func: decode_sse_table,
    },
    InstructionEncoding {
        operation: InstructionOperation::PSRLW as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PSRLD as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PSRLQ as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PADDQ as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PMULLW as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: 31u16,
        flags: 0,
        func: decode_sse_table,
    },
    InstructionEncoding {
        operation: 32u16,
        flags: 0,
        func: decode_sse_table,
    },
    InstructionEncoding {
        operation: InstructionOperation::PSUBUSB as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PSUBUSW as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PMINUB as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PAND as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PADDUSB as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PADDUSW as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PMAXUB as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PANDN as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PAVGB as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PSRAW as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PSRAD as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PAVGW as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PMULHUW as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PMULHW as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: 33u16,
        flags: 0,
        func: decode_sse_table,
    },
    InstructionEncoding {
        operation: 34u16,
        flags: DecodeFlags::FLIP_OPERANDS as u16,
        func: decode_sse_table,
    },
    InstructionEncoding {
        operation: InstructionOperation::PSUBSB as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PSUBSW as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PMINSW as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::POR as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PADDSB as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PADDSW as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PMAXSW as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PXOR as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: 35u16,
        flags: 0,
        func: decode_sse_table,
    },
    InstructionEncoding {
        operation: InstructionOperation::PSLLW as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PSLLD as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PSLLQ as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PMULUDQ as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PMADDWD as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PSADBW as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: 36u16,
        flags: 0,
        func: decode_sse_table,
    },
    InstructionEncoding {
        operation: InstructionOperation::PSUBB as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PSUBW as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PSUBD as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PSUBQ as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PADDB as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PADDW as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::PADDD as (u16),
        flags: 0,
        func: decode_mmx,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0,
        func: invalid_decode,
    },
];

#[repr(C)]
struct SparseInstructionEncoding {
    pub opcode: u8,
    pub encoding: InstructionEncoding,
}

static THREE_BYTE_0F38_MAP: [SparseInstructionEncoding; 48] = [
    SparseInstructionEncoding {
        opcode: 0x0u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PSHUFB as (u16),
            flags: 0,
            func: decode_mmx,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x1u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PHADDW as (u16),
            flags: 0,
            func: decode_mmx,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x2u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PHADDD as (u16),
            flags: 0,
            func: decode_mmx,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x3u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PHADDSW as (u16),
            flags: 0,
            func: decode_mmx,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x4u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PMADDUBSW as (u16),
            flags: 0,
            func: decode_mmx,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x5u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PHSUBW as (u16),
            flags: 0,
            func: decode_mmx,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x6u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PHSUBD as (u16),
            flags: 0,
            func: decode_mmx,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x7u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PHSUBSW as (u16),
            flags: 0,
            func: decode_mmx,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x8u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PSIGNB as (u16),
            flags: 0,
            func: decode_mmx,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x9u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PSIGNW as (u16),
            flags: 0,
            func: decode_mmx,
        },
    },
    SparseInstructionEncoding {
        opcode: 0xau8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PSIGND as (u16),
            flags: 0,
            func: decode_mmx,
        },
    },
    SparseInstructionEncoding {
        opcode: 0xbu8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PMULHRSW as (u16),
            flags: 0,
            func: decode_mmx,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x10u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PBLENDVB as (u16),
            flags: 0,
            func: decode_mmx_sse_only,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x14u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::BLENDVPS as (u16),
            flags: 0,
            func: decode_mmx_sse_only,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x15u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::BLENDVPD as (u16),
            flags: 0,
            func: decode_mmx_sse_only,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x17u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PTEST as (u16),
            flags: 0,
            func: decode_mmx_sse_only,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x1cu8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PABSB as (u16),
            flags: 0,
            func: decode_mmx,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x1du8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PABSW as (u16),
            flags: 0,
            func: decode_mmx,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x1eu8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PABSD as (u16),
            flags: 0,
            func: decode_mmx,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x20u8,
        encoding: InstructionEncoding {
            operation: 37u16,
            flags: 0,
            func: decode_sse_table,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x21u8,
        encoding: InstructionEncoding {
            operation: 38u16,
            flags: 0,
            func: decode_sse_table,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x22u8,
        encoding: InstructionEncoding {
            operation: 39u16,
            flags: 0,
            func: decode_sse_table,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x23u8,
        encoding: InstructionEncoding {
            operation: 40u16,
            flags: 0,
            func: decode_sse_table,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x24u8,
        encoding: InstructionEncoding {
            operation: 41u16,
            flags: 0,
            func: decode_sse_table,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x25u8,
        encoding: InstructionEncoding {
            operation: 42u16,
            flags: 0,
            func: decode_sse_table,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x28u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PMULDQ as (u16),
            flags: 0,
            func: decode_mmx_sse_only,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x29u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PCMPEQQ as (u16),
            flags: 0,
            func: decode_mmx_sse_only,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x2au8,
        encoding: InstructionEncoding {
            operation: 43u16,
            flags: 0,
            func: decode_sse_table,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x2bu8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PACKUSDW as (u16),
            flags: 0,
            func: decode_mmx_sse_only,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x30u8,
        encoding: InstructionEncoding {
            operation: 44u16,
            flags: 0,
            func: decode_sse_table,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x31u8,
        encoding: InstructionEncoding {
            operation: 45u16,
            flags: 0,
            func: decode_sse_table,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x32u8,
        encoding: InstructionEncoding {
            operation: 46u16,
            flags: 0,
            func: decode_sse_table,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x33u8,
        encoding: InstructionEncoding {
            operation: 47u16,
            flags: 0,
            func: decode_sse_table,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x34u8,
        encoding: InstructionEncoding {
            operation: 48u16,
            flags: 0,
            func: decode_sse_table,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x35u8,
        encoding: InstructionEncoding {
            operation: 49u16,
            flags: 0,
            func: decode_sse_table,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x37u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PCMPGTQ as (u16),
            flags: 0,
            func: decode_mmx_sse_only,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x38u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PMINSB as (u16),
            flags: 0,
            func: decode_mmx_sse_only,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x39u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PMINSD as (u16),
            flags: 0,
            func: decode_mmx_sse_only,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x3au8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PMINUW as (u16),
            flags: 0,
            func: decode_mmx_sse_only,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x3bu8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PMINUD as (u16),
            flags: 0,
            func: decode_mmx_sse_only,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x3cu8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PMAXSB as (u16),
            flags: 0,
            func: decode_mmx_sse_only,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x3du8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PMAXSD as (u16),
            flags: 0,
            func: decode_mmx_sse_only,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x3eu8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PMAXUW as (u16),
            flags: 0,
            func: decode_mmx_sse_only,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x3fu8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PMAXUD as (u16),
            flags: 0,
            func: decode_mmx_sse_only,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x40u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PMULLD as (u16),
            flags: 0,
            func: decode_mmx_sse_only,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x41u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PHMINPOSUW as (u16),
            flags: 0,
            func: decode_mmx_sse_only,
        },
    },
    SparseInstructionEncoding {
        opcode: 0xf0u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::CRC32 as (u16),
            flags: DecodeFlags::BYTE as u16,
            func: decode_crc_32,
        },
    },
    SparseInstructionEncoding {
        opcode: 0xf1u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::CRC32 as (u16),
            flags: 0,
            func: decode_crc_32,
        },
    },
];

static THREE_BYTE_0F3A_MAP: [SparseInstructionEncoding; 22] = [
    SparseInstructionEncoding {
        opcode: 0x8u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::ROUNDPS as (u16),
            flags: 0,
            func: decode_mmx_sse_only,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x9u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::ROUNDPD as (u16),
            flags: 0,
            func: decode_mmx_sse_only,
        },
    },
    SparseInstructionEncoding {
        opcode: 0xau8,
        encoding: InstructionEncoding {
            operation: 50u16,
            flags: 0,
            func: decode_sse_table,
        },
    },
    SparseInstructionEncoding {
        opcode: 0xbu8,
        encoding: InstructionEncoding {
            operation: 51u16,
            flags: 0,
            func: decode_sse_table,
        },
    },
    SparseInstructionEncoding {
        opcode: 0xcu8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::BLENDPS as (u16),
            flags: 0,
            func: decode_mmx_sse_only,
        },
    },
    SparseInstructionEncoding {
        opcode: 0xdu8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::BLENDPD as (u16),
            flags: 0,
            func: decode_mmx_sse_only,
        },
    },
    SparseInstructionEncoding {
        opcode: 0xeu8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PBLENDW as (u16),
            flags: 0,
            func: decode_mmx_sse_only,
        },
    },
    SparseInstructionEncoding {
        opcode: 0xfu8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PALIGNR as (u16),
            flags: 0,
            func: decode_mmx,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x14u8,
        encoding: InstructionEncoding {
            operation: 52u16,
            flags: DecodeFlags::FLIP_OPERANDS as u16,
            func: decode_sse_table_mem_8,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x15u8,
        encoding: InstructionEncoding {
            operation: 53u16,
            flags: DecodeFlags::FLIP_OPERANDS as u16,
            func: decode_sse_table,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x16u8,
        encoding: InstructionEncoding {
            operation: 54u16,
            flags: (DecodeFlags::INC_OPERATION_FOR_64 | DecodeFlags::FLIP_OPERANDS) as u16,
            func: decode_sse_table,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x17u8,
        encoding: InstructionEncoding {
            operation: 55u16,
            flags: DecodeFlags::FLIP_OPERANDS as u16,
            func: decode_sse_table,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x20u8,
        encoding: InstructionEncoding {
            operation: 56u16,
            flags: 0,
            func: decode_sse_table_mem_8,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x21u8,
        encoding: InstructionEncoding {
            operation: 57u16,
            flags: 0,
            func: decode_sse_table,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x22u8,
        encoding: InstructionEncoding {
            operation: 58u16,
            flags: DecodeFlags::INC_OPERATION_FOR_64 as u16,
            func: decode_sse_table,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x40u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::DPPS as (u16),
            flags: 0,
            func: decode_mmx_sse_only,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x41u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::DPPD as (u16),
            flags: 0,
            func: decode_mmx_sse_only,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x42u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::MPSADBW as (u16),
            flags: 0,
            func: decode_mmx_sse_only,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x60u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PCMPESTRM as (u16),
            flags: 0,
            func: decode_mmx_sse_only,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x61u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PCMPESTRI as (u16),
            flags: 0,
            func: decode_mmx_sse_only,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x62u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PCMPISTRM as (u16),
            flags: 0,
            func: decode_mmx_sse_only,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x63u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PCMPISTRI as (u16),
            flags: 0,
            func: decode_mmx_sse_only,
        },
    },
];

static FPU_MEM_OPCODE_MAP: [[InstructionEncoding; 8]; 8] = [
    [
        InstructionEncoding {
            operation: InstructionOperation::FADD as (u16),
            flags: 0,
            func: decode_mem_32,
        },
        InstructionEncoding {
            operation: InstructionOperation::FMUL as (u16),
            flags: 0,
            func: decode_mem_32,
        },
        InstructionEncoding {
            operation: InstructionOperation::FCOM as (u16),
            flags: 0,
            func: decode_mem_32,
        },
        InstructionEncoding {
            operation: InstructionOperation::FCOMP as (u16),
            flags: 0,
            func: decode_mem_32,
        },
        InstructionEncoding {
            operation: InstructionOperation::FSUB as (u16),
            flags: 0,
            func: decode_mem_32,
        },
        InstructionEncoding {
            operation: InstructionOperation::FSUBR as (u16),
            flags: 0,
            func: decode_mem_32,
        },
        InstructionEncoding {
            operation: InstructionOperation::FDIV as (u16),
            flags: 0,
            func: decode_mem_32,
        },
        InstructionEncoding {
            operation: InstructionOperation::FDIVR as (u16),
            flags: 0,
            func: decode_mem_32,
        },
    ],
    [
        InstructionEncoding {
            operation: InstructionOperation::FLD as (u16),
            flags: 0,
            func: decode_mem_32,
        },
        InstructionEncoding {
            operation: InstructionOperation::INVALID as (u16),
            flags: 0,
            func: invalid_decode,
        },
        InstructionEncoding {
            operation: InstructionOperation::FST as (u16),
            flags: 0,
            func: decode_mem_32,
        },
        InstructionEncoding {
            operation: InstructionOperation::FSTP as (u16),
            flags: 0,
            func: decode_mem_32,
        },
        InstructionEncoding {
            operation: InstructionOperation::FLDENV as (u16),
            flags: 0,
            func: decode_mem_float_env,
        },
        InstructionEncoding {
            operation: InstructionOperation::FLDCW as (u16),
            flags: 0,
            func: decode_mem_16,
        },
        InstructionEncoding {
            operation: InstructionOperation::FSTENV as (u16),
            flags: 0,
            func: decode_mem_float_env,
        },
        InstructionEncoding {
            operation: InstructionOperation::FSTCW as (u16),
            flags: 0,
            func: decode_mem_16,
        },
    ],
    [
        InstructionEncoding {
            operation: InstructionOperation::FIADD as (u16),
            flags: 0,
            func: decode_mem_32,
        },
        InstructionEncoding {
            operation: InstructionOperation::FIMUL as (u16),
            flags: 0,
            func: decode_mem_32,
        },
        InstructionEncoding {
            operation: InstructionOperation::FICOM as (u16),
            flags: 0,
            func: decode_mem_32,
        },
        InstructionEncoding {
            operation: InstructionOperation::FICOMP as (u16),
            flags: 0,
            func: decode_mem_32,
        },
        InstructionEncoding {
            operation: InstructionOperation::FISUB as (u16),
            flags: 0,
            func: decode_mem_32,
        },
        InstructionEncoding {
            operation: InstructionOperation::FISUBR as (u16),
            flags: 0,
            func: decode_mem_32,
        },
        InstructionEncoding {
            operation: InstructionOperation::FIDIV as (u16),
            flags: 0,
            func: decode_mem_32,
        },
        InstructionEncoding {
            operation: InstructionOperation::FIDIVR as (u16),
            flags: 0,
            func: decode_mem_32,
        },
    ],
    [
        InstructionEncoding {
            operation: InstructionOperation::FILD as (u16),
            flags: 0,
            func: decode_mem_32,
        },
        InstructionEncoding {
            operation: InstructionOperation::FISTTP as (u16),
            flags: 0,
            func: decode_mem_32,
        },
        InstructionEncoding {
            operation: InstructionOperation::FIST as (u16),
            flags: 0,
            func: decode_mem_32,
        },
        InstructionEncoding {
            operation: InstructionOperation::FISTP as (u16),
            flags: 0,
            func: decode_mem_32,
        },
        InstructionEncoding {
            operation: InstructionOperation::INVALID as (u16),
            flags: 0,
            func: invalid_decode,
        },
        InstructionEncoding {
            operation: InstructionOperation::FLD as (u16),
            flags: 0,
            func: decode_mem_80,
        },
        InstructionEncoding {
            operation: InstructionOperation::INVALID as (u16),
            flags: 0,
            func: invalid_decode,
        },
        InstructionEncoding {
            operation: InstructionOperation::FSTP as (u16),
            flags: 0,
            func: decode_mem_80,
        },
    ],
    [
        InstructionEncoding {
            operation: InstructionOperation::FADD as (u16),
            flags: 0,
            func: decode_mem_64,
        },
        InstructionEncoding {
            operation: InstructionOperation::FMUL as (u16),
            flags: 0,
            func: decode_mem_64,
        },
        InstructionEncoding {
            operation: InstructionOperation::FCOM as (u16),
            flags: 0,
            func: decode_mem_64,
        },
        InstructionEncoding {
            operation: InstructionOperation::FCOMP as (u16),
            flags: 0,
            func: decode_mem_64,
        },
        InstructionEncoding {
            operation: InstructionOperation::FSUB as (u16),
            flags: 0,
            func: decode_mem_64,
        },
        InstructionEncoding {
            operation: InstructionOperation::FSUBR as (u16),
            flags: 0,
            func: decode_mem_64,
        },
        InstructionEncoding {
            operation: InstructionOperation::FDIV as (u16),
            flags: 0,
            func: decode_mem_64,
        },
        InstructionEncoding {
            operation: InstructionOperation::FDIVR as (u16),
            flags: 0,
            func: decode_mem_64,
        },
    ],
    [
        InstructionEncoding {
            operation: InstructionOperation::FLD as (u16),
            flags: 0,
            func: decode_mem_64,
        },
        InstructionEncoding {
            operation: InstructionOperation::FISTTP as (u16),
            flags: 0,
            func: decode_mem_64,
        },
        InstructionEncoding {
            operation: InstructionOperation::FST as (u16),
            flags: 0,
            func: decode_mem_64,
        },
        InstructionEncoding {
            operation: InstructionOperation::FSTP as (u16),
            flags: 0,
            func: decode_mem_64,
        },
        InstructionEncoding {
            operation: InstructionOperation::FRSTOR as (u16),
            flags: 0,
            func: decode_mem_float_save,
        },
        InstructionEncoding {
            operation: InstructionOperation::INVALID as (u16),
            flags: 0,
            func: invalid_decode,
        },
        InstructionEncoding {
            operation: InstructionOperation::FSAVE as (u16),
            flags: 0,
            func: decode_mem_float_save,
        },
        InstructionEncoding {
            operation: InstructionOperation::FSTSW as (u16),
            flags: 0,
            func: decode_mem_16,
        },
    ],
    [
        InstructionEncoding {
            operation: InstructionOperation::FIADD as (u16),
            flags: 0,
            func: decode_mem_16,
        },
        InstructionEncoding {
            operation: InstructionOperation::FIMUL as (u16),
            flags: 0,
            func: decode_mem_16,
        },
        InstructionEncoding {
            operation: InstructionOperation::FICOM as (u16),
            flags: 0,
            func: decode_mem_16,
        },
        InstructionEncoding {
            operation: InstructionOperation::FICOMP as (u16),
            flags: 0,
            func: decode_mem_16,
        },
        InstructionEncoding {
            operation: InstructionOperation::FISUB as (u16),
            flags: 0,
            func: decode_mem_16,
        },
        InstructionEncoding {
            operation: InstructionOperation::FISUBR as (u16),
            flags: 0,
            func: decode_mem_16,
        },
        InstructionEncoding {
            operation: InstructionOperation::FIDIV as (u16),
            flags: 0,
            func: decode_mem_16,
        },
        InstructionEncoding {
            operation: InstructionOperation::FIDIVR as (u16),
            flags: 0,
            func: decode_mem_16,
        },
    ],
    [
        InstructionEncoding {
            operation: InstructionOperation::FILD as (u16),
            flags: 0,
            func: decode_mem_16,
        },
        InstructionEncoding {
            operation: InstructionOperation::FISTTP as (u16),
            flags: 0,
            func: decode_mem_16,
        },
        InstructionEncoding {
            operation: InstructionOperation::FIST as (u16),
            flags: 0,
            func: decode_mem_16,
        },
        InstructionEncoding {
            operation: InstructionOperation::FISTP as (u16),
            flags: 0,
            func: decode_mem_16,
        },
        InstructionEncoding {
            operation: InstructionOperation::FBLD as (u16),
            flags: 0,
            func: decode_mem_80,
        },
        InstructionEncoding {
            operation: InstructionOperation::FILD as (u16),
            flags: 0,
            func: decode_mem_64,
        },
        InstructionEncoding {
            operation: InstructionOperation::FBSTP as (u16),
            flags: 0,
            func: decode_mem_80,
        },
        InstructionEncoding {
            operation: InstructionOperation::FISTP as (u16),
            flags: 0,
            func: decode_mem_64,
        },
    ],
];

static FPU_REG_OPCODE_MAP: [[InstructionEncoding; 8]; 8] = [
    [
        InstructionEncoding {
            operation: InstructionOperation::FADD as (u16),
            flags: DecodeFlags::FLIP_OPERANDS as u16,
            func: decode_fpu_reg_st0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FMUL as (u16),
            flags: DecodeFlags::FLIP_OPERANDS as u16,
            func: decode_fpu_reg_st0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FCOM as (u16),
            flags: DecodeFlags::FLIP_OPERANDS as u16,
            func: decode_fpu_reg_st0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FCOMP as (u16),
            flags: DecodeFlags::FLIP_OPERANDS as u16,
            func: decode_fpu_reg_st0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FSUB as (u16),
            flags: DecodeFlags::FLIP_OPERANDS as u16,
            func: decode_fpu_reg_st0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FSUBR as (u16),
            flags: DecodeFlags::FLIP_OPERANDS as u16,
            func: decode_fpu_reg_st0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FDIV as (u16),
            flags: DecodeFlags::FLIP_OPERANDS as u16,
            func: decode_fpu_reg_st0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FDIVR as (u16),
            flags: DecodeFlags::FLIP_OPERANDS as u16,
            func: decode_fpu_reg_st0,
        },
    ],
    [
        InstructionEncoding {
            operation: InstructionOperation::FLD as (u16),
            flags: 0,
            func: decode_fpu_reg,
        },
        InstructionEncoding {
            operation: InstructionOperation::FXCH as (u16),
            flags: DecodeFlags::FLIP_OPERANDS as u16,
            func: decode_fpu_reg_st0,
        },
        InstructionEncoding {
            operation: 12u16,
            flags: 0,
            func: decode_reg_group_no_operands,
        },
        InstructionEncoding {
            operation: InstructionOperation::INVALID as (u16),
            flags: 0,
            func: invalid_decode,
        },
        InstructionEncoding {
            operation: 13u16,
            flags: 0,
            func: decode_reg_group_no_operands,
        },
        InstructionEncoding {
            operation: 14u16,
            flags: 0,
            func: decode_reg_group_no_operands,
        },
        InstructionEncoding {
            operation: 15u16,
            flags: 0,
            func: decode_reg_group_no_operands,
        },
        InstructionEncoding {
            operation: 16u16,
            flags: 0,
            func: decode_reg_group_no_operands,
        },
    ],
    [
        InstructionEncoding {
            operation: InstructionOperation::FCMOVB as (u16),
            flags: DecodeFlags::FLIP_OPERANDS as u16,
            func: decode_fpu_reg_st0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FCMOVE as (u16),
            flags: DecodeFlags::FLIP_OPERANDS as u16,
            func: decode_fpu_reg_st0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FCMOVBE as (u16),
            flags: DecodeFlags::FLIP_OPERANDS as u16,
            func: decode_fpu_reg_st0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FCMOVU as (u16),
            flags: DecodeFlags::FLIP_OPERANDS as u16,
            func: decode_fpu_reg_st0,
        },
        InstructionEncoding {
            operation: InstructionOperation::INVALID as (u16),
            flags: 0,
            func: invalid_decode,
        },
        InstructionEncoding {
            operation: 17u16,
            flags: 0,
            func: decode_reg_group_no_operands,
        },
        InstructionEncoding {
            operation: InstructionOperation::INVALID as (u16),
            flags: 0,
            func: invalid_decode,
        },
        InstructionEncoding {
            operation: InstructionOperation::INVALID as (u16),
            flags: 0,
            func: invalid_decode,
        },
    ],
    [
        InstructionEncoding {
            operation: InstructionOperation::FCMOVNB as (u16),
            flags: DecodeFlags::FLIP_OPERANDS as u16,
            func: decode_fpu_reg_st0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FCMOVNE as (u16),
            flags: DecodeFlags::FLIP_OPERANDS as u16,
            func: decode_fpu_reg_st0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FCMOVNBE as (u16),
            flags: DecodeFlags::FLIP_OPERANDS as u16,
            func: decode_fpu_reg_st0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FCMOVNU as (u16),
            flags: DecodeFlags::FLIP_OPERANDS as u16,
            func: decode_fpu_reg_st0,
        },
        InstructionEncoding {
            operation: 18u16,
            flags: 0,
            func: decode_reg_group_no_operands,
        },
        InstructionEncoding {
            operation: InstructionOperation::FUCOMI as (u16),
            flags: DecodeFlags::FLIP_OPERANDS as u16,
            func: decode_fpu_reg_st0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FCOMI as (u16),
            flags: DecodeFlags::FLIP_OPERANDS as u16,
            func: decode_fpu_reg_st0,
        },
        InstructionEncoding {
            operation: 21u16,
            flags: 0,
            func: decode_reg_group_no_operands,
        },
    ],
    [
        InstructionEncoding {
            operation: InstructionOperation::FADD as (u16),
            flags: 0,
            func: decode_fpu_reg_st0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FMUL as (u16),
            flags: 0,
            func: decode_fpu_reg_st0,
        },
        InstructionEncoding {
            operation: InstructionOperation::INVALID as (u16),
            flags: 0,
            func: invalid_decode,
        },
        InstructionEncoding {
            operation: InstructionOperation::INVALID as (u16),
            flags: 0,
            func: invalid_decode,
        },
        InstructionEncoding {
            operation: InstructionOperation::FSUBR as (u16),
            flags: 0,
            func: decode_fpu_reg_st0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FSUB as (u16),
            flags: 0,
            func: decode_fpu_reg_st0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FDIVR as (u16),
            flags: 0,
            func: decode_fpu_reg_st0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FDIV as (u16),
            flags: 0,
            func: decode_fpu_reg_st0,
        },
    ],
    [
        InstructionEncoding {
            operation: InstructionOperation::FFREE as (u16),
            flags: 0,
            func: decode_fpu_reg,
        },
        InstructionEncoding {
            operation: InstructionOperation::INVALID as (u16),
            flags: 0,
            func: invalid_decode,
        },
        InstructionEncoding {
            operation: InstructionOperation::FST as (u16),
            flags: 0,
            func: decode_fpu_reg,
        },
        InstructionEncoding {
            operation: InstructionOperation::FSTP as (u16),
            flags: 0,
            func: decode_fpu_reg,
        },
        InstructionEncoding {
            operation: InstructionOperation::FUCOM as (u16),
            flags: DecodeFlags::FLIP_OPERANDS as u16,
            func: decode_fpu_reg_st0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FUCOMP as (u16),
            flags: DecodeFlags::FLIP_OPERANDS as u16,
            func: decode_fpu_reg_st0,
        },
        InstructionEncoding {
            operation: InstructionOperation::INVALID as (u16),
            flags: 0,
            func: invalid_decode,
        },
        InstructionEncoding {
            operation: 22u16,
            flags: 0,
            func: decode_reg_group_no_operands,
        },
    ],
    [
        InstructionEncoding {
            operation: InstructionOperation::FADDP as (u16),
            flags: 0,
            func: decode_fpu_reg_st0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FMULP as (u16),
            flags: 0,
            func: decode_fpu_reg_st0,
        },
        InstructionEncoding {
            operation: InstructionOperation::INVALID as (u16),
            flags: 0,
            func: invalid_decode,
        },
        InstructionEncoding {
            operation: 19u16,
            flags: 0,
            func: decode_reg_group_no_operands,
        },
        InstructionEncoding {
            operation: InstructionOperation::FSUBRP as (u16),
            flags: 0,
            func: decode_fpu_reg_st0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FSUBP as (u16),
            flags: 0,
            func: decode_fpu_reg_st0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FDIVRP as (u16),
            flags: 0,
            func: decode_fpu_reg_st0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FDIVP as (u16),
            flags: 0,
            func: decode_fpu_reg_st0,
        },
    ],
    [
        InstructionEncoding {
            operation: InstructionOperation::FFREEP as (u16),
            flags: 0,
            func: decode_fpu_reg,
        },
        InstructionEncoding {
            operation: InstructionOperation::INVALID as (u16),
            flags: 0,
            func: invalid_decode,
        },
        InstructionEncoding {
            operation: InstructionOperation::INVALID as (u16),
            flags: 0,
            func: invalid_decode,
        },
        InstructionEncoding {
            operation: InstructionOperation::INVALID as (u16),
            flags: 0,
            func: invalid_decode,
        },
        InstructionEncoding {
            operation: 20u16,
            flags: 0,
            func: decode_reg_group_ax,
        },
        InstructionEncoding {
            operation: InstructionOperation::FUCOMIP as (u16),
            flags: DecodeFlags::FLIP_OPERANDS as u16,
            func: decode_fpu_reg_st0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FCOMIP as (u16),
            flags: DecodeFlags::FLIP_OPERANDS as u16,
            func: decode_fpu_reg_st0,
        },
        InstructionEncoding {
            operation: 23u16,
            flags: 0,
            func: decode_reg_group_no_operands,
        },
    ],
];

static GROUP_OPERATIONS: [[InstructionOperation; 8]; 26] = [
    [
        InstructionOperation::ADD,
        InstructionOperation::OR,
        InstructionOperation::ADC,
        InstructionOperation::SBB,
        InstructionOperation::AND,
        InstructionOperation::SUB,
        InstructionOperation::XOR,
        InstructionOperation::CMP,
    ],
    [
        InstructionOperation::ROL,
        InstructionOperation::ROR,
        InstructionOperation::RCL,
        InstructionOperation::RCR,
        InstructionOperation::SHL,
        InstructionOperation::SHR,
        InstructionOperation::SHL,
        InstructionOperation::SAR,
    ],
    [
        InstructionOperation::MOV,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
    ],
    [
        InstructionOperation::TEST,
        InstructionOperation::TEST,
        InstructionOperation::NOT,
        InstructionOperation::NEG,
        InstructionOperation::MUL,
        InstructionOperation::IMUL,
        InstructionOperation::DIV,
        InstructionOperation::IDIV,
    ],
    [
        InstructionOperation::INC,
        InstructionOperation::DEC,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
    ],
    [
        InstructionOperation::INC,
        InstructionOperation::DEC,
        InstructionOperation::CALL,
        InstructionOperation::CALLF,
        InstructionOperation::JMP,
        InstructionOperation::JMPF,
        InstructionOperation::PUSH,
        InstructionOperation::INVALID,
    ],
    [
        InstructionOperation::SLDT,
        InstructionOperation::STR,
        InstructionOperation::LLDT,
        InstructionOperation::LTR,
        InstructionOperation::VERR,
        InstructionOperation::VERW,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
    ],
    [
        InstructionOperation::SGDT,
        InstructionOperation::SIDT,
        InstructionOperation::LGDT,
        InstructionOperation::LIDT,
        InstructionOperation::SMSW,
        InstructionOperation::INVALID,
        InstructionOperation::LMSW,
        InstructionOperation::INVLPG,
    ],
    [
        InstructionOperation::PREFETCH,
        InstructionOperation::PREFETCHW,
        InstructionOperation::PREFETCH,
        InstructionOperation::PREFETCH,
        InstructionOperation::PREFETCH,
        InstructionOperation::PREFETCH,
        InstructionOperation::PREFETCH,
        InstructionOperation::PREFETCH,
    ],
    [
        InstructionOperation::PREFETCHNTA,
        InstructionOperation::PREFETCHT0,
        InstructionOperation::PREFETCHT1,
        InstructionOperation::PREFETCHT2,
        InstructionOperation::MMXNOP,
        InstructionOperation::MMXNOP,
        InstructionOperation::MMXNOP,
        InstructionOperation::MMXNOP,
    ],
    [
        InstructionOperation::MMXNOP,
        InstructionOperation::MMXNOP,
        InstructionOperation::MMXNOP,
        InstructionOperation::MMXNOP,
        InstructionOperation::MMXNOP,
        InstructionOperation::MMXNOP,
        InstructionOperation::MMXNOP,
        InstructionOperation::MMXNOP,
    ],
    [
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::BT,
        InstructionOperation::BTS,
        InstructionOperation::BTR,
        InstructionOperation::BTC,
    ],
    [
        InstructionOperation::FNOP,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
    ],
    [
        InstructionOperation::FCHS,
        InstructionOperation::FABS,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::FTST,
        InstructionOperation::FXAM,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
    ],
    [
        InstructionOperation::FLD1,
        InstructionOperation::FLDL2T,
        InstructionOperation::FLDL2E,
        InstructionOperation::FLDPI,
        InstructionOperation::FLDLG2,
        InstructionOperation::FLDLN2,
        InstructionOperation::FLDZ,
        InstructionOperation::INVALID,
    ],
    [
        InstructionOperation::F2XM1,
        InstructionOperation::FYL2X,
        InstructionOperation::FPTAN,
        InstructionOperation::FPATAN,
        InstructionOperation::FXTRACT,
        InstructionOperation::FPREM1,
        InstructionOperation::FDECSTP,
        InstructionOperation::FINCSTP,
    ],
    [
        InstructionOperation::FPREM,
        InstructionOperation::FYL2XP1,
        InstructionOperation::FSQRT,
        InstructionOperation::FSINCOS,
        InstructionOperation::FRNDINT,
        InstructionOperation::FSCALE,
        InstructionOperation::FSIN,
        InstructionOperation::FCOS,
    ],
    [
        InstructionOperation::INVALID,
        InstructionOperation::FUCOMPP,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
    ],
    [
        InstructionOperation::FENI,
        InstructionOperation::FDISI,
        InstructionOperation::FCLEX,
        InstructionOperation::FINIT,
        InstructionOperation::FSETPM,
        InstructionOperation::FRSTPM,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
    ],
    [
        InstructionOperation::INVALID,
        InstructionOperation::FCOMPP,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
    ],
    [
        InstructionOperation::FSTSW,
        InstructionOperation::FSTDW,
        InstructionOperation::FSTSG,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
    ],
    [
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::FRINT2,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
    ],
    [
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::FRICHOP,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
    ],
    [
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::FRINEAR,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
    ],
    [
        InstructionOperation::FXSAVE,
        InstructionOperation::FXRSTOR,
        InstructionOperation::LDMXCSR,
        InstructionOperation::STMXCSR,
        InstructionOperation::XSAVE,
        InstructionOperation::XRSTOR,
        InstructionOperation::INVALID,
        InstructionOperation::CLFLUSH,
    ],
    [
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::LFENCE,
        InstructionOperation::MFENCE,
        InstructionOperation::SFENCE,
    ],
];

static GROUP_0F01_REG_OPERATIONS: [[InstructionOperation; 8]; 8] = [
    [
        InstructionOperation::INVALID,
        InstructionOperation::VMCALL,
        InstructionOperation::VMLAUNCH,
        InstructionOperation::VMRESUME,
        InstructionOperation::VMXOFF,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
    ],
    [
        InstructionOperation::MONITOR,
        InstructionOperation::MWAIT,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
    ],
    [
        InstructionOperation::XGETBV,
        InstructionOperation::XSETBV,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
    ],
    [
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
    ],
    [
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
    ],
    [
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
    ],
    [
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
    ],
    [
        InstructionOperation::SWAPGS,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
        InstructionOperation::INVALID,
    ],
];

static MMX_GROUP_OPERATIONS: [[[InstructionOperation; 2]; 8]; 3] =
    [
        [
            [InstructionOperation::INVALID, InstructionOperation::INVALID],
            [InstructionOperation::INVALID, InstructionOperation::INVALID],
            [InstructionOperation::PSRLW, InstructionOperation::PSRLW],
            [InstructionOperation::INVALID, InstructionOperation::INVALID],
            [InstructionOperation::PSRAW, InstructionOperation::PSRAW],
            [InstructionOperation::INVALID, InstructionOperation::INVALID],
            [InstructionOperation::PSLLW, InstructionOperation::PSLLW],
            [InstructionOperation::INVALID, InstructionOperation::INVALID],
        ],
        [
            [InstructionOperation::INVALID, InstructionOperation::INVALID],
            [InstructionOperation::INVALID, InstructionOperation::INVALID],
            [InstructionOperation::PSRLD, InstructionOperation::PSRLD],
            [InstructionOperation::INVALID, InstructionOperation::INVALID],
            [InstructionOperation::PSRAD, InstructionOperation::PSRAD],
            [InstructionOperation::INVALID, InstructionOperation::INVALID],
            [InstructionOperation::PSLLD, InstructionOperation::PSLLD],
            [InstructionOperation::INVALID, InstructionOperation::INVALID],
        ],
        [
            [InstructionOperation::INVALID, InstructionOperation::INVALID],
            [InstructionOperation::INVALID, InstructionOperation::INVALID],
            [InstructionOperation::PSRLQ, InstructionOperation::PSRLQ],
            [InstructionOperation::INVALID, InstructionOperation::PSRLDQ],
            [InstructionOperation::INVALID, InstructionOperation::INVALID],
            [InstructionOperation::INVALID, InstructionOperation::INVALID],
            [InstructionOperation::PSLLQ, InstructionOperation::PSLLQ],
            [InstructionOperation::INVALID, InstructionOperation::PSLLDQ],
        ],
    ];

#[derive(Debug)]
#[repr(C)]
struct SSETableOperationEntry {
    pub operation: InstructionOperation,
    pub reg_type: SSETableOperandType,
    pub rm_type: SSETableOperandType,
}

#[derive(Debug)]
#[repr(C)]
struct SSETableEntry {
    pub reg_ops: [SSETableOperationEntry; 4],
    pub mem_ops: [SSETableOperationEntry; 4],
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(i32)]
enum SSETableOperandType {
    INVALID = 0,
    SSE_16,
    SSE_32,
    SSE_64,
    SSE_128,
    SSE_128_FLIP,
    GPR_32_OR_64,
    MMX_32,
    MMX_64,
}

static SSE_TABLE: [SSETableEntry; 58] = [
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVUPS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVUPD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVSD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVSS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVUPS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVUPD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVSD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVSS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_32,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVHLPS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVDDUP,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVSLDUP,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVLPS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVLPD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVDDUP,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVSLDUP,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVLPS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVLPD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::UNPCKLPS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::UNPCKLPD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::UNPCKLPS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::UNPCKLPD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::UNPCKHPS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::UNPCKHPD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::UNPCKHPS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::UNPCKHPD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVLHPS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVSHDUP,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVHPS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVHPD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVSHDUP,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVHPS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVHPD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVAPS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVAPD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVAPS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVAPD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPI2PS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::MMX_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPI2PD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::MMX_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTSI2SD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTSI2SS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::GPR_32_OR_64,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPI2PS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::MMX_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPI2PD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::MMX_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTSI2SD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTSI2SS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::GPR_32_OR_64,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVNTPS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVNTPD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::CVTTPS2PI,
                reg_type: SSETableOperandType::MMX_64,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTTPD2PI,
                reg_type: SSETableOperandType::MMX_64,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTTSD2SI,
                reg_type: SSETableOperandType::GPR_32_OR_64,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTTSS2SI,
                reg_type: SSETableOperandType::GPR_32_OR_64,
                rm_type: SSETableOperandType::SSE_128,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::CVTTPS2PI,
                reg_type: SSETableOperandType::MMX_64,
                rm_type: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTTPD2PI,
                reg_type: SSETableOperandType::MMX_64,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTTSD2SI,
                reg_type: SSETableOperandType::GPR_32_OR_64,
                rm_type: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTTSS2SI,
                reg_type: SSETableOperandType::GPR_32_OR_64,
                rm_type: SSETableOperandType::SSE_32,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPS2PI,
                reg_type: SSETableOperandType::MMX_64,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPD2PI,
                reg_type: SSETableOperandType::MMX_64,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTSD2SI,
                reg_type: SSETableOperandType::GPR_32_OR_64,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTSS2SI,
                reg_type: SSETableOperandType::GPR_32_OR_64,
                rm_type: SSETableOperandType::SSE_128,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPS2PI,
                reg_type: SSETableOperandType::MMX_64,
                rm_type: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPD2PI,
                reg_type: SSETableOperandType::MMX_64,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTSD2SI,
                reg_type: SSETableOperandType::GPR_32_OR_64,
                rm_type: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTSS2SI,
                reg_type: SSETableOperandType::GPR_32_OR_64,
                rm_type: SSETableOperandType::SSE_32,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::UCOMISS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::UCOMISD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::UCOMISS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_32,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::UCOMISD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::COMISS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::COMISD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::COMISS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_32,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::COMISD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVMSKPS,
                reg_type: SSETableOperandType::GPR_32_OR_64,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVMSKPD,
                reg_type: SSETableOperandType::GPR_32_OR_64,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPS2PD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPD2PS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTSD2SS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTSS2SD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPS2PD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPD2PS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTSD2SS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTSS2SD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_32,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::CVTDQ2PS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPS2DQ,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTTPS2DQ,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::CVTDQ2PS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPS2DQ,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTTPS2DQ,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::PUNPCKLBW,
                reg_type: SSETableOperandType::MMX_64,
                rm_type: SSETableOperandType::MMX_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PUNPCKLBW,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::PUNPCKLBW,
                reg_type: SSETableOperandType::MMX_64,
                rm_type: SSETableOperandType::MMX_32,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PUNPCKLBW,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::PUNPCKLWD,
                reg_type: SSETableOperandType::MMX_64,
                rm_type: SSETableOperandType::MMX_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PUNPCKLWD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::PUNPCKLWD,
                reg_type: SSETableOperandType::MMX_64,
                rm_type: SSETableOperandType::MMX_32,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PUNPCKLWD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::PUNPCKLDQ,
                reg_type: SSETableOperandType::MMX_64,
                rm_type: SSETableOperandType::MMX_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PUNPCKLDQ,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::PUNPCKLDQ,
                reg_type: SSETableOperandType::MMX_64,
                rm_type: SSETableOperandType::MMX_32,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PUNPCKLDQ,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVD,
                reg_type: SSETableOperandType::MMX_64,
                rm_type: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVD,
                reg_type: SSETableOperandType::MMX_64,
                rm_type: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVQ,
                reg_type: SSETableOperandType::MMX_64,
                rm_type: SSETableOperandType::MMX_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVDQA,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVDQU,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVQ,
                reg_type: SSETableOperandType::MMX_64,
                rm_type: SSETableOperandType::MMX_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVDQA,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVDQU,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::PSHUFW,
                reg_type: SSETableOperandType::MMX_64,
                rm_type: SSETableOperandType::MMX_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PSHUFD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PSHUFLW,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PSHUFHW,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::PSHUFW,
                reg_type: SSETableOperandType::MMX_64,
                rm_type: SSETableOperandType::MMX_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PSHUFD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PSHUFLW,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PSHUFHW,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::HADDPD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::HADDPS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::HADDPD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::HADDPS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::HSUBPD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::HSUBPS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::HSUBPD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::HSUBPS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVD,
                reg_type: SSETableOperandType::MMX_64,
                rm_type: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVQ,
                reg_type: SSETableOperandType::SSE_128_FLIP,
                rm_type: SSETableOperandType::SSE_128_FLIP,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVD,
                reg_type: SSETableOperandType::MMX_64,
                rm_type: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVQ,
                reg_type: SSETableOperandType::SSE_128_FLIP,
                rm_type: SSETableOperandType::SSE_128_FLIP,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::CMPPS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CMPPD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CMPSD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CMPSS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::CMPPS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CMPPD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CMPSD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CMPSS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_32,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::PINSRW,
                reg_type: SSETableOperandType::MMX_64,
                rm_type: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PINSRW,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::PINSRW,
                reg_type: SSETableOperandType::MMX_64,
                rm_type: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PINSRW,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::PEXTRW,
                reg_type: SSETableOperandType::MMX_64,
                rm_type: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PEXTRW,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::PEXTRW,
                reg_type: SSETableOperandType::MMX_64,
                rm_type: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PEXTRW,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::SHUFPS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::SHUFPD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::SHUFPS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::SHUFPD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::ADDSUBPD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::ADDSUBPS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::ADDSUBPD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::ADDSUBPS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVQ,
                reg_type: SSETableOperandType::SSE_128_FLIP,
                rm_type: SSETableOperandType::SSE_128_FLIP,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVDQ2Q,
                reg_type: SSETableOperandType::MMX_64,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVQ2DQ,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::MMX_64,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVQ,
                reg_type: SSETableOperandType::SSE_128_FLIP,
                rm_type: SSETableOperandType::SSE_128_FLIP,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVMSKB,
                reg_type: SSETableOperandType::GPR_32_OR_64,
                rm_type: SSETableOperandType::MMX_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVMSKB,
                reg_type: SSETableOperandType::GPR_32_OR_64,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTTPD2DQ,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPD2DQ,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTDQ2PD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTTPD2DQ,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPD2DQ,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTDQ2PD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVNTQ,
                reg_type: SSETableOperandType::MMX_64,
                rm_type: SSETableOperandType::MMX_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVNTDQ,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::LDDQU,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::MASKMOVQ,
                reg_type: SSETableOperandType::MMX_64,
                rm_type: SSETableOperandType::MMX_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MASKMOVDQU,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVSXBW,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVSXBW,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVSXBD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVSXBD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_32,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVSXBQ,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVSXBQ,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_16,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVSXWD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVSXWD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVSXWQ,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVSXWQ,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_32,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVSXDQ,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVSXDQ,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVNTDQA,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVZXBW,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVZXBW,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVZXBD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVZXBD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_32,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVZXBQ,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVZXBQ,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_16,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVZXWD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVZXWD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVZXWQ,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVZXWQ,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_32,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVZXDQ,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVZXDQ,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::ROUNDSS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::ROUNDSS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_32,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::ROUNDSD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::ROUNDSD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PEXTRB,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PEXTRB,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PEXTRW,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PEXTRW,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_16,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PEXTRD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PEXTRD,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::EXTRACTPS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::EXTRACTPS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_32,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PINSRB,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PINSRB,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        reg_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INSERTPS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
        mem_ops: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INSERTPS,
                reg_type: SSETableOperandType::SSE_128,
                rm_type: SSETableOperandType::SSE_32,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                reg_type: SSETableOperandType::INVALID,
                rm_type: SSETableOperandType::INVALID,
            },
        ],
    },
];

#[derive(Debug)]
#[repr(C)]
struct SparseOpEntry {
    pub opcode: u8,
    pub operation: InstructionOperation,
}

static SPARSE_3DNOW_OPCODES: [SparseOpEntry; 26] = [
    SparseOpEntry {
        opcode: 0xcu8,
        operation: InstructionOperation::PI2FW,
    },
    SparseOpEntry {
        opcode: 0xdu8,
        operation: InstructionOperation::PI2FD,
    },
    SparseOpEntry {
        opcode: 0x1cu8,
        operation: InstructionOperation::PF2IW,
    },
    SparseOpEntry {
        opcode: 0x1du8,
        operation: InstructionOperation::PF2ID,
    },
    SparseOpEntry {
        opcode: 0x86u8,
        operation: InstructionOperation::PFRCPV,
    },
    SparseOpEntry {
        opcode: 0x87u8,
        operation: InstructionOperation::PFRSQRTV,
    },
    SparseOpEntry {
        opcode: 0x8au8,
        operation: InstructionOperation::PFNACC,
    },
    SparseOpEntry {
        opcode: 0x8eu8,
        operation: InstructionOperation::PFPNACC,
    },
    SparseOpEntry {
        opcode: 0x90u8,
        operation: InstructionOperation::PFCMPGE,
    },
    SparseOpEntry {
        opcode: 0x94u8,
        operation: InstructionOperation::PFMIN,
    },
    SparseOpEntry {
        opcode: 0x96u8,
        operation: InstructionOperation::PFRCP,
    },
    SparseOpEntry {
        opcode: 0x97u8,
        operation: InstructionOperation::PFRSQRT,
    },
    SparseOpEntry {
        opcode: 0x9au8,
        operation: InstructionOperation::PFSUB,
    },
    SparseOpEntry {
        opcode: 0x9eu8,
        operation: InstructionOperation::PFADD,
    },
    SparseOpEntry {
        opcode: 0xa0u8,
        operation: InstructionOperation::PFCMPGT,
    },
    SparseOpEntry {
        opcode: 0xa4u8,
        operation: InstructionOperation::PFMAX,
    },
    SparseOpEntry {
        opcode: 0xa6u8,
        operation: InstructionOperation::PFRCPIT1,
    },
    SparseOpEntry {
        opcode: 0xa7u8,
        operation: InstructionOperation::PFRSQIT1,
    },
    SparseOpEntry {
        opcode: 0xaau8,
        operation: InstructionOperation::PFSUBR,
    },
    SparseOpEntry {
        opcode: 0xaeu8,
        operation: InstructionOperation::PFACC,
    },
    SparseOpEntry {
        opcode: 0xb0u8,
        operation: InstructionOperation::PFCMPEQ,
    },
    SparseOpEntry {
        opcode: 0xb4u8,
        operation: InstructionOperation::PFMUL,
    },
    SparseOpEntry {
        opcode: 0xb6u8,
        operation: InstructionOperation::PFRCPIT2,
    },
    SparseOpEntry {
        opcode: 0xb7u8,
        operation: InstructionOperation::PMULHRW,
    },
    SparseOpEntry {
        opcode: 0xbbu8,
        operation: InstructionOperation::PSWAPD,
    },
    SparseOpEntry {
        opcode: 0xbfu8,
        operation: InstructionOperation::PAVGUSB,
    },
];

static INVALID_REG_LIST: [OperandType; 0] = [];

static REG8_LIST: [OperandType; 8] = [
    OperandType::REG_AL,
    OperandType::REG_CL,
    OperandType::REG_DL,
    OperandType::REG_BL,
    OperandType::REG_AH,
    OperandType::REG_CH,
    OperandType::REG_DH,
    OperandType::REG_BH,
];

static REG8_LIST64: [OperandType; 16] = [
    OperandType::REG_AL,
    OperandType::REG_CL,
    OperandType::REG_DL,
    OperandType::REG_BL,
    OperandType::REG_SPL,
    OperandType::REG_BPL,
    OperandType::REG_SIL,
    OperandType::REG_DIL,
    OperandType::REG_R8B,
    OperandType::REG_R9B,
    OperandType::REG_R10B,
    OperandType::REG_R11B,
    OperandType::REG_R12B,
    OperandType::REG_R13B,
    OperandType::REG_R14B,
    OperandType::REG_R15B,
];

static REG16_LIST: [OperandType; 16] = [
    OperandType::REG_AX,
    OperandType::REG_CX,
    OperandType::REG_DX,
    OperandType::REG_BX,
    OperandType::REG_SP,
    OperandType::REG_BP,
    OperandType::REG_SI,
    OperandType::REG_DI,
    OperandType::REG_R8W,
    OperandType::REG_R9W,
    OperandType::REG_R10W,
    OperandType::REG_R11W,
    OperandType::REG_R12W,
    OperandType::REG_R13W,
    OperandType::REG_R14W,
    OperandType::REG_R15W,
];

static REG32_LIST: [OperandType; 16] = [
    OperandType::REG_EAX,
    OperandType::REG_ECX,
    OperandType::REG_EDX,
    OperandType::REG_EBX,
    OperandType::REG_ESP,
    OperandType::REG_EBP,
    OperandType::REG_ESI,
    OperandType::REG_EDI,
    OperandType::REG_R8D,
    OperandType::REG_R9D,
    OperandType::REG_R10D,
    OperandType::REG_R11D,
    OperandType::REG_R12D,
    OperandType::REG_R13D,
    OperandType::REG_R14D,
    OperandType::REG_R15D,
];

static REG64_LIST: [OperandType; 16] = [
    OperandType::REG_RAX,
    OperandType::REG_RCX,
    OperandType::REG_RDX,
    OperandType::REG_RBX,
    OperandType::REG_RSP,
    OperandType::REG_RBP,
    OperandType::REG_RSI,
    OperandType::REG_RDI,
    OperandType::REG_R8,
    OperandType::REG_R9,
    OperandType::REG_R10,
    OperandType::REG_R11,
    OperandType::REG_R12,
    OperandType::REG_R13,
    OperandType::REG_R14,
    OperandType::REG_R15,
];

static MMX_REG_LIST: [OperandType; 16] = [
    OperandType::REG_MM0,
    OperandType::REG_MM1,
    OperandType::REG_MM2,
    OperandType::REG_MM3,
    OperandType::REG_MM4,
    OperandType::REG_MM5,
    OperandType::REG_MM6,
    OperandType::REG_MM7,
    OperandType::REG_MM0,
    OperandType::REG_MM1,
    OperandType::REG_MM2,
    OperandType::REG_MM3,
    OperandType::REG_MM4,
    OperandType::REG_MM5,
    OperandType::REG_MM6,
    OperandType::REG_MM7,
];

static XMM_REG_LIST: [OperandType; 16] = [
    OperandType::REG_XMM0,
    OperandType::REG_XMM1,
    OperandType::REG_XMM2,
    OperandType::REG_XMM3,
    OperandType::REG_XMM4,
    OperandType::REG_XMM5,
    OperandType::REG_XMM6,
    OperandType::REG_XMM7,
    OperandType::REG_XMM8,
    OperandType::REG_XMM9,
    OperandType::REG_XMM10,
    OperandType::REG_XMM11,
    OperandType::REG_XMM12,
    OperandType::REG_XMM13,
    OperandType::REG_XMM14,
    OperandType::REG_XMM15,
];

static FPU_REG_LIST: [OperandType; 16] = [
    OperandType::REG_ST0,
    OperandType::REG_ST1,
    OperandType::REG_ST2,
    OperandType::REG_ST3,
    OperandType::REG_ST4,
    OperandType::REG_ST5,
    OperandType::REG_ST6,
    OperandType::REG_ST7,
    OperandType::REG_ST0,
    OperandType::REG_ST1,
    OperandType::REG_ST2,
    OperandType::REG_ST3,
    OperandType::REG_ST4,
    OperandType::REG_ST5,
    OperandType::REG_ST6,
    OperandType::REG_ST7,
];

fn invalid_decode(state: &mut DecodeState) {
    state.invalid = true;
}

fn read_8(state: &mut DecodeState) -> u8 {
    if state.len < 1 {
        state.invalid = true;
        state.insufficient_length = true;
        state.len = 0;
        0xcc
    } else {
        let val = unsafe {
            *{
                let _old = state.opcode;
                state.opcode = state.opcode.offset(1);
                _old
            }
        };
        state.len = state.len.wrapping_sub(1);
        val
    }
}

fn get_final_op_size(state: &DecodeState) -> u16 {
    if state.flags & DecodeFlags::BYTE != 0 {
        1u16
    } else {
        state.op_size
    }
}

fn process_encoding(state: &mut DecodeState, encoding: &InstructionEncoding) {
    state.result.operation = InstructionOperation::from_i32(i32::from(encoding.operation));
    state.flags = u32::from(encoding.flags);
    if state.using64 && (state.flags & DecodeFlags::INVALID_IN_64BIT != 0) {
        state.invalid = true;
    } else {
        if state.using64 && (state.flags & DecodeFlags::DEFAULT_TO_64BIT != 0) {
            state.op_size = if state.op_prefix { 4 } else { 8 };
        }
        state.final_op_size = get_final_op_size(state);
        if state.flags & DecodeFlags::FLIP_OPERANDS != 0 {
            state.operand0 = &mut state.result.operands[1] as (*mut InstructionOperand);
            state.operand1 = &mut state.result.operands[0] as (*mut InstructionOperand);
        } else {
            state.operand0 = &mut state.result.operands[0] as (*mut InstructionOperand);
            state.operand1 = &mut state.result.operands[1] as (*mut InstructionOperand);
        }
        if state.flags & DecodeFlags::FORCE_16BIT != 0 {
            state.final_op_size = 2;
        }
        if state.flags & DecodeFlags::OPERATION_OP_SIZE != 0 {
            if state.final_op_size == 4 {
                state.result.operation =
                    InstructionOperation::from_i32(state.result.operation as i32 + 1);
            } else if state.final_op_size == 8 {
                state.result.operation =
                    InstructionOperation::from_i32(state.result.operation as i32 + 2);
            }
        }
        if state.flags & DecodeFlags::REP != 0 {
            if state.rep != RepPrefix::NONE {
                state.result.flags |= X86Flag::REP;
            }
        } else if state.flags & DecodeFlags::REP_COND != 0 {
            if state.rep == RepPrefix::REPNE {
                state.result.flags |= X86Flag::REPNE;
            } else if state.rep == RepPrefix::REPE {
                state.result.flags |= X86Flag::REPE;
            }
        }
        (encoding.func)(state);
        if state.result.operation == InstructionOperation::INVALID {
            state.invalid = true;
        }
        if state.result.flags & X86Flag::LOCK != 0 {
            // Ensure instruction allows lock and it has proper semantics
            if state.flags & DecodeFlags::LOCK == 0 {
                state.invalid = true;
            } else if state.result.operation == InstructionOperation::CMP {
                state.invalid = true;
            } else if state.result.operands[0].operand != OperandType::MEM &&
                       (state.result.operands[1].operand != OperandType::MEM)
            {
                state.invalid = true;
            }
        }
    }
}

fn process_opcode(state: &mut DecodeState, map: &[InstructionEncoding], opcode: u8) {
    process_encoding(state, &map[opcode as usize]);
}

fn process_sparse_opcode(state: &mut DecodeState, map: &[SparseInstructionEncoding], opcode: u8) {
    state.result.operation = InstructionOperation::INVALID;
    if let Ok(idx) = map.binary_search_by_key(&opcode, |entry| entry.opcode) {
        process_encoding(state, &map[idx].encoding);
    }
}

fn set_operand_to_imm_8(state: &mut DecodeState, oper: *mut InstructionOperand) {
    unsafe {
        (*oper).operand = OperandType::IMM;
        (*oper).size = 1u16;
        (*oper).immediate = read_8(state) as (isize);
    }
}

fn decode_two_byte(state: &mut DecodeState) {
    let opcode: u8 = read_8(state);
    if opcode == 0x38 {
        let next_opcode = read_8(state);
        process_sparse_opcode(state, &THREE_BYTE_0F38_MAP, next_opcode);
    } else if opcode == 0x3a {
        let next_opcode = read_8(state);
        process_sparse_opcode(state, &THREE_BYTE_0F3A_MAP, next_opcode);
        let operand = &mut state.result.operands[2] as *mut InstructionOperand;
        set_operand_to_imm_8(state, operand);
    } else {
        process_opcode(state, &TWO_BYTE_OPCODE_MAP, opcode);
    }
}

fn peek_8(state: &mut DecodeState) -> u8 {
    if state.len < 1 {
        state.invalid = true;
        state.insufficient_length = true;
        state.len = 0;
        0xcc
    } else {
        unsafe { *state.opcode }
    }
}

fn decode_fpu(state: &mut DecodeState) {
    let mod_rm = peek_8(state);
    let reg = mod_rm >> 3 & 7;
    let op = state.result.operation as u8;
    let map = if mod_rm & 0xc0 == 0xc0 {
        &FPU_REG_OPCODE_MAP[op as usize]
    } else {
        &FPU_MEM_OPCODE_MAP[op as usize]
    };
    process_encoding(state, &map[reg as usize]);
}

fn decode_no_operands(_state: &mut DecodeState) {}

fn get_byte_reg_list(state: &DecodeState) -> &'static [OperandType] {
    if state.rex { &REG8_LIST64 } else { &REG8_LIST }
}

fn get_reg_list_for_final_op_size(state: &DecodeState) -> &'static [OperandType] {
    match state.final_op_size {
        8 => &REG64_LIST,
        4 => &REG32_LIST,
        2 => &REG16_LIST,
        1 => get_byte_reg_list(state),
        _ => &INVALID_REG_LIST,
    }
}

fn get_reg_list_for_addr_size(state: &DecodeState) -> &'static [OperandType] {
    match state.addr_size {
        8 => &REG64_LIST,
        4 => &REG32_LIST,
        2 => &REG16_LIST,
        _ => &INVALID_REG_LIST,
    }
}

fn read_32(state: &mut DecodeState) -> u32 {
    let val: u32;
    if state.len < 4 {
        state.invalid = true;
        state.insufficient_length = true;
        state.len = 0;
        0
    } else {
        unsafe {
            val = *(state.opcode as (*mut u32));
            state.opcode = state.opcode.offset(4);
            state.len = state.len.wrapping_sub(4);
            val
        }
    }
}

fn read_signed_32(state: &mut DecodeState) -> isize {
    read_32(state) as (i32) as (isize)
}

fn read_signed_8(state: &mut DecodeState) -> isize {
    read_8(state) as (i8) as (isize)
}

fn get_final_segment(state: &DecodeState, seg: SegmentRegister) -> SegmentRegister {
    if state.result.segment == SegmentRegister::DEFAULT {
        seg
    } else {
        state.result.segment
    }
}

#[derive(Debug)]
#[repr(C)]
struct RMDef {
    pub first: OperandType,
    pub second: OperandType,
    pub segment: SegmentRegister,
}

fn set_mem_operand(state: &DecodeState, oper: *mut InstructionOperand, def: &RMDef, immed: isize) {
    unsafe {
        (*oper).operand = OperandType::MEM;
        (*oper).components[0] = def.first;
        (*oper).components[1] = def.second;
        (*oper).immediate = immed;
        (*oper).segment = get_final_segment(state, def.segment);
    }
}

fn read_16(state: &mut DecodeState) -> u16 {
    let val: u16;
    if state.len < 2 {
        state.invalid = true;
        state.insufficient_length = true;
        state.len = 0;
        0
    } else {
        unsafe {
            val = *(state.opcode as (*mut u16));
            state.opcode = state.opcode.offset(2);
            state.len = state.len.wrapping_sub(2);
            val
        }
    }
}

fn read_signed_16(state: &mut DecodeState) -> isize {
    read_16(state) as (i16) as (isize)
}

fn decode_rm(
    state: &mut DecodeState,
    mut rm_oper: *mut InstructionOperand,
    reg_list: &[OperandType],
    rm_size: u16,
    reg_oper: *mut u8,
) {
    let rm_byte: u8 = read_8(state);
    let mod_: u8 = rm_byte >> 6;
    let mut rm: u8 = rm_byte & 7;
    let mut temp = InstructionOperand::default();
    unsafe {
        if !reg_oper.is_null() {
            *reg_oper = rm_byte >> 3 & 7;
        }
        if rm_oper.is_null() {
            rm_oper = &mut temp as (*mut InstructionOperand);
        }
        (*rm_oper).size = rm_size;
    }
    if state.addr_size == 2 {
        static RM16_COMPONENTS: [RMDef; 9] = [
            RMDef {
                first: OperandType::REG_BX,
                second: OperandType::REG_SI,
                segment: SegmentRegister::DS,
            },
            RMDef {
                first: OperandType::REG_BX,
                second: OperandType::REG_DI,
                segment: SegmentRegister::DS,
            },
            RMDef {
                first: OperandType::REG_BP,
                second: OperandType::REG_SI,
                segment: SegmentRegister::SS,
            },
            RMDef {
                first: OperandType::REG_BP,
                second: OperandType::REG_DI,
                segment: SegmentRegister::SS,
            },
            RMDef {
                first: OperandType::REG_SI,
                second: OperandType::NONE,
                segment: SegmentRegister::DS,
            },
            RMDef {
                first: OperandType::REG_DI,
                second: OperandType::NONE,
                segment: SegmentRegister::DS,
            },
            RMDef {
                first: OperandType::REG_BP,
                second: OperandType::NONE,
                segment: SegmentRegister::SS,
            },
            RMDef {
                first: OperandType::REG_BX,
                second: OperandType::NONE,
                segment: SegmentRegister::DS,
            },
            RMDef {
                first: OperandType::NONE,
                second: OperandType::NONE,
                segment: SegmentRegister::DS,
            },
        ];
        if mod_ == 3 {
            unsafe {
                (*rm_oper).operand = reg_list[rm as usize];
            }
        } else if mod_ == 2 {
            let immediate = read_signed_16(state);
            set_mem_operand(state, rm_oper, &RM16_COMPONENTS[rm as usize], immediate);
        } else if mod_ == 1 {
            let immediate = read_signed_8(state);
            set_mem_operand(state, rm_oper, &RM16_COMPONENTS[rm as usize], immediate);
        } else if mod_ == 0 {
            if rm == 6 {
                rm = 8;
                let immediate = read_16(state);
                set_mem_operand(
                    state,
                    rm_oper,
                    &RM16_COMPONENTS[rm as usize],
                    immediate as isize,
                );
            } else {
                set_mem_operand(state, rm_oper, &RM16_COMPONENTS[rm as usize], 0);
            }
        }
        unsafe {
            if (*rm_oper).components[0] == OperandType::NONE {
                (*rm_oper).immediate &= 0xffff;
            }
        }
    } else {
        let addr_reg_list = get_reg_list_for_addr_size(state);
        let rm_reg_1_offset: u8 = if state.rex_rm_1 { 8 } else { 0 };
        let rm_reg_2_offset: u8 = if state.rex_rm_2 { 8 } else { 0 };
        let mut seg: SegmentRegister = SegmentRegister::DEFAULT;
        unsafe {
            (*rm_oper).operand = OperandType::MEM;
            if mod_ != 3 && rm == 4 {
                let sib_byte: u8 = read_8(state);
                let base: u8 = sib_byte & 7;
                let index: u8 = sib_byte >> 3 & 7;
                (*rm_oper).scale = 1 << sib_byte >> 6;
                if mod_ != 0 || base != 5 {
                    (*rm_oper).components[0] = addr_reg_list[(base + rm_reg_1_offset) as usize];
                }
                if index + rm_reg_2_offset != 4 {
                    (*rm_oper).components[1] = addr_reg_list[(index + rm_reg_2_offset) as usize];
                }
                if mod_ == 2 {
                    (*rm_oper).immediate = read_signed_32(state);
                } else if mod_ == 1 {
                    (*rm_oper).immediate = read_signed_8(state);
                } else if mod_ == 0 && base == 5 {
                    (*rm_oper).immediate = read_signed_32(state);
                }
                if base + rm_reg_1_offset == 4 || base + rm_reg_1_offset == 5 {
                    seg = SegmentRegister::SS;
                } else {
                    seg = SegmentRegister::DS;
                }
            } else if mod_ == 3 {
                (*rm_oper).operand = reg_list[(rm + rm_reg_1_offset) as usize];
            } else if mod_ == 2 {
                (*rm_oper).components[0] = addr_reg_list[(rm + rm_reg_1_offset) as usize];
                (*rm_oper).immediate = read_signed_32(state);
                seg = if rm == 5 {
                    SegmentRegister::SS
                } else {
                    SegmentRegister::DS
                };
            } else if mod_ == 1 {
                (*rm_oper).components[0] = addr_reg_list[(rm + rm_reg_1_offset) as usize];
                (*rm_oper).immediate = read_signed_8(state);
                seg = if rm == 5 {
                    SegmentRegister::SS
                } else {
                    SegmentRegister::DS
                };
            } else if mod_ == 0 {
                if rm == 5 {
                    (*rm_oper).immediate = read_signed_32(state);
                    if state.addr_size == 8 {
                        state.rip_rel_fixup = &mut (*rm_oper).immediate as (*mut isize);
                    }
                } else {
                    (*rm_oper).components[0] = addr_reg_list[(rm + rm_reg_1_offset) as usize];
                }
                seg = SegmentRegister::DS;
            }
            if seg != SegmentRegister::DEFAULT {
                (*rm_oper).segment = get_final_segment(state, seg);
            }
        }
    }
}

fn decode_rm_reg(
    state: &mut DecodeState,
    rm_oper: *mut InstructionOperand,
    rm_reg_list: &[OperandType],
    rm_size: u16,
    reg_oper: *mut InstructionOperand,
    reg_list: &[OperandType],
    reg_size: u16,
) {
    let mut reg: u8 = 0;
    decode_rm(state, rm_oper, rm_reg_list, rm_size, &mut reg as (*mut u8));
    unsafe {
        if !reg_oper.is_null() {
            let reg_offset: u8 = if state.rex_reg { 8 } else { 0 };
            (*reg_oper).size = reg_size;
            (*reg_oper).operand = reg_list[(reg + reg_offset) as usize];
        }
    }
}

fn decode_reg_rm(state: &mut DecodeState) {
    let reg_list = get_reg_list_for_final_op_size(state);
    let size = match state.flags & DecodeFlags::REG_RM_SIZE_MASK {
        0 => state.final_op_size,
        DecodeFlags::REG_RM_2X_SIZE => state.final_op_size * 2,
        DecodeFlags::REG_RM_FAR_SIZE => state.final_op_size + 2,
        DecodeFlags::REG_RM_NO_SIZE => 0,
        _ => panic!("This isn't possible. This shouldn't be needed to suppress a warning."),
    };
    let operand0 = state.operand0;
    let operand1 = state.operand1;
    let final_op_size = state.final_op_size;
    decode_rm_reg(
        state,
        operand1,
        reg_list,
        size,
        operand0,
        reg_list,
        final_op_size,
    );
    unsafe {
        if size != state.final_op_size && ((*state.operand1).operand != OperandType::MEM) {
            state.invalid = true;
        }
    }
}

fn read_final_op_size(state: &mut DecodeState) -> isize {
    if state.flags & DecodeFlags::IMM_SX != 0 {
        read_signed_8(state)
    } else {
        match state.final_op_size {
            8 => read_signed_32(state),
            4 => read_32(state) as isize,
            2 => read_16(state) as isize,
            1 => read_8(state) as isize,
            _ => 0,
        }
    }
}

fn set_operand_to_imm(state: &mut DecodeState, oper: *mut InstructionOperand) {
    unsafe {
        (*oper).operand = OperandType::IMM;
        (*oper).size = state.final_op_size;
        (*oper).immediate = read_final_op_size(state);
    }
}

fn decode_reg_rm_imm(state: &mut DecodeState) {
    let reg_list = get_reg_list_for_final_op_size(state);
    let operand0 = state.operand0;
    let operand1 = state.operand1;
    let final_op_size = state.final_op_size;
    decode_rm_reg(
        state,
        operand1,
        reg_list,
        final_op_size,
        operand0,
        reg_list,
        final_op_size,
    );
    let imm_operand = &mut state.result.operands[2] as (*mut InstructionOperand);
    set_operand_to_imm(state, imm_operand);
}

fn decode_rm_reg_imm_8(state: &mut DecodeState) {
    let reg_list = get_reg_list_for_final_op_size(state);
    let operand0 = state.operand0;
    let operand1 = state.operand1;
    let final_op_size = state.final_op_size;
    decode_rm_reg(
        state,
        operand0,
        reg_list,
        final_op_size,
        operand1,
        reg_list,
        final_op_size,
    );
    let imm_operand = &mut state.result.operands[2] as (*mut InstructionOperand);
    set_operand_to_imm_8(state, imm_operand);
}

fn decode_rm_reg_cl(state: &mut DecodeState) {
    let reg_list = get_reg_list_for_final_op_size(state);
    let operand0 = state.operand0;
    let operand1 = state.operand1;
    let final_op_size = state.final_op_size;
    decode_rm_reg(
        state,
        operand0,
        reg_list,
        final_op_size,
        operand1,
        reg_list,
        final_op_size,
    );
    state.result.operands[2].operand = OperandType::REG_CL;
    state.result.operands[2].size = 1;
}

fn set_operand_to_eax_final_op_size(state: &DecodeState, oper: *mut InstructionOperand) {
    let reg_list = get_reg_list_for_final_op_size(state);
    unsafe {
        (*oper).operand = reg_list[0];
        (*oper).size = state.final_op_size;
    }
}

fn decode_eax_imm(state: &mut DecodeState) {
    let operand0 = state.operand0;
    set_operand_to_eax_final_op_size(state, operand0);
    let operand1 = state.operand1;
    set_operand_to_imm(state, operand1);
}

fn decode_push_pop_seg(state: &mut DecodeState) {
    unsafe {
        let offset: i32 = if *state.opcode.offset(-1) >= 0xa0 {
            // FS/GS
            -16
        } else {
            0
        };
        (*state.operand0).operand = OperandType::from_i32(
            OperandType::REG_ES as i32 + i32::from((*state.opcode.offset(-1)) >> 3) +
                offset,
        );
        (*state.operand0).size = state.op_size;
    }
}

fn set_operand_to_op_reg(state: &DecodeState, oper: *mut InstructionOperand) {
    let reg_list = get_reg_list_for_final_op_size(state);
    let reg_offset: usize = if state.rex_rm_1 { 8 } else { 0 };
    unsafe {
        (*oper).operand = reg_list[(*state.opcode.offset(-1) as usize & 7) + reg_offset];
        (*oper).size = state.final_op_size;
    }
}

fn decode_op_reg(state: &mut DecodeState) {
    set_operand_to_op_reg(state, state.operand0);
}

fn decode_eax_op_reg(state: &mut DecodeState) {
    set_operand_to_eax_final_op_size(state, state.operand0);
    set_operand_to_op_reg(state, state.operand1);
}

fn read_64(state: &mut DecodeState) -> usize {
    if state.len < 8 {
        state.invalid = true;
        state.insufficient_length = true;
        state.len = 0;
        0
    } else {
        unsafe {
            let old_val = (*state.opcode) as usize;
            state.opcode = state.opcode.offset(8);
            state.len = state.len.wrapping_sub(8);
            old_val
        }
    }
}

fn decode_op_reg_imm(state: &mut DecodeState) {
    set_operand_to_op_reg(state, state.operand0);
    unsafe {
        (*state.operand1).operand = OperandType::IMM;
        (*state.operand1).size = state.final_op_size;
        (*state.operand1).immediate = if state.op_size == 8 {
            read_64(state) as isize
        } else {
            read_final_op_size(state)
        };
    }
}

fn decode_nop(state: &mut DecodeState) {
    if state.rex_rm_1 {
        state.result.operation = InstructionOperation::XCHG;
        decode_eax_op_reg(state);
    }
}

fn decode_imm(state: &mut DecodeState) {
    let operand0 = state.operand0;
    set_operand_to_imm(state, operand0);
}

fn set_operand_to_imm_16(state: &mut DecodeState, oper: *mut InstructionOperand) {
    unsafe {
        (*oper).operand = OperandType::IMM;
        (*oper).size = 2;
        (*oper).immediate = read_16(state) as (isize);
    }
}

fn decode_imm_16_imm_8(state: &mut DecodeState) {
    let operand0 = state.operand0;
    set_operand_to_imm_16(state, operand0);
    let operand1 = state.operand1;
    set_operand_to_imm_8(state, operand1);
}

fn set_operand_to_es_edi(state: &DecodeState, oper: *mut InstructionOperand, size: u16) {
    let addr_reg_list = get_reg_list_for_addr_size(state);
    unsafe {
        (*oper).operand = OperandType::MEM;
        (*oper).components[0] = addr_reg_list[7];
        (*oper).size = size;
        (*oper).segment = SegmentRegister::ES;
    }
}

fn decode_edi_dx(state: &mut DecodeState) {
    set_operand_to_es_edi(state, state.operand0, state.final_op_size);
    unsafe {
        (*state.operand1).operand = OperandType::REG_DX;
        (*state.operand1).size = 2u16;
    }
}

fn set_operand_to_ds_esi(state: &DecodeState, oper: *mut InstructionOperand, size: u16) {
    let addr_reg_list = get_reg_list_for_addr_size(state);
    unsafe {
        (*oper).operand = OperandType::MEM;
        (*oper).components[0usize] = addr_reg_list[6];
        (*oper).size = size;
        (*oper).segment = get_final_segment(state, SegmentRegister::DS);
    }
}

fn decode_dx_esi(state: &mut DecodeState) {
    unsafe {
        (*state.operand0).operand = OperandType::REG_DX;
        (*state.operand0).size = 2u16;
    }
    set_operand_to_ds_esi(state, state.operand1, state.final_op_size);
}

fn read_signed_final_op_size(state: &mut DecodeState) -> isize {
    match state.final_op_size {
        4 | 8 => read_signed_32(state),
        2 => read_signed_16(state),
        1 => read_signed_8(state),
        _ => 0,
    }
}

fn decode_rel_imm(state: &mut DecodeState) {
    unsafe {
        (*state.operand0).operand = OperandType::IMM;
        (*state.operand0).size = state.op_size;
        (*state.operand0).immediate = read_signed_final_op_size(state);
        (*state.operand0).immediate =
            ((*state.operand0).immediate as (usize)).wrapping_add(state.addr.wrapping_add(
                ((state.opcode as (isize)).wrapping_sub(state.opcode_start as (isize)) /
                     ::std::mem::size_of::<u8>() as (isize)) as
                    (usize),
            )) as (isize);
    }
}

fn update_operation_for_addr_size(state: &mut DecodeState) {
    if state.addr_size == 4 {
        state.result.operation = InstructionOperation::from_i32(state.result.operation as i32 + 1);
    } else if state.addr_size == 8 {
        state.result.operation = InstructionOperation::from_i32(state.result.operation as i32 + 2);
    }
}

fn decode_rel_imm_addr_size(state: &mut DecodeState) {
    decode_rel_imm(state);
    update_operation_for_addr_size(state);
}

fn decode_group_rm(state: &mut DecodeState) {
    let operand0 = state.operand0;
    let reg_list = get_reg_list_for_final_op_size(state);
    let reg_size = state.final_op_size;
    let mut reg_field: u8 = 0;
    decode_rm(
        state,
        operand0,
        reg_list,
        reg_size,
        &mut reg_field as (*mut u8),
    );
    state.result.operation = GROUP_OPERATIONS[state.result.operation as usize][reg_field as usize];
}

fn decode_group_rm_imm(state: &mut DecodeState) {
    decode_group_rm(state);
    let operand1 = state.operand1;
    set_operand_to_imm(state, operand1);
}

fn decode_group_rm_imm_8v(state: &mut DecodeState) {
    decode_group_rm(state);
    let operand1 = state.operand1;
    set_operand_to_imm_8(state, operand1);
}

fn decode_group_rm_one(state: &mut DecodeState) {
    decode_group_rm(state);
    unsafe {
        (*state.operand1).operand = OperandType::IMM;
        (*state.operand1).size = 1;
        (*state.operand1).immediate = 1;
    }
}

fn decode_group_rm_cl(state: &mut DecodeState) {
    decode_group_rm(state);
    unsafe {
        (*state.operand1).operand = OperandType::REG_CL;
        (*state.operand1).size = 1;
    }
}

fn decode_group_f6f7(state: &mut DecodeState) {
    decode_group_rm(state);
    if state.result.operation == InstructionOperation::TEST {
        let operand1 = state.operand1;
        set_operand_to_imm(state, operand1);
    }
    // Check for valid locking semantics
    if state.result.flags & X86Flag::LOCK != 0 &&
        (state.result.operation != InstructionOperation::NOT) &&
        (state.result.operation != InstructionOperation::NEG)
    {
        state.invalid = true;
    }
}

fn decode_group_ff(state: &mut DecodeState) {
    if state.using64 {
        // Default to 64-bit for jumps and calls
        let rm: u8 = peek_8(state);
        let reg_field: u8 = rm >> 3 & 7;
        if reg_field >= 2 && reg_field <= 5 {
            state.final_op_size = {
                state.op_size = if state.op_prefix { 4 } else { 8 };
                state.op_size
            };
        }
    }
    decode_group_rm(state);
    // Check for valid jump/call semantics
    if state.result.operation == InstructionOperation::CALLF ||
        state.result.operation == InstructionOperation::JMPF
    {
        unsafe {
            if (*state.operand0).operand != OperandType::MEM {
                state.invalid = true;
            }
            (*state.operand0).size += 2;
        }
    }
    // Check for valid locking semantics
    if state.result.flags & X86Flag::LOCK != 0 &&
        (state.result.operation != InstructionOperation::INC) &&
        (state.result.operation != InstructionOperation::DEC)
    {
        state.invalid = true;
    }
}

fn decode_group_0f00(state: &mut DecodeState) {
    let rm: u8 = peek_8(state);
    let reg_field: u8 = rm >> 3 & 7;
    if reg_field >= 2 {
        state.op_size = 2;
    }
    decode_group_rm(state);
}

fn decode_group_0f01(state: &mut DecodeState) {
    let rm: u8 = peek_8(state);
    let mod_field: u8 = rm >> 6 & 3;
    let reg_field: u8 = rm >> 3 & 7;
    let rm_field: u8 = rm & 7;
    if mod_field == 3 && reg_field != 4 && reg_field != 6 {
        state.result.operation = GROUP_0F01_REG_OPERATIONS[rm_field as usize][reg_field as usize];
    } else {
        if reg_field < 4 {
            state.op_size = if state.using64 { 10 } else { 6 };
        } else if reg_field != 7 {
            state.op_size = 2;
        } else {
            state.op_size = 1;
        }
        decode_group_rm(state);
    }
}

fn decode_group_0fae(state: &mut DecodeState) {
    let rm: u8 = peek_8(state);
    let mod_field: u8 = rm >> 6 & 3;
    let reg_field: u8 = rm >> 3 & 7;
    if mod_field == 3 {
        state.result.operation = GROUP_OPERATIONS[(state.result.operation as usize + 1)]
            [reg_field as usize];
    } else {
        if reg_field & 2 == 0 {
            state.op_size = 512;
        } else if reg_field & 6 == 2 {
            state.op_size = 4;
        } else {
            state.op_size = 1;
        }
        decode_group_rm(state);
    }
}

fn decode_0fb8(state: &mut DecodeState) {
    if state.rep != RepPrefix::REPE {
        if state.using64 {
            state.op_size = if state.op_prefix { 4 } else { 8 };
        }
        state.final_op_size = get_final_op_size(state);
        decode_rel_imm(state);
    } else {
        decode_reg_rm(state);
    }
}

fn get_reg_list_for_op_size(state: &DecodeState) -> &'static [OperandType] {
    match state.op_size {
        8 => &REG64_LIST,
        4 => &REG32_LIST,
        2 => &REG16_LIST,
        _ => &INVALID_REG_LIST,
    }
}

fn decode_rms_reg_v(state: &mut DecodeState) {
    let operand0 = state.operand0;
    let reg_list = get_reg_list_for_op_size(state);
    let reg_size = state.op_size;
    let mut reg_field: u8 = 0;
    decode_rm(
        state,
        operand0,
        reg_list,
        reg_size,
        &mut reg_field as (*mut u8),
    );
    if reg_field >= 6 {
        state.invalid = true;
    }
    unsafe {
        (*state.operand1).operand =
            OperandType::from_i32(OperandType::REG_ES as i32 + i32::from(reg_field));
        (*state.operand1).size = 2;
        if state.result.operands[0].operand == OperandType::REG_CS {
            state.invalid = true;
        }
    }
}

fn decode_rm8(state: &mut DecodeState) {
    let operand0 = state.operand0;
    let reg_list = get_byte_reg_list(state);
    decode_rm(state, operand0, reg_list, 1, ptr::null_mut());
}

fn decode_rmv(state: &mut DecodeState) {
    let operand0 = state.operand0;
    let reg_list = get_reg_list_for_op_size(state);
    let reg_size = state.op_size;
    decode_rm(state, operand0, reg_list, reg_size, ptr::null_mut());
}

fn decode_far_imm(state: &mut DecodeState) {
    let operand1 = state.operand1;
    set_operand_to_imm(state, operand1);
    let operand0 = state.operand0;
    set_operand_to_imm_16(state, operand0);
}

fn read_addr_size(state: &mut DecodeState) -> isize {
    match state.addr_size {
        4 | 8 => read_32(state) as isize,
        2 => read_16(state) as isize,
        _ => 0,
    }
}

fn set_operand_to_imm_addr(state: &mut DecodeState, oper: *mut InstructionOperand) {
    unsafe {
        (*oper).operand = OperandType::MEM;
        (*oper).immediate = read_addr_size(state);
        (*oper).segment = get_final_segment(state, SegmentRegister::DS);
        (*oper).size = state.final_op_size;
    }
}

fn decode_eax_addr(state: &mut DecodeState) {
    set_operand_to_eax_final_op_size(state, state.operand0);
    let operand1 = state.operand1;
    set_operand_to_imm_addr(state, operand1);
}

fn decode_edi_esi(state: &mut DecodeState) {
    set_operand_to_es_edi(state, state.operand0, state.final_op_size);
    set_operand_to_ds_esi(state, state.operand1, state.final_op_size);
}

fn decode_edi_eax(state: &mut DecodeState) {
    set_operand_to_es_edi(state, state.operand0, state.final_op_size);
    set_operand_to_eax_final_op_size(state, state.operand1);
}

fn decode_eax_esi(state: &mut DecodeState) {
    set_operand_to_eax_final_op_size(state, state.operand0);
    set_operand_to_ds_esi(state, state.operand1, state.final_op_size);
}

fn decode_al_ebx_al(state: &mut DecodeState) {
    let reg_list = get_reg_list_for_addr_size(state);
    unsafe {
        (*state.operand0).operand = OperandType::REG_AL;
        (*state.operand0).size = 1;
        (*state.operand1).operand = OperandType::MEM;
        (*state.operand1).components[0] = reg_list[3];
        (*state.operand1).components[1] = OperandType::REG_AL;
        (*state.operand1).size = 1;
        (*state.operand1).segment = get_final_segment(state, SegmentRegister::DS);
    }
}

fn decode_eax_imm_8(state: &mut DecodeState) {
    let operand0 = state.operand0;
    set_operand_to_eax_final_op_size(state, operand0);
    let operand1 = state.operand1;
    set_operand_to_imm_8(state, operand1);
}

fn decode_eax_dx(state: &mut DecodeState) {
    let operand0 = state.operand0;
    set_operand_to_eax_final_op_size(state, operand0);
    unsafe {
        (*state.operand1).operand = OperandType::REG_DX;
        (*state.operand1).size = 2;
    }
}

fn decode_3dnow(state: &mut DecodeState) {
    let operand0 = state.operand0;
    let operand1 = state.operand1;
    decode_rm_reg(
        state,
        operand1,
        &MMX_REG_LIST,
        8,
        operand0,
        &MMX_REG_LIST,
        8,
    );
    let op = read_8(state);
    state.result.operation =
        match SPARSE_3DNOW_OPCODES.binary_search_by_key(&op, |entry| entry.opcode) {
            Ok(idx) => SPARSE_3DNOW_OPCODES[idx].operation,
            Err(_) => InstructionOperation::INVALID,
        }
}

fn decode_sse_prefix(state: &mut DecodeState) -> u8 {
    if state.op_prefix {
        state.op_prefix = false;
        1
    } else if state.rep == RepPrefix::REPNE {
        state.rep = RepPrefix::NONE;
        2
    } else if state.rep == RepPrefix::REPE {
        state.rep = RepPrefix::NONE;
        3
    } else {
        0
    }
}

fn get_operand_for_sse_entry_type(
    state: &DecodeState,
    entry_type: SSETableOperandType,
    operand_index: u8,
) -> *mut InstructionOperand {
    let operand_index = if entry_type == SSETableOperandType::SSE_128_FLIP {
        1 - operand_index
    } else {
        operand_index
    };
    if operand_index == 0 {
        state.operand0
    } else {
        state.operand1
    }
}

fn get_reg_list_for_sse_entry_type(
    state: &DecodeState,
    entry_type: SSETableOperandType,
) -> &'static [OperandType] {
    if entry_type == SSETableOperandType::GPR_32_OR_64 {
        (if state.op_size == 8 {
             &REG64_LIST
         } else {
             &REG32_LIST
         })
    } else if entry_type == SSETableOperandType::MMX_64 ||
               entry_type == SSETableOperandType::MMX_32
    {
        &MMX_REG_LIST
    } else {
        &XMM_REG_LIST
    }
}

fn get_size_for_sse_entry_type(state: &DecodeState, entry_type: SSETableOperandType) -> u16 {
    match entry_type {
        SSETableOperandType::GPR_32_OR_64 => if state.op_size == 8 { 8 } else { 4 },
        SSETableOperandType::MMX_64 |
        SSETableOperandType::SSE_64 => 8,
        SSETableOperandType::MMX_32 |
        SSETableOperandType::SSE_32 => 4,
        SSETableOperandType::SSE_16 => 2,
        _ => 16,
    }
}

fn update_operation_for_sse_entry_type(state: &mut DecodeState, entry_type: SSETableOperandType) {
    if entry_type == SSETableOperandType::GPR_32_OR_64 && (state.op_size == 8) {
        state.result.operation =
            InstructionOperation::from_i32(state.result.operation as (i32) + 1);
    }
}

fn decode_sse_table(state: &mut DecodeState) {
    let entry_type = decode_sse_prefix(state) as usize;
    let rm: u8 = peek_8(state);
    let mod_field: u8 = rm >> 6 & 3;
    let entry = &SSE_TABLE[state.result.operation as usize];
    let op_entry = if mod_field == 3 {
        &entry.reg_ops[entry_type]
    } else {
        &entry.mem_ops[entry_type]
    };
    state.result.operation = op_entry.operation;
    let operand1 = get_operand_for_sse_entry_type(state, op_entry.rm_type, 1);
    let rm_reg_list = get_reg_list_for_sse_entry_type(state, op_entry.rm_type);
    let rm_reg_size = get_size_for_sse_entry_type(state, op_entry.rm_type);
    let operand0 = get_operand_for_sse_entry_type(state, op_entry.reg_type, 0);
    let reg_list = get_reg_list_for_sse_entry_type(state, op_entry.reg_type);
    let reg_size = get_size_for_sse_entry_type(state, op_entry.reg_type);
    decode_rm_reg(
        state,
        operand1,
        rm_reg_list,
        rm_reg_size,
        operand0,
        reg_list,
        reg_size,
    );
    if state.flags & DecodeFlags::INC_OPERATION_FOR_64 != 0 {
        update_operation_for_sse_entry_type(state, op_entry.reg_type);
        update_operation_for_sse_entry_type(state, op_entry.rm_type);
    }
}

fn decode_see_table_imm_8(state: &mut DecodeState) {
    decode_sse_table(state);
    let operand = &mut state.result.operands[2] as (*mut InstructionOperand);
    set_operand_to_imm_8(state, operand);
}

fn decode_sse_table_mem_8(state: &mut DecodeState) {
    decode_sse_table(state);
    unsafe {
        if (*state.operand0).operand == OperandType::MEM {
            (*state.operand0).size = 1;
        }
        if (*state.operand1).operand == OperandType::MEM {
            (*state.operand1).size = 1;
        }
    }
}

fn get_size_for_sse_type(type_: u8) -> u16 {
    match type_ {
        2 => 8,
        3 => 4,
        _ => 16,
    }
}

fn decode_sse(state: &mut DecodeState) {
    let type_: u8 = decode_sse_prefix(state);
    let rm: u8 = peek_8(state);
    let mod_field: u8 = rm >> 6 & 3;
    state.result.operation =
        InstructionOperation::from_i32(state.result.operation as i32 + i32::from(type_));
    let size: u16 = if mod_field == 3 {
        16
    } else {
        get_size_for_sse_type(type_)
    };
    let operand0 = state.operand0;
    let operand1 = state.operand1;
    decode_rm_reg(
        state,
        operand1,
        &XMM_REG_LIST,
        size,
        operand0,
        &XMM_REG_LIST,
        16,
    );
}

fn decode_sse_single(state: &mut DecodeState) {
    let type_: u8 = decode_sse_prefix(state);
    if type_ == 1 || type_ == 2 {
        state.invalid = true;
    } else {
        state.result.operation =
            InstructionOperation::from_i32(state.result.operation as i32 + i32::from((type_) & 1));
        let operand0 = state.operand0;
        let operand1 = state.operand1;
        decode_rm_reg(
            state,
            operand1,
            &XMM_REG_LIST,
            16,
            operand0,
            &XMM_REG_LIST,
            16,
        );
    }
}

fn decode_sse_packed(state: &mut DecodeState) {
    let type_: u8 = decode_sse_prefix(state);
    if type_ == 2 || type_ == 3 {
        state.invalid = true;
    } else {
        state.result.operation =
            InstructionOperation::from_i32(state.result.operation as i32 + i32::from((type_) & 1));
        let operand0 = state.operand0;
        let operand1 = state.operand1;
        decode_rm_reg(
            state,
            operand1,
            &XMM_REG_LIST,
            16,
            operand0,
            &XMM_REG_LIST,
            16,
        );
    }
}

fn decode_mmx(state: &mut DecodeState) {
    let operand0 = state.operand0;
    let operand1 = state.operand1;
    if state.op_prefix {
        decode_rm_reg(
            state,
            operand1,
            &XMM_REG_LIST,
            16,
            operand0,
            &XMM_REG_LIST,
            16,
        );
    } else {
        decode_rm_reg(
            state,
            operand1,
            &MMX_REG_LIST,
            8,
            operand0,
            &MMX_REG_LIST,
            8,
        );
    }
}

fn decode_mmx_sse_only(state: &mut DecodeState) {
    if state.op_prefix {
        let operand0 = state.operand0;
        let operand1 = state.operand1;
        decode_rm_reg(
            state,
            operand1,
            &XMM_REG_LIST,
            16,
            operand0,
            &XMM_REG_LIST,
            16,
        );
    } else {
        state.invalid = true;
    }
}

fn decode_mmx_group(state: &mut DecodeState) {
    let mut reg_field: u8 = 0;
    if state.op_prefix {
        let operand0 = state.operand0;
        decode_rm(
            state,
            operand0,
            &XMM_REG_LIST,
            16,
            &mut reg_field as (*mut u8),
        );
        state.result.operation =
            MMX_GROUP_OPERATIONS[state.result.operation as usize][reg_field as (usize)][1];
    } else {
        let operand0 = state.operand0;
        decode_rm(
            state,
            operand0,
            &MMX_REG_LIST,
            8,
            &mut reg_field as (*mut u8),
        );
        state.result.operation =
            MMX_GROUP_OPERATIONS[state.result.operation as usize][reg_field as (usize)][0];
    }
    let operand1 = state.operand1;
    set_operand_to_imm_8(state, operand1);
}

fn decode_pinsrw(state: &mut DecodeState) {
    decode_see_table_imm_8(state);
    unsafe {
        if (*state.operand1).operand == OperandType::MEM {
            (*state.operand1).size = 2;
        }
    }
}

fn decode_reg_cr(state: &mut DecodeState) {
    if state.op_size == 2 {
        state.op_size = 4;
    }
    let reg_list = get_reg_list_for_op_size(state);
    let reg = read_8(state);
    if state.result.flags & X86Flag::LOCK != 0 {
        state.result.flags &= !X86Flag::LOCK;
        state.rex_reg = true;
    }
    unsafe {
        (*state.operand0).operand = reg_list[((reg & 7) + if (*state).rex_rm_1 { 8 } else { 0 }) as
                                                 usize];
        (*state.operand0).size = (*state).op_size;
        (*state.operand1).operand = OperandType::from_i32(
            state.result.operation as i32 + (i32::from(reg) >> 3 & 7) +
                if state.rex_reg { 8 } else { 0 },
        );
        (*state.operand1).size = state.op_size;
    }
    state.result.operation = InstructionOperation::MOV;
}

fn decode_mov_sxzx8(state: &mut DecodeState) {
    let operand0 = state.operand0;
    let operand1 = state.operand1;
    let rm_reg_list = get_byte_reg_list(state);
    let reg_list = get_reg_list_for_op_size(state);
    let reg_size = state.op_size;
    decode_rm_reg(
        state,
        operand1,
        rm_reg_list,
        1,
        operand0,
        reg_list,
        reg_size,
    );
}

fn decode_mov_sxzx16(state: &mut DecodeState) {
    let operand0 = state.operand0;
    let operand1 = state.operand1;
    let reg_list = get_reg_list_for_op_size(state);
    let reg_size = state.op_size;
    decode_rm_reg(
        state,
        operand1,
        &REG16_LIST,
        2,
        operand0,
        reg_list,
        reg_size,
    );
}

fn decode_mem_16(state: &mut DecodeState) {
    let operand0 = state.operand0;
    decode_rm(state, operand0, &REG32_LIST, 2, ptr::null_mut());
    unsafe {
        if (*state.operand0).operand != OperandType::MEM {
            state.invalid = true;
        }
    }
}

fn decode_mem_32(state: &mut DecodeState) {
    let operand0 = state.operand0;
    decode_rm(state, operand0, &REG32_LIST, 4, ptr::null_mut());
    unsafe {
        if (*state.operand0).operand != OperandType::MEM {
            state.invalid = true;
        }
    }
}

fn decode_mem_64(state: &mut DecodeState) {
    let operand0 = state.operand0;
    decode_rm(state, operand0, &REG32_LIST, 8, ptr::null_mut());
    unsafe {
        if (*state.operand0).operand != OperandType::MEM {
            state.invalid = true;
        }
    }
}

fn decode_mem_80(state: &mut DecodeState) {
    let operand0 = state.operand0;
    decode_rm(state, operand0, &REG32_LIST, 10, ptr::null_mut());
    unsafe {
        if (*state.operand0).operand != OperandType::MEM {
            state.invalid = true;
        }
    }
}

fn decode_mem_float_env(state: &mut DecodeState) {
    let operand0 = state.operand0;
    let rm_size = if state.op_size == 2 { 14 } else { 28 };
    decode_rm(state, operand0, &REG32_LIST, rm_size, ptr::null_mut());
    unsafe {
        if (*state.operand0).operand != OperandType::MEM {
            state.invalid = true;
        }
    }
}

fn decode_mem_float_save(state: &mut DecodeState) {
    let operand0 = state.operand0;
    let rm_size = if state.op_size == 2 { 94 } else { 108 };
    decode_rm(state, operand0, &REG32_LIST, rm_size, ptr::null_mut());
    unsafe {
        if (*state.operand0).operand != OperandType::MEM {
            state.invalid = true;
        }
    }
}

fn decode_fpu_reg(state: &mut DecodeState) {
    let operand0 = state.operand0;
    decode_rm(state, operand0, &FPU_REG_LIST, 10, ptr::null_mut());
}

fn decode_fpu_reg_st0(state: &mut DecodeState) {
    decode_fpu_reg(state);
    unsafe {
        (*state.operand1).operand = OperandType::REG_ST0;
        (*state.operand1).size = 10;
    }
}

fn decode_reg_group_no_operands(state: &mut DecodeState) {
    let rm_byte: u8 = read_8(state);
    state.result.operation = GROUP_OPERATIONS[state.result.operation as usize][(rm_byte & 7) as
                                                                                   usize];
}

fn decode_reg_group_ax(state: &mut DecodeState) {
    decode_reg_group_no_operands(state);
    unsafe {
        (*state.operand0).operand = OperandType::REG_AX;
        (*state.operand0).size = 2u16;
    }
}

fn decode_cmp_xch_8b(state: &mut DecodeState) {
    let rm: u8 = peek_8(state);
    let reg_field: u8 = rm >> 3 & 7;
    if reg_field == 1 {
        if state.op_size == 2 {
            state.op_size = 4;
        } else if state.op_size == 8 {
            state.result.operation = InstructionOperation::CMPXCH16B;
        }
        let operand0 = state.operand0;
        let rm_size = state.op_size * 2;
        let reg_list = get_reg_list_for_op_size(state);
        decode_rm(state, operand0, reg_list, rm_size, ptr::null_mut());
    } else if reg_field == 6 {
        if state.op_prefix {
            state.result.operation = InstructionOperation::VMCLEAR;
        } else if state.rep == RepPrefix::REPE {
            state.result.operation = InstructionOperation::VMXON;
        } else {
            state.result.operation = InstructionOperation::VMPTRLD;
        }
        let operand0 = state.operand0;
        decode_rm(state, operand0, &REG64_LIST, 8, ptr::null_mut());
    } else if reg_field == 7 {
        state.result.operation = InstructionOperation::VMPTRST;
        let operand0 = state.operand0;
        decode_rm(state, operand0, &REG64_LIST, 8, ptr::null_mut());
    } else {
        state.invalid = true;
    }
    unsafe {
        if (*state.operand0).operand != OperandType::MEM {
            state.invalid = true;
        }
    }
}

fn decode_mov_nti(state: &mut DecodeState) {
    if state.op_size == 2 {
        state.op_size = 4;
    }
    let operand0 = state.operand0;
    let operand1 = state.operand1;
    let op_size = state.op_size;
    let reg_list = get_reg_list_for_op_size(state);
    decode_rm_reg(
        state,
        operand0,
        reg_list,
        op_size,
        operand1,
        reg_list,
        op_size,
    );
    unsafe {
        if (*state.operand0).operand != OperandType::MEM {
            state.invalid = true;
        }
    }
}

fn decode_crc_32(state: &mut DecodeState) {
    let src_reg_list = get_reg_list_for_final_op_size(state);
    let dest_reg_list = if (*state).op_size == 8 {
        &REG64_LIST
    } else {
        &REG32_LIST
    };
    let dest_size: u16 = if state.op_size == 8 { 8 } else { 4 };
    let operand0 = state.operand0;
    let operand1 = state.operand1;
    let final_op_size = state.final_op_size;
    decode_rm_reg(
        state,
        operand1,
        src_reg_list,
        final_op_size,
        operand0,
        dest_reg_list,
        dest_size,
    );
}

fn decode_arpl(state: &mut DecodeState) {
    if state.using64 {
        // In 64-bit, ARPL is repurposed to MOVSXD
        let reg_list = get_reg_list_for_final_op_size(state);
        state.result.operation = InstructionOperation::MOVSXD;
        let operand0 = state.operand0;
        let operand1 = state.operand1;
        let final_op_size = state.final_op_size;
        decode_rm_reg(
            state,
            operand1,
            &REG32_LIST,
            4,
            operand0,
            reg_list,
            final_op_size,
        );
    } else {
        // ARPL instruction
        state.operand0 = &mut state.result.operands[1] as (*mut InstructionOperand);
        state.operand1 = &mut state.result.operands[0] as (*mut InstructionOperand);
        state.final_op_size = 2;
        decode_reg_rm(state);
    }
}

fn process_prefixes(state: &mut DecodeState) {
    let mut rex: u8 = 0;
    let mut addr_prefix: bool = false;
    loop {
        if !!state.invalid {
            break;
        }
        let prefix: u8 = read_8(state);
        if state.invalid {
            break;
        }
        if prefix >= 0x26 && (prefix <= 0x3e) && (prefix & 7 == 6) {
            // Segment prefix
            let prefix = i32::from(prefix);
            state.result.segment =
                SegmentRegister::from_i32(SegmentRegister::ES as i32 + ((prefix >> 3) - 4));
        } else if prefix == 0x64 || prefix == 0x65 {
            // FS/GS prefix
            let prefix = i32::from(prefix);
            state.result.segment =
                SegmentRegister::from_i32(SegmentRegister::ES as i32 + (prefix - 0x60));
        } else if prefix == 0x66 {
            state.op_prefix = true;
            state.result.flags |= X86Flag::OPSIZE;
        } else if prefix == 0x67 {
            addr_prefix = true;
            state.result.flags |= X86Flag::ADDRSIZE;
        } else if prefix == 0xf0 {
            state.result.flags |= X86Flag::LOCK;
        } else if prefix == 0xf2 {
            state.rep = RepPrefix::REPNE;
        } else if prefix == 0xf3 {
            state.rep = RepPrefix::REPE;
        } else {
            if !(state.using64 && (prefix >= 0x40) && (prefix <= 0x4f)) {
                // Not a prefix, continue instruction processing.
                state.opcode = unsafe { state.opcode.offset(-1) };
                state.len = state.len.wrapping_add(1);
                break;
            }
            // REX prefix
            rex = prefix;
            continue;
        }
        // Force ignore REX unless it is the last prefix
        rex = 0;
    }
    if state.op_prefix {
        state.op_size = if state.op_size == 2 { 4 } else { 2 };
    }
    if addr_prefix {
        state.addr_size = if state.addr_size == 4 { 2 } else { 4 };
    }
    if rex != 0 {
        // REX prefix found before opcode
        state.rex = true;
        state.rex_rm_1 = rex & 1 != 0;
        state.rex_rm_2 = rex & 2 != 0;
        state.rex_reg = rex & 4 != 0;
        if rex & 8 != 0 {
            state.op_size = 8;
        }
    }
}

fn finish_disassemble(state: &mut DecodeState) {
    state.result.length = (state.opcode as usize).wrapping_sub(state.opcode_start as usize) /
        ::std::mem::size_of::<u8>();
    if !state.rip_rel_fixup.is_null() {
        unsafe {
            *state.rip_rel_fixup = (*state.rip_rel_fixup as (usize)).wrapping_add(
                state.addr.wrapping_add(
                    state
                        .result
                        .length,
                ),
            ) as (isize);
        }
    }
    if state.insufficient_length && (state.original_length < 15) {
        state.result.flags |= X86Flag::INSUFFICIENT_LENGTH;
    }
}

/// Disassemble a single x86 instruction in 16 bit mode from a stream of
/// opcodes.
///
/// ```
/// use burst::x86::*;
///
/// let data = [0u8, 0u8];
/// if let Ok(instr) = disassemble_16(&data, 0, data.len()) {
///     // ...
/// }
/// ```
pub fn disassemble_16(opcode: &[u8], addr: usize, max_length: usize) -> Result<Instruction, ()> {
    let max_length = cmp::min(max_length, 15);
    let mut state = DecodeState {
        opcode_start: opcode.as_ptr(),
        opcode: opcode.as_ptr(),
        addr: addr,
        len: max_length,
        original_length: max_length,
        addr_size: 2,
        op_size: 2,
        using64: false,
        ..Default::default()
    };
    process_prefixes(&mut state);
    let next_opcode = read_8(&mut state);
    process_opcode(&mut state, &MAIN_OPCODE_MAP, next_opcode);
    finish_disassemble(&mut state);
    if state.invalid {
        Err(())
    } else {
        Ok(state.result)
    }
}

/// Disassemble a single x86 instruction in 32 bit mode from a stream of
/// opcodes.
///
/// ```
/// use burst::x86::*;
///
/// let data = [0u8, 0u8];
/// if let Ok(instr) = disassemble_32(&data, 0, data.len()) {
///     // ...
/// }
/// ```
pub fn disassemble_32(opcode: &[u8], addr: usize, max_length: usize) -> Result<Instruction, ()> {
    let max_length = cmp::min(max_length, 15);
    let mut state = DecodeState {
        opcode_start: opcode.as_ptr(),
        opcode: opcode.as_ptr(),
        addr: addr,
        len: max_length,
        original_length: max_length,
        addr_size: 4,
        op_size: 4,
        using64: false,
        ..Default::default()
    };
    process_prefixes(&mut state);
    let next_opcode = read_8(&mut state);
    process_opcode(&mut state, &MAIN_OPCODE_MAP, next_opcode);
    finish_disassemble(&mut state);
    if state.invalid {
        Err(())
    } else {
        Ok(state.result)
    }
}

/// Disassemble a single x86 instruction in 64 bit mode from a stream of
/// opcodes.
///
/// ```
/// use burst::x86::*;
///
/// let data = [0u8, 0u8];
/// if let Ok(instr) = disassemble_64(&data, 0, data.len()) {
///     // ...
/// }
/// ```
pub fn disassemble_64(opcode: &[u8], addr: usize, max_length: usize) -> Result<Instruction, ()> {
    let max_length = cmp::min(max_length, 15);
    let mut state = DecodeState {
        opcode_start: opcode.as_ptr(),
        opcode: opcode.as_ptr(),
        addr: addr,
        len: max_length,
        original_length: max_length,
        addr_size: 8,
        op_size: 4,
        using64: true,
        ..Default::default()
    };
    process_prefixes(&mut state);
    let next_opcode = read_8(&mut state);
    process_opcode(&mut state, &MAIN_OPCODE_MAP, next_opcode);
    finish_disassemble(&mut state);
    if state.invalid {
        Err(())
    } else {
        Ok(state.result)
    }
}

fn write_operand(
    stream: &mut fmt::Write,
    type_: OperandType,
    scale: u8,
    plus: bool,
) -> fmt::Result {
    if plus {
        try!(stream.write_char('+'));
    }
    try!(stream.write_str(OPERAND_TYPE_TABLE[type_ as (usize)].name));
    if scale != 1 {
        try!(stream.write_char('*'));
        try!(stream.write_char((scale + b'0') as char));
    }
    Ok(())
}

fn get_size_string(size: u16) -> &'static str {
    match size {
        16 => "oword ",
        10 => "tword ",
        8 => "qword ",
        6 => "fword ",
        4 => "dword ",
        2 => "word ",
        1 => "byte ",
        _ => "",
    }
}

/// Write an `Instruction` to a stream.
///
/// The `fmt` string can contain these specifiers:
///
/// * `%a`: Shows the address of the instruction as passed
///   in the `addr` parameter.
/// * `%b`: Shows the bytes of the instruction. The `opcode`
///   parameter must contain the same contents as the call to
///   to the disassemble function that produced the instruction.
/// * `%i`: Shows the operation mnemonic.
/// * `%o`: Shows the operands.
///
/// In the future, this may be replaced by something that is more
/// like the `std::fmt` features of the Rust standard library.
///
/// This currently doesn't support any configurable syntax support
/// for AT&T style syntax.
///
/// ```
/// use burst::x86::*;
///
/// let data = [0u8, 0u8];
/// if let Ok(instr) = disassemble_64(&data, 0, data.len()) {
///     let mut out = String::new();
///     format_instruction_string(&mut out, "%a %b %i %o", &data, 0, &instr);
///     assert_eq!("0000000000000000 0000 add byte [rax], al", out);
/// }
/// ```
pub fn format_instruction_string(
    stream: &mut fmt::Write,
    fmt: &str,
    opcode: &[u8],
    addr: usize,
    instr: &Instruction,
) -> fmt::Result {
    let fmt = fmt.chars().collect::<Vec<_>>();
    let mut f = 0;
    loop {
        if f >= fmt.len() {
            break;
        }
        if fmt[f] == '%' {
            let mut width: usize = 0;
            f += 1;
            if f >= fmt.len() {
                break;
            }
            if fmt[f] == 'a' {
                if width == 0 {
                    width = ::std::mem::size_of::<*mut ::std::os::raw::c_void>() * 2;
                }
                try!(write!(stream, "{:0width$x}", addr, width = width));
            } else if fmt[f] == 'b' {
                for byte in opcode.iter().take(instr.length) {
                    try!(write!(stream, "{:02x}", byte));
                }
                for _i in instr.length..width {
                    try!(stream.write_str("  "));
                }
            } else if fmt[f] == 'i' {
                if instr.flags & X86Flag::ANY_REP != 0 {
                    try!(stream.write_str("rep"));
                    if instr.flags & X86Flag::REPNE != 0 {
                        try!(stream.write_char('n'));
                    }
                    if instr.flags & (X86Flag::REPNE | X86Flag::REPE) != 0 {
                        try!(stream.write_char('e'));
                    }
                    try!(stream.write_char('b'));
                }
                if instr.flags & X86Flag::LOCK != 0 {
                    try!(stream.write_str("lock "));
                }
                try!(stream.write_str(instr.operation.mnemonic()));
            } else if fmt[f] == 'o' {
                let mut i: usize = 0;
                loop {
                    if !(i < 3) {
                        break;
                    }
                    if instr.operands[i].operand == OperandType::NONE {
                        break;
                    }
                    if i != 0 {
                        try!(stream.write_str(", "));
                    }
                    if instr.operands[i].operand == OperandType::IMM {
                        try!(write!(
                            stream,
                            "{:#width$x}",
                            instr.operands[i].immediate,
                            width = (instr.operands[i].size * 2) as usize
                        ));
                    } else if instr.operands[i].operand == OperandType::MEM {
                        let mut plus: bool = false;
                        try!(stream.write_str(get_size_string(instr.operands[i].size)));
                        if instr.segment != SegmentRegister::DEFAULT ||
                            instr.operands[i].segment == SegmentRegister::ES
                        {
                            try!(write_operand(
                                stream,
                                OperandType::from_i32(
                                    instr.operands[i].segment as (i32) +
                                        OperandType::REG_ES as (i32),
                                ),
                                1,
                                false,
                            ));
                            try!(stream.write_char(':'));
                        }
                        try!(stream.write_char('['));
                        if instr.operands[i].components[0] != OperandType::NONE {
                            try!(write_operand(
                                stream,
                                instr.operands[i].components[0],
                                1,
                                false,
                            ));
                            plus = true;
                        }
                        if instr.operands[i].components[1] != OperandType::NONE {
                            try!(write_operand(
                                stream,
                                instr.operands[i].components[1],
                                instr.operands[i].scale,
                                plus,
                            ));
                            plus = true;
                        }
                        if instr.operands[i].immediate != 0 ||
                            instr.operands[i].components[0] == OperandType::NONE &&
                                (instr.operands[i].components[1] == OperandType::NONE)
                        {
                            if plus && (instr.operands[i].immediate >= -0x80) &&
                                (instr.operands[i].immediate < 0)
                            {
                                try!(write!(stream, "-{:#02x}", -instr.operands[i].immediate));
                            } else if plus && (instr.operands[i].immediate > 0) &&
                                       (instr.operands[i].immediate <= 0x7f)
                            {
                                try!(write!(stream, "+{:#02x}", instr.operands[i].immediate));
                            } else {
                                if plus {
                                    try!(stream.write_char('+'));
                                }
                                try!(write!(stream, "{:#08x}", instr.operands[i].immediate));
                            }
                        }
                        try!(stream.write_char(']'));
                    } else {
                        try!(write_operand(stream, instr.operands[i].operand, 1, false));
                    }
                    i += 1;
                }
            } else if !(fmt[f] >= '0' && fmt[f] <= '9') {
                try!(stream.write_char(fmt[f]));
            }
        } else {
            try!(stream.write_char(fmt[f]));
        }
        f += 1;
    }
    Ok(())
}
