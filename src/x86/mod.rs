// Licensed under the 2-Clause BSD license <LICENSE or
// https://opensource.org/licenses/BSD-2-Clause>. This
// file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_camel_case_types,
         non_snake_case)]

mod instruction_operations;
mod operand_types;

pub use self::instruction_operations::*;
pub use self::operand_types::*;

use std::fmt;
use std::ptr;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(i32)]
pub enum SegmentRegister {
    SEG_ES = 0i32,
    SEG_CS = 1i32,
    SEG_SS = 2i32,
    SEG_DS = 3i32,
    SEG_FS = 4i32,
    SEG_GS = 5i32,
    SEG_DEFAULT = 7i32,
}

impl SegmentRegister {
    pub fn from_i32(i: i32) -> Self {
        match i {
            0 => SegmentRegister::SEG_ES,
            1 => SegmentRegister::SEG_CS,
            2 => SegmentRegister::SEG_SS,
            3 => SegmentRegister::SEG_DS,
            4 => SegmentRegister::SEG_FS,
            5 => SegmentRegister::SEG_GS,
            7 => SegmentRegister::SEG_DEFAULT,
            _ => panic!("Unknown segment register {}", i),
        }
    }
}

impl Default for SegmentRegister {
    fn default() -> Self {
        SegmentRegister::SEG_DEFAULT
    }
}

#[derive(Default, Debug)]
#[repr(C)]
pub struct InstructionOperand {
    pub operand: OperandType,
    pub components: [OperandType; 2],
    pub scale: u8,
    pub size: u16,
    pub immediate: isize,
    pub segment: SegmentRegister,
}

#[derive(Default, Debug)]
#[repr(C)]
pub struct Instruction {
    pub operation: InstructionOperation,
    pub operands: [InstructionOperand; 3],
    pub flags: u32,
    pub segment: SegmentRegister,
    pub length: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(i32)]
enum RepPrefix {
    REP_PREFIX_NONE = 0i32,
    REP_PREFIX_REPNE,
    REP_PREFIX_REPE,
}

#[derive(Debug)]
#[repr(C)]
struct DecodeState {
    result: *mut Instruction,
    operand0: *mut InstructionOperand,
    operand1: *mut InstructionOperand,
    opcodeStart: *const u8,
    opcode: *const u8,
    addr: usize,
    len: usize,
    origLen: usize,
    opSize: u16,
    finalOpSize: u16,
    addrSize: u16,
    flags: u32,
    invalid: bool,
    insufficientLength: bool,
    opPrefix: bool,
    rep: RepPrefix,
    using64: bool,
    rex: bool,
    rexRM1: bool,
    rexRM2: bool,
    rexReg: bool,
    ripRelFixup: *mut isize,
}

impl Default for DecodeState {
    fn default() -> Self {
        DecodeState {
            result: ptr::null_mut(),
            operand0: ptr::null_mut(),
            operand1: ptr::null_mut(),
            opcodeStart: ptr::null(),
            opcode: ptr::null(),
            addr: 0usize,
            len: 0usize,
            origLen: 0usize,
            opSize: 0u16,
            finalOpSize: 0u16,
            addrSize: 0u16,
            flags: 0u32,
            invalid: false,
            insufficientLength: false,
            opPrefix: false,
            rep: RepPrefix::REP_PREFIX_NONE,
            using64: false,
            rex: false,
            rexRM1: false,
            rexRM2: false,
            rexReg: false,
            ripRelFixup: ptr::null_mut(),
        }
    }
}

#[repr(C)]
struct InstructionEncoding {
    pub operation: u16,
    pub flags: u16,
    pub func: unsafe fn(&mut DecodeState),
}

static MAIN_OPCODE_MAP: [InstructionEncoding; 256] = [
    InstructionEncoding {
        operation: InstructionOperation::ADD as (u16),
        flags: (0x100i32 | 0x200i32 | 0x20i32) as (u16),
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::ADD as (u16),
        flags: (0x200i32 | 0x20i32) as (u16),
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::ADD as (u16),
        flags: 0x100u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::ADD as (u16),
        flags: 0u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::ADD as (u16),
        flags: 0x100u16,
        func: DecodeEaxImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::ADD as (u16),
        flags: 0u16,
        func: DecodeEaxImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUSH as (u16),
        flags: 0u16,
        func: DecodePushPopSeg,
    },
    InstructionEncoding {
        operation: InstructionOperation::POP as (u16),
        flags: 0u16,
        func: DecodePushPopSeg,
    },
    InstructionEncoding {
        operation: InstructionOperation::OR as (u16),
        flags: (0x100i32 | 0x200i32 | 0x20i32) as (u16),
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::OR as (u16),
        flags: (0x200i32 | 0x20i32) as (u16),
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::OR as (u16),
        flags: 0x100u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::OR as (u16),
        flags: 0u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::OR as (u16),
        flags: 0x100u16,
        func: DecodeEaxImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::OR as (u16),
        flags: 0u16,
        func: DecodeEaxImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUSH as (u16),
        flags: 0u16,
        func: DecodePushPopSeg,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0u16,
        func: DecodeTwoByte,
    },
    InstructionEncoding {
        operation: InstructionOperation::ADC as (u16),
        flags: (0x100i32 | 0x200i32 | 0x20i32) as (u16),
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::ADC as (u16),
        flags: (0x200i32 | 0x20i32) as (u16),
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::ADC as (u16),
        flags: 0x100u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::ADC as (u16),
        flags: 0u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::ADC as (u16),
        flags: 0x100u16,
        func: DecodeEaxImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::ADC as (u16),
        flags: 0u16,
        func: DecodeEaxImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUSH as (u16),
        flags: 0u16,
        func: DecodePushPopSeg,
    },
    InstructionEncoding {
        operation: InstructionOperation::POP as (u16),
        flags: 0u16,
        func: DecodePushPopSeg,
    },
    InstructionEncoding {
        operation: InstructionOperation::SBB as (u16),
        flags: (0x100i32 | 0x200i32 | 0x20i32) as (u16),
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::SBB as (u16),
        flags: (0x200i32 | 0x20i32) as (u16),
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::SBB as (u16),
        flags: 0x100u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::SBB as (u16),
        flags: 0u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::SBB as (u16),
        flags: 0x100u16,
        func: DecodeEaxImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::SBB as (u16),
        flags: 0u16,
        func: DecodeEaxImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUSH as (u16),
        flags: 0u16,
        func: DecodePushPopSeg,
    },
    InstructionEncoding {
        operation: InstructionOperation::POP as (u16),
        flags: 0u16,
        func: DecodePushPopSeg,
    },
    InstructionEncoding {
        operation: InstructionOperation::AND as (u16),
        flags: (0x100i32 | 0x200i32 | 0x20i32) as (u16),
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::AND as (u16),
        flags: (0x200i32 | 0x20i32) as (u16),
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::AND as (u16),
        flags: 0x100u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::AND as (u16),
        flags: 0u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::AND as (u16),
        flags: 0x100u16,
        func: DecodeEaxImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::AND as (u16),
        flags: 0u16,
        func: DecodeEaxImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0u16,
        func: InvalidDecode,
    },
    InstructionEncoding {
        operation: InstructionOperation::DAA as (u16),
        flags: 0u16,
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: InstructionOperation::SUB as (u16),
        flags: (0x100i32 | 0x200i32 | 0x20i32) as (u16),
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::SUB as (u16),
        flags: (0x200i32 | 0x20i32) as (u16),
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::SUB as (u16),
        flags: 0x100u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::SUB as (u16),
        flags: 0u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::SUB as (u16),
        flags: 0x100u16,
        func: DecodeEaxImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::SUB as (u16),
        flags: 0u16,
        func: DecodeEaxImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0u16,
        func: InvalidDecode,
    },
    InstructionEncoding {
        operation: InstructionOperation::DAS as (u16),
        flags: 0u16,
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: InstructionOperation::XOR as (u16),
        flags: (0x100i32 | 0x200i32 | 0x20i32) as (u16),
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::XOR as (u16),
        flags: (0x200i32 | 0x20i32) as (u16),
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::XOR as (u16),
        flags: 0x100u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::XOR as (u16),
        flags: 0u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::XOR as (u16),
        flags: 0x100u16,
        func: DecodeEaxImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::XOR as (u16),
        flags: 0u16,
        func: DecodeEaxImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0u16,
        func: InvalidDecode,
    },
    InstructionEncoding {
        operation: InstructionOperation::AAA as (u16),
        flags: 0u16,
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMP as (u16),
        flags: (0x100i32 | 0x200i32) as (u16),
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMP as (u16),
        flags: 0x200u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMP as (u16),
        flags: 0x100u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMP as (u16),
        flags: 0u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMP as (u16),
        flags: 0x100u16,
        func: DecodeEaxImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMP as (u16),
        flags: 0u16,
        func: DecodeEaxImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0u16,
        func: InvalidDecode,
    },
    InstructionEncoding {
        operation: InstructionOperation::AAS as (u16),
        flags: 0u16,
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: InstructionOperation::INC as (u16),
        flags: 0u16,
        func: DecodeOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::INC as (u16),
        flags: 0u16,
        func: DecodeOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::INC as (u16),
        flags: 0u16,
        func: DecodeOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::INC as (u16),
        flags: 0u16,
        func: DecodeOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::INC as (u16),
        flags: 0u16,
        func: DecodeOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::INC as (u16),
        flags: 0u16,
        func: DecodeOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::INC as (u16),
        flags: 0u16,
        func: DecodeOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::INC as (u16),
        flags: 0u16,
        func: DecodeOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::DEC as (u16),
        flags: 0u16,
        func: DecodeOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::DEC as (u16),
        flags: 0u16,
        func: DecodeOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::DEC as (u16),
        flags: 0u16,
        func: DecodeOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::DEC as (u16),
        flags: 0u16,
        func: DecodeOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::DEC as (u16),
        flags: 0u16,
        func: DecodeOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::DEC as (u16),
        flags: 0u16,
        func: DecodeOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::DEC as (u16),
        flags: 0u16,
        func: DecodeOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::DEC as (u16),
        flags: 0u16,
        func: DecodeOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUSH as (u16),
        flags: 0x8000u16,
        func: DecodeOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUSH as (u16),
        flags: 0x8000u16,
        func: DecodeOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUSH as (u16),
        flags: 0x8000u16,
        func: DecodeOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUSH as (u16),
        flags: 0x8000u16,
        func: DecodeOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUSH as (u16),
        flags: 0x8000u16,
        func: DecodeOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUSH as (u16),
        flags: 0x8000u16,
        func: DecodeOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUSH as (u16),
        flags: 0x8000u16,
        func: DecodeOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUSH as (u16),
        flags: 0x8000u16,
        func: DecodeOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::POP as (u16),
        flags: 0x8000u16,
        func: DecodeOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::POP as (u16),
        flags: 0x8000u16,
        func: DecodeOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::POP as (u16),
        flags: 0x8000u16,
        func: DecodeOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::POP as (u16),
        flags: 0x8000u16,
        func: DecodeOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::POP as (u16),
        flags: 0x8000u16,
        func: DecodeOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::POP as (u16),
        flags: 0x8000u16,
        func: DecodeOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::POP as (u16),
        flags: 0x8000u16,
        func: DecodeOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::POP as (u16),
        flags: 0x8000u16,
        func: DecodeOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUSHA as (u16),
        flags: (0x4000i32 | 0x1000i32) as (u16),
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: InstructionOperation::POPA as (u16),
        flags: (0x4000i32 | 0x1000i32) as (u16),
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: InstructionOperation::BOUND as (u16),
        flags: 0x1u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::ARPL as (u16),
        flags: 0u16,
        func: DecodeArpl,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0u16,
        func: InvalidDecode,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0u16,
        func: InvalidDecode,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0u16,
        func: InvalidDecode,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0u16,
        func: InvalidDecode,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUSH as (u16),
        flags: 0x8000u16,
        func: DecodeImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::IMUL as (u16),
        flags: 0u16,
        func: DecodeRegRMImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUSH as (u16),
        flags: (0x400i32 | 0x8000i32) as (u16),
        func: DecodeImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::IMUL as (u16),
        flags: 0x400u16,
        func: DecodeRegRMImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::INSB as (u16),
        flags: (0x100i32 | 0x1000i32 | 0x40i32) as (u16),
        func: DecodeEdiDx,
    },
    InstructionEncoding {
        operation: InstructionOperation::INSW as (u16),
        flags: (0x1000i32 | 0x40i32) as (u16),
        func: DecodeEdiDx,
    },
    InstructionEncoding {
        operation: InstructionOperation::OUTSB as (u16),
        flags: (0x100i32 | 0x1000i32 | 0x40i32) as (u16),
        func: DecodeDxEsi,
    },
    InstructionEncoding {
        operation: InstructionOperation::OUTSW as (u16),
        flags: (0x1000i32 | 0x40i32) as (u16),
        func: DecodeDxEsi,
    },
    InstructionEncoding {
        operation: InstructionOperation::JO as (u16),
        flags: (0x100i32 | 0x8000i32) as (u16),
        func: DecodeRelImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JNO as (u16),
        flags: (0x100i32 | 0x8000i32) as (u16),
        func: DecodeRelImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JB as (u16),
        flags: (0x100i32 | 0x8000i32) as (u16),
        func: DecodeRelImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JAE as (u16),
        flags: (0x100i32 | 0x8000i32) as (u16),
        func: DecodeRelImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JE as (u16),
        flags: (0x100i32 | 0x8000i32) as (u16),
        func: DecodeRelImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JNE as (u16),
        flags: (0x100i32 | 0x8000i32) as (u16),
        func: DecodeRelImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JBE as (u16),
        flags: (0x100i32 | 0x8000i32) as (u16),
        func: DecodeRelImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JA as (u16),
        flags: (0x100i32 | 0x8000i32) as (u16),
        func: DecodeRelImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JS as (u16),
        flags: (0x100i32 | 0x8000i32) as (u16),
        func: DecodeRelImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JNS as (u16),
        flags: (0x100i32 | 0x8000i32) as (u16),
        func: DecodeRelImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JPE as (u16),
        flags: (0x100i32 | 0x8000i32) as (u16),
        func: DecodeRelImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JPO as (u16),
        flags: (0x100i32 | 0x8000i32) as (u16),
        func: DecodeRelImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JL as (u16),
        flags: (0x100i32 | 0x8000i32) as (u16),
        func: DecodeRelImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JGE as (u16),
        flags: (0x100i32 | 0x8000i32) as (u16),
        func: DecodeRelImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JLE as (u16),
        flags: (0x100i32 | 0x8000i32) as (u16),
        func: DecodeRelImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JG as (u16),
        flags: (0x100i32 | 0x8000i32) as (u16),
        func: DecodeRelImm,
    },
    InstructionEncoding {
        operation: 0u16,
        flags: (0x100i32 | 0x20i32) as (u16),
        func: DecodeGroupRMImm,
    },
    InstructionEncoding {
        operation: 0u16,
        flags: 0x20u16,
        func: DecodeGroupRMImm,
    },
    InstructionEncoding {
        operation: 0u16,
        flags: (0x100i32 | 0x4000i32 | 0x20i32) as (u16),
        func: DecodeGroupRMImm,
    },
    InstructionEncoding {
        operation: 0u16,
        flags: (0x400i32 | 0x20i32) as (u16),
        func: DecodeGroupRMImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::TEST as (u16),
        flags: (0x100i32 | 0x200i32) as (u16),
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::TEST as (u16),
        flags: 0x200u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::XCHG as (u16),
        flags: (0x100i32 | 0x200i32 | 0x20i32) as (u16),
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::XCHG as (u16),
        flags: (0x200i32 | 0x20i32) as (u16),
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: (0x100i32 | 0x200i32) as (u16),
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: 0x200u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: 0x100u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: 0u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: 0u16,
        func: DecodeRMSRegV,
    },
    InstructionEncoding {
        operation: InstructionOperation::LEA as (u16),
        flags: 0x3u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: 0x200u16,
        func: DecodeRMSRegV,
    },
    InstructionEncoding {
        operation: InstructionOperation::POP as (u16),
        flags: 0x8000u16,
        func: DecodeRMV,
    },
    InstructionEncoding {
        operation: InstructionOperation::NOP as (u16),
        flags: 0u16,
        func: DecodeNop,
    },
    InstructionEncoding {
        operation: InstructionOperation::XCHG as (u16),
        flags: 0u16,
        func: DecodeEaxOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::XCHG as (u16),
        flags: 0u16,
        func: DecodeEaxOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::XCHG as (u16),
        flags: 0u16,
        func: DecodeEaxOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::XCHG as (u16),
        flags: 0u16,
        func: DecodeEaxOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::XCHG as (u16),
        flags: 0u16,
        func: DecodeEaxOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::XCHG as (u16),
        flags: 0u16,
        func: DecodeEaxOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::XCHG as (u16),
        flags: 0u16,
        func: DecodeEaxOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::CBW as (u16),
        flags: 0x1000u16,
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: InstructionOperation::CWD as (u16),
        flags: 0x1000u16,
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: InstructionOperation::CALLF as (u16),
        flags: 0x4000u16,
        func: DecodeFarImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::FWAIT as (u16),
        flags: 0u16,
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUSHF as (u16),
        flags: (0x8000i32 | 0x1000i32) as (u16),
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: InstructionOperation::POPF as (u16),
        flags: (0x8000i32 | 0x1000i32) as (u16),
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: InstructionOperation::SAHF as (u16),
        flags: 0u16,
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: InstructionOperation::LAHF as (u16),
        flags: 0u16,
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: 0x100u16,
        func: DecodeEaxAddr,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: 0u16,
        func: DecodeEaxAddr,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: (0x100i32 | 0x200i32) as (u16),
        func: DecodeEaxAddr,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: 0x200u16,
        func: DecodeEaxAddr,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOVSB as (u16),
        flags: (0x100i32 | 0x1000i32 | 0x40i32) as (u16),
        func: DecodeEdiEsi,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOVSW as (u16),
        flags: (0x1000i32 | 0x40i32) as (u16),
        func: DecodeEdiEsi,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMPSB as (u16),
        flags: (0x100i32 | 0x200i32 | 0x1000i32 | 0x80i32) as (u16),
        func: DecodeEdiEsi,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMPSW as (u16),
        flags: (0x200i32 | 0x1000i32 | 0x80i32) as (u16),
        func: DecodeEdiEsi,
    },
    InstructionEncoding {
        operation: InstructionOperation::TEST as (u16),
        flags: 0x100u16,
        func: DecodeEaxImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::TEST as (u16),
        flags: 0u16,
        func: DecodeEaxImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::STOSB as (u16),
        flags: (0x100i32 | 0x1000i32 | 0x40i32) as (u16),
        func: DecodeEdiEax,
    },
    InstructionEncoding {
        operation: InstructionOperation::STOSW as (u16),
        flags: (0x1000i32 | 0x40i32) as (u16),
        func: DecodeEdiEax,
    },
    InstructionEncoding {
        operation: InstructionOperation::LODSB as (u16),
        flags: (0x100i32 | 0x1000i32 | 0x40i32) as (u16),
        func: DecodeEaxEsi,
    },
    InstructionEncoding {
        operation: InstructionOperation::LODSW as (u16),
        flags: (0x1000i32 | 0x40i32) as (u16),
        func: DecodeEaxEsi,
    },
    InstructionEncoding {
        operation: InstructionOperation::SCASB as (u16),
        flags: (0x100i32 | 0x200i32 | 0x1000i32 | 0x80i32) as (u16),
        func: DecodeEdiEax,
    },
    InstructionEncoding {
        operation: InstructionOperation::SCASW as (u16),
        flags: (0x200i32 | 0x1000i32 | 0x80i32) as (u16),
        func: DecodeEdiEax,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: 0x100u16,
        func: DecodeOpRegImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: 0x100u16,
        func: DecodeOpRegImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: 0x100u16,
        func: DecodeOpRegImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: 0x100u16,
        func: DecodeOpRegImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: 0x100u16,
        func: DecodeOpRegImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: 0x100u16,
        func: DecodeOpRegImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: 0x100u16,
        func: DecodeOpRegImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: 0x100u16,
        func: DecodeOpRegImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: 0u16,
        func: DecodeOpRegImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: 0u16,
        func: DecodeOpRegImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: 0u16,
        func: DecodeOpRegImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: 0u16,
        func: DecodeOpRegImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: 0u16,
        func: DecodeOpRegImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: 0u16,
        func: DecodeOpRegImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: 0u16,
        func: DecodeOpRegImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOV as (u16),
        flags: 0u16,
        func: DecodeOpRegImm,
    },
    InstructionEncoding {
        operation: 1u16,
        flags: 0x100u16,
        func: DecodeGroupRMImm,
    },
    InstructionEncoding {
        operation: 1u16,
        flags: 0u16,
        func: DecodeGroupRMImm8V,
    },
    InstructionEncoding {
        operation: InstructionOperation::RETN as (u16),
        flags: 0x2000u16,
        func: DecodeImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::RETN as (u16),
        flags: 0u16,
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: InstructionOperation::LES as (u16),
        flags: 0x2u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::LDS as (u16),
        flags: 0x2u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: 2u16,
        flags: 0x100u16,
        func: DecodeGroupRMImm,
    },
    InstructionEncoding {
        operation: 2u16,
        flags: 0u16,
        func: DecodeGroupRMImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::ENTER as (u16),
        flags: 0u16,
        func: DecodeImm16Imm8,
    },
    InstructionEncoding {
        operation: InstructionOperation::LEAVE as (u16),
        flags: 0u16,
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: InstructionOperation::RETF as (u16),
        flags: 0x2000u16,
        func: DecodeImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::RETF as (u16),
        flags: 0u16,
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: InstructionOperation::INT3 as (u16),
        flags: 0u16,
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: InstructionOperation::INT as (u16),
        flags: 0x100u16,
        func: DecodeImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::INTO as (u16),
        flags: 0u16,
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: InstructionOperation::IRET as (u16),
        flags: 0u16,
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: 1u16,
        flags: 0x100u16,
        func: DecodeGroupRMOne,
    },
    InstructionEncoding {
        operation: 1u16,
        flags: 0u16,
        func: DecodeGroupRMOne,
    },
    InstructionEncoding {
        operation: 1u16,
        flags: 0x100u16,
        func: DecodeGroupRMCl,
    },
    InstructionEncoding {
        operation: 1u16,
        flags: 0u16,
        func: DecodeGroupRMCl,
    },
    InstructionEncoding {
        operation: InstructionOperation::AAM as (u16),
        flags: 0x100u16,
        func: DecodeImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::AAD as (u16),
        flags: 0x100u16,
        func: DecodeImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0u16,
        func: InvalidDecode,
    },
    InstructionEncoding {
        operation: InstructionOperation::XLAT as (u16),
        flags: 0u16,
        func: DecodeAlEbxAl,
    },
    InstructionEncoding {
        operation: 0u16,
        flags: 0u16,
        func: DecodeFpu,
    },
    InstructionEncoding {
        operation: 1u16,
        flags: 0u16,
        func: DecodeFpu,
    },
    InstructionEncoding {
        operation: 2u16,
        flags: 0u16,
        func: DecodeFpu,
    },
    InstructionEncoding {
        operation: 3u16,
        flags: 0u16,
        func: DecodeFpu,
    },
    InstructionEncoding {
        operation: 4u16,
        flags: 0u16,
        func: DecodeFpu,
    },
    InstructionEncoding {
        operation: 5u16,
        flags: 0u16,
        func: DecodeFpu,
    },
    InstructionEncoding {
        operation: 6u16,
        flags: 0u16,
        func: DecodeFpu,
    },
    InstructionEncoding {
        operation: 7u16,
        flags: 0u16,
        func: DecodeFpu,
    },
    InstructionEncoding {
        operation: InstructionOperation::LOOPNE as (u16),
        flags: (0x100i32 | 0x8000i32) as (u16),
        func: DecodeRelImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::LOOPE as (u16),
        flags: (0x100i32 | 0x8000i32) as (u16),
        func: DecodeRelImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::LOOP as (u16),
        flags: (0x100i32 | 0x8000i32) as (u16),
        func: DecodeRelImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JCXZ as (u16),
        flags: (0x100i32 | 0x8000i32) as (u16),
        func: DecodeRelImmAddrSize,
    },
    InstructionEncoding {
        operation: InstructionOperation::IN as (u16),
        flags: 0x100u16,
        func: DecodeEaxImm8,
    },
    InstructionEncoding {
        operation: InstructionOperation::IN as (u16),
        flags: 0u16,
        func: DecodeEaxImm8,
    },
    InstructionEncoding {
        operation: InstructionOperation::OUT as (u16),
        flags: (0x100i32 | 0x200i32) as (u16),
        func: DecodeEaxImm8,
    },
    InstructionEncoding {
        operation: InstructionOperation::OUT as (u16),
        flags: 0x200u16,
        func: DecodeEaxImm8,
    },
    InstructionEncoding {
        operation: InstructionOperation::CALL as (u16),
        flags: 0x8000u16,
        func: DecodeRelImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JMP as (u16),
        flags: 0x8000u16,
        func: DecodeRelImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JMPF as (u16),
        flags: 0x4000u16,
        func: DecodeFarImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JMP as (u16),
        flags: (0x100i32 | 0x8000i32) as (u16),
        func: DecodeRelImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::IN as (u16),
        flags: 0x100u16,
        func: DecodeEaxDx,
    },
    InstructionEncoding {
        operation: InstructionOperation::IN as (u16),
        flags: 0u16,
        func: DecodeEaxDx,
    },
    InstructionEncoding {
        operation: InstructionOperation::OUT as (u16),
        flags: (0x100i32 | 0x200i32) as (u16),
        func: DecodeEaxDx,
    },
    InstructionEncoding {
        operation: InstructionOperation::OUT as (u16),
        flags: 0x200u16,
        func: DecodeEaxDx,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0u16,
        func: InvalidDecode,
    },
    InstructionEncoding {
        operation: InstructionOperation::INT1 as (u16),
        flags: 0u16,
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0u16,
        func: InvalidDecode,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0u16,
        func: InvalidDecode,
    },
    InstructionEncoding {
        operation: InstructionOperation::HLT as (u16),
        flags: 0u16,
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMC as (u16),
        flags: 0u16,
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: 3u16,
        flags: (0x100i32 | 0x20i32) as (u16),
        func: DecodeGroupF6F7,
    },
    InstructionEncoding {
        operation: 3u16,
        flags: 0x20u16,
        func: DecodeGroupF6F7,
    },
    InstructionEncoding {
        operation: InstructionOperation::CLC as (u16),
        flags: 0u16,
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: InstructionOperation::STC as (u16),
        flags: 0u16,
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: InstructionOperation::CLI as (u16),
        flags: 0u16,
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: InstructionOperation::STI as (u16),
        flags: 0u16,
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: InstructionOperation::CLD as (u16),
        flags: 0u16,
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: InstructionOperation::STD as (u16),
        flags: 0u16,
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: 4u16,
        flags: (0x100i32 | 0x20i32) as (u16),
        func: DecodeGroupRM,
    },
    InstructionEncoding {
        operation: 5u16,
        flags: 0x20u16,
        func: DecodeGroupFF,
    },
];

