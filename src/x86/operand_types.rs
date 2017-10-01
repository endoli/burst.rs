// Licensed under the 2-Clause BSD license <LICENSE or
// https://opensource.org/licenses/BSD-2-Clause>. This
// file may not be copied, modified, or distributed
// except according to those terms.

#[allow(missing_docs, non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(i32)]
pub enum OperandType {
    NONE = 0i32,
    IMM,
    MEM,
    REG_AL,
    REG_CL,
    REG_DL,
    REG_BL,
    REG_AH,
    REG_CH,
    REG_DH,
    REG_BH,
    REG_SPL,
    REG_BPL,
    REG_SIL,
    REG_DIL,
    REG_R8B,
    REG_R9B,
    REG_R10B,
    REG_R11B,
    REG_R12B,
    REG_R13B,
    REG_R14B,
    REG_R15B,
    REG_AX,
    REG_CX,
    REG_DX,
    REG_BX,
    REG_SP,
    REG_BP,
    REG_SI,
    REG_DI,
    REG_R8W,
    REG_R9W,
    REG_R10W,
    REG_R11W,
    REG_R12W,
    REG_R13W,
    REG_R14W,
    REG_R15W,
    REG_EAX,
    REG_ECX,
    REG_EDX,
    REG_EBX,
    REG_ESP,
    REG_EBP,
    REG_ESI,
    REG_EDI,
    REG_R8D,
    REG_R9D,
    REG_R10D,
    REG_R11D,
    REG_R12D,
    REG_R13D,
    REG_R14D,
    REG_R15D,
    REG_RAX,
    REG_RCX,
    REG_RDX,
    REG_RBX,
    REG_RSP,
    REG_RBP,
    REG_RSI,
    REG_RDI,
    REG_R8,
    REG_R9,
    REG_R10,
    REG_R11,
    REG_R12,
    REG_R13,
    REG_R14,
    REG_R15,
    REG_ST0,
    REG_ST1,
    REG_ST2,
    REG_ST3,
    REG_ST4,
    REG_ST5,
    REG_ST6,
    REG_ST7,
    REG_MM0,
    REG_MM1,
    REG_MM2,
    REG_MM3,
    REG_MM4,
    REG_MM5,
    REG_MM6,
    REG_MM7,
    REG_XMM0,
    REG_XMM1,
    REG_XMM2,
    REG_XMM3,
    REG_XMM4,
    REG_XMM5,
    REG_XMM6,
    REG_XMM7,
    REG_XMM8,
    REG_XMM9,
    REG_XMM10,
    REG_XMM11,
    REG_XMM12,
    REG_XMM13,
    REG_XMM14,
    REG_XMM15,
    REG_CR0,
    REG_CR1,
    REG_CR2,
    REG_CR3,
    REG_CR4,
    REG_CR5,
    REG_CR6,
    REG_CR7,
    REG_CR8,
    REG_CR9,
    REG_CR10,
    REG_CR11,
    REG_CR12,
    REG_CR13,
    REG_CR14,
    REG_CR15,
    REG_DR0,
    REG_DR1,
    REG_DR2,
    REG_DR3,
    REG_DR4,
    REG_DR5,
    REG_DR6,
    REG_DR7,
    REG_DR8,
    REG_DR9,
    REG_DR10,
    REG_DR11,
    REG_DR12,
    REG_DR13,
    REG_DR14,
    REG_DR15,
    REG_TR0,
    REG_TR1,
    REG_TR2,
    REG_TR3,
    REG_TR4,
    REG_TR5,
    REG_TR6,
    REG_TR7,
    REG_TR8,
    REG_TR9,
    REG_TR10,
    REG_TR11,
    REG_TR12,
    REG_TR13,
    REG_TR14,
    REG_TR15,
    REG_ES,
    REG_CS,
    REG_SS,
    REG_DS,
    REG_FS,
    REG_GS,
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
