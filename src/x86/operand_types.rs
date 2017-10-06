// Licensed under the 2-Clause BSD license <LICENSE or
// https://opensource.org/licenses/BSD-2-Clause>. This
// file may not be copied, modified, or distributed
// except according to those terms.

/// The location used by an operand.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(i32)]
pub enum OperandType {
    /// Invalid / no operand.
    NONE = 0i32,
    /// An immediate operand.
    IMM,
    /// A memory operand.
    MEM,
    /// GPR. Accumulator register. 8 LSB bits of 16.
    REG_AL,
    /// GPR. Counter register. 8 LSB bits of 16.
    REG_CL,
    /// GPR. Data register. 8 LSB bits of 16.
    REG_DL,
    /// GPR. Base register. 8 LSB bits of 16.
    REG_BL,
    /// GPR. Accumulator register. 8 MSB bits of 16.
    REG_AH,
    /// GPR. Counter register. 8 MSB bits of 16.
    REG_CH,
    /// GPR. Data register. 8 MSB bits of 16.
    REG_DH,
    /// GPR. Base register. 8 MSB bits of 16.
    REG_BH,
    #[allow(missing_docs)]
    REG_SPL,
    #[allow(missing_docs)]
    REG_BPL,
    #[allow(missing_docs)]
    REG_SIL,
    #[allow(missing_docs)]
    REG_DIL,
    /// x86_64 GPR. Lowermost 8 bits of 64.
    REG_R8B,
    /// x86_64 GPR. Lowermost 8 bits of 64.
    REG_R9B,
    /// x86_64 GPR. Lowermost 8 bits of 64.
    REG_R10B,
    /// x86_64 GPR. Lowermost 8 bits of 64.
    REG_R11B,
    /// x86_64 GPR. Lowermost 8 bits of 64.
    REG_R12B,
    /// x86_64 GPR. Lowermost 8 bits of 64.
    REG_R13B,
    /// x86_64 GPR. Lowermost 8 bits of 64.
    REG_R14B,
    /// x86_64 GPR. Lowermost 8 bits of 64.
    REG_R15B,
    /// GPR. Accumulator register. 16 bits.
    REG_AX,
    /// GPR. Counter register. 16 bits.
    REG_CX,
    /// GPR. Data register. 16 bits.
    REG_DX,
    /// GPR. Base register. 16 bits.
    REG_BX,
    /// GPR. Stack pointer register. 16 bits.
    REG_SP,
    /// GPR. Stack base pointer register. 16 bits.
    REG_BP,
    /// GPR. Source index register. 16 bits.
    REG_SI,
    /// GPR. Destination index register. 16 bits.
    REG_DI,
    /// x86_64 GPR. Lowermost 16 bits of 64.
    REG_R8W,
    /// x86_64 GPR. Lowermost 16 bits of 64.
    REG_R9W,
    /// x86_64 GPR. Lowermost 16 bits of 64.
    REG_R10W,
    /// x86_64 GPR. Lowermost 16 bits of 64.
    REG_R11W,
    /// x86_64 GPR. Lowermost 16 bits of 64.
    REG_R12W,
    /// x86_64 GPR. Lowermost 16 bits of 64.
    REG_R13W,
    /// x86_64 GPR. Lowermost 16 bits of 64.
    REG_R14W,
    /// x86_64 GPR. Lowermost 16 bits of 64.
    REG_R15W,
    /// GPR. Accumulator register. 32 bits.
    REG_EAX,
    /// GPR. Counter register. 32 bits.
    REG_ECX,
    /// GPR. Data register. 32 bits.
    REG_EDX,
    /// GPR. Base register. 32 bits.
    REG_EBX,
    /// GPR. Stack pointer register. 32 bits.
    REG_ESP,
    /// GPR. Stack base pointer register. 32 bits.
    REG_EBP,
    /// GPR. Source index register. 32 bits.
    REG_ESI,
    /// GPR. Destination index register. 32 bits.
    REG_EDI,
    /// x86_64 GPR. Lowermost 32 bits of 64.
    REG_R8D,
    /// x86_64 GPR. Lowermost 32 bits of 64.
    REG_R9D,
    /// x86_64 GPR. Lowermost 32 bits of 64.
    REG_R10D,
    /// x86_64 GPR. Lowermost 32 bits of 64.
    REG_R11D,
    /// x86_64 GPR. Lowermost 32 bits of 64.
    REG_R12D,
    /// x86_64 GPR. Lowermost 32 bits of 64.
    REG_R13D,
    /// x86_64 GPR. Lowermost 32 bits of 64.
    REG_R14D,
    /// x86_64 GPR. Lowermost 32 bits of 64.
    REG_R15D,
    /// GPR. Accumulator register. 64 bits.
    REG_RAX,
    /// GPR. Counter register. 64 bits.
    REG_RCX,
    /// GPR. Data register. 64 bits.
    REG_RDX,
    /// GPR. Base register. 64 bits.
    REG_RBX,
    /// GPR. Stack pointer register. 64 bits.
    REG_RSP,
    /// GPR. Stack base pointer register. 64 bits.
    REG_RBP,
    /// GPR. Source index register. 64 bits.
    REG_RSI,
    /// GPR. Destination index register. 64 bits.
    REG_RDI,
    /// x86_64 GPR. 64 bits.
    REG_R8,
    /// x86_64 GPR. 64 bits.
    REG_R9,
    /// x86_64 GPR. 64 bits.
    REG_R10,
    /// x86_64 GPR. 64 bits.
    REG_R11,
    /// x86_64 GPR. 64 bits.
    REG_R12,
    /// x86_64 GPR. 64 bits.
    REG_R13,
    /// x86_64 GPR. 64 bits.
    REG_R14,
    /// x86_64 GPR. 64 bits.
    REG_R15,
    /// FPU register.
    REG_ST0,
    /// FPU register.
    REG_ST1,
    /// FPU register.
    REG_ST2,
    /// FPU register.
    REG_ST3,
    /// FPU register.
    REG_ST4,
    /// FPU register.
    REG_ST5,
    /// FPU register.
    REG_ST6,
    /// FPU register.
    REG_ST7,
    /// MMX register.
    REG_MM0,
    /// MMX register.
    REG_MM1,
    /// MMX register.
    REG_MM2,
    /// MMX register.
    REG_MM3,
    /// MMX register.
    REG_MM4,
    /// MMX register.
    REG_MM5,
    /// MMX register.
    REG_MM6,
    /// MMX register.
    REG_MM7,
    /// SSE register. 128 bits.
    REG_XMM0,
    /// SSE register. 128 bits.
    REG_XMM1,
    /// SSE register. 128 bits.
    REG_XMM2,
    /// SSE register. 128 bits.
    REG_XMM3,
    /// SSE register. 128 bits.
    REG_XMM4,
    /// SSE register. 128 bits.
    REG_XMM5,
    /// SSE register. 128 bits.
    REG_XMM6,
    /// SSE register. 128 bits.
    REG_XMM7,
    /// SSE register. 128 bits.
    REG_XMM8,
    /// SSE register. 128 bits.
    REG_XMM9,
    /// SSE register. 128 bits.
    REG_XMM10,
    /// SSE register. 128 bits.
    REG_XMM11,
    /// SSE register. 128 bits.
    REG_XMM12,
    /// SSE register. 128 bits.
    REG_XMM13,
    /// SSE register. 128 bits.
    REG_XMM14,
    /// SSE register. 128 bits.
    REG_XMM15,
    /// Control register.
    REG_CR0,
    /// Control register.
    REG_CR1,
    /// Control register.
    REG_CR2,
    /// Control register.
    REG_CR3,
    /// Control register.
    REG_CR4,
    /// Control register.
    REG_CR5,
    /// Control register.
    REG_CR6,
    /// Control register.
    REG_CR7,
    /// Control register.
    REG_CR8,
    /// Control register.
    REG_CR9,
    /// Control register.
    REG_CR10,
    /// Control register.
    REG_CR11,
    /// Control register.
    REG_CR12,
    /// Control register.
    REG_CR13,
    /// Control register.
    REG_CR14,
    /// Control register.
    REG_CR15,
    /// Debug register.
    REG_DR0,
    /// Debug register.
    REG_DR1,
    /// Debug register.
    REG_DR2,
    /// Debug register.
    REG_DR3,
    /// Debug register.
    REG_DR4,
    /// Debug register.
    REG_DR5,
    /// Debug register.
    REG_DR6,
    /// Debug register.
    REG_DR7,
    /// Debug register.
    REG_DR8,
    /// Debug register.
    REG_DR9,
    /// Debug register.
    REG_DR10,
    /// Debug register.
    REG_DR11,
    /// Debug register.
    REG_DR12,
    /// Debug register.
    REG_DR13,
    /// Debug register.
    REG_DR14,
    /// Debug register.
    REG_DR15,
    /// Task register.
    REG_TR0,
    /// Task register.
    REG_TR1,
    /// Task register.
    REG_TR2,
    /// Task register.
    REG_TR3,
    /// Task register.
    REG_TR4,
    /// Task register.
    REG_TR5,
    /// Task register.
    REG_TR6,
    /// Task register.
    REG_TR7,
    /// Task register.
    REG_TR8,
    /// Task register.
    REG_TR9,
    /// Task register.
    REG_TR10,
    /// Task register.
    REG_TR11,
    /// Task register.
    REG_TR12,
    /// Task register.
    REG_TR13,
    /// Task register.
    REG_TR14,
    /// Task register.
    REG_TR15,
    /// Segment register. Pointer to extra data.
    REG_ES,
    /// Segment register. Pointer to the code.
    REG_CS,
    /// Segment register. Pointer to the stack.
    REG_SS,
    /// Segment register. Pointer to the data.
    REG_DS,
    /// Segment register. Pointer to extra data. Used as a thread register on some platforms.
    REG_FS,
    /// Segment register. Pointer to extra data. Used as a thread register on some platforms.
    REG_GS,
    /// Instruction pointer. 64 bits.
    REG_RIP,
}