static TWO_BYTE_OPCODE_MAP: [InstructionEncoding; 256] = [
    InstructionEncoding {
        operation: 6u16,
        flags: 0u16,
        func: DecodeGroup0F00,
    },
    InstructionEncoding {
        operation: 7u16,
        flags: 0u16,
        func: DecodeGroup0F01,
    },
    InstructionEncoding {
        operation: InstructionOperation::LAR as (u16),
        flags: 0u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::LSL as (u16),
        flags: 0u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0u16,
        func: InvalidDecode,
    },
    InstructionEncoding {
        operation: InstructionOperation::SYSCALL as (u16),
        flags: 0u16,
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: InstructionOperation::CLTS as (u16),
        flags: 0u16,
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: InstructionOperation::SYSRET as (u16),
        flags: 0u16,
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVD as (u16),
        flags: 0u16,
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: InstructionOperation::WBINVD as (u16),
        flags: 0u16,
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0u16,
        func: InvalidDecode,
    },
    InstructionEncoding {
        operation: InstructionOperation::UD2 as (u16),
        flags: 0u16,
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0u16,
        func: InvalidDecode,
    },
    InstructionEncoding {
        operation: 8u16,
        flags: 0x3u16,
        func: DecodeGroupRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::FEMMS as (u16),
        flags: 0u16,
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: 0u16,
        flags: 0u16,
        func: Decode3DNow,
    },
    InstructionEncoding {
        operation: 0u16,
        flags: 0u16,
        func: DecodeSSETable,
    },
    InstructionEncoding {
        operation: 0u16,
        flags: 0x200u16,
        func: DecodeSSETable,
    },
    InstructionEncoding {
        operation: 1u16,
        flags: 0u16,
        func: DecodeSSETable,
    },
    InstructionEncoding {
        operation: 2u16,
        flags: 0x200u16,
        func: DecodeSSETable,
    },
    InstructionEncoding {
        operation: 3u16,
        flags: 0u16,
        func: DecodeSSETable,
    },
    InstructionEncoding {
        operation: 4u16,
        flags: 0u16,
        func: DecodeSSETable,
    },
    InstructionEncoding {
        operation: 5u16,
        flags: 0u16,
        func: DecodeSSETable,
    },
    InstructionEncoding {
        operation: 6u16,
        flags: 0x200u16,
        func: DecodeSSETable,
    },
    InstructionEncoding {
        operation: 9u16,
        flags: 0x3u16,
        func: DecodeGroupRM,
    },
    InstructionEncoding {
        operation: 10u16,
        flags: 0x3u16,
        func: DecodeGroupRM,
    },
    InstructionEncoding {
        operation: 10u16,
        flags: 0x3u16,
        func: DecodeGroupRM,
    },
    InstructionEncoding {
        operation: 10u16,
        flags: 0x3u16,
        func: DecodeGroupRM,
    },
    InstructionEncoding {
        operation: 10u16,
        flags: 0x3u16,
        func: DecodeGroupRM,
    },
    InstructionEncoding {
        operation: 10u16,
        flags: 0x3u16,
        func: DecodeGroupRM,
    },
    InstructionEncoding {
        operation: 10u16,
        flags: 0x3u16,
        func: DecodeGroupRM,
    },
    InstructionEncoding {
        operation: 10u16,
        flags: 0x3u16,
        func: DecodeGroupRM,
    },
    InstructionEncoding {
        operation: OperandType::REG_CR0 as (u16),
        flags: (0x8000i32 | 0x20i32) as (u16),
        func: DecodeRegCR,
    },
    InstructionEncoding {
        operation: OperandType::REG_DR0 as (u16),
        flags: (0x8000i32 | 0x20i32) as (u16),
        func: DecodeRegCR,
    },
    InstructionEncoding {
        operation: OperandType::REG_CR0 as (u16),
        flags: (0x200i32 | 0x8000i32 | 0x20i32) as (u16),
        func: DecodeRegCR,
    },
    InstructionEncoding {
        operation: OperandType::REG_DR0 as (u16),
        flags: (0x200i32 | 0x8000i32 | 0x20i32) as (u16),
        func: DecodeRegCR,
    },
    InstructionEncoding {
        operation: OperandType::REG_TR0 as (u16),
        flags: (0x8000i32 | 0x20i32) as (u16),
        func: DecodeRegCR,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0u16,
        func: InvalidDecode,
    },
    InstructionEncoding {
        operation: OperandType::REG_TR0 as (u16),
        flags: (0x200i32 | 0x8000i32 | 0x20i32) as (u16),
        func: DecodeRegCR,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0u16,
        func: InvalidDecode,
    },
    InstructionEncoding {
        operation: 7u16,
        flags: 0u16,
        func: DecodeSSETable,
    },
    InstructionEncoding {
        operation: 7u16,
        flags: 0x200u16,
        func: DecodeSSETable,
    },
    InstructionEncoding {
        operation: 8u16,
        flags: 0u16,
        func: DecodeSSETable,
    },
    InstructionEncoding {
        operation: 9u16,
        flags: 0x200u16,
        func: DecodeSSETable,
    },
    InstructionEncoding {
        operation: 10u16,
        flags: 0u16,
        func: DecodeSSETable,
    },
    InstructionEncoding {
        operation: 11u16,
        flags: 0u16,
        func: DecodeSSETable,
    },
    InstructionEncoding {
        operation: 12u16,
        flags: 0u16,
        func: DecodeSSETable,
    },
    InstructionEncoding {
        operation: 13u16,
        flags: 0u16,
        func: DecodeSSETable,
    },
    InstructionEncoding {
        operation: InstructionOperation::WRMSR as (u16),
        flags: 0u16,
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: InstructionOperation::RDTSC as (u16),
        flags: 0u16,
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: InstructionOperation::RDMSR as (u16),
        flags: 0u16,
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: InstructionOperation::RDPMC as (u16),
        flags: 0u16,
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: InstructionOperation::SYSENTER as (u16),
        flags: 0u16,
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: InstructionOperation::SYSEXIT as (u16),
        flags: 0u16,
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0u16,
        func: InvalidDecode,
    },
    InstructionEncoding {
        operation: InstructionOperation::GETSEC as (u16),
        flags: 0u16,
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0u16,
        func: InvalidDecode,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0u16,
        func: InvalidDecode,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0u16,
        func: InvalidDecode,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0u16,
        func: InvalidDecode,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0u16,
        func: InvalidDecode,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0u16,
        func: InvalidDecode,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0u16,
        func: InvalidDecode,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0u16,
        func: InvalidDecode,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMOVO as (u16),
        flags: 0u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMOVNO as (u16),
        flags: 0u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMOVB as (u16),
        flags: 0u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMOVAE as (u16),
        flags: 0u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMOVE as (u16),
        flags: 0u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMOVNE as (u16),
        flags: 0u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMOVBE as (u16),
        flags: 0u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMOVA as (u16),
        flags: 0u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMOVS as (u16),
        flags: 0u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMOVNS as (u16),
        flags: 0u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMOVPE as (u16),
        flags: 0u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMOVPO as (u16),
        flags: 0u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMOVL as (u16),
        flags: 0u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMOVGE as (u16),
        flags: 0u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMOVLE as (u16),
        flags: 0u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMOVG as (u16),
        flags: 0u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: 14u16,
        flags: 0u16,
        func: DecodeSSETable,
    },
    InstructionEncoding {
        operation: InstructionOperation::SQRTPS as (u16),
        flags: 0u16,
        func: DecodeSSE,
    },
    InstructionEncoding {
        operation: InstructionOperation::RSQRTPS as (u16),
        flags: 0u16,
        func: DecodeSSESingle,
    },
    InstructionEncoding {
        operation: InstructionOperation::RCPPS as (u16),
        flags: 0u16,
        func: DecodeSSESingle,
    },
    InstructionEncoding {
        operation: InstructionOperation::ANDPS as (u16),
        flags: 0u16,
        func: DecodeSSEPacked,
    },
    InstructionEncoding {
        operation: InstructionOperation::ANDNPS as (u16),
        flags: 0u16,
        func: DecodeSSEPacked,
    },
    InstructionEncoding {
        operation: InstructionOperation::ORPS as (u16),
        flags: 0u16,
        func: DecodeSSEPacked,
    },
    InstructionEncoding {
        operation: InstructionOperation::XORPS as (u16),
        flags: 0u16,
        func: DecodeSSEPacked,
    },
    InstructionEncoding {
        operation: InstructionOperation::ADDPS as (u16),
        flags: 0u16,
        func: DecodeSSE,
    },
    InstructionEncoding {
        operation: InstructionOperation::MULPS as (u16),
        flags: 0u16,
        func: DecodeSSE,
    },
    InstructionEncoding {
        operation: 15u16,
        flags: 0u16,
        func: DecodeSSETable,
    },
    InstructionEncoding {
        operation: 16u16,
        flags: 0u16,
        func: DecodeSSETable,
    },
    InstructionEncoding {
        operation: InstructionOperation::SUBPS as (u16),
        flags: 0u16,
        func: DecodeSSE,
    },
    InstructionEncoding {
        operation: InstructionOperation::MINPS as (u16),
        flags: 0u16,
        func: DecodeSSE,
    },
    InstructionEncoding {
        operation: InstructionOperation::DIVPS as (u16),
        flags: 0u16,
        func: DecodeSSE,
    },
    InstructionEncoding {
        operation: InstructionOperation::MAXPS as (u16),
        flags: 0u16,
        func: DecodeSSE,
    },
    InstructionEncoding {
        operation: 17u16,
        flags: 0u16,
        func: DecodeSSETable,
    },
    InstructionEncoding {
        operation: 18u16,
        flags: 0u16,
        func: DecodeSSETable,
    },
    InstructionEncoding {
        operation: 19u16,
        flags: 0u16,
        func: DecodeSSETable,
    },
    InstructionEncoding {
        operation: InstructionOperation::PACKSSWB as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PCMPGTB as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PCMPGTW as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PCMPGTD as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PACKUSWB as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUNPCKHBW as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUNPCKHWD as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUNPCKHDQ as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PACKSSDW as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUNPCKLQDQ as (u16),
        flags: 0u16,
        func: DecodeMMXSSEOnly,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUNPCKHQDQ as (u16),
        flags: 0u16,
        func: DecodeMMXSSEOnly,
    },
    InstructionEncoding {
        operation: 20u16,
        flags: 0x800u16,
        func: DecodeSSETable,
    },
    InstructionEncoding {
        operation: 21u16,
        flags: 0u16,
        func: DecodeSSETable,
    },
    InstructionEncoding {
        operation: 22u16,
        flags: 0u16,
        func: DecodeSSETableImm8,
    },
    InstructionEncoding {
        operation: 0u16,
        flags: 0u16,
        func: DecodeMMXGroup,
    },
    InstructionEncoding {
        operation: 1u16,
        flags: 0u16,
        func: DecodeMMXGroup,
    },
    InstructionEncoding {
        operation: 2u16,
        flags: 0u16,
        func: DecodeMMXGroup,
    },
    InstructionEncoding {
        operation: InstructionOperation::PCMPEQB as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PCMPEQW as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PCMPEQD as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::EMMS as (u16),
        flags: 0u16,
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: InstructionOperation::VMREAD as (u16),
        flags: (0x200i32 | 0x8000i32) as (u16),
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::VMWRITE as (u16),
        flags: (0x200i32 | 0x8000i32) as (u16),
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0u16,
        func: InvalidDecode,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0u16,
        func: InvalidDecode,
    },
    InstructionEncoding {
        operation: 23u16,
        flags: 0u16,
        func: DecodeSSETable,
    },
    InstructionEncoding {
        operation: 24u16,
        flags: 0u16,
        func: DecodeSSETable,
    },
    InstructionEncoding {
        operation: 25u16,
        flags: (0x800i32 | 0x200i32) as (u16),
        func: DecodeSSETable,
    },
    InstructionEncoding {
        operation: 21u16,
        flags: 0x200u16,
        func: DecodeSSETable,
    },
    InstructionEncoding {
        operation: InstructionOperation::JO as (u16),
        flags: 0x8000u16,
        func: DecodeRelImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JNO as (u16),
        flags: 0x8000u16,
        func: DecodeRelImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JB as (u16),
        flags: 0x8000u16,
        func: DecodeRelImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JAE as (u16),
        flags: 0x8000u16,
        func: DecodeRelImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JE as (u16),
        flags: 0x8000u16,
        func: DecodeRelImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JNE as (u16),
        flags: 0x8000u16,
        func: DecodeRelImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JBE as (u16),
        flags: 0x8000u16,
        func: DecodeRelImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JA as (u16),
        flags: 0x8000u16,
        func: DecodeRelImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JS as (u16),
        flags: 0x8000u16,
        func: DecodeRelImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JNS as (u16),
        flags: 0x8000u16,
        func: DecodeRelImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JPE as (u16),
        flags: 0x8000u16,
        func: DecodeRelImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JPO as (u16),
        flags: 0x8000u16,
        func: DecodeRelImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JL as (u16),
        flags: 0x8000u16,
        func: DecodeRelImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JGE as (u16),
        flags: 0x8000u16,
        func: DecodeRelImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JLE as (u16),
        flags: 0x8000u16,
        func: DecodeRelImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::JG as (u16),
        flags: 0x8000u16,
        func: DecodeRelImm,
    },
    InstructionEncoding {
        operation: InstructionOperation::SETO as (u16),
        flags: 0u16,
        func: DecodeRM8,
    },
    InstructionEncoding {
        operation: InstructionOperation::SETNO as (u16),
        flags: 0u16,
        func: DecodeRM8,
    },
    InstructionEncoding {
        operation: InstructionOperation::SETB as (u16),
        flags: 0u16,
        func: DecodeRM8,
    },
    InstructionEncoding {
        operation: InstructionOperation::SETAE as (u16),
        flags: 0u16,
        func: DecodeRM8,
    },
    InstructionEncoding {
        operation: InstructionOperation::SETE as (u16),
        flags: 0u16,
        func: DecodeRM8,
    },
    InstructionEncoding {
        operation: InstructionOperation::SETNE as (u16),
        flags: 0u16,
        func: DecodeRM8,
    },
    InstructionEncoding {
        operation: InstructionOperation::SETBE as (u16),
        flags: 0u16,
        func: DecodeRM8,
    },
    InstructionEncoding {
        operation: InstructionOperation::SETA as (u16),
        flags: 0u16,
        func: DecodeRM8,
    },
    InstructionEncoding {
        operation: InstructionOperation::SETS as (u16),
        flags: 0u16,
        func: DecodeRM8,
    },
    InstructionEncoding {
        operation: InstructionOperation::SETNS as (u16),
        flags: 0u16,
        func: DecodeRM8,
    },
    InstructionEncoding {
        operation: InstructionOperation::SETPE as (u16),
        flags: 0u16,
        func: DecodeRM8,
    },
    InstructionEncoding {
        operation: InstructionOperation::SETPO as (u16),
        flags: 0u16,
        func: DecodeRM8,
    },
    InstructionEncoding {
        operation: InstructionOperation::SETL as (u16),
        flags: 0u16,
        func: DecodeRM8,
    },
    InstructionEncoding {
        operation: InstructionOperation::SETGE as (u16),
        flags: 0u16,
        func: DecodeRM8,
    },
    InstructionEncoding {
        operation: InstructionOperation::SETLE as (u16),
        flags: 0u16,
        func: DecodeRM8,
    },
    InstructionEncoding {
        operation: InstructionOperation::SETG as (u16),
        flags: 0u16,
        func: DecodeRM8,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUSH as (u16),
        flags: 0u16,
        func: DecodePushPopSeg,
    },
    InstructionEncoding {
        operation: InstructionOperation::POP as (u16),
        flags: 0u16,
        func: DecodePushPopSeg,
    },
    InstructionEncoding {
        operation: InstructionOperation::CPUID as (u16),
        flags: 0u16,
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: InstructionOperation::BT as (u16),
        flags: 0x200u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::SHLD as (u16),
        flags: 0u16,
        func: DecodeRMRegImm8,
    },
    InstructionEncoding {
        operation: InstructionOperation::SHLD as (u16),
        flags: 0u16,
        func: DecodeRMRegCL,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0u16,
        func: InvalidDecode,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0u16,
        func: InvalidDecode,
    },
    InstructionEncoding {
        operation: InstructionOperation::PUSH as (u16),
        flags: 0u16,
        func: DecodePushPopSeg,
    },
    InstructionEncoding {
        operation: InstructionOperation::POP as (u16),
        flags: 0u16,
        func: DecodePushPopSeg,
    },
    InstructionEncoding {
        operation: InstructionOperation::RSM as (u16),
        flags: 0u16,
        func: DecodeNoOperands,
    },
    InstructionEncoding {
        operation: InstructionOperation::BTS as (u16),
        flags: 0x200u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::SHRD as (u16),
        flags: 0u16,
        func: DecodeRMRegImm8,
    },
    InstructionEncoding {
        operation: InstructionOperation::SHRD as (u16),
        flags: 0u16,
        func: DecodeRMRegCL,
    },
    InstructionEncoding {
        operation: 24u16,
        flags: 0u16,
        func: DecodeGroup0FAE,
    },
    InstructionEncoding {
        operation: InstructionOperation::IMUL as (u16),
        flags: 0u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMPXCHG as (u16),
        flags: (0x100i32 | 0x200i32 | 0x20i32) as (u16),
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMPXCHG as (u16),
        flags: (0x200i32 | 0x20i32) as (u16),
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::LSS as (u16),
        flags: 0x2u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::BTR as (u16),
        flags: 0x200u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::LFS as (u16),
        flags: 0x2u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::LGS as (u16),
        flags: 0x2u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOVZX as (u16),
        flags: 0u16,
        func: DecodeMovSXZX8,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOVZX as (u16),
        flags: 0u16,
        func: DecodeMovSXZX16,
    },
    InstructionEncoding {
        operation: InstructionOperation::POPCNT as (u16),
        flags: 0u16,
        func: Decode0FB8,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0u16,
        func: InvalidDecode,
    },
    InstructionEncoding {
        operation: 11u16,
        flags: 0u16,
        func: DecodeGroupRMImm8V,
    },
    InstructionEncoding {
        operation: InstructionOperation::BTC as (u16),
        flags: 0x200u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::BSF as (u16),
        flags: 0u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::BSR as (u16),
        flags: 0u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOVSX as (u16),
        flags: 0u16,
        func: DecodeMovSXZX8,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOVSX as (u16),
        flags: 0u16,
        func: DecodeMovSXZX16,
    },
    InstructionEncoding {
        operation: InstructionOperation::XADD as (u16),
        flags: (0x100i32 | 0x200i32) as (u16),
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: InstructionOperation::XADD as (u16),
        flags: 0x200u16,
        func: DecodeRegRM,
    },
    InstructionEncoding {
        operation: 26u16,
        flags: 0u16,
        func: DecodeSSETableImm8,
    },
    InstructionEncoding {
        operation: InstructionOperation::MOVNTI as (u16),
        flags: 0u16,
        func: DecodeMovNti,
    },
    InstructionEncoding {
        operation: 27u16,
        flags: 0u16,
        func: DecodePinsrw,
    },
    InstructionEncoding {
        operation: 28u16,
        flags: 0x200u16,
        func: DecodeSSETableImm8,
    },
    InstructionEncoding {
        operation: 29u16,
        flags: 0u16,
        func: DecodeSSETableImm8,
    },
    InstructionEncoding {
        operation: InstructionOperation::CMPXCH8B as (u16),
        flags: 0u16,
        func: DecodeCmpXch8B,
    },
    InstructionEncoding {
        operation: InstructionOperation::BSWAP as (u16),
        flags: 0u16,
        func: DecodeOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::BSWAP as (u16),
        flags: 0u16,
        func: DecodeOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::BSWAP as (u16),
        flags: 0u16,
        func: DecodeOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::BSWAP as (u16),
        flags: 0u16,
        func: DecodeOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::BSWAP as (u16),
        flags: 0u16,
        func: DecodeOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::BSWAP as (u16),
        flags: 0u16,
        func: DecodeOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::BSWAP as (u16),
        flags: 0u16,
        func: DecodeOpReg,
    },
    InstructionEncoding {
        operation: InstructionOperation::BSWAP as (u16),
        flags: 0u16,
        func: DecodeOpReg,
    },
    InstructionEncoding {
        operation: 30u16,
        flags: 0u16,
        func: DecodeSSETable,
    },
    InstructionEncoding {
        operation: InstructionOperation::PSRLW as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PSRLD as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PSRLQ as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PADDQ as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PMULLW as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: 31u16,
        flags: 0u16,
        func: DecodeSSETable,
    },
    InstructionEncoding {
        operation: 32u16,
        flags: 0u16,
        func: DecodeSSETable,
    },
    InstructionEncoding {
        operation: InstructionOperation::PSUBUSB as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PSUBUSW as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PMINUB as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PAND as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PADDUSB as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PADDUSW as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PMAXUB as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PANDN as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PAVGB as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PSRAW as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PSRAD as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PAVGW as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PMULHUW as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PMULHW as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: 33u16,
        flags: 0u16,
        func: DecodeSSETable,
    },
    InstructionEncoding {
        operation: 34u16,
        flags: 0x200u16,
        func: DecodeSSETable,
    },
    InstructionEncoding {
        operation: InstructionOperation::PSUBSB as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PSUBSW as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PMINSW as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::POR as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PADDSB as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PADDSW as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PMAXSW as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PXOR as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: 35u16,
        flags: 0u16,
        func: DecodeSSETable,
    },
    InstructionEncoding {
        operation: InstructionOperation::PSLLW as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PSLLD as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PSLLQ as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PMULUDQ as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PMADDWD as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PSADBW as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: 36u16,
        flags: 0u16,
        func: DecodeSSETable,
    },
    InstructionEncoding {
        operation: InstructionOperation::PSUBB as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PSUBW as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PSUBD as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PSUBQ as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PADDB as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PADDW as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::PADDD as (u16),
        flags: 0u16,
        func: DecodeMMX,
    },
    InstructionEncoding {
        operation: InstructionOperation::INVALID as (u16),
        flags: 0u16,
        func: InvalidDecode,
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
            flags: 0u16,
            func: DecodeMMX,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x1u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PHADDW as (u16),
            flags: 0u16,
            func: DecodeMMX,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x2u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PHADDD as (u16),
            flags: 0u16,
            func: DecodeMMX,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x3u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PHADDSW as (u16),
            flags: 0u16,
            func: DecodeMMX,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x4u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PMADDUBSW as (u16),
            flags: 0u16,
            func: DecodeMMX,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x5u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PHSUBW as (u16),
            flags: 0u16,
            func: DecodeMMX,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x6u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PHSUBD as (u16),
            flags: 0u16,
            func: DecodeMMX,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x7u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PHSUBSW as (u16),
            flags: 0u16,
            func: DecodeMMX,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x8u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PSIGNB as (u16),
            flags: 0u16,
            func: DecodeMMX,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x9u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PSIGNW as (u16),
            flags: 0u16,
            func: DecodeMMX,
        },
    },
    SparseInstructionEncoding {
        opcode: 0xau8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PSIGND as (u16),
            flags: 0u16,
            func: DecodeMMX,
        },
    },
    SparseInstructionEncoding {
        opcode: 0xbu8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PMULHRSW as (u16),
            flags: 0u16,
            func: DecodeMMX,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x10u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PBLENDVB as (u16),
            flags: 0u16,
            func: DecodeMMXSSEOnly,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x14u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::BLENDVPS as (u16),
            flags: 0u16,
            func: DecodeMMXSSEOnly,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x15u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::BLENDVPD as (u16),
            flags: 0u16,
            func: DecodeMMXSSEOnly,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x17u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PTEST as (u16),
            flags: 0u16,
            func: DecodeMMXSSEOnly,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x1cu8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PABSB as (u16),
            flags: 0u16,
            func: DecodeMMX,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x1du8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PABSW as (u16),
            flags: 0u16,
            func: DecodeMMX,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x1eu8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PABSD as (u16),
            flags: 0u16,
            func: DecodeMMX,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x20u8,
        encoding: InstructionEncoding {
            operation: 37u16,
            flags: 0u16,
            func: DecodeSSETable,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x21u8,
        encoding: InstructionEncoding {
            operation: 38u16,
            flags: 0u16,
            func: DecodeSSETable,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x22u8,
        encoding: InstructionEncoding {
            operation: 39u16,
            flags: 0u16,
            func: DecodeSSETable,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x23u8,
        encoding: InstructionEncoding {
            operation: 40u16,
            flags: 0u16,
            func: DecodeSSETable,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x24u8,
        encoding: InstructionEncoding {
            operation: 41u16,
            flags: 0u16,
            func: DecodeSSETable,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x25u8,
        encoding: InstructionEncoding {
            operation: 42u16,
            flags: 0u16,
            func: DecodeSSETable,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x28u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PMULDQ as (u16),
            flags: 0u16,
            func: DecodeMMXSSEOnly,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x29u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PCMPEQQ as (u16),
            flags: 0u16,
            func: DecodeMMXSSEOnly,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x2au8,
        encoding: InstructionEncoding {
            operation: 43u16,
            flags: 0u16,
            func: DecodeSSETable,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x2bu8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PACKUSDW as (u16),
            flags: 0u16,
            func: DecodeMMXSSEOnly,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x30u8,
        encoding: InstructionEncoding {
            operation: 44u16,
            flags: 0u16,
            func: DecodeSSETable,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x31u8,
        encoding: InstructionEncoding {
            operation: 45u16,
            flags: 0u16,
            func: DecodeSSETable,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x32u8,
        encoding: InstructionEncoding {
            operation: 46u16,
            flags: 0u16,
            func: DecodeSSETable,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x33u8,
        encoding: InstructionEncoding {
            operation: 47u16,
            flags: 0u16,
            func: DecodeSSETable,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x34u8,
        encoding: InstructionEncoding {
            operation: 48u16,
            flags: 0u16,
            func: DecodeSSETable,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x35u8,
        encoding: InstructionEncoding {
            operation: 49u16,
            flags: 0u16,
            func: DecodeSSETable,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x37u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PCMPGTQ as (u16),
            flags: 0u16,
            func: DecodeMMXSSEOnly,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x38u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PMINSB as (u16),
            flags: 0u16,
            func: DecodeMMXSSEOnly,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x39u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PMINSD as (u16),
            flags: 0u16,
            func: DecodeMMXSSEOnly,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x3au8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PMINUW as (u16),
            flags: 0u16,
            func: DecodeMMXSSEOnly,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x3bu8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PMINUD as (u16),
            flags: 0u16,
            func: DecodeMMXSSEOnly,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x3cu8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PMAXSB as (u16),
            flags: 0u16,
            func: DecodeMMXSSEOnly,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x3du8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PMAXSD as (u16),
            flags: 0u16,
            func: DecodeMMXSSEOnly,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x3eu8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PMAXUW as (u16),
            flags: 0u16,
            func: DecodeMMXSSEOnly,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x3fu8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PMAXUD as (u16),
            flags: 0u16,
            func: DecodeMMXSSEOnly,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x40u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PMULLD as (u16),
            flags: 0u16,
            func: DecodeMMXSSEOnly,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x41u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PHMINPOSUW as (u16),
            flags: 0u16,
            func: DecodeMMXSSEOnly,
        },
    },
    SparseInstructionEncoding {
        opcode: 0xf0u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::CRC32 as (u16),
            flags: 0x100u16,
            func: DecodeCrc32,
        },
    },
    SparseInstructionEncoding {
        opcode: 0xf1u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::CRC32 as (u16),
            flags: 0u16,
            func: DecodeCrc32,
        },
    },
];

static THREE_BYTE_0F3A_MAP: [SparseInstructionEncoding; 22] = [
    SparseInstructionEncoding {
        opcode: 0x8u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::ROUNDPS as (u16),
            flags: 0u16,
            func: DecodeMMXSSEOnly,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x9u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::ROUNDPD as (u16),
            flags: 0u16,
            func: DecodeMMXSSEOnly,
        },
    },
    SparseInstructionEncoding {
        opcode: 0xau8,
        encoding: InstructionEncoding {
            operation: 50u16,
            flags: 0u16,
            func: DecodeSSETable,
        },
    },
    SparseInstructionEncoding {
        opcode: 0xbu8,
        encoding: InstructionEncoding {
            operation: 51u16,
            flags: 0u16,
            func: DecodeSSETable,
        },
    },
    SparseInstructionEncoding {
        opcode: 0xcu8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::BLENDPS as (u16),
            flags: 0u16,
            func: DecodeMMXSSEOnly,
        },
    },
    SparseInstructionEncoding {
        opcode: 0xdu8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::BLENDPD as (u16),
            flags: 0u16,
            func: DecodeMMXSSEOnly,
        },
    },
    SparseInstructionEncoding {
        opcode: 0xeu8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PBLENDW as (u16),
            flags: 0u16,
            func: DecodeMMXSSEOnly,
        },
    },
    SparseInstructionEncoding {
        opcode: 0xfu8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PALIGNR as (u16),
            flags: 0u16,
            func: DecodeMMX,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x14u8,
        encoding: InstructionEncoding {
            operation: 52u16,
            flags: 0x200u16,
            func: DecodeSSETableMem8,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x15u8,
        encoding: InstructionEncoding {
            operation: 53u16,
            flags: 0x200u16,
            func: DecodeSSETable,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x16u8,
        encoding: InstructionEncoding {
            operation: 54u16,
            flags: (0x800i32 | 0x200i32) as (u16),
            func: DecodeSSETable,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x17u8,
        encoding: InstructionEncoding {
            operation: 55u16,
            flags: 0x200u16,
            func: DecodeSSETable,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x20u8,
        encoding: InstructionEncoding {
            operation: 56u16,
            flags: 0u16,
            func: DecodeSSETableMem8,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x21u8,
        encoding: InstructionEncoding {
            operation: 57u16,
            flags: 0u16,
            func: DecodeSSETable,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x22u8,
        encoding: InstructionEncoding {
            operation: 58u16,
            flags: 0x800u16,
            func: DecodeSSETable,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x40u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::DPPS as (u16),
            flags: 0u16,
            func: DecodeMMXSSEOnly,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x41u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::DPPD as (u16),
            flags: 0u16,
            func: DecodeMMXSSEOnly,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x42u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::MPSADBW as (u16),
            flags: 0u16,
            func: DecodeMMXSSEOnly,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x60u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PCMPESTRM as (u16),
            flags: 0u16,
            func: DecodeMMXSSEOnly,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x61u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PCMPESTRI as (u16),
            flags: 0u16,
            func: DecodeMMXSSEOnly,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x62u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PCMPISTRM as (u16),
            flags: 0u16,
            func: DecodeMMXSSEOnly,
        },
    },
    SparseInstructionEncoding {
        opcode: 0x63u8,
        encoding: InstructionEncoding {
            operation: InstructionOperation::PCMPISTRI as (u16),
            flags: 0u16,
            func: DecodeMMXSSEOnly,
        },
    },
];

static FPU_MEM_OPCODE_MAP: [[InstructionEncoding; 8]; 8] = [
    [
        InstructionEncoding {
            operation: InstructionOperation::FADD as (u16),
            flags: 0u16,
            func: DecodeMem32,
        },
        InstructionEncoding {
            operation: InstructionOperation::FMUL as (u16),
            flags: 0u16,
            func: DecodeMem32,
        },
        InstructionEncoding {
            operation: InstructionOperation::FCOM as (u16),
            flags: 0u16,
            func: DecodeMem32,
        },
        InstructionEncoding {
            operation: InstructionOperation::FCOMP as (u16),
            flags: 0u16,
            func: DecodeMem32,
        },
        InstructionEncoding {
            operation: InstructionOperation::FSUB as (u16),
            flags: 0u16,
            func: DecodeMem32,
        },
        InstructionEncoding {
            operation: InstructionOperation::FSUBR as (u16),
            flags: 0u16,
            func: DecodeMem32,
        },
        InstructionEncoding {
            operation: InstructionOperation::FDIV as (u16),
            flags: 0u16,
            func: DecodeMem32,
        },
        InstructionEncoding {
            operation: InstructionOperation::FDIVR as (u16),
            flags: 0u16,
            func: DecodeMem32,
        },
    ],
    [
        InstructionEncoding {
            operation: InstructionOperation::FLD as (u16),
            flags: 0u16,
            func: DecodeMem32,
        },
        InstructionEncoding {
            operation: InstructionOperation::INVALID as (u16),
            flags: 0u16,
            func: InvalidDecode,
        },
        InstructionEncoding {
            operation: InstructionOperation::FST as (u16),
            flags: 0u16,
            func: DecodeMem32,
        },
        InstructionEncoding {
            operation: InstructionOperation::FSTP as (u16),
            flags: 0u16,
            func: DecodeMem32,
        },
        InstructionEncoding {
            operation: InstructionOperation::FLDENV as (u16),
            flags: 0u16,
            func: DecodeMemFloatEnv,
        },
        InstructionEncoding {
            operation: InstructionOperation::FLDCW as (u16),
            flags: 0u16,
            func: DecodeMem16,
        },
        InstructionEncoding {
            operation: InstructionOperation::FSTENV as (u16),
            flags: 0u16,
            func: DecodeMemFloatEnv,
        },
        InstructionEncoding {
            operation: InstructionOperation::FSTCW as (u16),
            flags: 0u16,
            func: DecodeMem16,
        },
    ],
    [
        InstructionEncoding {
            operation: InstructionOperation::FIADD as (u16),
            flags: 0u16,
            func: DecodeMem32,
        },
        InstructionEncoding {
            operation: InstructionOperation::FIMUL as (u16),
            flags: 0u16,
            func: DecodeMem32,
        },
        InstructionEncoding {
            operation: InstructionOperation::FICOM as (u16),
            flags: 0u16,
            func: DecodeMem32,
        },
        InstructionEncoding {
            operation: InstructionOperation::FICOMP as (u16),
            flags: 0u16,
            func: DecodeMem32,
        },
        InstructionEncoding {
            operation: InstructionOperation::FISUB as (u16),
            flags: 0u16,
            func: DecodeMem32,
        },
        InstructionEncoding {
            operation: InstructionOperation::FISUBR as (u16),
            flags: 0u16,
            func: DecodeMem32,
        },
        InstructionEncoding {
            operation: InstructionOperation::FIDIV as (u16),
            flags: 0u16,
            func: DecodeMem32,
        },
        InstructionEncoding {
            operation: InstructionOperation::FIDIVR as (u16),
            flags: 0u16,
            func: DecodeMem32,
        },
    ],
    [
        InstructionEncoding {
            operation: InstructionOperation::FILD as (u16),
            flags: 0u16,
            func: DecodeMem32,
        },
        InstructionEncoding {
            operation: InstructionOperation::FISTTP as (u16),
            flags: 0u16,
            func: DecodeMem32,
        },
        InstructionEncoding {
            operation: InstructionOperation::FIST as (u16),
            flags: 0u16,
            func: DecodeMem32,
        },
        InstructionEncoding {
            operation: InstructionOperation::FISTP as (u16),
            flags: 0u16,
            func: DecodeMem32,
        },
        InstructionEncoding {
            operation: InstructionOperation::INVALID as (u16),
            flags: 0u16,
            func: InvalidDecode,
        },
        InstructionEncoding {
            operation: InstructionOperation::FLD as (u16),
            flags: 0u16,
            func: DecodeMem80,
        },
        InstructionEncoding {
            operation: InstructionOperation::INVALID as (u16),
            flags: 0u16,
            func: InvalidDecode,
        },
        InstructionEncoding {
            operation: InstructionOperation::FSTP as (u16),
            flags: 0u16,
            func: DecodeMem80,
        },
    ],
    [
        InstructionEncoding {
            operation: InstructionOperation::FADD as (u16),
            flags: 0u16,
            func: DecodeMem64,
        },
        InstructionEncoding {
            operation: InstructionOperation::FMUL as (u16),
            flags: 0u16,
            func: DecodeMem64,
        },
        InstructionEncoding {
            operation: InstructionOperation::FCOM as (u16),
            flags: 0u16,
            func: DecodeMem64,
        },
        InstructionEncoding {
            operation: InstructionOperation::FCOMP as (u16),
            flags: 0u16,
            func: DecodeMem64,
        },
        InstructionEncoding {
            operation: InstructionOperation::FSUB as (u16),
            flags: 0u16,
            func: DecodeMem64,
        },
        InstructionEncoding {
            operation: InstructionOperation::FSUBR as (u16),
            flags: 0u16,
            func: DecodeMem64,
        },
        InstructionEncoding {
            operation: InstructionOperation::FDIV as (u16),
            flags: 0u16,
            func: DecodeMem64,
        },
        InstructionEncoding {
            operation: InstructionOperation::FDIVR as (u16),
            flags: 0u16,
            func: DecodeMem64,
        },
    ],
    [
        InstructionEncoding {
            operation: InstructionOperation::FLD as (u16),
            flags: 0u16,
            func: DecodeMem64,
        },
        InstructionEncoding {
            operation: InstructionOperation::FISTTP as (u16),
            flags: 0u16,
            func: DecodeMem64,
        },
        InstructionEncoding {
            operation: InstructionOperation::FST as (u16),
            flags: 0u16,
            func: DecodeMem64,
        },
        InstructionEncoding {
            operation: InstructionOperation::FSTP as (u16),
            flags: 0u16,
            func: DecodeMem64,
        },
        InstructionEncoding {
            operation: InstructionOperation::FRSTOR as (u16),
            flags: 0u16,
            func: DecodeMemFloatSave,
        },
        InstructionEncoding {
            operation: InstructionOperation::INVALID as (u16),
            flags: 0u16,
            func: InvalidDecode,
        },
        InstructionEncoding {
            operation: InstructionOperation::FSAVE as (u16),
            flags: 0u16,
            func: DecodeMemFloatSave,
        },
        InstructionEncoding {
            operation: InstructionOperation::FSTSW as (u16),
            flags: 0u16,
            func: DecodeMem16,
        },
    ],
    [
        InstructionEncoding {
            operation: InstructionOperation::FIADD as (u16),
            flags: 0u16,
            func: DecodeMem16,
        },
        InstructionEncoding {
            operation: InstructionOperation::FIMUL as (u16),
            flags: 0u16,
            func: DecodeMem16,
        },
        InstructionEncoding {
            operation: InstructionOperation::FICOM as (u16),
            flags: 0u16,
            func: DecodeMem16,
        },
        InstructionEncoding {
            operation: InstructionOperation::FICOMP as (u16),
            flags: 0u16,
            func: DecodeMem16,
        },
        InstructionEncoding {
            operation: InstructionOperation::FISUB as (u16),
            flags: 0u16,
            func: DecodeMem16,
        },
        InstructionEncoding {
            operation: InstructionOperation::FISUBR as (u16),
            flags: 0u16,
            func: DecodeMem16,
        },
        InstructionEncoding {
            operation: InstructionOperation::FIDIV as (u16),
            flags: 0u16,
            func: DecodeMem16,
        },
        InstructionEncoding {
            operation: InstructionOperation::FIDIVR as (u16),
            flags: 0u16,
            func: DecodeMem16,
        },
    ],
    [
        InstructionEncoding {
            operation: InstructionOperation::FILD as (u16),
            flags: 0u16,
            func: DecodeMem16,
        },
        InstructionEncoding {
            operation: InstructionOperation::FISTTP as (u16),
            flags: 0u16,
            func: DecodeMem16,
        },
        InstructionEncoding {
            operation: InstructionOperation::FIST as (u16),
            flags: 0u16,
            func: DecodeMem16,
        },
        InstructionEncoding {
            operation: InstructionOperation::FISTP as (u16),
            flags: 0u16,
            func: DecodeMem16,
        },
        InstructionEncoding {
            operation: InstructionOperation::FBLD as (u16),
            flags: 0u16,
            func: DecodeMem80,
        },
        InstructionEncoding {
            operation: InstructionOperation::FILD as (u16),
            flags: 0u16,
            func: DecodeMem64,
        },
        InstructionEncoding {
            operation: InstructionOperation::FBSTP as (u16),
            flags: 0u16,
            func: DecodeMem80,
        },
        InstructionEncoding {
            operation: InstructionOperation::FISTP as (u16),
            flags: 0u16,
            func: DecodeMem64,
        },
    ],
];

static FPU_REG_OPCODE_MAP: [[InstructionEncoding; 8]; 8] = [
    [
        InstructionEncoding {
            operation: InstructionOperation::FADD as (u16),
            flags: 0x200u16,
            func: DecodeFPURegST0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FMUL as (u16),
            flags: 0x200u16,
            func: DecodeFPURegST0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FCOM as (u16),
            flags: 0x200u16,
            func: DecodeFPURegST0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FCOMP as (u16),
            flags: 0x200u16,
            func: DecodeFPURegST0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FSUB as (u16),
            flags: 0x200u16,
            func: DecodeFPURegST0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FSUBR as (u16),
            flags: 0x200u16,
            func: DecodeFPURegST0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FDIV as (u16),
            flags: 0x200u16,
            func: DecodeFPURegST0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FDIVR as (u16),
            flags: 0x200u16,
            func: DecodeFPURegST0,
        },
    ],
    [
        InstructionEncoding {
            operation: InstructionOperation::FLD as (u16),
            flags: 0u16,
            func: DecodeFPUReg,
        },
        InstructionEncoding {
            operation: InstructionOperation::FXCH as (u16),
            flags: 0x200u16,
            func: DecodeFPURegST0,
        },
        InstructionEncoding {
            operation: 12u16,
            flags: 0u16,
            func: DecodeRegGroupNoOperands,
        },
        InstructionEncoding {
            operation: InstructionOperation::INVALID as (u16),
            flags: 0u16,
            func: InvalidDecode,
        },
        InstructionEncoding {
            operation: 13u16,
            flags: 0u16,
            func: DecodeRegGroupNoOperands,
        },
        InstructionEncoding {
            operation: 14u16,
            flags: 0u16,
            func: DecodeRegGroupNoOperands,
        },
        InstructionEncoding {
            operation: 15u16,
            flags: 0u16,
            func: DecodeRegGroupNoOperands,
        },
        InstructionEncoding {
            operation: 16u16,
            flags: 0u16,
            func: DecodeRegGroupNoOperands,
        },
    ],
    [
        InstructionEncoding {
            operation: InstructionOperation::FCMOVB as (u16),
            flags: 0x200u16,
            func: DecodeFPURegST0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FCMOVE as (u16),
            flags: 0x200u16,
            func: DecodeFPURegST0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FCMOVBE as (u16),
            flags: 0x200u16,
            func: DecodeFPURegST0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FCMOVU as (u16),
            flags: 0x200u16,
            func: DecodeFPURegST0,
        },
        InstructionEncoding {
            operation: InstructionOperation::INVALID as (u16),
            flags: 0u16,
            func: InvalidDecode,
        },
        InstructionEncoding {
            operation: 17u16,
            flags: 0u16,
            func: DecodeRegGroupNoOperands,
        },
        InstructionEncoding {
            operation: InstructionOperation::INVALID as (u16),
            flags: 0u16,
            func: InvalidDecode,
        },
        InstructionEncoding {
            operation: InstructionOperation::INVALID as (u16),
            flags: 0u16,
            func: InvalidDecode,
        },
    ],
    [
        InstructionEncoding {
            operation: InstructionOperation::FCMOVNB as (u16),
            flags: 0x200u16,
            func: DecodeFPURegST0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FCMOVNE as (u16),
            flags: 0x200u16,
            func: DecodeFPURegST0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FCMOVNBE as (u16),
            flags: 0x200u16,
            func: DecodeFPURegST0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FCMOVNU as (u16),
            flags: 0x200u16,
            func: DecodeFPURegST0,
        },
        InstructionEncoding {
            operation: 18u16,
            flags: 0u16,
            func: DecodeRegGroupNoOperands,
        },
        InstructionEncoding {
            operation: InstructionOperation::FUCOMI as (u16),
            flags: 0x200u16,
            func: DecodeFPURegST0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FCOMI as (u16),
            flags: 0x200u16,
            func: DecodeFPURegST0,
        },
        InstructionEncoding {
            operation: 21u16,
            flags: 0u16,
            func: DecodeRegGroupNoOperands,
        },
    ],
    [
        InstructionEncoding {
            operation: InstructionOperation::FADD as (u16),
            flags: 0u16,
            func: DecodeFPURegST0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FMUL as (u16),
            flags: 0u16,
            func: DecodeFPURegST0,
        },
        InstructionEncoding {
            operation: InstructionOperation::INVALID as (u16),
            flags: 0u16,
            func: InvalidDecode,
        },
        InstructionEncoding {
            operation: InstructionOperation::INVALID as (u16),
            flags: 0u16,
            func: InvalidDecode,
        },
        InstructionEncoding {
            operation: InstructionOperation::FSUBR as (u16),
            flags: 0u16,
            func: DecodeFPURegST0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FSUB as (u16),
            flags: 0u16,
            func: DecodeFPURegST0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FDIVR as (u16),
            flags: 0u16,
            func: DecodeFPURegST0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FDIV as (u16),
            flags: 0u16,
            func: DecodeFPURegST0,
        },
    ],
    [
        InstructionEncoding {
            operation: InstructionOperation::FFREE as (u16),
            flags: 0u16,
            func: DecodeFPUReg,
        },
        InstructionEncoding {
            operation: InstructionOperation::INVALID as (u16),
            flags: 0u16,
            func: InvalidDecode,
        },
        InstructionEncoding {
            operation: InstructionOperation::FST as (u16),
            flags: 0u16,
            func: DecodeFPUReg,
        },
        InstructionEncoding {
            operation: InstructionOperation::FSTP as (u16),
            flags: 0u16,
            func: DecodeFPUReg,
        },
        InstructionEncoding {
            operation: InstructionOperation::FUCOM as (u16),
            flags: 0x200u16,
            func: DecodeFPURegST0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FUCOMP as (u16),
            flags: 0x200u16,
            func: DecodeFPURegST0,
        },
        InstructionEncoding {
            operation: InstructionOperation::INVALID as (u16),
            flags: 0u16,
            func: InvalidDecode,
        },
        InstructionEncoding {
            operation: 22u16,
            flags: 0u16,
            func: DecodeRegGroupNoOperands,
        },
    ],
    [
        InstructionEncoding {
            operation: InstructionOperation::FADDP as (u16),
            flags: 0u16,
            func: DecodeFPURegST0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FMULP as (u16),
            flags: 0u16,
            func: DecodeFPURegST0,
        },
        InstructionEncoding {
            operation: InstructionOperation::INVALID as (u16),
            flags: 0u16,
            func: InvalidDecode,
        },
        InstructionEncoding {
            operation: 19u16,
            flags: 0u16,
            func: DecodeRegGroupNoOperands,
        },
        InstructionEncoding {
            operation: InstructionOperation::FSUBRP as (u16),
            flags: 0u16,
            func: DecodeFPURegST0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FSUBP as (u16),
            flags: 0u16,
            func: DecodeFPURegST0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FDIVRP as (u16),
            flags: 0u16,
            func: DecodeFPURegST0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FDIVP as (u16),
            flags: 0u16,
            func: DecodeFPURegST0,
        },
    ],
    [
        InstructionEncoding {
            operation: InstructionOperation::FFREEP as (u16),
            flags: 0u16,
            func: DecodeFPUReg,
        },
        InstructionEncoding {
            operation: InstructionOperation::INVALID as (u16),
            flags: 0u16,
            func: InvalidDecode,
        },
        InstructionEncoding {
            operation: InstructionOperation::INVALID as (u16),
            flags: 0u16,
            func: InvalidDecode,
        },
        InstructionEncoding {
            operation: InstructionOperation::INVALID as (u16),
            flags: 0u16,
            func: InvalidDecode,
        },
        InstructionEncoding {
            operation: 20u16,
            flags: 0u16,
            func: DecodeRegGroupAX,
        },
        InstructionEncoding {
            operation: InstructionOperation::FUCOMIP as (u16),
            flags: 0x200u16,
            func: DecodeFPURegST0,
        },
        InstructionEncoding {
            operation: InstructionOperation::FCOMIP as (u16),
            flags: 0x200u16,
            func: DecodeFPURegST0,
        },
        InstructionEncoding {
            operation: 23u16,
            flags: 0u16,
            func: DecodeRegGroupNoOperands,
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
    pub regType: SSETableOperandType,
    pub rmType: SSETableOperandType,
}

#[derive(Debug)]
#[repr(C)]
struct SSETableEntry {
    pub regOps: [SSETableOperationEntry; 4],
    pub memOps: [SSETableOperationEntry; 4],
}

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
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVUPS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVUPD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVSD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVSS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVUPS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVUPD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVSD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVSS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_32,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVHLPS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVDDUP,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVSLDUP,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVLPS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVLPD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVDDUP,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVSLDUP,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVLPS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVLPD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::UNPCKLPS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::UNPCKLPD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::UNPCKLPS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::UNPCKLPD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::UNPCKHPS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::UNPCKHPD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::UNPCKHPS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::UNPCKHPD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVLHPS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVSHDUP,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVHPS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVHPD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVSHDUP,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVHPS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVHPD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVAPS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVAPD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVAPS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVAPD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPI2PS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::MMX_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPI2PD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::MMX_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTSI2SD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTSI2SS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::GPR_32_OR_64,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPI2PS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::MMX_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPI2PD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::MMX_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTSI2SD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTSI2SS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::GPR_32_OR_64,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVNTPS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVNTPD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::CVTTPS2PI,
                regType: SSETableOperandType::MMX_64,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTTPD2PI,
                regType: SSETableOperandType::MMX_64,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTTSD2SI,
                regType: SSETableOperandType::GPR_32_OR_64,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTTSS2SI,
                regType: SSETableOperandType::GPR_32_OR_64,
                rmType: SSETableOperandType::SSE_128,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::CVTTPS2PI,
                regType: SSETableOperandType::MMX_64,
                rmType: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTTPD2PI,
                regType: SSETableOperandType::MMX_64,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTTSD2SI,
                regType: SSETableOperandType::GPR_32_OR_64,
                rmType: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTTSS2SI,
                regType: SSETableOperandType::GPR_32_OR_64,
                rmType: SSETableOperandType::SSE_32,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPS2PI,
                regType: SSETableOperandType::MMX_64,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPD2PI,
                regType: SSETableOperandType::MMX_64,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTSD2SI,
                regType: SSETableOperandType::GPR_32_OR_64,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTSS2SI,
                regType: SSETableOperandType::GPR_32_OR_64,
                rmType: SSETableOperandType::SSE_128,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPS2PI,
                regType: SSETableOperandType::MMX_64,
                rmType: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPD2PI,
                regType: SSETableOperandType::MMX_64,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTSD2SI,
                regType: SSETableOperandType::GPR_32_OR_64,
                rmType: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTSS2SI,
                regType: SSETableOperandType::GPR_32_OR_64,
                rmType: SSETableOperandType::SSE_32,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::UCOMISS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::UCOMISD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::UCOMISS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_32,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::UCOMISD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::COMISS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::COMISD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::COMISS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_32,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::COMISD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVMSKPS,
                regType: SSETableOperandType::GPR_32_OR_64,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVMSKPD,
                regType: SSETableOperandType::GPR_32_OR_64,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPS2PD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPD2PS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTSD2SS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTSS2SD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPS2PD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPD2PS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTSD2SS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTSS2SD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_32,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::CVTDQ2PS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPS2DQ,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTTPS2DQ,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::CVTDQ2PS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPS2DQ,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTTPS2DQ,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::PUNPCKLBW,
                regType: SSETableOperandType::MMX_64,
                rmType: SSETableOperandType::MMX_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PUNPCKLBW,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::PUNPCKLBW,
                regType: SSETableOperandType::MMX_64,
                rmType: SSETableOperandType::MMX_32,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PUNPCKLBW,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::PUNPCKLWD,
                regType: SSETableOperandType::MMX_64,
                rmType: SSETableOperandType::MMX_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PUNPCKLWD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::PUNPCKLWD,
                regType: SSETableOperandType::MMX_64,
                rmType: SSETableOperandType::MMX_32,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PUNPCKLWD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::PUNPCKLDQ,
                regType: SSETableOperandType::MMX_64,
                rmType: SSETableOperandType::MMX_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PUNPCKLDQ,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::PUNPCKLDQ,
                regType: SSETableOperandType::MMX_64,
                rmType: SSETableOperandType::MMX_32,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PUNPCKLDQ,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVD,
                regType: SSETableOperandType::MMX_64,
                rmType: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVD,
                regType: SSETableOperandType::MMX_64,
                rmType: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVQ,
                regType: SSETableOperandType::MMX_64,
                rmType: SSETableOperandType::MMX_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVDQA,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVDQU,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVQ,
                regType: SSETableOperandType::MMX_64,
                rmType: SSETableOperandType::MMX_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVDQA,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVDQU,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::PSHUFW,
                regType: SSETableOperandType::MMX_64,
                rmType: SSETableOperandType::MMX_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PSHUFD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PSHUFLW,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PSHUFHW,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::PSHUFW,
                regType: SSETableOperandType::MMX_64,
                rmType: SSETableOperandType::MMX_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PSHUFD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PSHUFLW,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PSHUFHW,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::HADDPD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::HADDPS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::HADDPD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::HADDPS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::HSUBPD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::HSUBPS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::HSUBPD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::HSUBPS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVD,
                regType: SSETableOperandType::MMX_64,
                rmType: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVQ,
                regType: SSETableOperandType::SSE_128_FLIP,
                rmType: SSETableOperandType::SSE_128_FLIP,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVD,
                regType: SSETableOperandType::MMX_64,
                rmType: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVQ,
                regType: SSETableOperandType::SSE_128_FLIP,
                rmType: SSETableOperandType::SSE_128_FLIP,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::CMPPS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CMPPD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CMPSD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CMPSS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::CMPPS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CMPPD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CMPSD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CMPSS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_32,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::PINSRW,
                regType: SSETableOperandType::MMX_64,
                rmType: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PINSRW,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::PINSRW,
                regType: SSETableOperandType::MMX_64,
                rmType: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PINSRW,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::PEXTRW,
                regType: SSETableOperandType::MMX_64,
                rmType: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PEXTRW,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::PEXTRW,
                regType: SSETableOperandType::MMX_64,
                rmType: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PEXTRW,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::SHUFPS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::SHUFPD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::SHUFPS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::SHUFPD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::ADDSUBPD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::ADDSUBPS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::ADDSUBPD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::ADDSUBPS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVQ,
                regType: SSETableOperandType::SSE_128_FLIP,
                rmType: SSETableOperandType::SSE_128_FLIP,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVDQ2Q,
                regType: SSETableOperandType::MMX_64,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVQ2DQ,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::MMX_64,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVQ,
                regType: SSETableOperandType::SSE_128_FLIP,
                rmType: SSETableOperandType::SSE_128_FLIP,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVMSKB,
                regType: SSETableOperandType::GPR_32_OR_64,
                rmType: SSETableOperandType::MMX_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVMSKB,
                regType: SSETableOperandType::GPR_32_OR_64,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTTPD2DQ,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPD2DQ,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTDQ2PD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTTPD2DQ,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPD2DQ,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTDQ2PD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVNTQ,
                regType: SSETableOperandType::MMX_64,
                rmType: SSETableOperandType::MMX_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVNTDQ,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::LDDQU,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::MASKMOVQ,
                regType: SSETableOperandType::MMX_64,
                rmType: SSETableOperandType::MMX_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MASKMOVDQU,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVSXBW,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVSXBW,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVSXBD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVSXBD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_32,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVSXBQ,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVSXBQ,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_16,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVSXWD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVSXWD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVSXWQ,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVSXWQ,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_32,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVSXDQ,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVSXDQ,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVNTDQA,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVZXBW,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVZXBW,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVZXBD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVZXBD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_32,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVZXBQ,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVZXBQ,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_16,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVZXWD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVZXWD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVZXWQ,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVZXWQ,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_32,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVZXDQ,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVZXDQ,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::ROUNDSS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::ROUNDSS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_32,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::ROUNDSD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::ROUNDSD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PEXTRB,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PEXTRB,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PEXTRW,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PEXTRW,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_16,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PEXTRD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PEXTRD,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::EXTRACTPS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::EXTRACTPS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_32,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PINSRB,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PINSRB,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::GPR_32_OR_64,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INSERTPS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_128,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INSERTPS,
                regType: SSETableOperandType::SSE_128,
                rmType: SSETableOperandType::SSE_32,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID,
                regType: SSETableOperandType::INVALID,
                rmType: SSETableOperandType::INVALID,
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

fn InvalidDecode(state: &mut DecodeState) {
    state.invalid = true;
}

unsafe fn Read8(state: &mut DecodeState) -> u8 {
    if state.len < 1 {
        state.invalid = true;
        state.insufficientLength = true;
        state.len = 0;
        0xcc
    } else {
        let val = *{
            let _old = state.opcode;
            state.opcode = state.opcode.offset(1);
            _old
        };
        state.len = state.len.wrapping_sub(1);
        val
    }
}

fn GetFinalOpSize(state: &mut DecodeState) -> u16 {
    if state.flags & 0x100u32 != 0 {
        1u16
    } else {
        state.opSize
    }
}

unsafe fn ProcessEncoding(state: &mut DecodeState, encoding: &InstructionEncoding) {
    (*state.result).operation = InstructionOperation::from_i32((*encoding).operation as i32);
    state.flags = (*encoding).flags as u32;
    if state.using64 && (state.flags & 0x4000 != 0) {
        state.invalid = true;
    } else {
        if state.using64 && (state.flags & 0x8000 != 0) {
            state.opSize = if state.opPrefix { 4 } else { 8 };
        }
        state.finalOpSize = GetFinalOpSize(state);
        if state.flags & 0x200 != 0 {
            state.operand0 = &mut (*state.result).operands[1usize] as (*mut InstructionOperand);
            state.operand1 = &mut (*state.result).operands[0usize] as (*mut InstructionOperand);
        } else {
            state.operand0 = &mut (*state.result).operands[0usize] as (*mut InstructionOperand);
            state.operand1 = &mut (*state.result).operands[1usize] as (*mut InstructionOperand);
        }
        if state.flags & 0x2000 != 0 {
            state.finalOpSize = 2;
        }
        if state.flags & 0x1000 != 0 {
            if state.finalOpSize == 4 {
                (*state.result).operation =
                    InstructionOperation::from_i32((*state.result).operation as i32 + 1);
            } else if state.finalOpSize == 8 {
                (*state.result).operation =
                    InstructionOperation::from_i32((*state.result).operation as i32 + 2);
            }
        }
        if state.flags & 0x40 != 0 {
            if state.rep != RepPrefix::REP_PREFIX_NONE {
                (*state.result).flags |= 2;
            }
        } else if state.flags & 0x80 != 0 {
            if state.rep == RepPrefix::REP_PREFIX_REPNE {
                (*state.result).flags |= 4;
            } else if state.rep == RepPrefix::REP_PREFIX_REPE {
                (*state.result).flags |= 8;
            }
        }
        ((*encoding).func)(state);
        if (*state.result).operation == InstructionOperation::INVALID {
            state.invalid = true;
        }
        if (*state.result).flags & 1 != 0 {
            if state.flags & 0x20 == 0 {
                state.invalid = true;
            } else if (*state.result).operation == InstructionOperation::CMP {
                state.invalid = true;
            } else if (*state.result).operands[0].operand != OperandType::MEM &&
                       ((*state.result).operands[1].operand != OperandType::MEM)
            {
                state.invalid = true;
            }
        }
    }
}

unsafe fn ProcessOpcode(state: &mut DecodeState, map: &[InstructionEncoding], opcode: u8) {
    ProcessEncoding(state, &map[opcode as usize]);
}

unsafe fn ProcessSparseOpcode(
    state: &mut DecodeState,
    map: &[SparseInstructionEncoding],
    opcode: u8,
) {
    (*state.result).operation = InstructionOperation::INVALID;
    if let Ok(idx) = map.binary_search_by_key(&opcode, |entry| entry.opcode) {
        ProcessEncoding(state, &map[idx].encoding);
    }
}

unsafe fn SetOperandToImm8(state: &mut DecodeState, oper: *mut InstructionOperand) {
    (*oper).operand = OperandType::IMM;
    (*oper).size = 1u16;
    (*oper).immediate = Read8(state) as (isize);
}

unsafe fn DecodeTwoByte(state: &mut DecodeState) {
    let opcode: u8 = Read8(state);
    if opcode == 0x38 {
        let next_opcode = Read8(state);
        ProcessSparseOpcode(state, &THREE_BYTE_0F38_MAP, next_opcode);
    } else if opcode == 0x3a {
        let next_opcode = Read8(state);
        ProcessSparseOpcode(state, &THREE_BYTE_0F3A_MAP, next_opcode);
        SetOperandToImm8(
            state,
            &mut (*state.result).operands[2usize] as (*mut InstructionOperand),
        );
    } else {
        ProcessOpcode(state, &TWO_BYTE_OPCODE_MAP, opcode);
    }
}

unsafe fn Peek8(state: &mut DecodeState) -> u8 {
    if state.len < 1 {
        state.invalid = true;
        state.insufficientLength = true;
        state.len = 0;
        0xcc
    } else {
        *state.opcode
    }
}

unsafe fn DecodeFpu(state: &mut DecodeState) {
    let modRM = Peek8(state);
    let reg = modRM >> 3 & 7;
    let op = (*state.result).operation as u8;
    let map = if modRM & 0xc0 == 0xc0 {
        &FPU_REG_OPCODE_MAP[op as usize]
    } else {
        &FPU_MEM_OPCODE_MAP[op as usize]
    };
    ProcessEncoding(state, &map[reg as usize]);
}

fn DecodeNoOperands(_state: &mut DecodeState) {}

fn GetByteRegList(state: &DecodeState) -> &'static [OperandType] {
    if state.rex { &REG8_LIST64 } else { &REG8_LIST }
}

fn GetRegListForFinalOpSize(state: &DecodeState) -> &'static [OperandType] {
    match state.finalOpSize {
        8 => &REG64_LIST,
        4 => &REG32_LIST,
        2 => &REG16_LIST,
        1 => GetByteRegList(state),
        _ => &INVALID_REG_LIST,
    }
}

fn GetRegListForAddrSize(state: &DecodeState) -> &'static [OperandType] {
    match state.addrSize {
        8 => &REG64_LIST,
        4 => &REG32_LIST,
        2 => &REG16_LIST,
        _ => &INVALID_REG_LIST,
    }
}

unsafe fn Read32(state: &mut DecodeState) -> u32 {
    let val: u32;
    if state.len < 4 {
        state.invalid = true;
        state.insufficientLength = true;
        state.len = 0;
        0
    } else {
        val = *(state.opcode as (*mut u32));
        state.opcode = state.opcode.offset(4);
        state.len = state.len.wrapping_sub(4);
        val
    }
}

unsafe fn ReadSigned32(state: &mut DecodeState) -> isize {
    Read32(state) as (i32) as (isize)
}

unsafe fn ReadSigned8(state: &mut DecodeState) -> isize {
    Read8(state) as (i8) as (isize)
}

unsafe fn GetFinalSegment(state: &DecodeState, seg: SegmentRegister) -> SegmentRegister {
    if (*state.result).segment == SegmentRegister::SEG_DEFAULT {
        seg
    } else {
        (*state.result).segment
    }
}

#[derive(Debug)]
#[repr(C)]
struct RMDef {
    pub first: OperandType,
    pub second: OperandType,
    pub segment: SegmentRegister,
}

unsafe fn SetMemOperand(
    state: &DecodeState,
    oper: *mut InstructionOperand,
    def: &RMDef,
    immed: isize,
) {
    (*oper).operand = OperandType::MEM;
    (*oper).components[0] = def.first;
    (*oper).components[1] = def.second;
    (*oper).immediate = immed;
    (*oper).segment = GetFinalSegment(state, def.segment);
}

unsafe fn Read16(state: &mut DecodeState) -> u16 {
    let val: u16;
    if state.len < 2 {
        state.invalid = true;
        state.insufficientLength = true;
        state.len = 0;
        0
    } else {
        val = *(state.opcode as (*mut u16));
        state.opcode = state.opcode.offset(2);
        state.len = state.len.wrapping_sub(2);
        val
    }
}

unsafe fn ReadSigned16(state: &mut DecodeState) -> isize {
    Read16(state) as (i16) as (isize)
}

unsafe fn DecodeRM(
    state: &mut DecodeState,
    mut rmOper: *mut InstructionOperand,
    regList: &[OperandType],
    rmSize: u16,
    regOper: *mut u8,
) {
    let rmByte: u8 = Read8(state);
    let mod_: u8 = rmByte >> 6;
    let mut rm: u8 = rmByte & 7;
    let mut temp = InstructionOperand::default();
    if !regOper.is_null() {
        *regOper = rmByte >> 3 & 7;
    }
    if rmOper.is_null() {
        rmOper = &mut temp as (*mut InstructionOperand);
    }
    (*rmOper).size = rmSize;
    if state.addrSize == 2 {
        static mut RM16_COMPONENTS: [RMDef; 9] = [
            RMDef {
                first: OperandType::REG_BX,
                second: OperandType::REG_SI,
                segment: SegmentRegister::SEG_DS,
            },
            RMDef {
                first: OperandType::REG_BX,
                second: OperandType::REG_DI,
                segment: SegmentRegister::SEG_DS,
            },
            RMDef {
                first: OperandType::REG_BP,
                second: OperandType::REG_SI,
                segment: SegmentRegister::SEG_SS,
            },
            RMDef {
                first: OperandType::REG_BP,
                second: OperandType::REG_DI,
                segment: SegmentRegister::SEG_SS,
            },
            RMDef {
                first: OperandType::REG_SI,
                second: OperandType::NONE,
                segment: SegmentRegister::SEG_DS,
            },
            RMDef {
                first: OperandType::REG_DI,
                second: OperandType::NONE,
                segment: SegmentRegister::SEG_DS,
            },
            RMDef {
                first: OperandType::REG_BP,
                second: OperandType::NONE,
                segment: SegmentRegister::SEG_SS,
            },
            RMDef {
                first: OperandType::REG_BX,
                second: OperandType::NONE,
                segment: SegmentRegister::SEG_DS,
            },
            RMDef {
                first: OperandType::NONE,
                second: OperandType::NONE,
                segment: SegmentRegister::SEG_DS,
            },
        ];
        if mod_ == 3 {
            (*rmOper).operand = regList[rm as usize];
        } else if mod_ == 2 {
            let immediate = ReadSigned16(state);
            SetMemOperand(state, rmOper, &RM16_COMPONENTS[rm as usize], immediate);
        } else if mod_ == 1 {
            let immediate = ReadSigned8(state);
            SetMemOperand(state, rmOper, &RM16_COMPONENTS[rm as usize], immediate);
        } else if mod_ == 0 {
            if rm == 6 {
                rm = 8;
                let immediate = Read16(state);
                SetMemOperand(
                    state,
                    rmOper,
                    &RM16_COMPONENTS[rm as usize],
                    immediate as isize,
                );
            } else {
                SetMemOperand(state, rmOper, &RM16_COMPONENTS[rm as usize], 0);
            }
        }
        if (*rmOper).components[0] == OperandType::NONE {
            (*rmOper).immediate &= 0xffff;
        }
    } else {
        let addrRegList = GetRegListForAddrSize(state);
        let rmReg1Offset: u8 = if state.rexRM1 { 8 } else { 0 };
        let rmReg2Offset: u8 = if state.rexRM2 { 8 } else { 0 };
        let mut seg: SegmentRegister = SegmentRegister::SEG_DEFAULT;
        (*rmOper).operand = OperandType::MEM;
        if mod_ != 3 && rm == 4 {
            let sibByte: u8 = Read8(state);
            let base: u8 = sibByte & 7;
            let index: u8 = sibByte >> 3 & 7;
            (*rmOper).scale = 1 << sibByte >> 6;
            if mod_ != 0 || base != 5 {
                (*rmOper).components[0] = addrRegList[(base + rmReg1Offset) as usize];
            }
            if index + rmReg2Offset != 4 {
                (*rmOper).components[1] = addrRegList[(index + rmReg2Offset) as usize];
            }
            if mod_ == 2 {
                (*rmOper).immediate = ReadSigned32(state);
            } else if mod_ == 1 {
                (*rmOper).immediate = ReadSigned8(state);
            } else if mod_ == 0 && base == 5 {
                (*rmOper).immediate = ReadSigned32(state);
            }
            if base + rmReg1Offset == 4 || base + rmReg1Offset == 5 {
                seg = SegmentRegister::SEG_SS;
            } else {
                seg = SegmentRegister::SEG_DS;
            }
        } else if mod_ == 3 {
            (*rmOper).operand = regList[(rm + rmReg1Offset) as usize];
        } else if mod_ == 2 {
            (*rmOper).components[0] = addrRegList[(rm + rmReg1Offset) as usize];
            (*rmOper).immediate = ReadSigned32(state);
            seg = if rm == 5 {
                SegmentRegister::SEG_SS
            } else {
                SegmentRegister::SEG_DS
            };
        } else if mod_ == 1 {
            (*rmOper).components[0] = addrRegList[(rm + rmReg1Offset) as usize];
            (*rmOper).immediate = ReadSigned8(state);
            seg = if rm == 5 {
                SegmentRegister::SEG_SS
            } else {
                SegmentRegister::SEG_DS
            };
        } else if mod_ == 0 {
            if rm == 5 {
                (*rmOper).immediate = ReadSigned32(state);
                if state.addrSize == 8 {
                    state.ripRelFixup = &mut (*rmOper).immediate as (*mut isize);
                }
            } else {
                (*rmOper).components[0] = addrRegList[(rm + rmReg1Offset) as usize];
            }
            seg = SegmentRegister::SEG_DS;
        }
        if seg != SegmentRegister::SEG_DEFAULT {
            (*rmOper).segment = GetFinalSegment(state, seg);
        }
    }
}

unsafe fn DecodeRMReg(
    state: &mut DecodeState,
    rmOper: *mut InstructionOperand,
    rmRegList: &[OperandType],
    rmSize: u16,
    regOper: *mut InstructionOperand,
    regList: &[OperandType],
    regSize: u16,
) {
    let mut reg: u8 = 0;
    DecodeRM(state, rmOper, rmRegList, rmSize, &mut reg as (*mut u8));
    if !regOper.is_null() {
        let regOffset: u8 = if state.rexReg { 8 } else { 0 };
        (*regOper).size = regSize;
        (*regOper).operand = regList[(reg + regOffset) as usize];
    }
}

unsafe fn DecodeRegRM(state: &mut DecodeState) {
    let regList = GetRegListForFinalOpSize(state);
    let size = match state.flags & 0x3 {
        0 => state.finalOpSize,
        0x1 => state.finalOpSize * 2,
        0x2 => state.finalOpSize + 2,
        0x3 => 0,
        _ => panic!("This isn't possible. This shouldn't be needed to suppress a warning."),
    };
    let operand0 = state.operand0;
    let operand1 = state.operand1;
    let finalOpSize = state.finalOpSize;
    DecodeRMReg(
        state,
        operand1,
        regList,
        size,
        operand0,
        regList,
        finalOpSize,
    );
    if size != state.finalOpSize && ((*state.operand1).operand != OperandType::MEM) {
        state.invalid = true;
    }
}

unsafe fn ReadFinalOpSize(state: &mut DecodeState) -> isize {
    if state.flags & 0x400 != 0 {
        ReadSigned8(state)
    } else {
        match state.finalOpSize {
            8 => ReadSigned32(state),
            4 => Read32(state) as isize,
            2 => Read16(state) as isize,
            1 => Read8(state) as isize,
            _ => 0,
        }
    }
}

unsafe fn SetOperandToImm(state: &mut DecodeState, oper: *mut InstructionOperand) {
    (*oper).operand = OperandType::IMM;
    (*oper).size = state.finalOpSize;
    (*oper).immediate = ReadFinalOpSize(state);
}

unsafe fn DecodeRegRMImm(state: &mut DecodeState) {
    let regList = GetRegListForFinalOpSize(state);
    let operand0 = state.operand0;
    let operand1 = state.operand1;
    let finalOpSize = state.finalOpSize;
    DecodeRMReg(
        state,
        operand1,
        regList,
        finalOpSize,
        operand0,
        regList,
        finalOpSize,
    );
    SetOperandToImm(
        state,
        &mut (*state.result).operands[2] as (*mut InstructionOperand),
    );
}

unsafe fn DecodeRMRegImm8(state: &mut DecodeState) {
    let regList = GetRegListForFinalOpSize(state);
    let operand0 = state.operand0;
    let operand1 = state.operand1;
    let finalOpSize = state.finalOpSize;
    DecodeRMReg(
        state,
        operand0,
        regList,
        finalOpSize,
        operand1,
        regList,
        finalOpSize,
    );
    SetOperandToImm8(
        state,
        &mut (*state.result).operands[2] as (*mut InstructionOperand),
    );
}

unsafe fn DecodeRMRegCL(state: &mut DecodeState) {
    let regList = GetRegListForFinalOpSize(state);
    let operand0 = state.operand0;
    let operand1 = state.operand1;
    let finalOpSize = state.finalOpSize;
    DecodeRMReg(
        state,
        operand0,
        regList,
        finalOpSize,
        operand1,
        regList,
        finalOpSize,
    );
    (*state.result).operands[2].operand = OperandType::REG_CL;
    (*state.result).operands[2].size = 1;
}

unsafe fn SetOperandToEaxFinalOpSize(state: &DecodeState, oper: *mut InstructionOperand) {
    let regList = GetRegListForFinalOpSize(state);
    (*oper).operand = regList[0];
    (*oper).size = state.finalOpSize;
}

unsafe fn DecodeEaxImm(state: &mut DecodeState) {
    let operand0 = state.operand0;
    SetOperandToEaxFinalOpSize(state, operand0);
    let operand1 = state.operand1;
    SetOperandToImm(state, operand1);
}

unsafe fn DecodePushPopSeg(state: &mut DecodeState) {
    let offset: i32 = if *state.opcode.offset(-1) >= 0xa0 {
        -16
    } else {
        0
    };
    (*state.operand0).operand = OperandType::from_i32(
        OperandType::REG_ES as i32 +
            (*state.opcode.offset(-1) as i32 >> 3) + offset,
    );
    (*state.operand0).size = state.opSize;
}

unsafe fn SetOperandToOpReg(state: &DecodeState, oper: *mut InstructionOperand) {
    let regList = GetRegListForFinalOpSize(state);
    let regOffset: usize = if state.rexRM1 { 8 } else { 0 };
    (*oper).operand = regList[(*state.opcode.offset(-1) as usize & 7) + regOffset];
    (*oper).size = state.finalOpSize;
}

unsafe fn DecodeOpReg(state: &mut DecodeState) {
    SetOperandToOpReg(state, state.operand0);
}

unsafe fn DecodeEaxOpReg(state: &mut DecodeState) {
    SetOperandToEaxFinalOpSize(state, state.operand0);
    SetOperandToOpReg(state, state.operand1);
}

unsafe fn Read64(state: &mut DecodeState) -> usize {
    if state.len < 8 {
        state.invalid = true;
        state.insufficientLength = true;
        state.len = 0;
        0
    } else {
        let old_val = (*state.opcode) as usize;
        state.opcode = state.opcode.offset(8);
        state.len = state.len.wrapping_sub(8);
        old_val
    }
}

unsafe fn DecodeOpRegImm(state: &mut DecodeState) {
    SetOperandToOpReg(state, state.operand0);
    (*state.operand1).operand = OperandType::IMM;
    (*state.operand1).size = state.finalOpSize;
    (*state.operand1).immediate = if state.opSize == 8 {
        Read64(state) as isize
    } else {
        ReadFinalOpSize(state)
    };
}

unsafe fn DecodeNop(state: &mut DecodeState) {
    if state.rexRM1 {
        (*state.result).operation = InstructionOperation::XCHG;
        DecodeEaxOpReg(state);
    }
}

unsafe fn DecodeImm(state: &mut DecodeState) {
    let operand0 = state.operand0;
    SetOperandToImm(state, operand0);
}

unsafe fn SetOperandToImm16(state: &mut DecodeState, oper: *mut InstructionOperand) {
    (*oper).operand = OperandType::IMM;
    (*oper).size = 2;
    (*oper).immediate = Read16(state) as (isize);
}

unsafe fn DecodeImm16Imm8(state: &mut DecodeState) {
    let operand0 = state.operand0;
    SetOperandToImm16(state, operand0);
    let operand1 = state.operand1;
    SetOperandToImm8(state, operand1);
}

unsafe fn SetOperandToEsEdi(state: &DecodeState, oper: *mut InstructionOperand, size: u16) {
    let addrRegList = GetRegListForAddrSize(state);
    (*oper).operand = OperandType::MEM;
    (*oper).components[0] = addrRegList[7];
    (*oper).size = size;
    (*oper).segment = SegmentRegister::SEG_ES;
}

unsafe fn DecodeEdiDx(state: &mut DecodeState) {
    SetOperandToEsEdi(state, state.operand0, state.finalOpSize);
    (*state.operand1).operand = OperandType::REG_DX;
    (*state.operand1).size = 2u16;
}

unsafe fn SetOperandToDsEsi(state: &DecodeState, oper: *mut InstructionOperand, size: u16) {
    let addrRegList = GetRegListForAddrSize(state);
    (*oper).operand = OperandType::MEM;
    (*oper).components[0usize] = addrRegList[6];
    (*oper).size = size;
    (*oper).segment = GetFinalSegment(state, SegmentRegister::SEG_DS);
}

unsafe fn DecodeDxEsi(state: &mut DecodeState) {
    (*state.operand0).operand = OperandType::REG_DX;
    (*state.operand0).size = 2u16;
    SetOperandToDsEsi(state, state.operand1, state.finalOpSize);
}

unsafe fn ReadSignedFinalOpSize(state: &mut DecodeState) -> isize {
    match state.finalOpSize {
        4 | 8 => ReadSigned32(state),
        2 => ReadSigned16(state),
        1 => ReadSigned8(state),
        _ => 0,
    }
}

unsafe fn DecodeRelImm(state: &mut DecodeState) {
    (*state.operand0).operand = OperandType::IMM;
    (*state.operand0).size = state.opSize;
    (*state.operand0).immediate = ReadSignedFinalOpSize(state);
    (*state.operand0).immediate =
        ((*state.operand0).immediate as (usize)).wrapping_add(state.addr.wrapping_add(
            ((state.opcode as (isize)).wrapping_sub(state.opcodeStart as (isize)) /
                 ::std::mem::size_of::<u8>() as (isize)) as
                (usize),
        )) as (isize);
}

unsafe fn UpdateOperationForAddrSize(state: &mut DecodeState) {
    if state.addrSize == 4 {
        (*state.result).operation =
            InstructionOperation::from_i32((*state.result).operation as i32 + 1);
    } else if state.addrSize == 8 {
        (*state.result).operation =
            InstructionOperation::from_i32((*state.result).operation as i32 + 2);
    }
}

unsafe fn DecodeRelImmAddrSize(state: &mut DecodeState) {
    DecodeRelImm(state);
    UpdateOperationForAddrSize(state);
}

unsafe fn DecodeGroupRM(state: &mut DecodeState) {
    let operand0 = state.operand0;
    let regList = GetRegListForFinalOpSize(state);
    let regSize = state.finalOpSize;
    let mut regField: u8 = 0;
    DecodeRM(
        state,
        operand0,
        regList,
        regSize,
        &mut regField as (*mut u8),
    );
    (*state.result).operation = GROUP_OPERATIONS[(*state.result).operation as usize][regField as
                                                                                         usize];
}

unsafe fn DecodeGroupRMImm(state: &mut DecodeState) {
    DecodeGroupRM(state);
    let operand1 = state.operand1;
    SetOperandToImm(state, operand1);
}

unsafe fn DecodeGroupRMImm8V(state: &mut DecodeState) {
    DecodeGroupRM(state);
    let operand1 = state.operand1;
    SetOperandToImm8(state, operand1);
}

unsafe fn DecodeGroupRMOne(state: &mut DecodeState) {
    DecodeGroupRM(state);
    (*state.operand1).operand = OperandType::IMM;
    (*state.operand1).size = 1;
    (*state.operand1).immediate = 1;
}

unsafe fn DecodeGroupRMCl(state: &mut DecodeState) {
    DecodeGroupRM(state);
    (*state.operand1).operand = OperandType::REG_CL;
    (*state.operand1).size = 1;
}

unsafe fn DecodeGroupF6F7(state: &mut DecodeState) {
    DecodeGroupRM(state);
    if (*state.result).operation == InstructionOperation::TEST {
        let operand1 = state.operand1;
        SetOperandToImm(state, operand1);
    }
    if (*state.result).flags & 1 != 0 && ((*state.result).operation != InstructionOperation::NOT) &&
        ((*state.result).operation != InstructionOperation::NEG)
    {
        state.invalid = true;
    }
}

unsafe fn DecodeGroupFF(state: &mut DecodeState) {
    if state.using64 {
        let rm: u8 = Peek8(state);
        let regField: u8 = rm >> 3 & 7;
        if regField >= 2 && regField <= 5 {
            state.finalOpSize = {
                state.opSize = if state.opPrefix { 4 } else { 8 };
                state.opSize
            };
        }
    }
    DecodeGroupRM(state);
    if (*state.result).operation == InstructionOperation::CALLF ||
        (*state.result).operation == InstructionOperation::JMPF
    {
        if (*state.operand0).operand != OperandType::MEM {
            state.invalid = true;
        }
        (*state.operand0).size += 2;
    }
    if (*state.result).flags & 1 != 0 && ((*state.result).operation != InstructionOperation::INC) &&
        ((*state.result).operation != InstructionOperation::DEC)
    {
        state.invalid = true;
    }
}

unsafe fn DecodeGroup0F00(state: &mut DecodeState) {
    let rm: u8 = Peek8(state);
    let regField: u8 = rm >> 3 & 7;
    if regField >= 2 {
        state.opSize = 2;
    }
    DecodeGroupRM(state);
}

unsafe fn DecodeGroup0F01(state: &mut DecodeState) {
    let rm: u8 = Peek8(state);
    let modField: u8 = rm >> 6 & 3;
    let regField: u8 = rm >> 3 & 7;
    let rmField: u8 = rm & 7;
    if modField == 3 && regField != 4 && regField != 6 {
        (*state.result).operation = GROUP_0F01_REG_OPERATIONS[rmField as usize][regField as usize];
    } else {
        if regField < 4 {
            state.opSize = if state.using64 { 10 } else { 6 };
        } else if regField != 7 {
            state.opSize = 2;
        } else {
            state.opSize = 1;
        }
        DecodeGroupRM(state);
    }
}

unsafe fn DecodeGroup0FAE(state: &mut DecodeState) {
    let rm: u8 = Peek8(state);
    let modField: u8 = rm >> 6 & 3;
    let regField: u8 = rm >> 3 & 7;
    if modField == 3 {
        (*state.result).operation = GROUP_OPERATIONS[((*state.result).operation as usize + 1)]
            [regField as usize];
    } else {
        if regField & 2 == 0 {
            state.opSize = 512;
        } else if regField & 6 == 2 {
            state.opSize = 4;
        } else {
            state.opSize = 1;
        }
        DecodeGroupRM(state);
    }
}

unsafe fn Decode0FB8(state: &mut DecodeState) {
    if state.rep != RepPrefix::REP_PREFIX_REPE {
        if state.using64 {
            state.opSize = if state.opPrefix { 4 } else { 8 };
        }
        state.finalOpSize = GetFinalOpSize(state);
        DecodeRelImm(state);
    } else {
        DecodeRegRM(state);
    }
}

fn GetRegListForOpSize(state: &DecodeState) -> &'static [OperandType] {
    match state.opSize {
        8 => &REG64_LIST,
        4 => &REG32_LIST,
        2 => &REG16_LIST,
        _ => &INVALID_REG_LIST,
    }
}

unsafe fn DecodeRMSRegV(state: &mut DecodeState) {
    let operand0 = state.operand0;
    let regList = GetRegListForOpSize(state);
    let regSize = state.opSize;
    let mut regField: u8 = 0;
    DecodeRM(
        state,
        operand0,
        regList,
        regSize,
        &mut regField as (*mut u8),
    );
    if regField >= 6 {
        state.invalid = true;
    }
    (*state.operand1).operand =
        OperandType::from_i32(OperandType::REG_ES as (i32) + regField as (i32));
    (*state.operand1).size = 2;
    if (*state.result).operands[0].operand == OperandType::REG_CS {
        state.invalid = true;
    }
}

unsafe fn DecodeRM8(state: &mut DecodeState) {
    let operand0 = state.operand0;
    let regList = GetByteRegList(state);
    DecodeRM(state, operand0, regList, 1, ptr::null_mut());
}

unsafe fn DecodeRMV(state: &mut DecodeState) {
    let operand0 = state.operand0;
    let regList = GetRegListForOpSize(state);
    let regSize = state.opSize;
    DecodeRM(state, operand0, regList, regSize, ptr::null_mut());
}

unsafe fn DecodeFarImm(state: &mut DecodeState) {
    let operand1 = state.operand1;
    SetOperandToImm(state, operand1);
    let operand0 = state.operand0;
    SetOperandToImm16(state, operand0);
}

unsafe fn ReadAddrSize(state: &mut DecodeState) -> isize {
    match state.addrSize {
        4 | 8 => Read32(state) as isize,
        2 => Read16(state) as isize,
        _ => 0,
    }
}

unsafe fn SetOperandToImmAddr(state: &mut DecodeState, oper: *mut InstructionOperand) {
    (*oper).operand = OperandType::MEM;
    (*oper).immediate = ReadAddrSize(state);
    (*oper).segment = GetFinalSegment(state, SegmentRegister::SEG_DS);
    (*oper).size = state.finalOpSize;
}

unsafe fn DecodeEaxAddr(state: &mut DecodeState) {
    SetOperandToEaxFinalOpSize(state, state.operand0);
    let operand1 = state.operand1;
    SetOperandToImmAddr(state, operand1);
}

unsafe fn DecodeEdiEsi(state: &mut DecodeState) {
    SetOperandToEsEdi(state, state.operand0, state.finalOpSize);
    SetOperandToDsEsi(state, state.operand1, state.finalOpSize);
}

unsafe fn DecodeEdiEax(state: &mut DecodeState) {
    SetOperandToEsEdi(state, state.operand0, state.finalOpSize);
    SetOperandToEaxFinalOpSize(state, state.operand1);
}

unsafe fn DecodeEaxEsi(state: &mut DecodeState) {
    SetOperandToEaxFinalOpSize(state, state.operand0);
    SetOperandToDsEsi(state, state.operand1, state.finalOpSize);
}

unsafe fn DecodeAlEbxAl(state: &mut DecodeState) {
    let regList = GetRegListForAddrSize(state);
    (*state.operand0).operand = OperandType::REG_AL;
    (*state.operand0).size = 1;
    (*state.operand1).operand = OperandType::MEM;
    (*state.operand1).components[0] = regList[3];
    (*state.operand1).components[1] = OperandType::REG_AL;
    (*state.operand1).size = 1;
    (*state.operand1).segment = GetFinalSegment(state, SegmentRegister::SEG_DS);
}

unsafe fn DecodeEaxImm8(state: &mut DecodeState) {
    let operand0 = state.operand0;
    SetOperandToEaxFinalOpSize(state, operand0);
    let operand1 = state.operand1;
    SetOperandToImm8(state, operand1);
}

unsafe fn DecodeEaxDx(state: &mut DecodeState) {
    let operand0 = state.operand0;
    SetOperandToEaxFinalOpSize(state, operand0);
    (*state.operand1).operand = OperandType::REG_DX;
    (*state.operand1).size = 2;
}

unsafe fn Decode3DNow(state: &mut DecodeState) {
    let operand0 = state.operand0;
    let operand1 = state.operand1;
    DecodeRMReg(
        state,
        operand1,
        &MMX_REG_LIST,
        8,
        operand0,
        &MMX_REG_LIST,
        8,
    );
    let op = Read8(state);
    (*state.result).operation =
        match SPARSE_3DNOW_OPCODES.binary_search_by_key(&op, |entry| entry.opcode) {
            Ok(idx) => SPARSE_3DNOW_OPCODES[idx].operation,
            Err(_) => InstructionOperation::INVALID,
        }
}

unsafe fn DecodeSSEPrefix(state: &mut DecodeState) -> u8 {
    if state.opPrefix {
        state.opPrefix = false;
        1
    } else if state.rep == RepPrefix::REP_PREFIX_REPNE {
        state.rep = RepPrefix::REP_PREFIX_NONE;
        2
    } else if state.rep == RepPrefix::REP_PREFIX_REPE {
        state.rep = RepPrefix::REP_PREFIX_NONE;
        3
    } else {
        0
    }
}

fn GetOperandForSSEEntryType(
    state: &DecodeState,
    entry_type: SSETableOperandType,
    mut operandIndex: u8,
) -> *mut InstructionOperand {
    if entry_type == SSETableOperandType::SSE_128_FLIP {
        operandIndex = (1 - operandIndex as i32) as u8;
    }
    if operandIndex == 0 {
        state.operand0
    } else {
        state.operand1
    }
}

fn GetRegListForSSEEntryType(
    state: &mut DecodeState,
    entry_type: SSETableOperandType,
) -> &'static [OperandType] {
    if entry_type == SSETableOperandType::GPR_32_OR_64 {
        (if state.opSize == 8 {
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

fn GetSizeForSSEEntryType(state: &DecodeState, entry_type: SSETableOperandType) -> u16 {
    match entry_type {
        SSETableOperandType::GPR_32_OR_64 => if state.opSize == 8 { 8 } else { 4 },
        SSETableOperandType::MMX_64 |
        SSETableOperandType::SSE_64 => 8,
        SSETableOperandType::MMX_32 |
        SSETableOperandType::SSE_32 => 4,
        SSETableOperandType::SSE_16 => 2,
        _ => 16,
    }
}

unsafe fn UpdateOperationForSSEEntryType(state: &DecodeState, entry_type: SSETableOperandType) {
    if entry_type == SSETableOperandType::GPR_32_OR_64 && (state.opSize == 8) {
        (*state.result).operation =
            InstructionOperation::from_i32((*state.result).operation as (i32) + 1);
    }
}

unsafe fn DecodeSSETable(state: &mut DecodeState) {
    let entry_type = DecodeSSEPrefix(state) as usize;
    let rm: u8 = Peek8(state);
    let modField: u8 = rm >> 6 & 3;
    let entry = &SSE_TABLE[(*state.result).operation as usize];
    let opEntry = if modField == 3 {
        &entry.regOps[entry_type]
    } else {
        &entry.memOps[entry_type]
    };
    (*state.result).operation = opEntry.operation;
    let operand1 = GetOperandForSSEEntryType(state, opEntry.rmType, 1u8);
    let rmRegList = GetRegListForSSEEntryType(state, opEntry.rmType);
    let rmRegSize = GetSizeForSSEEntryType(state, opEntry.rmType);
    let operand0 = GetOperandForSSEEntryType(state, opEntry.regType, 0u8);
    let regList = GetRegListForSSEEntryType(state, opEntry.regType);
    let regSize = GetSizeForSSEEntryType(state, opEntry.regType);
    DecodeRMReg(
        state,
        operand1,
        rmRegList,
        rmRegSize,
        operand0,
        regList,
        regSize,
    );
    if state.flags & 0x800u32 != 0 {
        UpdateOperationForSSEEntryType(state, opEntry.regType);
        UpdateOperationForSSEEntryType(state, opEntry.rmType);
    }
}

unsafe fn DecodeSSETableImm8(state: &mut DecodeState) {
    DecodeSSETable(state);
    SetOperandToImm8(
        state,
        &mut (*state.result).operands[2usize] as (*mut InstructionOperand),
    );
}

unsafe fn DecodeSSETableMem8(state: &mut DecodeState) {
    DecodeSSETable(state);
    if (*state.operand0).operand == OperandType::MEM {
        (*state.operand0).size = 1;
    }
    if (*state.operand1).operand == OperandType::MEM {
        (*state.operand1).size = 1;
    }
}

fn GetSizeForSSEType(type_: u8) -> u16 {
    match type_ {
        2 => 8,
        3 => 4,
        _ => 16,
    }
}

unsafe fn DecodeSSE(state: &mut DecodeState) {
    let type_: u8 = DecodeSSEPrefix(state);
    let rm: u8 = Peek8(state);
    let modField: u8 = rm >> 6 & 3;
    (*state.result).operation =
        InstructionOperation::from_i32((*state.result).operation as (i32) + type_ as i32);
    let size: u16 = if modField == 3 {
        16
    } else {
        GetSizeForSSEType(type_)
    };
    let operand0 = state.operand0;
    let operand1 = state.operand1;
    DecodeRMReg(
        state,
        operand1,
        &XMM_REG_LIST,
        size,
        operand0,
        &XMM_REG_LIST,
        16,
    );
}

unsafe fn DecodeSSESingle(state: &mut DecodeState) {
    let type_: u8 = DecodeSSEPrefix(state);
    if type_ == 1 || type_ == 2 {
        state.invalid = true;
    } else {
        (*state.result).operation = InstructionOperation::from_i32(
            (*state.result).operation as (i32) + (type_ as (i32) & 1),
        );
        let operand0 = state.operand0;
        let operand1 = state.operand1;
        DecodeRMReg(
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

unsafe fn DecodeSSEPacked(state: &mut DecodeState) {
    let type_: u8 = DecodeSSEPrefix(state);
    if type_ == 2 || type_ == 3 {
        state.invalid = true;
    } else {
        (*state.result).operation = InstructionOperation::from_i32(
            (*state.result).operation as (i32) + (type_ as (i32) & 1),
        );
        let operand0 = state.operand0;
        let operand1 = state.operand1;
        DecodeRMReg(
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

unsafe fn DecodeMMX(state: &mut DecodeState) {
    let operand0 = state.operand0;
    let operand1 = state.operand1;
    if state.opPrefix {
        DecodeRMReg(
            state,
            operand1,
            &XMM_REG_LIST,
            16,
            operand0,
            &XMM_REG_LIST,
            16,
        );
    } else {
        DecodeRMReg(
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

unsafe fn DecodeMMXSSEOnly(state: &mut DecodeState) {
    if state.opPrefix {
        let operand0 = state.operand0;
        let operand1 = state.operand1;
        DecodeRMReg(
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

unsafe fn DecodeMMXGroup(state: &mut DecodeState) {
    let mut regField: u8 = 0;
    if state.opPrefix {
        let operand0 = state.operand0;
        DecodeRM(
            state,
            operand0,
            &XMM_REG_LIST,
            16,
            &mut regField as (*mut u8),
        );
        (*state.result).operation =
            MMX_GROUP_OPERATIONS[(*state.result).operation as usize][regField as (usize)][1];
    } else {
        let operand0 = state.operand0;
        DecodeRM(
            state,
            operand0,
            &MMX_REG_LIST,
            8,
            &mut regField as (*mut u8),
        );
        (*state.result).operation =
            MMX_GROUP_OPERATIONS[(*state.result).operation as usize][regField as (usize)][0];
    }
    let operand1 = state.operand1;
    SetOperandToImm8(state, operand1);
}

unsafe fn DecodePinsrw(state: &mut DecodeState) {
    DecodeSSETableImm8(state);
    if (*state.operand1).operand == OperandType::MEM {
        (*state.operand1).size = 2;
    }
}

unsafe fn DecodeRegCR(state: &mut DecodeState) {
    if state.opSize == 2 {
        state.opSize = 4;
    }
    let regList = GetRegListForOpSize(state);
    let reg = Read8(state);
    if (*state.result).flags & 1 != 0 {
        (*state.result).flags &= !1;
        state.rexReg = true;
    }
    (*state.operand0).operand = regList[((reg & 7) + if (*state).rexRM1 { 8 } else { 0 }) as usize];
    (*state.operand0).size = (*state).opSize;
    (*state.operand1).operand = OperandType::from_i32(
        ((*state.result).operation as (i32) + (reg as (i32) >> 3 & 7) +
             if state.rexReg { 8 } else { 0 }) as (i32),
    );
    (*state.operand1).size = state.opSize;
    (*state.result).operation = InstructionOperation::MOV;
}

unsafe fn DecodeMovSXZX8(state: &mut DecodeState) {
    let operand0 = state.operand0;
    let operand1 = state.operand1;
    let rmRegList = GetByteRegList(state);
    let regList = GetRegListForOpSize(state);
    let regSize = state.opSize;
    DecodeRMReg(state, operand1, rmRegList, 1, operand0, regList, regSize);
}

unsafe fn DecodeMovSXZX16(state: &mut DecodeState) {
    let operand0 = state.operand0;
    let operand1 = state.operand1;
    let regList = GetRegListForOpSize(state);
    let regSize = state.opSize;
    DecodeRMReg(state, operand1, &REG16_LIST, 2, operand0, regList, regSize);
}

unsafe fn DecodeMem16(state: &mut DecodeState) {
    let operand0 = state.operand0;
    DecodeRM(state, operand0, &REG32_LIST, 2, ptr::null_mut());
    if (*state.operand0).operand != OperandType::MEM {
        state.invalid = true;
    }
}

unsafe fn DecodeMem32(state: &mut DecodeState) {
    let operand0 = state.operand0;
    DecodeRM(state, operand0, &REG32_LIST, 4, ptr::null_mut());
    if (*state.operand0).operand != OperandType::MEM {
        state.invalid = true;
    }
}

unsafe fn DecodeMem64(state: &mut DecodeState) {
    let operand0 = state.operand0;
    DecodeRM(state, operand0, &REG32_LIST, 8, ptr::null_mut());
    if (*state.operand0).operand != OperandType::MEM {
        state.invalid = true;
    }
}

unsafe fn DecodeMem80(state: &mut DecodeState) {
    let operand0 = state.operand0;
    DecodeRM(state, operand0, &REG32_LIST, 10, ptr::null_mut());
    if (*state.operand0).operand != OperandType::MEM {
        state.invalid = true;
    }
}

unsafe fn DecodeMemFloatEnv(state: &mut DecodeState) {
    let operand0 = state.operand0;
    let rmSize = if state.opSize == 2 { 14 } else { 28 };
    DecodeRM(state, operand0, &REG32_LIST, rmSize, ptr::null_mut());
    if (*state.operand0).operand != OperandType::MEM {
        state.invalid = true;
    }
}

unsafe fn DecodeMemFloatSave(state: &mut DecodeState) {
    let operand0 = state.operand0;
    let rmSize = if state.opSize == 2 { 94 } else { 108 };
    DecodeRM(state, operand0, &REG32_LIST, rmSize, ptr::null_mut());
    if (*state.operand0).operand != OperandType::MEM {
        state.invalid = true;
    }
}

unsafe fn DecodeFPUReg(state: &mut DecodeState) {
    let operand0 = state.operand0;
    DecodeRM(state, operand0, &FPU_REG_LIST, 10, ptr::null_mut());
}

unsafe fn DecodeFPURegST0(state: &mut DecodeState) {
    DecodeFPUReg(state);
    (*state.operand1).operand = OperandType::REG_ST0;
    (*state.operand1).size = 10;
}

unsafe fn DecodeRegGroupNoOperands(state: &mut DecodeState) {
    let rmByte: u8 = Read8(state);
    (*state.result).operation = GROUP_OPERATIONS[(*state.result).operation as usize][(rmByte & 7) as
                                                                                         usize];
}

unsafe fn DecodeRegGroupAX(state: &mut DecodeState) {
    DecodeRegGroupNoOperands(state);
    (*state.operand0).operand = OperandType::REG_AX;
    (*state.operand0).size = 2u16;
}

unsafe fn DecodeCmpXch8B(state: &mut DecodeState) {
    let rm: u8 = Peek8(state);
    let regField: u8 = rm >> 3 & 7;
    if regField == 1 {
        if state.opSize == 2 {
            state.opSize = 4;
        } else if state.opSize == 8 {
            (*state.result).operation = InstructionOperation::CMPXCH16B;
        }
        let operand0 = state.operand0;
        let rmSize = state.opSize * 2;
        let regList = GetRegListForOpSize(state);
        DecodeRM(state, operand0, regList, rmSize, ptr::null_mut());
    } else if regField == 6 {
        if state.opPrefix {
            (*state.result).operation = InstructionOperation::VMCLEAR;
        } else if state.rep == RepPrefix::REP_PREFIX_REPE {
            (*state.result).operation = InstructionOperation::VMXON;
        } else {
            (*state.result).operation = InstructionOperation::VMPTRLD;
        }
        let operand0 = state.operand0;
        DecodeRM(state, operand0, &REG64_LIST, 8, ptr::null_mut());
    } else if regField == 7 {
        (*state.result).operation = InstructionOperation::VMPTRST;
        let operand0 = state.operand0;
        DecodeRM(state, operand0, &REG64_LIST, 8, ptr::null_mut());
    } else {
        state.invalid = true;
    }
    if (*state.operand0).operand != OperandType::MEM {
        state.invalid = true;
    }
}

unsafe fn DecodeMovNti(state: &mut DecodeState) {
    if state.opSize == 2 {
        state.opSize = 4;
    }
    let operand0 = state.operand0;
    let operand1 = state.operand1;
    let opSize = state.opSize;
    let regList = GetRegListForOpSize(state);
    DecodeRMReg(state, operand0, regList, opSize, operand1, regList, opSize);
    if (*state.operand0).operand != OperandType::MEM {
        state.invalid = true;
    }
}

unsafe fn DecodeCrc32(state: &mut DecodeState) {
    let srcRegList = GetRegListForFinalOpSize(state);
    let destRegList = if (*state).opSize == 8 {
        &REG64_LIST
    } else {
        &REG32_LIST
    };
    let destSize: u16 = if state.opSize == 8 { 8 } else { 4 };
    let operand0 = state.operand0;
    let operand1 = state.operand1;
    let finalOpSize = state.finalOpSize;
    DecodeRMReg(
        state,
        operand1,
        srcRegList,
        finalOpSize,
        operand0,
        destRegList,
        destSize,
    );
}

unsafe fn DecodeArpl(state: &mut DecodeState) {
    if state.using64 {
        let regList = GetRegListForFinalOpSize(state);
        (*state.result).operation = InstructionOperation::MOVSXD;
        let operand0 = state.operand0;
        let operand1 = state.operand1;
        let finalOpSize = state.finalOpSize;
        DecodeRMReg(
            state,
            operand1,
            &REG32_LIST,
            4,
            operand0,
            regList,
            finalOpSize,
        );
    } else {
        state.operand0 = &mut (*state.result).operands[1] as (*mut InstructionOperand);
        state.operand1 = &mut (*state.result).operands[0] as (*mut InstructionOperand);
        state.finalOpSize = 2;
        DecodeRegRM(state);
    }
}

unsafe fn ClearOperand(oper: &mut InstructionOperand) {
    oper.operand = OperandType::NONE;
    oper.components[0] = OperandType::NONE;
    oper.components[1] = OperandType::NONE;
    oper.scale = 1;
    oper.immediate = 0;
}

unsafe fn InitDisassemble(state: &mut DecodeState) {
    ClearOperand(&mut (*(*state).result).operands[0]);
    ClearOperand(&mut (*(*state).result).operands[1]);
    ClearOperand(&mut (*(*state).result).operands[2]);
    (*state.result).operation = InstructionOperation::INVALID;
    (*state.result).flags = 0;
    (*state.result).segment = SegmentRegister::SEG_DEFAULT;
    state.invalid = false;
    state.insufficientLength = false;
    state.opPrefix = false;
    state.rep = RepPrefix::REP_PREFIX_NONE;
    state.ripRelFixup = ptr::null_mut();
    state.rex = false;
    state.rexReg = false;
    state.rexRM1 = false;
    state.rexRM2 = false;
    state.origLen = state.len;
}

unsafe fn ProcessPrefixes(state: &mut DecodeState) {
    let mut rex: u8 = 0;
    let mut addrPrefix: bool = false;
    loop {
        if !!state.invalid {
            break;
        }
        let prefix: u8 = Read8(state);
        if state.invalid {
            break;
        }
        if prefix >= 0x26 && (prefix <= 0x3e) && (prefix & 7 == 6) {
            (*state.result).segment = SegmentRegister::from_i32(
                SegmentRegister::SEG_ES as i32 + ((prefix as i32 >> 3) - 4),
            );
        } else if prefix == 0x64 || prefix == 0x65 {
            (*state.result).segment =
                SegmentRegister::from_i32(SegmentRegister::SEG_ES as i32 + (prefix as i32 - 0x60));
        } else if prefix == 0x66 {
            state.opPrefix = true;
            (*state.result).flags |= 16;
        } else if prefix == 0x67 {
            addrPrefix = true;
            (*state.result).flags |= 32;
        } else if prefix == 0xf0 {
            (*state.result).flags |= 1;
        } else if prefix == 0xf2 {
            state.rep = RepPrefix::REP_PREFIX_REPNE;
        } else if prefix == 0xf3 {
            state.rep = RepPrefix::REP_PREFIX_REPE;
        } else {
            if !(state.using64 && (prefix >= 0x40) && (prefix <= 0x4f)) {
                state.opcode = state.opcode.offset(-1);
                state.len = state.len.wrapping_add(1);
                break;
            }
            rex = prefix;
            continue;
        }
        rex = 0u8;
    }
    if state.opPrefix {
        state.opSize = if state.opSize == 2 { 4 } else { 2 };
    }
    if addrPrefix {
        state.addrSize = if state.addrSize == 4 { 2 } else { 4 };
    }
    if rex != 0 {
        state.rex = true;
        state.rexRM1 = rex & 1 != 0;
        state.rexRM2 = rex & 2 != 0;
        state.rexReg = rex & 4 != 0;
        if rex & 8 != 0 {
            state.opSize = 8;
        }
    }
}

unsafe fn FinishDisassemble(state: &mut DecodeState) {
    (*state.result).length = ((state.opcode as (isize)).wrapping_sub(
        state.opcodeStart as (isize),
    ) / ::std::mem::size_of::<u8>() as (isize)) as (usize);
    if !state.ripRelFixup.is_null() {
        *state.ripRelFixup = (*state.ripRelFixup as (usize)).wrapping_add(
            state.addr.wrapping_add(
                (*state.result)
                    .length,
            ),
        ) as (isize);
    }
    if state.insufficientLength && (state.origLen < 15) {
        (*state.result).flags |= 0x8000_0000;
    }
}

pub fn Disassemble16(opcode: &[u8], addr: usize, maxLen: usize, result: &mut Instruction) -> bool {
    unsafe {
        let mut state = DecodeState::default();
        state.result = result as *mut Instruction;
        state.opcodeStart = opcode.as_ptr();
        state.opcode = opcode.as_ptr();
        state.addr = addr;
        state.len = if maxLen > 15 { 15 } else { maxLen };
        state.addrSize = 2;
        state.opSize = 2;
        state.using64 = false;
        InitDisassemble(&mut state);
        ProcessPrefixes(&mut state);
        let next_opcode = Read8(&mut state);
        ProcessOpcode(&mut state, &MAIN_OPCODE_MAP, next_opcode);
        FinishDisassemble(&mut state);
        !state.invalid
    }
}

pub fn Disassemble32(opcode: &[u8], addr: usize, maxLen: usize, result: &mut Instruction) -> bool {
    unsafe {
        let mut state = DecodeState::default();
        state.result = result as *mut Instruction;
        state.opcodeStart = opcode.as_ptr();
        state.opcode = opcode.as_ptr();
        state.addr = addr;
        state.len = if maxLen > 15 { 15 } else { maxLen };
        state.addrSize = 4;
        state.opSize = 4;
        state.using64 = false;
        InitDisassemble(&mut state);
        ProcessPrefixes(&mut state);
        let next_opcode = Read8(&mut state);
        ProcessOpcode(&mut state, &MAIN_OPCODE_MAP, next_opcode);
        FinishDisassemble(&mut state);
        !state.invalid
    }
}

pub fn Disassemble64(opcode: &[u8], addr: usize, maxLen: usize, result: &mut Instruction) -> bool {
    unsafe {
        let mut state = DecodeState::default();
        state.result = result as *mut Instruction;
        state.opcodeStart = opcode.as_ptr();
        state.opcode = opcode.as_ptr();
        state.addr = addr;
        state.len = if maxLen > 15 { 15 } else { maxLen };
        state.addrSize = 8;
        state.opSize = 4;
        state.using64 = true;
        InitDisassemble(&mut state);
        ProcessPrefixes(&mut state);
        let next_opcode = Read8(&mut state);
        ProcessOpcode(&mut state, &MAIN_OPCODE_MAP, next_opcode);
        FinishDisassemble(&mut state);
        !state.invalid
    }
}

fn WriteOperand(stream: &mut fmt::Write, type_: OperandType, scale: u8, plus: bool) -> fmt::Result {
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

fn GetSizeString(size: u16) -> &'static str {
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

pub fn FormatInstructionString(
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
                for _i in instr.length..(width as usize) {
                    try!(stream.write_str("  "));
                }
            } else if fmt[f] == 'i' {
                if instr.flags & (2 | 8 | 4) != 0 {
                    try!(stream.write_str("rep"));
                    if instr.flags & 4 != 0 {
                        try!(stream.write_char('n'));
                    }
                    if instr.flags & (4 | 8) != 0 {
                        try!(stream.write_char('e'));
                    }
                    try!(stream.write_char('b'));
                }
                if instr.flags & 1 != 0 {
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
                        try!(stream.write_str(GetSizeString(instr.operands[i].size)));
                        if instr.segment != SegmentRegister::SEG_DEFAULT ||
                            instr.operands[i].segment == SegmentRegister::SEG_ES
                        {
                            try!(WriteOperand(
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
                            try!(WriteOperand(
                                stream,
                                instr.operands[i].components[0],
                                1,
                                false,
                            ));
                            plus = true;
                        }
                        if instr.operands[i].components[1] != OperandType::NONE {
                            try!(WriteOperand(
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
                        try!(WriteOperand(stream, instr.operands[i].operand, 1, false));
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

pub fn DisassembleToString16(
    stream: &mut fmt::Write,
    fmt: &str,
    opcode: &[u8],
    addr: usize,
    maxLen: usize,
    instr: &mut Instruction,
) -> fmt::Result {
    if !Disassemble16(opcode, addr, maxLen, instr) {
        Ok(())
    } else {
        FormatInstructionString(stream, fmt, opcode, addr, instr)
    }
}

pub fn DisassembleToString32(
    stream: &mut fmt::Write,
    fmt: &str,
    opcode: &[u8],
    addr: usize,
    maxLen: usize,
    instr: &mut Instruction,
) -> fmt::Result {
    if !Disassemble32(opcode, addr, maxLen, instr) {
        Ok(())
    } else {
        FormatInstructionString(stream, fmt, opcode, addr, instr)
    }
}

pub fn DisassembleToString64(
    stream: &mut fmt::Write,
    fmt: &str,
    opcode: &[u8],
    addr: usize,
    maxLen: usize,
    instr: &mut Instruction,
) -> fmt::Result {
    if !Disassemble64(opcode, addr, maxLen, instr) {
        Ok(())
    } else {
        FormatInstructionString(stream, fmt, opcode, addr, instr)
    }
}