impl OperandType {
    /// Look up an `OperandType` given its `i32` value.
    pub fn from_i32(i: i32) -> Self {
        OPERAND_TYPE_TABLE[i as usize].value
    }
}

impl Default for OperandType {
    fn default() -> Self {
        OperandType::NONE
    }
}

pub(crate) struct OperandTypeInfo {
    pub name: &'static str,
    pub value: OperandType,
}

pub(crate) static OPERAND_TYPE_TABLE: [OperandTypeInfo; 158] = [
    OperandTypeInfo {
        name: "",
        value: OperandType::NONE,
    },
    OperandTypeInfo {
        name: "",
        value: OperandType::IMM,
    },
    OperandTypeInfo {
        name: "",
        value: OperandType::MEM,
    },
    OperandTypeInfo {
        name: "al",
        value: OperandType::REG_AL,
    },
    OperandTypeInfo {
        name: "cl",
        value: OperandType::REG_CL,
    },
    OperandTypeInfo {
        name: "dl",
        value: OperandType::REG_DL,
    },
    OperandTypeInfo {
        name: "bl",
        value: OperandType::REG_BL,
    },
    OperandTypeInfo {
        name: "ah",
        value: OperandType::REG_AH,
    },
    OperandTypeInfo {
        name: "ch",
        value: OperandType::REG_CH,
    },
    OperandTypeInfo {
        name: "dh",
        value: OperandType::REG_DH,
    },
    OperandTypeInfo {
        name: "bh",
        value: OperandType::REG_BH,
    },
    OperandTypeInfo {
        name: "spl",
        value: OperandType::REG_SPL,
    },
    OperandTypeInfo {
        name: "bpl",
        value: OperandType::REG_BPL,
    },
    OperandTypeInfo {
        name: "sil",
        value: OperandType::REG_SIL,
    },
    OperandTypeInfo {
        name: "dil",
        value: OperandType::REG_DIL,
    },
    OperandTypeInfo {
        name: "r8b",
        value: OperandType::REG_R8B,
    },
    OperandTypeInfo {
        name: "r9b",
        value: OperandType::REG_R9B,
    },
    OperandTypeInfo {
        name: "r10b",
        value: OperandType::REG_R10B,
    },
    OperandTypeInfo {
        name: "r11b",
        value: OperandType::REG_R11B,
    },
    OperandTypeInfo {
        name: "r12b",
        value: OperandType::REG_R12B,
    },
    OperandTypeInfo {
        name: "r13b",
        value: OperandType::REG_R13B,
    },
    OperandTypeInfo {
        name: "r14b",
        value: OperandType::REG_R14B,
    },
    OperandTypeInfo {
        name: "r15b",
        value: OperandType::REG_R15B,
    },
    OperandTypeInfo {
        name: "ax",
        value: OperandType::REG_AX,
    },
    OperandTypeInfo {
        name: "cx",
        value: OperandType::REG_CX,
    },
    OperandTypeInfo {
        name: "dx",
        value: OperandType::REG_DX,
    },
    OperandTypeInfo {
        name: "bx",
        value: OperandType::REG_BX,
    },
    OperandTypeInfo {
        name: "sp",
        value: OperandType::REG_SP,
    },
    OperandTypeInfo {
        name: "bp",
        value: OperandType::REG_BP,
    },
    OperandTypeInfo {
        name: "si",
        value: OperandType::REG_SI,
    },
    OperandTypeInfo {
        name: "di",
        value: OperandType::REG_DI,
    },
    OperandTypeInfo {
        name: "r8w",
        value: OperandType::REG_R8W,
    },
    OperandTypeInfo {
        name: "r9w",
        value: OperandType::REG_R9W,
    },
    OperandTypeInfo {
        name: "r10w",
        value: OperandType::REG_R10W,
    },
    OperandTypeInfo {
        name: "r11w",
        value: OperandType::REG_R11W,
    },
    OperandTypeInfo {
        name: "r12w",
        value: OperandType::REG_R12W,
    },
    OperandTypeInfo {
        name: "r13w",
        value: OperandType::REG_R13W,
    },
    OperandTypeInfo {
        name: "r14w",
        value: OperandType::REG_R14W,
    },
    OperandTypeInfo {
        name: "r15w",
        value: OperandType::REG_R15W,
    },
    OperandTypeInfo {
        name: "eax",
        value: OperandType::REG_EAX,
    },
    OperandTypeInfo {
        name: "ecx",
        value: OperandType::REG_ECX,
    },
    OperandTypeInfo {
        name: "edx",
        value: OperandType::REG_EDX,
    },
    OperandTypeInfo {
        name: "ebx",
        value: OperandType::REG_EBX,
    },
    OperandTypeInfo {
        name: "esp",
        value: OperandType::REG_ESP,
    },
    OperandTypeInfo {
        name: "ebp",
        value: OperandType::REG_EBP,
    },
    OperandTypeInfo {
        name: "esi",
        value: OperandType::REG_ESI,
    },
    OperandTypeInfo {
        name: "edi",
        value: OperandType::REG_EDI,
    },
    OperandTypeInfo {
        name: "r8d",
        value: OperandType::REG_R8D,
    },
    OperandTypeInfo {
        name: "r9d",
        value: OperandType::REG_R9D,
    },
    OperandTypeInfo {
        name: "r10d",
        value: OperandType::REG_R10D,
    },
    OperandTypeInfo {
        name: "r11d",
        value: OperandType::REG_R11D,
    },
    OperandTypeInfo {
        name: "r12d",
        value: OperandType::REG_R12D,
    },
    OperandTypeInfo {
        name: "r13d",
        value: OperandType::REG_R13D,
    },
    OperandTypeInfo {
        name: "r14d",
        value: OperandType::REG_R14D,
    },
    OperandTypeInfo {
        name: "r15d",
        value: OperandType::REG_R15D,
    },
    OperandTypeInfo {
        name: "rax",
        value: OperandType::REG_RAX,
    },
    OperandTypeInfo {
        name: "rcx",
        value: OperandType::REG_RCX,
    },
    OperandTypeInfo {
        name: "rdx",
        value: OperandType::REG_RDX,
    },
    OperandTypeInfo {
        name: "rbx",
        value: OperandType::REG_RBX,
    },
    OperandTypeInfo {
        name: "rsp",
        value: OperandType::REG_RSP,
    },
    OperandTypeInfo {
        name: "rbp",
        value: OperandType::REG_RBP,
    },
    OperandTypeInfo {
        name: "rsi",
        value: OperandType::REG_RSI,
    },
    OperandTypeInfo {
        name: "rdi",
        value: OperandType::REG_RDI,
    },
    OperandTypeInfo {
        name: "r8",
        value: OperandType::REG_R8,
    },
    OperandTypeInfo {
        name: "r9",
        value: OperandType::REG_R9,
    },
    OperandTypeInfo {
        name: "r10",
        value: OperandType::REG_R10,
    },
    OperandTypeInfo {
        name: "r11",
        value: OperandType::REG_R11,
    },
    OperandTypeInfo {
        name: "r12",
        value: OperandType::REG_R12,
    },
    OperandTypeInfo {
        name: "r13",
        value: OperandType::REG_R13,
    },
    OperandTypeInfo {
        name: "r14",
        value: OperandType::REG_R14,
    },
    OperandTypeInfo {
        name: "r15",
        value: OperandType::REG_R15,
    },
    OperandTypeInfo {
        name: "st0",
        value: OperandType::REG_ST0,
    },
    OperandTypeInfo {
        name: "st1",
        value: OperandType::REG_ST1,
    },
    OperandTypeInfo {
        name: "st2",
        value: OperandType::REG_ST2,
    },
    OperandTypeInfo {
        name: "st3",
        value: OperandType::REG_ST3,
    },
    OperandTypeInfo {
        name: "st4",
        value: OperandType::REG_ST4,
    },
    OperandTypeInfo {
        name: "st5",
        value: OperandType::REG_ST5,
    },
    OperandTypeInfo {
        name: "st6",
        value: OperandType::REG_ST6,
    },
    OperandTypeInfo {
        name: "st7",
        value: OperandType::REG_ST7,
    },
    OperandTypeInfo {
        name: "mm0",
        value: OperandType::REG_MM0,
    },
    OperandTypeInfo {
        name: "mm1",
        value: OperandType::REG_MM1,
    },
    OperandTypeInfo {
        name: "mm2",
        value: OperandType::REG_MM2,
    },
    OperandTypeInfo {
        name: "mm3",
        value: OperandType::REG_MM3,
    },
    OperandTypeInfo {
        name: "mm4",
        value: OperandType::REG_MM4,
    },
    OperandTypeInfo {
        name: "mm5",
        value: OperandType::REG_MM5,
    },
    OperandTypeInfo {
        name: "mm6",
        value: OperandType::REG_MM6,
    },
    OperandTypeInfo {
        name: "mm7",
        value: OperandType::REG_MM7,
    },
    OperandTypeInfo {
        name: "xmm0",
        value: OperandType::REG_XMM0,
    },
    OperandTypeInfo {
        name: "xmm1",
        value: OperandType::REG_XMM1,
    },
    OperandTypeInfo {
        name: "xmm2",
        value: OperandType::REG_XMM2,
    },
    OperandTypeInfo {
        name: "xmm3",
        value: OperandType::REG_XMM3,
    },
    OperandTypeInfo {
        name: "xmm4",
        value: OperandType::REG_XMM4,
    },
    OperandTypeInfo {
        name: "xmm5",
        value: OperandType::REG_XMM5,
    },
    OperandTypeInfo {
        name: "xmm6",
        value: OperandType::REG_XMM6,
    },
    OperandTypeInfo {
        name: "xmm7",
        value: OperandType::REG_XMM7,
    },
    OperandTypeInfo {
        name: "xmm8",
        value: OperandType::REG_XMM8,
    },
    OperandTypeInfo {
        name: "xmm9",
        value: OperandType::REG_XMM9,
    },
    OperandTypeInfo {
        name: "xmm10",
        value: OperandType::REG_XMM10,
    },
    OperandTypeInfo {
        name: "xmm11",
        value: OperandType::REG_XMM11,
    },
    OperandTypeInfo {
        name: "xmm12",
        value: OperandType::REG_XMM12,
    },
    OperandTypeInfo {
        name: "xmm13",
        value: OperandType::REG_XMM13,
    },
    OperandTypeInfo {
        name: "xmm14",
        value: OperandType::REG_XMM14,
    },
    OperandTypeInfo {
        name: "xmm15",
        value: OperandType::REG_XMM15,
    },
    OperandTypeInfo {
        name: "cr0",
        value: OperandType::REG_CR0,
    },
    OperandTypeInfo {
        name: "cr1",
        value: OperandType::REG_CR1,
    },
    OperandTypeInfo {
        name: "cr2",
        value: OperandType::REG_CR2,
    },
    OperandTypeInfo {
        name: "cr3",
        value: OperandType::REG_CR3,
    },
    OperandTypeInfo {
        name: "cr4",
        value: OperandType::REG_CR4,
    },
    OperandTypeInfo {
        name: "cr5",
        value: OperandType::REG_CR5,
    },
    OperandTypeInfo {
        name: "cr6",
        value: OperandType::REG_CR6,
    },
    OperandTypeInfo {
        name: "cr7",
        value: OperandType::REG_CR7,
    },
    OperandTypeInfo {
        name: "cr8",
        value: OperandType::REG_CR8,
    },
    OperandTypeInfo {
        name: "cr9",
        value: OperandType::REG_CR9,
    },
    OperandTypeInfo {
        name: "cr10",
        value: OperandType::REG_CR10,
    },
    OperandTypeInfo {
        name: "cr11",
        value: OperandType::REG_CR11,
    },
    OperandTypeInfo {
        name: "cr12",
        value: OperandType::REG_CR12,
    },
    OperandTypeInfo {
        name: "cr13",
        value: OperandType::REG_CR13,
    },
    OperandTypeInfo {
        name: "cr14",
        value: OperandType::REG_CR14,
    },
    OperandTypeInfo {
        name: "cr15",
        value: OperandType::REG_CR15,
    },
    OperandTypeInfo {
        name: "dr0",
        value: OperandType::REG_DR0,
    },
    OperandTypeInfo {
        name: "dr1",
        value: OperandType::REG_DR1,
    },
    OperandTypeInfo {
        name: "dr2",
        value: OperandType::REG_DR2,
    },
    OperandTypeInfo {
        name: "dr3",
        value: OperandType::REG_DR3,
    },
    OperandTypeInfo {
        name: "dr4",
        value: OperandType::REG_DR4,
    },
    OperandTypeInfo {
        name: "dr5",
        value: OperandType::REG_DR5,
    },
    OperandTypeInfo {
        name: "dr6",
        value: OperandType::REG_DR6,
    },
    OperandTypeInfo {
        name: "dr7",
        value: OperandType::REG_DR7,
    },
    OperandTypeInfo {
        name: "dr8",
        value: OperandType::REG_DR8,
    },
    OperandTypeInfo {
        name: "dr9",
        value: OperandType::REG_DR9,
    },
    OperandTypeInfo {
        name: "dr10",
        value: OperandType::REG_DR10,
    },
    OperandTypeInfo {
        name: "dr11",
        value: OperandType::REG_DR11,
    },
    OperandTypeInfo {
        name: "dr12",
        value: OperandType::REG_DR12,
    },
    OperandTypeInfo {
        name: "dr13",
        value: OperandType::REG_DR13,
    },
    OperandTypeInfo {
        name: "dr14",
        value: OperandType::REG_DR14,
    },
    OperandTypeInfo {
        name: "dr15",
        value: OperandType::REG_DR15,
    },
    OperandTypeInfo {
        name: "tr0",
        value: OperandType::REG_TR0,
    },
    OperandTypeInfo {
        name: "tr1",
        value: OperandType::REG_TR1,
    },
    OperandTypeInfo {
        name: "tr2",
        value: OperandType::REG_TR2,
    },
    OperandTypeInfo {
        name: "tr3",
        value: OperandType::REG_TR3,
    },
    OperandTypeInfo {
        name: "tr4",
        value: OperandType::REG_TR4,
    },
    OperandTypeInfo {
        name: "tr5",
        value: OperandType::REG_TR5,
    },
    OperandTypeInfo {
        name: "tr6",
        value: OperandType::REG_TR6,
    },
    OperandTypeInfo {
        name: "tr7",
        value: OperandType::REG_TR7,
    },
    OperandTypeInfo {
        name: "tr8",
        value: OperandType::REG_TR8,
    },
    OperandTypeInfo {
        name: "tr9",
        value: OperandType::REG_TR9,
    },
    OperandTypeInfo {
        name: "tr10",
        value: OperandType::REG_TR10,
    },
    OperandTypeInfo {
        name: "tr11",
        value: OperandType::REG_TR11,
    },
    OperandTypeInfo {
        name: "tr12",
        value: OperandType::REG_TR12,
    },
    OperandTypeInfo {
        name: "tr13",
        value: OperandType::REG_TR13,
    },
    OperandTypeInfo {
        name: "tr14",
        value: OperandType::REG_TR14,
    },
    OperandTypeInfo {
        name: "tr15",
        value: OperandType::REG_TR15,
    },
    OperandTypeInfo {
        name: "es",
        value: OperandType::REG_ES,
    },
    OperandTypeInfo {
        name: "cs",
        value: OperandType::REG_CS,
    },
    OperandTypeInfo {
        name: "ss",
        value: OperandType::REG_SS,
    },
    OperandTypeInfo {
        name: "ds",
        value: OperandType::REG_DS,
    },
    OperandTypeInfo {
        name: "fs",
        value: OperandType::REG_FS,
    },
    OperandTypeInfo {
        name: "gs",
        value: OperandType::REG_GS,
    },
    OperandTypeInfo {
        name: "rip",
        value: OperandType::REG_RIP,
    },
];
