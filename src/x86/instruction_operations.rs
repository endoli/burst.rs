// Licensed under the 2-Clause BSD license <LICENSE or
// https://opensource.org/licenses/BSD-2-Clause>. This
// file may not be copied, modified, or distributed
// except according to those terms.

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(i32)]
pub enum InstructionOperation {
    INVALID = 0i32,
    AAA,
    AAD,
    AAM,
    AAS,
    ADD,
    ADC,
    AND,
    ARPL,
    BLENDPD,
    BLENDPS,
    BLENDVPD,
    BLENDVPS,
    BOUND,
    BSF,
    BSR,
    BSWAP,
    BT,
    BTC,
    BTR,
    BTS,
    CALLF,
    CALL,
    CLC,
    CLD,
    CLFLUSH,
    CLI,
    CLTS,
    CMC,
    CMP,
    CMPXCH16B,
    CMPXCH8B,
    CMPXCHG,
    CPUID,
    CRC32,
    DAA,
    DAS,
    DEC,
    DIV,
    DPPD,
    DPPS,
    EMMS,
    ENTER,
    F2XM1,
    FABS,
    FADD,
    FADDP,
    FBLD,
    FBSTP,
    FCHS,
    FCLEX,
    FCMOVB,
    FCMOVBE,
    FCMOVE,
    FCMOVNB,
    FCMOVNBE,
    FCMOVNE,
    FCMOVNU,
    FCMOVU,
    FCOM,
    FCOMI,
    FCOMIP,
    FCOMP,
    FCOMPP,
    FCOS,
    FDECSTP,
    FDISI,
    FDIV,
    FDIVP,
    FDIVR,
    FDIVRP,
    FEMMS,
    FENI,
    FFREE,
    FFREEP,
    FIADD,
    FICOM,
    FICOMP,
    FIDIV,
    FIDIVR,
    FILD,
    FIMUL,
    FINCSTP,
    FINIT,
    FIST,
    FISTP,
    FISTTP,
    FISUB,
    FISUBR,
    FLD,
    FLD1,
    FLDCW,
    FLDENV,
    FLDL2E,
    FLDL2T,
    FLDLG2,
    FLDLN2,
    FLDPI,
    FLDZ,
    FMUL,
    FMULP,
    FNOP,
    FPATAN,
    FPREM,
    FPREM1,
    FPTAN,
    FRICHOP,
    FRINEAR,
    FRINT2,
    FRNDINT,
    FRSTOR,
    FRSTPM,
    FSAVE,
    FSCALE,
    FSETPM,
    FSIN,
    FSINCOS,
    FSQRT,
    FST,
    FSTCW,
    FSTDW,
    FSTENV,
    FSTP,
    FSTSG,
    FSTSW,
    FSUB,
    FSUBP,
    FSUBR,
    FSUBRP,
    FTST,
    FUCOM,
    FUCOMI,
    FUCOMIP,
    FUCOMP,
    FUCOMPP,
    FWAIT,
    FXAM,
    FXCH,
    FXRSTOR,
    FXSAVE,
    FXTRACT,
    FYL2X,
    FYL2XP1,
    GETSEC,
    HLT,
    IDIV,
    IMUL,
    IN,
    INC,
    INT,
    INT1,
    INT3,
    INTO,
    INVD,
    INVLPG,
    IRET,
    JMPF,
    JMP,
    LAHF,
    LAR,
    LDMXCSR,
    LDS,
    LEA,
    LEAVE,
    LES,
    LFENCE,
    LFS,
    LGS,
    LOOP,
    LOOPE,
    LOOPNE,
    LSL,
    LSS,
    MFENCE,
    MOV,
    MOVNTI,
    MOVSS,
    MOVSX,
    MOVSXD,
    MOVUPD,
    MOVUPS,
    MOVZX,
    MPSADBW,
    MUL,
    NEG,
    NOP,
    NOT,
    OR,
    OUT,
    PACKSSDW,
    PACKSSWB,
    PACKUSDW,
    PACKUSWB,
    PABSB,
    PABSD,
    PABSW,
    PADDB,
    PADDD,
    PADDQ,
    PADDW,
    PADDSB,
    PADDSW,
    PADDUSB,
    PADDUSW,
    PALIGNR,
    PAND,
    PANDN,
    PAUSE,
    PAVGB,
    PAVGUSB,
    PAVGW,
    PBLENDVB,
    PBLENDW,
    PCMPEQB,
    PCMPEQD,
    PCMPEQQ,
    PCMPEQW,
    PCMPESTRI,
    PCMPESTRM,
    PCMPGTB,
    PCMPGTD,
    PCMPGTQ,
    PCMPGTW,
    PCMPISTRI,
    PCMPISTRM,
    PF2ID,
    PF2IW,
    PFACC,
    PFADD,
    PFCMPEQ,
    PFCMPGE,
    PFCMPGT,
    PFMAX,
    PFMIN,
    PFMUL,
    PFNACC,
    PFPNACC,
    PFRCP,
    PFRCPIT1,
    PFRCPIT2,
    PFRCPV,
    PFRSQIT1,
    PFRSQRT,
    PFRSQRTV,
    PFSUB,
    PFSUBR,
    PHADDD,
    PHADDSW,
    PHADDW,
    PHMINPOSUW,
    PHSUBD,
    PHSUBSW,
    PHSUBW,
    PI2FD,
    PI2FW,
    PMADDWD,
    PMADDUBSW,
    PMAXSB,
    PMAXSD,
    PMAXSW,
    PMAXUB,
    PMAXUD,
    PMAXUW,
    PMINSB,
    PMINSD,
    PMINSW,
    PMINUB,
    PMINUD,
    PMINUW,
    PMULDQ,
    PMULHRSW,
    PMULHRW,
    PMULHUW,
    PMULHW,
    PMULLD,
    PMULLW,
    PMULUDQ,
    POP,
    POPCNT,
    POR,
    PSADBW,
    PSHUFB,
    PSIGNB,
    PSIGND,
    PSIGNW,
    PSLLD,
    PSLLDQ,
    PSLLQ,
    PSLLW,
    PSRAD,
    PSRAW,
    PSRLD,
    PSRLDQ,
    PSRLQ,
    PSRLW,
    PSUBB,
    PSUBD,
    PSUBQ,
    PSUBW,
    PSUBSB,
    PSUBSW,
    PSUBUSB,
    PSUBUSW,
    PSWAPD,
    PTEST,
    PUNPCKHBW,
    PUNPCKHDQ,
    PUNPCKHQDQ,
    PUNPCKHWD,
    PUNPCKLQDQ,
    PUSH,
    PXOR,
    RDMSR,
    RDPMC,
    RDTSC,
    RETF,
    RETN,
    RCL,
    RCR,
    ROL,
    ROR,
    ROUNDPS,
    ROUNDPD,
    RSM,
    SAHF,
    SALC,
    SAR,
    SBB,
    SFENCE,
    SHL,
    SHLD,
    SHR,
    SHRD,
    SUB,
    STC,
    STD,
    STI,
    STMXCSR,
    SYSCALL,
    SYSENTER,
    SYSEXIT,
    SYSRET,
    TEST,
    UD2,
    VMREAD,
    VMWRITE,
    WBINVD,
    WRMSR,
    XCHG,
    XLAT,
    XADD,
    XOR,
    XRSTOR,
    XSAVE,
    ADDPS,
    ADDPD,
    ADDSD,
    ADDSS,
    ADDSUBPD,
    ADDSUBPS,
    ANDNPS,
    ANDNPD,
    ANDPS,
    ANDPD,
    CBW,
    CWDE,
    CDQE,
    CMPSB,
    CMPSW,
    CMPSD,
    CMPSQ,
    CMOVO,
    CMOVNO,
    CMOVB,
    CMOVAE,
    CMOVE,
    CMOVNE,
    CMOVBE,
    CMOVA,
    CMOVS,
    CMOVNS,
    CMOVPE,
    CMOVPO,
    CMOVL,
    CMOVGE,
    CMOVLE,
    CMOVG,
    CWD,
    CDQ,
    CQO,
    DIVPS,
    DIVPD,
    DIVSD,
    DIVSS,
    INSB,
    INSW,
    INSD,
    INSQ,
    JCXZ,
    JECXZ,
    JRCXZ,
    JO,
    JNO,
    JB,
    JAE,
    JE,
    JNE,
    JBE,
    JA,
    JS,
    JNS,
    JPE,
    JPO,
    JL,
    JGE,
    JLE,
    JG,
    LODSB,
    LODSW,
    LODSD,
    LODSQ,
    MAXPS,
    MAXPD,
    MAXSD,
    MAXSS,
    MINPS,
    MINPD,
    MINSD,
    MINSS,
    MOVD,
    MOVQ,
    MOVSB,
    MOVSW,
    MOVSD,
    MOVSQ,
    MULPS,
    MULPD,
    MULSD,
    MULSS,
    ORPS,
    ORPD,
    OUTSB,
    OUTSW,
    OUTSD,
    OUTSQ,
    PEXTRD,
    PEXTRQ,
    PINSRD,
    PINSRQ,
    POPA,
    POPAD,
    POPF,
    POPFD,
    POPFQ,
    PUSHA,
    PUSHAD,
    PUSHF,
    PUSHFD,
    PUSHFQ,
    RCPPS,
    RCPSS,
    RSQRTPS,
    RSQRTSS,
    SCASB,
    SCASW,
    SCASD,
    SCASQ,
    SETO,
    SETNO,
    SETB,
    SETAE,
    SETE,
    SETNE,
    SETBE,
    SETA,
    SETS,
    SETNS,
    SETPE,
    SETPO,
    SETL,
    SETGE,
    SETLE,
    SETG,
    SQRTPS,
    SQRTPD,
    SQRTSD,
    SQRTSS,
    STOSB,
    STOSW,
    STOSD,
    STOSQ,
    SUBPS,
    SUBPD,
    SUBSD,
    SUBSS,
    XORPS,
    XORPD,
    CMPPD,
    CMPPS,
    CMPSS,
    COMISD,
    COMISS,
    CVTDQ2PD,
    CVTDQ2PS,
    CVTPD2DQ,
    CVTPD2PI,
    CVTPD2PS,
    CVTPI2PD,
    CVTPI2PS,
    CVTPS2DQ,
    CVTPS2PD,
    CVTPS2PI,
    CVTSD2SI,
    CVTSD2SS,
    CVTSI2SD,
    CVTSI2SS,
    CVTSS2SD,
    CVTSS2SI,
    CVTTPD2DQ,
    CVTTPD2PI,
    CVTTPS2DQ,
    CVTTPS2PI,
    CVTTSD2SI,
    CVTTSS2SI,
    EXTRACTPS,
    HADDPD,
    HADDPS,
    HSUBPD,
    HSUBPS,
    INSERTPS,
    LDDQU,
    LGDT,
    LIDT,
    LLDT,
    LMSW,
    LTR,
    MASKMOVQ,
    MASKMOVDQU,
    MMXNOP,
    MONITOR,
    MOVAPD,
    MOVAPS,
    MOVDDUP,
    MOVDQ2Q,
    MOVDQA,
    MOVDQU,
    MOVHLPS,
    MOVHPD,
    MOVHPS,
    MOVSHDUP,
    MOVSLDUP,
    MOVLHPS,
    MOVLPD,
    MOVLPS,
    MOVMSKPD,
    MOVMSKPS,
    MOVNTDQ,
    MOVNTDQA,
    MOVNTPD,
    MOVNTPS,
    MOVNTQ,
    MOVQ2DQ,
    MWAIT,
    PINSRB,
    PINSRW,
    PEXTRB,
    PEXTRW,
    PMOVMSKB,
    PMOVSXBD,
    PMOVSXBQ,
    PMOVSXDQ,
    PMOVSXBW,
    PMOVSXWD,
    PMOVSXWQ,
    PMOVZXBD,
    PMOVZXBQ,
    PMOVZXDQ,
    PMOVZXBW,
    PMOVZXWD,
    PMOVZXWQ,
    PREFETCH,
    PREFETCHNTA,
    PREFETCHT0,
    PREFETCHT1,
    PREFETCHT2,
    PREFETCHW,
    PSHUFD,
    PSHUFHW,
    PSHUFLW,
    PSHUFW,
    PUNPCKLBW,
    PUNPCKLDQ,
    PUNPCKLWD,
    ROUNDSD,
    ROUNDSS,
    SGDT,
    SIDT,
    SLDT,
    SHUFPD,
    SHUFPS,
    SMSW,
    STR,
    SWAPGS,
    UCOMISD,
    UCOMISS,
    UNPCKHPD,
    UNPCKHPS,
    UNPCKLPD,
    UNPCKLPS,
    VERR,
    VERW,
    VMCALL,
    VMCLEAR,
    VMLAUNCH,
    VMPTRLD,
    VMPTRST,
    VMRESUME,
    VMXOFF,
    VMXON,
    XGETBV,
    XSETBV,
}

impl InstructionOperation {
    pub fn from_i32(i: i32) -> Self {
        INSTRUCTION_OPERATION_TABLE[i as usize].value
    }
}

impl Default for InstructionOperation {
    fn default() -> Self {
        InstructionOperation::INVALID
    }
}

pub(crate) struct InstructionOperationInfo {
    pub name: &'static str,
    pub value: InstructionOperation,
}

pub(crate) static INSTRUCTION_OPERATION_TABLE: [InstructionOperationInfo; 621] =
    [
        InstructionOperationInfo {
            name: "",
            value: InstructionOperation::INVALID,
        },
        InstructionOperationInfo {
            name: "aaa",
            value: InstructionOperation::AAA,
        },
        InstructionOperationInfo {
            name: "aad",
            value: InstructionOperation::AAD,
        },
        InstructionOperationInfo {
            name: "aam",
            value: InstructionOperation::AAM,
        },
        InstructionOperationInfo {
            name: "aas",
            value: InstructionOperation::AAS,
        },
        InstructionOperationInfo {
            name: "add",
            value: InstructionOperation::ADD,
        },
        InstructionOperationInfo {
            name: "adc",
            value: InstructionOperation::ADC,
        },
        InstructionOperationInfo {
            name: "and",
            value: InstructionOperation::AND,
        },
        InstructionOperationInfo {
            name: "arpl",
            value: InstructionOperation::ARPL,
        },
        InstructionOperationInfo {
            name: "blendpd",
            value: InstructionOperation::BLENDPD,
        },
        InstructionOperationInfo {
            name: "blendps",
            value: InstructionOperation::BLENDPS,
        },
        InstructionOperationInfo {
            name: "blendvpd",
            value: InstructionOperation::BLENDVPD,
        },
        InstructionOperationInfo {
            name: "blendvps",
            value: InstructionOperation::BLENDVPS,
        },
        InstructionOperationInfo {
            name: "bound",
            value: InstructionOperation::BOUND,
        },
        InstructionOperationInfo {
            name: "bsf",
            value: InstructionOperation::BSF,
        },
        InstructionOperationInfo {
            name: "bsr",
            value: InstructionOperation::BSR,
        },
        InstructionOperationInfo {
            name: "bswap",
            value: InstructionOperation::BSWAP,
        },
        InstructionOperationInfo {
            name: "bt",
            value: InstructionOperation::BT,
        },
        InstructionOperationInfo {
            name: "btc",
            value: InstructionOperation::BTC,
        },
        InstructionOperationInfo {
            name: "btr",
            value: InstructionOperation::BTR,
        },
        InstructionOperationInfo {
            name: "bts",
            value: InstructionOperation::BTS,
        },
        InstructionOperationInfo {
            name: "callf",
            value: InstructionOperation::CALLF,
        },
        InstructionOperationInfo {
            name: "call",
            value: InstructionOperation::CALL,
        },
        InstructionOperationInfo {
            name: "clc",
            value: InstructionOperation::CLC,
        },
        InstructionOperationInfo {
            name: "cld",
            value: InstructionOperation::CLD,
        },
        InstructionOperationInfo {
            name: "clflush",
            value: InstructionOperation::CLFLUSH,
        },
        InstructionOperationInfo {
            name: "cli",
            value: InstructionOperation::CLI,
        },
        InstructionOperationInfo {
            name: "clts",
            value: InstructionOperation::CLTS,
        },
        InstructionOperationInfo {
            name: "cmc",
            value: InstructionOperation::CMC,
        },
        InstructionOperationInfo {
            name: "cmp",
            value: InstructionOperation::CMP,
        },
        InstructionOperationInfo {
            name: "cmpxch16b",
            value: InstructionOperation::CMPXCH16B,
        },
        InstructionOperationInfo {
            name: "cmpxch8b",
            value: InstructionOperation::CMPXCH8B,
        },
        InstructionOperationInfo {
            name: "cmpxchg",
            value: InstructionOperation::CMPXCHG,
        },
        InstructionOperationInfo {
            name: "cpuid",
            value: InstructionOperation::CPUID,
        },
        InstructionOperationInfo {
            name: "crc32",
            value: InstructionOperation::CRC32,
        },
        InstructionOperationInfo {
            name: "daa",
            value: InstructionOperation::DAA,
        },
        InstructionOperationInfo {
            name: "das",
            value: InstructionOperation::DAS,
        },
        InstructionOperationInfo {
            name: "dec",
            value: InstructionOperation::DEC,
        },
        InstructionOperationInfo {
            name: "div",
            value: InstructionOperation::DIV,
        },
        InstructionOperationInfo {
            name: "dppd",
            value: InstructionOperation::DPPD,
        },
        InstructionOperationInfo {
            name: "dpps",
            value: InstructionOperation::DPPS,
        },
        InstructionOperationInfo {
            name: "emms",
            value: InstructionOperation::EMMS,
        },
        InstructionOperationInfo {
            name: "enter",
            value: InstructionOperation::ENTER,
        },
        InstructionOperationInfo {
            name: "f2xm1",
            value: InstructionOperation::F2XM1,
        },
        InstructionOperationInfo {
            name: "fabs",
            value: InstructionOperation::FABS,
        },
        InstructionOperationInfo {
            name: "fadd",
            value: InstructionOperation::FADD,
        },
        InstructionOperationInfo {
            name: "faddp",
            value: InstructionOperation::FADDP,
        },
        InstructionOperationInfo {
            name: "fbld",
            value: InstructionOperation::FBLD,
        },
        InstructionOperationInfo {
            name: "fbstp",
            value: InstructionOperation::FBSTP,
        },
        InstructionOperationInfo {
            name: "fchs",
            value: InstructionOperation::FCHS,
        },
        InstructionOperationInfo {
            name: "fclex",
            value: InstructionOperation::FCLEX,
        },
        InstructionOperationInfo {
            name: "fcmovb",
            value: InstructionOperation::FCMOVB,
        },
        InstructionOperationInfo {
            name: "fcmovbe",
            value: InstructionOperation::FCMOVBE,
        },
        InstructionOperationInfo {
            name: "fcmove",
            value: InstructionOperation::FCMOVE,
        },
        InstructionOperationInfo {
            name: "fcmovnb",
            value: InstructionOperation::FCMOVNB,
        },
        InstructionOperationInfo {
            name: "fcmovnbe",
            value: InstructionOperation::FCMOVNBE,
        },
        InstructionOperationInfo {
            name: "fcmovne",
            value: InstructionOperation::FCMOVNE,
        },
        InstructionOperationInfo {
            name: "fcmovnu",
            value: InstructionOperation::FCMOVNU,
        },
        InstructionOperationInfo {
            name: "fcmovu",
            value: InstructionOperation::FCMOVU,
        },
        InstructionOperationInfo {
            name: "fcom",
            value: InstructionOperation::FCOM,
        },
        InstructionOperationInfo {
            name: "fcomi",
            value: InstructionOperation::FCOMI,
        },
        InstructionOperationInfo {
            name: "fcomip",
            value: InstructionOperation::FCOMIP,
        },
        InstructionOperationInfo {
            name: "fcomp",
            value: InstructionOperation::FCOMP,
        },
        InstructionOperationInfo {
            name: "fcompp",
            value: InstructionOperation::FCOMPP,
        },
        InstructionOperationInfo {
            name: "fcos",
            value: InstructionOperation::FCOS,
        },
        InstructionOperationInfo {
            name: "fdecstp",
            value: InstructionOperation::FDECSTP,
        },
        InstructionOperationInfo {
            name: "fdisi",
            value: InstructionOperation::FDISI,
        },
        InstructionOperationInfo {
            name: "fdiv",
            value: InstructionOperation::FDIV,
        },
        InstructionOperationInfo {
            name: "fdivp",
            value: InstructionOperation::FDIVP,
        },
        InstructionOperationInfo {
            name: "fdivr",
            value: InstructionOperation::FDIVR,
        },
        InstructionOperationInfo {
            name: "fdivrp",
            value: InstructionOperation::FDIVRP,
        },
        InstructionOperationInfo {
            name: "femms",
            value: InstructionOperation::FEMMS,
        },
        InstructionOperationInfo {
            name: "feni",
            value: InstructionOperation::FENI,
        },
        InstructionOperationInfo {
            name: "ffree",
            value: InstructionOperation::FFREE,
        },
        InstructionOperationInfo {
            name: "ffreep",
            value: InstructionOperation::FFREEP,
        },
        InstructionOperationInfo {
            name: "fiadd",
            value: InstructionOperation::FIADD,
        },
        InstructionOperationInfo {
            name: "ficom",
            value: InstructionOperation::FICOM,
        },
        InstructionOperationInfo {
            name: "ficomp",
            value: InstructionOperation::FICOMP,
        },
        InstructionOperationInfo {
            name: "fidiv",
            value: InstructionOperation::FIDIV,
        },
        InstructionOperationInfo {
            name: "fidivr",
            value: InstructionOperation::FIDIVR,
        },
        InstructionOperationInfo {
            name: "fild",
            value: InstructionOperation::FILD,
        },
        InstructionOperationInfo {
            name: "fimul",
            value: InstructionOperation::FIMUL,
        },
        InstructionOperationInfo {
            name: "fincstp",
            value: InstructionOperation::FINCSTP,
        },
        InstructionOperationInfo {
            name: "finit",
            value: InstructionOperation::FINIT,
        },
        InstructionOperationInfo {
            name: "fist",
            value: InstructionOperation::FIST,
        },
        InstructionOperationInfo {
            name: "fistp",
            value: InstructionOperation::FISTP,
        },
        InstructionOperationInfo {
            name: "fisttp",
            value: InstructionOperation::FISTTP,
        },
        InstructionOperationInfo {
            name: "fisub",
            value: InstructionOperation::FISUB,
        },
        InstructionOperationInfo {
            name: "fisubr",
            value: InstructionOperation::FISUBR,
        },
        InstructionOperationInfo {
            name: "fld",
            value: InstructionOperation::FLD,
        },
        InstructionOperationInfo {
            name: "fld1",
            value: InstructionOperation::FLD1,
        },
        InstructionOperationInfo {
            name: "fldcw",
            value: InstructionOperation::FLDCW,
        },
        InstructionOperationInfo {
            name: "fldenv",
            value: InstructionOperation::FLDENV,
        },
        InstructionOperationInfo {
            name: "fldl2e",
            value: InstructionOperation::FLDL2E,
        },
        InstructionOperationInfo {
            name: "fldl2t",
            value: InstructionOperation::FLDL2T,
        },
        InstructionOperationInfo {
            name: "fldlg2",
            value: InstructionOperation::FLDLG2,
        },
        InstructionOperationInfo {
            name: "fldln2",
            value: InstructionOperation::FLDLN2,
        },
        InstructionOperationInfo {
            name: "fldpi",
            value: InstructionOperation::FLDPI,
        },
        InstructionOperationInfo {
            name: "fldz",
            value: InstructionOperation::FLDZ,
        },
        InstructionOperationInfo {
            name: "fmul",
            value: InstructionOperation::FMUL,
        },
        InstructionOperationInfo {
            name: "fmulp",
            value: InstructionOperation::FMULP,
        },
        InstructionOperationInfo {
            name: "fnop",
            value: InstructionOperation::FNOP,
        },
        InstructionOperationInfo {
            name: "fpatan",
            value: InstructionOperation::FPATAN,
        },
        InstructionOperationInfo {
            name: "fprem",
            value: InstructionOperation::FPREM,
        },
        InstructionOperationInfo {
            name: "fprem1",
            value: InstructionOperation::FPREM1,
        },
        InstructionOperationInfo {
            name: "fptan",
            value: InstructionOperation::FPTAN,
        },
        InstructionOperationInfo {
            name: "frichop",
            value: InstructionOperation::FRICHOP,
        },
        InstructionOperationInfo {
            name: "frinear",
            value: InstructionOperation::FRINEAR,
        },
        InstructionOperationInfo {
            name: "frint2",
            value: InstructionOperation::FRINT2,
        },
        InstructionOperationInfo {
            name: "frndint",
            value: InstructionOperation::FRNDINT,
        },
        InstructionOperationInfo {
            name: "frstor",
            value: InstructionOperation::FRSTOR,
        },
        InstructionOperationInfo {
            name: "frstpm",
            value: InstructionOperation::FRSTPM,
        },
        InstructionOperationInfo {
            name: "fsave",
            value: InstructionOperation::FSAVE,
        },
        InstructionOperationInfo {
            name: "fscale",
            value: InstructionOperation::FSCALE,
        },
        InstructionOperationInfo {
            name: "fsetpm",
            value: InstructionOperation::FSETPM,
        },
        InstructionOperationInfo {
            name: "fsin",
            value: InstructionOperation::FSIN,
        },
        InstructionOperationInfo {
            name: "fsincos",
            value: InstructionOperation::FSINCOS,
        },
        InstructionOperationInfo {
            name: "fsqrt",
            value: InstructionOperation::FSQRT,
        },
        InstructionOperationInfo {
            name: "fst",
            value: InstructionOperation::FST,
        },
        InstructionOperationInfo {
            name: "fstcw",
            value: InstructionOperation::FSTCW,
        },
        InstructionOperationInfo {
            name: "fstdw",
            value: InstructionOperation::FSTDW,
        },
        InstructionOperationInfo {
            name: "fstenv",
            value: InstructionOperation::FSTENV,
        },
        InstructionOperationInfo {
            name: "fstp",
            value: InstructionOperation::FSTP,
        },
        InstructionOperationInfo {
            name: "fstsg",
            value: InstructionOperation::FSTSG,
        },
        InstructionOperationInfo {
            name: "fstsw",
            value: InstructionOperation::FSTSW,
        },
        InstructionOperationInfo {
            name: "fsub",
            value: InstructionOperation::FSUB,
        },
        InstructionOperationInfo {
            name: "fsubp",
            value: InstructionOperation::FSUBP,
        },
        InstructionOperationInfo {
            name: "fsubr",
            value: InstructionOperation::FSUBR,
        },
        InstructionOperationInfo {
            name: "fsubrp",
            value: InstructionOperation::FSUBRP,
        },
        InstructionOperationInfo {
            name: "ftst",
            value: InstructionOperation::FTST,
        },
        InstructionOperationInfo {
            name: "fucom",
            value: InstructionOperation::FUCOM,
        },
        InstructionOperationInfo {
            name: "fucomi",
            value: InstructionOperation::FUCOMI,
        },
        InstructionOperationInfo {
            name: "fucomip",
            value: InstructionOperation::FUCOMIP,
        },
        InstructionOperationInfo {
            name: "fucomp",
            value: InstructionOperation::FUCOMP,
        },
        InstructionOperationInfo {
            name: "fucompp",
            value: InstructionOperation::FUCOMPP,
        },
        InstructionOperationInfo {
            name: "fwait",
            value: InstructionOperation::FWAIT,
        },
        InstructionOperationInfo {
            name: "fxam",
            value: InstructionOperation::FXAM,
        },
        InstructionOperationInfo {
            name: "fxch",
            value: InstructionOperation::FXCH,
        },
        InstructionOperationInfo {
            name: "fxrstor",
            value: InstructionOperation::FXRSTOR,
        },
        InstructionOperationInfo {
            name: "fxsave",
            value: InstructionOperation::FXSAVE,
        },
        InstructionOperationInfo {
            name: "fxtract",
            value: InstructionOperation::FXTRACT,
        },
        InstructionOperationInfo {
            name: "fyl2x",
            value: InstructionOperation::FYL2X,
        },
        InstructionOperationInfo {
            name: "fyl2xp1",
            value: InstructionOperation::FYL2XP1,
        },
        InstructionOperationInfo {
            name: "getsec",
            value: InstructionOperation::GETSEC,
        },
        InstructionOperationInfo {
            name: "hlt",
            value: InstructionOperation::HLT,
        },
        InstructionOperationInfo {
            name: "idiv",
            value: InstructionOperation::IDIV,
        },
        InstructionOperationInfo {
            name: "imul",
            value: InstructionOperation::IMUL,
        },
        InstructionOperationInfo {
            name: "in",
            value: InstructionOperation::IN,
        },
        InstructionOperationInfo {
            name: "inc",
            value: InstructionOperation::INC,
        },
        InstructionOperationInfo {
            name: "int",
            value: InstructionOperation::INT,
        },
        InstructionOperationInfo {
            name: "int1",
            value: InstructionOperation::INT1,
        },
        InstructionOperationInfo {
            name: "int3",
            value: InstructionOperation::INT3,
        },
        InstructionOperationInfo {
            name: "into",
            value: InstructionOperation::INTO,
        },
        InstructionOperationInfo {
            name: "invd",
            value: InstructionOperation::INVD,
        },
        InstructionOperationInfo {
            name: "invlpg",
            value: InstructionOperation::INVLPG,
        },
        InstructionOperationInfo {
            name: "iret",
            value: InstructionOperation::IRET,
        },
        InstructionOperationInfo {
            name: "jmpf",
            value: InstructionOperation::JMPF,
        },
        InstructionOperationInfo {
            name: "jmp",
            value: InstructionOperation::JMP,
        },
        InstructionOperationInfo {
            name: "lahf",
            value: InstructionOperation::LAHF,
        },
        InstructionOperationInfo {
            name: "lar",
            value: InstructionOperation::LAR,
        },
        InstructionOperationInfo {
            name: "ldmxcsr",
            value: InstructionOperation::LDMXCSR,
        },
        InstructionOperationInfo {
            name: "lds",
            value: InstructionOperation::LDS,
        },
        InstructionOperationInfo {
            name: "lea",
            value: InstructionOperation::LEA,
        },
        InstructionOperationInfo {
            name: "leave",
            value: InstructionOperation::LEAVE,
        },
        InstructionOperationInfo {
            name: "les",
            value: InstructionOperation::LES,
        },
        InstructionOperationInfo {
            name: "lfence",
            value: InstructionOperation::LFENCE,
        },
        InstructionOperationInfo {
            name: "lfs",
            value: InstructionOperation::LFS,
        },
        InstructionOperationInfo {
            name: "lgs",
            value: InstructionOperation::LGS,
        },
        InstructionOperationInfo {
            name: "loop",
            value: InstructionOperation::LOOP,
        },
        InstructionOperationInfo {
            name: "loope",
            value: InstructionOperation::LOOPE,
        },
        InstructionOperationInfo {
            name: "loopne",
            value: InstructionOperation::LOOPNE,
        },
        InstructionOperationInfo {
            name: "lsl",
            value: InstructionOperation::LSL,
        },
        InstructionOperationInfo {
            name: "lss",
            value: InstructionOperation::LSS,
        },
        InstructionOperationInfo {
            name: "mfence",
            value: InstructionOperation::MFENCE,
        },
        InstructionOperationInfo {
            name: "mov",
            value: InstructionOperation::MOV,
        },
        InstructionOperationInfo {
            name: "movnti",
            value: InstructionOperation::MOVNTI,
        },
        InstructionOperationInfo {
            name: "movss",
            value: InstructionOperation::MOVSS,
        },
        InstructionOperationInfo {
            name: "movsx",
            value: InstructionOperation::MOVSX,
        },
        InstructionOperationInfo {
            name: "movsxd",
            value: InstructionOperation::MOVSXD,
        },
        InstructionOperationInfo {
            name: "movupd",
            value: InstructionOperation::MOVUPD,
        },
        InstructionOperationInfo {
            name: "movups",
            value: InstructionOperation::MOVUPS,
        },
        InstructionOperationInfo {
            name: "movzx",
            value: InstructionOperation::MOVZX,
        },
        InstructionOperationInfo {
            name: "mpsadbw",
            value: InstructionOperation::MPSADBW,
        },
        InstructionOperationInfo {
            name: "mul",
            value: InstructionOperation::MUL,
        },
        InstructionOperationInfo {
            name: "neg",
            value: InstructionOperation::NEG,
        },
        InstructionOperationInfo {
            name: "nop",
            value: InstructionOperation::NOP,
        },
        InstructionOperationInfo {
            name: "not",
            value: InstructionOperation::NOT,
        },
        InstructionOperationInfo {
            name: "or",
            value: InstructionOperation::OR,
        },
        InstructionOperationInfo {
            name: "out",
            value: InstructionOperation::OUT,
        },
        InstructionOperationInfo {
            name: "packssdw",
            value: InstructionOperation::PACKSSDW,
        },
        InstructionOperationInfo {
            name: "packsswb",
            value: InstructionOperation::PACKSSWB,
        },
        InstructionOperationInfo {
            name: "packusdw",
            value: InstructionOperation::PACKUSDW,
        },
        InstructionOperationInfo {
            name: "packuswb",
            value: InstructionOperation::PACKUSWB,
        },
        InstructionOperationInfo {
            name: "pabsb",
            value: InstructionOperation::PABSB,
        },
        InstructionOperationInfo {
            name: "pabsd",
            value: InstructionOperation::PABSD,
        },
        InstructionOperationInfo {
            name: "pabsw",
            value: InstructionOperation::PABSW,
        },
        InstructionOperationInfo {
            name: "paddb",
            value: InstructionOperation::PADDB,
        },
        InstructionOperationInfo {
            name: "paddd",
            value: InstructionOperation::PADDD,
        },
        InstructionOperationInfo {
            name: "paddq",
            value: InstructionOperation::PADDQ,
        },
        InstructionOperationInfo {
            name: "paddw",
            value: InstructionOperation::PADDW,
        },
        InstructionOperationInfo {
            name: "paddsb",
            value: InstructionOperation::PADDSB,
        },
        InstructionOperationInfo {
            name: "paddsw",
            value: InstructionOperation::PADDSW,
        },
        InstructionOperationInfo {
            name: "paddusb",
            value: InstructionOperation::PADDUSB,
        },
        InstructionOperationInfo {
            name: "paddusw",
            value: InstructionOperation::PADDUSW,
        },
        InstructionOperationInfo {
            name: "palignr",
            value: InstructionOperation::PALIGNR,
        },
        InstructionOperationInfo {
            name: "pand",
            value: InstructionOperation::PAND,
        },
        InstructionOperationInfo {
            name: "pandn",
            value: InstructionOperation::PANDN,
        },
        InstructionOperationInfo {
            name: "pause",
            value: InstructionOperation::PAUSE,
        },
        InstructionOperationInfo {
            name: "pavgb",
            value: InstructionOperation::PAVGB,
        },
        InstructionOperationInfo {
            name: "pavgusb",
            value: InstructionOperation::PAVGUSB,
        },
        InstructionOperationInfo {
            name: "pavgw",
            value: InstructionOperation::PAVGW,
        },
        InstructionOperationInfo {
            name: "pblendvb",
            value: InstructionOperation::PBLENDVB,
        },
        InstructionOperationInfo {
            name: "pblendw",
            value: InstructionOperation::PBLENDW,
        },
        InstructionOperationInfo {
            name: "pcmpeqb",
            value: InstructionOperation::PCMPEQB,
        },
        InstructionOperationInfo {
            name: "pcmpeqd",
            value: InstructionOperation::PCMPEQD,
        },
        InstructionOperationInfo {
            name: "pcmpeqq",
            value: InstructionOperation::PCMPEQQ,
        },
        InstructionOperationInfo {
            name: "pcmpeqw",
            value: InstructionOperation::PCMPEQW,
        },
        InstructionOperationInfo {
            name: "pcmpestri",
            value: InstructionOperation::PCMPESTRI,
        },
        InstructionOperationInfo {
            name: "pcmpestrm",
            value: InstructionOperation::PCMPESTRM,
        },
        InstructionOperationInfo {
            name: "pcmpgtb",
            value: InstructionOperation::PCMPGTB,
        },
        InstructionOperationInfo {
            name: "pcmpgtd",
            value: InstructionOperation::PCMPGTD,
        },
        InstructionOperationInfo {
            name: "pcmpgtq",
            value: InstructionOperation::PCMPGTQ,
        },
        InstructionOperationInfo {
            name: "pcmpgtw",
            value: InstructionOperation::PCMPGTW,
        },
        InstructionOperationInfo {
            name: "pcmpistri",
            value: InstructionOperation::PCMPISTRI,
        },
        InstructionOperationInfo {
            name: "pcmpistrm",
            value: InstructionOperation::PCMPISTRM,
        },
        InstructionOperationInfo {
            name: "pf2id",
            value: InstructionOperation::PF2ID,
        },
        InstructionOperationInfo {
            name: "pf2iw",
            value: InstructionOperation::PF2IW,
        },
        InstructionOperationInfo {
            name: "pfacc",
            value: InstructionOperation::PFACC,
        },
        InstructionOperationInfo {
            name: "pfadd",
            value: InstructionOperation::PFADD,
        },
        InstructionOperationInfo {
            name: "pfcmpeq",
            value: InstructionOperation::PFCMPEQ,
        },
        InstructionOperationInfo {
            name: "pfcmpge",
            value: InstructionOperation::PFCMPGE,
        },
        InstructionOperationInfo {
            name: "pfcmpgt",
            value: InstructionOperation::PFCMPGT,
        },
        InstructionOperationInfo {
            name: "pfmax",
            value: InstructionOperation::PFMAX,
        },
        InstructionOperationInfo {
            name: "pfmin",
            value: InstructionOperation::PFMIN,
        },
        InstructionOperationInfo {
            name: "pfmul",
            value: InstructionOperation::PFMUL,
        },
        InstructionOperationInfo {
            name: "pfnacc",
            value: InstructionOperation::PFNACC,
        },
        InstructionOperationInfo {
            name: "pfpnacc",
            value: InstructionOperation::PFPNACC,
        },
        InstructionOperationInfo {
            name: "pfrcp",
            value: InstructionOperation::PFRCP,
        },
        InstructionOperationInfo {
            name: "pfrcpit1",
            value: InstructionOperation::PFRCPIT1,
        },
        InstructionOperationInfo {
            name: "pfrcpit2",
            value: InstructionOperation::PFRCPIT2,
        },
        InstructionOperationInfo {
            name: "pfrcpv",
            value: InstructionOperation::PFRCPV,
        },
        InstructionOperationInfo {
            name: "pfrsqit1",
            value: InstructionOperation::PFRSQIT1,
        },
        InstructionOperationInfo {
            name: "pfrsqrt",
            value: InstructionOperation::PFRSQRT,
        },
        InstructionOperationInfo {
            name: "pfrsqrtv",
            value: InstructionOperation::PFRSQRTV,
        },
        InstructionOperationInfo {
            name: "pfsub",
            value: InstructionOperation::PFSUB,
        },
        InstructionOperationInfo {
            name: "pfsubr",
            value: InstructionOperation::PFSUBR,
        },
        InstructionOperationInfo {
            name: "phaddd",
            value: InstructionOperation::PHADDD,
        },
        InstructionOperationInfo {
            name: "phaddsw",
            value: InstructionOperation::PHADDSW,
        },
        InstructionOperationInfo {
            name: "phaddw",
            value: InstructionOperation::PHADDW,
        },
        InstructionOperationInfo {
            name: "phminposuw",
            value: InstructionOperation::PHMINPOSUW,
        },
        InstructionOperationInfo {
            name: "phsubd",
            value: InstructionOperation::PHSUBD,
        },
        InstructionOperationInfo {
            name: "phsubsw",
            value: InstructionOperation::PHSUBSW,
        },
        InstructionOperationInfo {
            name: "phsubw",
            value: InstructionOperation::PHSUBW,
        },
        InstructionOperationInfo {
            name: "pi2fd",
            value: InstructionOperation::PI2FD,
        },
        InstructionOperationInfo {
            name: "pi2fw",
            value: InstructionOperation::PI2FW,
        },
        InstructionOperationInfo {
            name: "pmaddwd",
            value: InstructionOperation::PMADDWD,
        },
        InstructionOperationInfo {
            name: "pmaddubsw",
            value: InstructionOperation::PMADDUBSW,
        },
        InstructionOperationInfo {
            name: "pmaxsb",
            value: InstructionOperation::PMAXSB,
        },
        InstructionOperationInfo {
            name: "pmaxsd",
            value: InstructionOperation::PMAXSD,
        },
        InstructionOperationInfo {
            name: "pmaxsw",
            value: InstructionOperation::PMAXSW,
        },
        InstructionOperationInfo {
            name: "pmaxub",
            value: InstructionOperation::PMAXUB,
        },
        InstructionOperationInfo {
            name: "pmaxud",
            value: InstructionOperation::PMAXUD,
        },
        InstructionOperationInfo {
            name: "pmaxuw",
            value: InstructionOperation::PMAXUW,
        },
        InstructionOperationInfo {
            name: "pminsb",
            value: InstructionOperation::PMINSB,
        },
        InstructionOperationInfo {
            name: "pminsd",
            value: InstructionOperation::PMINSD,
        },
        InstructionOperationInfo {
            name: "pminsw",
            value: InstructionOperation::PMINSW,
        },
        InstructionOperationInfo {
            name: "pminub",
            value: InstructionOperation::PMINUB,
        },
        InstructionOperationInfo {
            name: "pminud",
            value: InstructionOperation::PMINUD,
        },
        InstructionOperationInfo {
            name: "pminuw",
            value: InstructionOperation::PMINUW,
        },
        InstructionOperationInfo {
            name: "pmuldq",
            value: InstructionOperation::PMULDQ,
        },
        InstructionOperationInfo {
            name: "pmulhrsw",
            value: InstructionOperation::PMULHRSW,
        },
        InstructionOperationInfo {
            name: "pmulhrw",
            value: InstructionOperation::PMULHRW,
        },
        InstructionOperationInfo {
            name: "pmulhuw",
            value: InstructionOperation::PMULHUW,
        },
        InstructionOperationInfo {
            name: "pmulhw",
            value: InstructionOperation::PMULHW,
        },
        InstructionOperationInfo {
            name: "pmulld",
            value: InstructionOperation::PMULLD,
        },
        InstructionOperationInfo {
            name: "pmullw",
            value: InstructionOperation::PMULLW,
        },
        InstructionOperationInfo {
            name: "pmuludq",
            value: InstructionOperation::PMULUDQ,
        },
        InstructionOperationInfo {
            name: "pop",
            value: InstructionOperation::POP,
        },
        InstructionOperationInfo {
            name: "popcnt",
            value: InstructionOperation::POPCNT,
        },
        InstructionOperationInfo {
            name: "por",
            value: InstructionOperation::POR,
        },
        InstructionOperationInfo {
            name: "psadbw",
            value: InstructionOperation::PSADBW,
        },
        InstructionOperationInfo {
            name: "pshufb",
            value: InstructionOperation::PSHUFB,
        },
        InstructionOperationInfo {
            name: "psignb",
            value: InstructionOperation::PSIGNB,
        },
        InstructionOperationInfo {
            name: "psignd",
            value: InstructionOperation::PSIGND,
        },
        InstructionOperationInfo {
            name: "psignw",
            value: InstructionOperation::PSIGNW,
        },
        InstructionOperationInfo {
            name: "pslld",
            value: InstructionOperation::PSLLD,
        },
        InstructionOperationInfo {
            name: "pslldq",
            value: InstructionOperation::PSLLDQ,
        },
        InstructionOperationInfo {
            name: "psllq",
            value: InstructionOperation::PSLLQ,
        },
        InstructionOperationInfo {
            name: "psllw",
            value: InstructionOperation::PSLLW,
        },
        InstructionOperationInfo {
            name: "psrad",
            value: InstructionOperation::PSRAD,
        },
        InstructionOperationInfo {
            name: "psraw",
            value: InstructionOperation::PSRAW,
        },
        InstructionOperationInfo {
            name: "psrld",
            value: InstructionOperation::PSRLD,
        },
        InstructionOperationInfo {
            name: "psrldq",
            value: InstructionOperation::PSRLDQ,
        },
        InstructionOperationInfo {
            name: "psrlq",
            value: InstructionOperation::PSRLQ,
        },
        InstructionOperationInfo {
            name: "psrlw",
            value: InstructionOperation::PSRLW,
        },
        InstructionOperationInfo {
            name: "psubb",
            value: InstructionOperation::PSUBB,
        },
        InstructionOperationInfo {
            name: "psubd",
            value: InstructionOperation::PSUBD,
        },
        InstructionOperationInfo {
            name: "psubq",
            value: InstructionOperation::PSUBQ,
        },
        InstructionOperationInfo {
            name: "psubw",
            value: InstructionOperation::PSUBW,
        },
        InstructionOperationInfo {
            name: "psubsb",
            value: InstructionOperation::PSUBSB,
        },
        InstructionOperationInfo {
            name: "psubsw",
            value: InstructionOperation::PSUBSW,
        },
        InstructionOperationInfo {
            name: "psubusb",
            value: InstructionOperation::PSUBUSB,
        },
        InstructionOperationInfo {
            name: "psubusw",
            value: InstructionOperation::PSUBUSW,
        },
        InstructionOperationInfo {
            name: "pswapd",
            value: InstructionOperation::PSWAPD,
        },
        InstructionOperationInfo {
            name: "ptest",
            value: InstructionOperation::PTEST,
        },
        InstructionOperationInfo {
            name: "punpckhbw",
            value: InstructionOperation::PUNPCKHBW,
        },
        InstructionOperationInfo {
            name: "punpckhdq",
            value: InstructionOperation::PUNPCKHDQ,
        },
        InstructionOperationInfo {
            name: "punpckhqdq",
            value: InstructionOperation::PUNPCKHQDQ,
        },
        InstructionOperationInfo {
            name: "punpckhwd",
            value: InstructionOperation::PUNPCKHWD,
        },
        InstructionOperationInfo {
            name: "punpcklqdq",
            value: InstructionOperation::PUNPCKLQDQ,
        },
        InstructionOperationInfo {
            name: "push",
            value: InstructionOperation::PUSH,
        },
        InstructionOperationInfo {
            name: "pxor",
            value: InstructionOperation::PXOR,
        },
        InstructionOperationInfo {
            name: "rdmsr",
            value: InstructionOperation::RDMSR,
        },
        InstructionOperationInfo {
            name: "rdpmc",
            value: InstructionOperation::RDPMC,
        },
        InstructionOperationInfo {
            name: "rdtsc",
            value: InstructionOperation::RDTSC,
        },
        InstructionOperationInfo {
            name: "retf",
            value: InstructionOperation::RETF,
        },
        InstructionOperationInfo {
            name: "retn",
            value: InstructionOperation::RETN,
        },
        InstructionOperationInfo {
            name: "rcl",
            value: InstructionOperation::RCL,
        },
        InstructionOperationInfo {
            name: "rcr",
            value: InstructionOperation::RCR,
        },
        InstructionOperationInfo {
            name: "rol",
            value: InstructionOperation::ROL,
        },
        InstructionOperationInfo {
            name: "ror",
            value: InstructionOperation::ROR,
        },
        InstructionOperationInfo {
            name: "roundps",
            value: InstructionOperation::ROUNDPS,
        },
        InstructionOperationInfo {
            name: "roundpd",
            value: InstructionOperation::ROUNDPD,
        },
        InstructionOperationInfo {
            name: "rsm",
            value: InstructionOperation::RSM,
        },
        InstructionOperationInfo {
            name: "sahf",
            value: InstructionOperation::SAHF,
        },
        InstructionOperationInfo {
            name: "salc",
            value: InstructionOperation::SALC,
        },
        InstructionOperationInfo {
            name: "sar",
            value: InstructionOperation::SAR,
        },
        InstructionOperationInfo {
            name: "sbb",
            value: InstructionOperation::SBB,
        },
        InstructionOperationInfo {
            name: "sfence",
            value: InstructionOperation::SFENCE,
        },
        InstructionOperationInfo {
            name: "shl",
            value: InstructionOperation::SHL,
        },
        InstructionOperationInfo {
            name: "shld",
            value: InstructionOperation::SHLD,
        },
        InstructionOperationInfo {
            name: "shr",
            value: InstructionOperation::SHR,
        },
        InstructionOperationInfo {
            name: "shrd",
            value: InstructionOperation::SHRD,
        },
        InstructionOperationInfo {
            name: "sub",
            value: InstructionOperation::SUB,
        },
        InstructionOperationInfo {
            name: "stc",
            value: InstructionOperation::STC,
        },
        InstructionOperationInfo {
            name: "std",
            value: InstructionOperation::STD,
        },
        InstructionOperationInfo {
            name: "sti",
            value: InstructionOperation::STI,
        },
        InstructionOperationInfo {
            name: "stmxcsr",
            value: InstructionOperation::STMXCSR,
        },
        InstructionOperationInfo {
            name: "syscall",
            value: InstructionOperation::SYSCALL,
        },
        InstructionOperationInfo {
            name: "sysenter",
            value: InstructionOperation::SYSENTER,
        },
        InstructionOperationInfo {
            name: "sysexit",
            value: InstructionOperation::SYSEXIT,
        },
        InstructionOperationInfo {
            name: "sysret",
            value: InstructionOperation::SYSRET,
        },
        InstructionOperationInfo {
            name: "test",
            value: InstructionOperation::TEST,
        },
        InstructionOperationInfo {
            name: "ud2",
            value: InstructionOperation::UD2,
        },
        InstructionOperationInfo {
            name: "vmread",
            value: InstructionOperation::VMREAD,
        },
        InstructionOperationInfo {
            name: "vmwrite",
            value: InstructionOperation::VMWRITE,
        },
        InstructionOperationInfo {
            name: "wbinvd",
            value: InstructionOperation::WBINVD,
        },
        InstructionOperationInfo {
            name: "wrmsr",
            value: InstructionOperation::WRMSR,
        },
        InstructionOperationInfo {
            name: "xchg",
            value: InstructionOperation::XCHG,
        },
        InstructionOperationInfo {
            name: "xlat",
            value: InstructionOperation::XLAT,
        },
        InstructionOperationInfo {
            name: "xadd",
            value: InstructionOperation::XADD,
        },
        InstructionOperationInfo {
            name: "xor",
            value: InstructionOperation::XOR,
        },
        InstructionOperationInfo {
            name: "xrstor",
            value: InstructionOperation::XRSTOR,
        },
        InstructionOperationInfo {
            name: "xsave",
            value: InstructionOperation::XSAVE,
        },
        InstructionOperationInfo {
            name: "addps",
            value: InstructionOperation::ADDPS,
        },
        InstructionOperationInfo {
            name: "addpd",
            value: InstructionOperation::ADDPD,
        },
        InstructionOperationInfo {
            name: "addsd",
            value: InstructionOperation::ADDSD,
        },
        InstructionOperationInfo {
            name: "addss",
            value: InstructionOperation::ADDSS,
        },
        InstructionOperationInfo {
            name: "addsubpd",
            value: InstructionOperation::ADDSUBPD,
        },
        InstructionOperationInfo {
            name: "addsubps",
            value: InstructionOperation::ADDSUBPS,
        },
        InstructionOperationInfo {
            name: "andnps",
            value: InstructionOperation::ANDNPS,
        },
        InstructionOperationInfo {
            name: "andnpd",
            value: InstructionOperation::ANDNPD,
        },
        InstructionOperationInfo {
            name: "andps",
            value: InstructionOperation::ANDPS,
        },
        InstructionOperationInfo {
            name: "andpd",
            value: InstructionOperation::ANDPD,
        },
        InstructionOperationInfo {
            name: "cbw",
            value: InstructionOperation::CBW,
        },
        InstructionOperationInfo {
            name: "cwde",
            value: InstructionOperation::CWDE,
        },
        InstructionOperationInfo {
            name: "cdqe",
            value: InstructionOperation::CDQE,
        },
        InstructionOperationInfo {
            name: "cmpsb",
            value: InstructionOperation::CMPSB,
        },
        InstructionOperationInfo {
            name: "cmpsw",
            value: InstructionOperation::CMPSW,
        },
        InstructionOperationInfo {
            name: "cmpsd",
            value: InstructionOperation::CMPSD,
        },
        InstructionOperationInfo {
            name: "cmpsq",
            value: InstructionOperation::CMPSQ,
        },
        InstructionOperationInfo {
            name: "cmovo",
            value: InstructionOperation::CMOVO,
        },
        InstructionOperationInfo {
            name: "cmovno",
            value: InstructionOperation::CMOVNO,
        },
        InstructionOperationInfo {
            name: "cmovb",
            value: InstructionOperation::CMOVB,
        },
        InstructionOperationInfo {
            name: "cmovae",
            value: InstructionOperation::CMOVAE,
        },
        InstructionOperationInfo {
            name: "cmove",
            value: InstructionOperation::CMOVE,
        },
        InstructionOperationInfo {
            name: "cmovne",
            value: InstructionOperation::CMOVNE,
        },
        InstructionOperationInfo {
            name: "cmovbe",
            value: InstructionOperation::CMOVBE,
        },
        InstructionOperationInfo {
            name: "cmova",
            value: InstructionOperation::CMOVA,
        },
        InstructionOperationInfo {
            name: "cmovs",
            value: InstructionOperation::CMOVS,
        },
        InstructionOperationInfo {
            name: "cmovns",
            value: InstructionOperation::CMOVNS,
        },
        InstructionOperationInfo {
            name: "cmovpe",
            value: InstructionOperation::CMOVPE,
        },
        InstructionOperationInfo {
            name: "cmovpo",
            value: InstructionOperation::CMOVPO,
        },
        InstructionOperationInfo {
            name: "cmovl",
            value: InstructionOperation::CMOVL,
        },
        InstructionOperationInfo {
            name: "cmovge",
            value: InstructionOperation::CMOVGE,
        },
        InstructionOperationInfo {
            name: "cmovle",
            value: InstructionOperation::CMOVLE,
        },
        InstructionOperationInfo {
            name: "cmovg",
            value: InstructionOperation::CMOVG,
        },
        InstructionOperationInfo {
            name: "cwd",
            value: InstructionOperation::CWD,
        },
        InstructionOperationInfo {
            name: "cdq",
            value: InstructionOperation::CDQ,
        },
        InstructionOperationInfo {
            name: "cqo",
            value: InstructionOperation::CQO,
        },
        InstructionOperationInfo {
            name: "divps",
            value: InstructionOperation::DIVPS,
        },
        InstructionOperationInfo {
            name: "divpd",
            value: InstructionOperation::DIVPD,
        },
        InstructionOperationInfo {
            name: "divsd",
            value: InstructionOperation::DIVSD,
        },
        InstructionOperationInfo {
            name: "divss",
            value: InstructionOperation::DIVSS,
        },
        InstructionOperationInfo {
            name: "insb",
            value: InstructionOperation::INSB,
        },
        InstructionOperationInfo {
            name: "insw",
            value: InstructionOperation::INSW,
        },
        InstructionOperationInfo {
            name: "insd",
            value: InstructionOperation::INSD,
        },
        InstructionOperationInfo {
            name: "insq",
            value: InstructionOperation::INSQ,
        },
        InstructionOperationInfo {
            name: "jcxz",
            value: InstructionOperation::JCXZ,
        },
        InstructionOperationInfo {
            name: "jecxz",
            value: InstructionOperation::JECXZ,
        },
        InstructionOperationInfo {
            name: "jrcxz",
            value: InstructionOperation::JRCXZ,
        },
        InstructionOperationInfo {
            name: "jo",
            value: InstructionOperation::JO,
        },
        InstructionOperationInfo {
            name: "jno",
            value: InstructionOperation::JNO,
        },
        InstructionOperationInfo {
            name: "jb",
            value: InstructionOperation::JB,
        },
        InstructionOperationInfo {
            name: "jae",
            value: InstructionOperation::JAE,
        },
        InstructionOperationInfo {
            name: "je",
            value: InstructionOperation::JE,
        },
        InstructionOperationInfo {
            name: "jne",
            value: InstructionOperation::JNE,
        },
        InstructionOperationInfo {
            name: "jbe",
            value: InstructionOperation::JBE,
        },
        InstructionOperationInfo {
            name: "ja",
            value: InstructionOperation::JA,
        },
        InstructionOperationInfo {
            name: "js",
            value: InstructionOperation::JS,
        },
        InstructionOperationInfo {
            name: "jns",
            value: InstructionOperation::JNS,
        },
        InstructionOperationInfo {
            name: "jpe",
            value: InstructionOperation::JPE,
        },
        InstructionOperationInfo {
            name: "jpo",
            value: InstructionOperation::JPO,
        },
        InstructionOperationInfo {
            name: "jl",
            value: InstructionOperation::JL,
        },
        InstructionOperationInfo {
            name: "jge",
            value: InstructionOperation::JGE,
        },
        InstructionOperationInfo {
            name: "jle",
            value: InstructionOperation::JLE,
        },
        InstructionOperationInfo {
            name: "jg",
            value: InstructionOperation::JG,
        },
        InstructionOperationInfo {
            name: "lodsb",
            value: InstructionOperation::LODSB,
        },
        InstructionOperationInfo {
            name: "lodsw",
            value: InstructionOperation::LODSW,
        },
        InstructionOperationInfo {
            name: "lodsd",
            value: InstructionOperation::LODSD,
        },
        InstructionOperationInfo {
            name: "lodsq",
            value: InstructionOperation::LODSQ,
        },
        InstructionOperationInfo {
            name: "maxps",
            value: InstructionOperation::MAXPS,
        },
        InstructionOperationInfo {
            name: "maxpd",
            value: InstructionOperation::MAXPD,
        },
        InstructionOperationInfo {
            name: "maxsd",
            value: InstructionOperation::MAXSD,
        },
        InstructionOperationInfo {
            name: "maxss",
            value: InstructionOperation::MAXSS,
        },
        InstructionOperationInfo {
            name: "minps",
            value: InstructionOperation::MINPS,
        },
        InstructionOperationInfo {
            name: "minpd",
            value: InstructionOperation::MINPD,
        },
        InstructionOperationInfo {
            name: "minsd",
            value: InstructionOperation::MINSD,
        },
        InstructionOperationInfo {
            name: "minss",
            value: InstructionOperation::MINSS,
        },
        InstructionOperationInfo {
            name: "movd",
            value: InstructionOperation::MOVD,
        },
        InstructionOperationInfo {
            name: "movq",
            value: InstructionOperation::MOVQ,
        },
        InstructionOperationInfo {
            name: "movsb",
            value: InstructionOperation::MOVSB,
        },
        InstructionOperationInfo {
            name: "movsw",
            value: InstructionOperation::MOVSW,
        },
        InstructionOperationInfo {
            name: "movsd",
            value: InstructionOperation::MOVSD,
        },
        InstructionOperationInfo {
            name: "movsq",
            value: InstructionOperation::MOVSQ,
        },
        InstructionOperationInfo {
            name: "mulps",
            value: InstructionOperation::MULPS,
        },
        InstructionOperationInfo {
            name: "mulpd",
            value: InstructionOperation::MULPD,
        },
        InstructionOperationInfo {
            name: "mulsd",
            value: InstructionOperation::MULSD,
        },
        InstructionOperationInfo {
            name: "mulss",
            value: InstructionOperation::MULSS,
        },
        InstructionOperationInfo {
            name: "orps",
            value: InstructionOperation::ORPS,
        },
        InstructionOperationInfo {
            name: "orpd",
            value: InstructionOperation::ORPD,
        },
        InstructionOperationInfo {
            name: "outsb",
            value: InstructionOperation::OUTSB,
        },
        InstructionOperationInfo {
            name: "outsw",
            value: InstructionOperation::OUTSW,
        },
        InstructionOperationInfo {
            name: "outsd",
            value: InstructionOperation::OUTSD,
        },
        InstructionOperationInfo {
            name: "outsq",
            value: InstructionOperation::OUTSQ,
        },
        InstructionOperationInfo {
            name: "pextrd",
            value: InstructionOperation::PEXTRD,
        },
        InstructionOperationInfo {
            name: "pextrq",
            value: InstructionOperation::PEXTRQ,
        },
        InstructionOperationInfo {
            name: "pinsrd",
            value: InstructionOperation::PINSRD,
        },
        InstructionOperationInfo {
            name: "pinsrq",
            value: InstructionOperation::PINSRQ,
        },
        InstructionOperationInfo {
            name: "popa",
            value: InstructionOperation::POPA,
        },
        InstructionOperationInfo {
            name: "popad",
            value: InstructionOperation::POPAD,
        },
        InstructionOperationInfo {
            name: "popf",
            value: InstructionOperation::POPF,
        },
        InstructionOperationInfo {
            name: "popfd",
            value: InstructionOperation::POPFD,
        },
        InstructionOperationInfo {
            name: "popfq",
            value: InstructionOperation::POPFQ,
        },
        InstructionOperationInfo {
            name: "pusha",
            value: InstructionOperation::PUSHA,
        },
        InstructionOperationInfo {
            name: "pushad",
            value: InstructionOperation::PUSHAD,
        },
        InstructionOperationInfo {
            name: "pushf",
            value: InstructionOperation::PUSHF,
        },
        InstructionOperationInfo {
            name: "pushfd",
            value: InstructionOperation::PUSHFD,
        },
        InstructionOperationInfo {
            name: "pushfq",
            value: InstructionOperation::PUSHFQ,
        },
        InstructionOperationInfo {
            name: "rcpps",
            value: InstructionOperation::RCPPS,
        },
        InstructionOperationInfo {
            name: "rcpss",
            value: InstructionOperation::RCPSS,
        },
        InstructionOperationInfo {
            name: "rsqrtps",
            value: InstructionOperation::RSQRTPS,
        },
        InstructionOperationInfo {
            name: "rsqrtss",
            value: InstructionOperation::RSQRTSS,
        },
        InstructionOperationInfo {
            name: "scasb",
            value: InstructionOperation::SCASB,
        },
        InstructionOperationInfo {
            name: "scasw",
            value: InstructionOperation::SCASW,
        },
        InstructionOperationInfo {
            name: "scasd",
            value: InstructionOperation::SCASD,
        },
        InstructionOperationInfo {
            name: "scasq",
            value: InstructionOperation::SCASQ,
        },
        InstructionOperationInfo {
            name: "seto",
            value: InstructionOperation::SETO,
        },
        InstructionOperationInfo {
            name: "setno",
            value: InstructionOperation::SETNO,
        },
        InstructionOperationInfo {
            name: "setb",
            value: InstructionOperation::SETB,
        },
        InstructionOperationInfo {
            name: "setae",
            value: InstructionOperation::SETAE,
        },
        InstructionOperationInfo {
            name: "sete",
            value: InstructionOperation::SETE,
        },
        InstructionOperationInfo {
            name: "setne",
            value: InstructionOperation::SETNE,
        },
        InstructionOperationInfo {
            name: "setbe",
            value: InstructionOperation::SETBE,
        },
        InstructionOperationInfo {
            name: "seta",
            value: InstructionOperation::SETA,
        },
        InstructionOperationInfo {
            name: "sets",
            value: InstructionOperation::SETS,
        },
        InstructionOperationInfo {
            name: "setns",
            value: InstructionOperation::SETNS,
        },
        InstructionOperationInfo {
            name: "setpe",
            value: InstructionOperation::SETPE,
        },
        InstructionOperationInfo {
            name: "setpo",
            value: InstructionOperation::SETPO,
        },
        InstructionOperationInfo {
            name: "setl",
            value: InstructionOperation::SETL,
        },
        InstructionOperationInfo {
            name: "setge",
            value: InstructionOperation::SETGE,
        },
        InstructionOperationInfo {
            name: "setle",
            value: InstructionOperation::SETLE,
        },
        InstructionOperationInfo {
            name: "setg",
            value: InstructionOperation::SETG,
        },
        InstructionOperationInfo {
            name: "sqrtps",
            value: InstructionOperation::SQRTPS,
        },
        InstructionOperationInfo {
            name: "sqrtpd",
            value: InstructionOperation::SQRTPD,
        },
        InstructionOperationInfo {
            name: "sqrtsd",
            value: InstructionOperation::SQRTSD,
        },
        InstructionOperationInfo {
            name: "sqrtss",
            value: InstructionOperation::SQRTSS,
        },
        InstructionOperationInfo {
            name: "stosb",
            value: InstructionOperation::STOSB,
        },
        InstructionOperationInfo {
            name: "stosw",
            value: InstructionOperation::STOSW,
        },
        InstructionOperationInfo {
            name: "stosd",
            value: InstructionOperation::STOSD,
        },
        InstructionOperationInfo {
            name: "stosq",
            value: InstructionOperation::STOSQ,
        },
        InstructionOperationInfo {
            name: "subps",
            value: InstructionOperation::SUBPS,
        },
        InstructionOperationInfo {
            name: "subpd",
            value: InstructionOperation::SUBPD,
        },
        InstructionOperationInfo {
            name: "subsd",
            value: InstructionOperation::SUBSD,
        },
        InstructionOperationInfo {
            name: "subss",
            value: InstructionOperation::SUBSS,
        },
        InstructionOperationInfo {
            name: "xorps",
            value: InstructionOperation::XORPS,
        },
        InstructionOperationInfo {
            name: "xorpd",
            value: InstructionOperation::XORPD,
        },
        InstructionOperationInfo {
            name: "cmppd",
            value: InstructionOperation::CMPPD,
        },
        InstructionOperationInfo {
            name: "cmpps",
            value: InstructionOperation::CMPPS,
        },
        InstructionOperationInfo {
            name: "cmpss",
            value: InstructionOperation::CMPSS,
        },
        InstructionOperationInfo {
            name: "comisd",
            value: InstructionOperation::COMISD,
        },
        InstructionOperationInfo {
            name: "comiss",
            value: InstructionOperation::COMISS,
        },
        InstructionOperationInfo {
            name: "cvtdq2pd",
            value: InstructionOperation::CVTDQ2PD,
        },
        InstructionOperationInfo {
            name: "cvtdq2ps",
            value: InstructionOperation::CVTDQ2PS,
        },
        InstructionOperationInfo {
            name: "cvtpd2dq",
            value: InstructionOperation::CVTPD2DQ,
        },
        InstructionOperationInfo {
            name: "cvtpd2pi",
            value: InstructionOperation::CVTPD2PI,
        },
        InstructionOperationInfo {
            name: "cvtpd2ps",
            value: InstructionOperation::CVTPD2PS,
        },
        InstructionOperationInfo {
            name: "cvtpi2pd",
            value: InstructionOperation::CVTPI2PD,
        },
        InstructionOperationInfo {
            name: "cvtpi2ps",
            value: InstructionOperation::CVTPI2PS,
        },
        InstructionOperationInfo {
            name: "cvtps2dq",
            value: InstructionOperation::CVTPS2DQ,
        },
        InstructionOperationInfo {
            name: "cvtps2pd",
            value: InstructionOperation::CVTPS2PD,
        },
        InstructionOperationInfo {
            name: "cvtps2pi",
            value: InstructionOperation::CVTPS2PI,
        },
        InstructionOperationInfo {
            name: "cvtsd2si",
            value: InstructionOperation::CVTSD2SI,
        },
        InstructionOperationInfo {
            name: "cvtsd2ss",
            value: InstructionOperation::CVTSD2SS,
        },
        InstructionOperationInfo {
            name: "cvtsi2sd",
            value: InstructionOperation::CVTSI2SD,
        },
        InstructionOperationInfo {
            name: "cvtsi2ss",
            value: InstructionOperation::CVTSI2SS,
        },
        InstructionOperationInfo {
            name: "cvtss2sd",
            value: InstructionOperation::CVTSS2SD,
        },
        InstructionOperationInfo {
            name: "cvtss2si",
            value: InstructionOperation::CVTSS2SI,
        },
        InstructionOperationInfo {
            name: "cvttpd2dq",
            value: InstructionOperation::CVTTPD2DQ,
        },
        InstructionOperationInfo {
            name: "cvttpd2pi",
            value: InstructionOperation::CVTTPD2PI,
        },
        InstructionOperationInfo {
            name: "cvttps2dq",
            value: InstructionOperation::CVTTPS2DQ,
        },
        InstructionOperationInfo {
            name: "cvttps2pi",
            value: InstructionOperation::CVTTPS2PI,
        },
        InstructionOperationInfo {
            name: "cvttsd2si",
            value: InstructionOperation::CVTTSD2SI,
        },
        InstructionOperationInfo {
            name: "cvttss2si",
            value: InstructionOperation::CVTTSS2SI,
        },
        InstructionOperationInfo {
            name: "extractps",
            value: InstructionOperation::EXTRACTPS,
        },
        InstructionOperationInfo {
            name: "haddpd",
            value: InstructionOperation::HADDPD,
        },
        InstructionOperationInfo {
            name: "haddps",
            value: InstructionOperation::HADDPS,
        },
        InstructionOperationInfo {
            name: "hsubpd",
            value: InstructionOperation::HSUBPD,
        },
        InstructionOperationInfo {
            name: "hsubps",
            value: InstructionOperation::HSUBPS,
        },
        InstructionOperationInfo {
            name: "insertps",
            value: InstructionOperation::INSERTPS,
        },
        InstructionOperationInfo {
            name: "lddqu",
            value: InstructionOperation::LDDQU,
        },
        InstructionOperationInfo {
            name: "lgdt",
            value: InstructionOperation::LGDT,
        },
        InstructionOperationInfo {
            name: "lidt",
            value: InstructionOperation::LIDT,
        },
        InstructionOperationInfo {
            name: "lldt",
            value: InstructionOperation::LLDT,
        },
        InstructionOperationInfo {
            name: "lmsw",
            value: InstructionOperation::LMSW,
        },
        InstructionOperationInfo {
            name: "ltr",
            value: InstructionOperation::LTR,
        },
        InstructionOperationInfo {
            name: "maskmovq",
            value: InstructionOperation::MASKMOVQ,
        },
        InstructionOperationInfo {
            name: "maskmovdqu",
            value: InstructionOperation::MASKMOVDQU,
        },
        InstructionOperationInfo {
            name: "mmxnop",
            value: InstructionOperation::MMXNOP,
        },
        InstructionOperationInfo {
            name: "monitor",
            value: InstructionOperation::MONITOR,
        },
        InstructionOperationInfo {
            name: "movapd",
            value: InstructionOperation::MOVAPD,
        },
        InstructionOperationInfo {
            name: "movaps",
            value: InstructionOperation::MOVAPS,
        },
        InstructionOperationInfo {
            name: "movddup",
            value: InstructionOperation::MOVDDUP,
        },
        InstructionOperationInfo {
            name: "movdq2q",
            value: InstructionOperation::MOVDQ2Q,
        },
        InstructionOperationInfo {
            name: "movdqa",
            value: InstructionOperation::MOVDQA,
        },
        InstructionOperationInfo {
            name: "movdqu",
            value: InstructionOperation::MOVDQU,
        },
        InstructionOperationInfo {
            name: "movhlps",
            value: InstructionOperation::MOVHLPS,
        },
        InstructionOperationInfo {
            name: "movhpd",
            value: InstructionOperation::MOVHPD,
        },
        InstructionOperationInfo {
            name: "movhps",
            value: InstructionOperation::MOVHPS,
        },
        InstructionOperationInfo {
            name: "movshdup",
            value: InstructionOperation::MOVSHDUP,
        },
        InstructionOperationInfo {
            name: "movsldup",
            value: InstructionOperation::MOVSLDUP,
        },
        InstructionOperationInfo {
            name: "movlhps",
            value: InstructionOperation::MOVLHPS,
        },
        InstructionOperationInfo {
            name: "movlpd",
            value: InstructionOperation::MOVLPD,
        },
        InstructionOperationInfo {
            name: "movlps",
            value: InstructionOperation::MOVLPS,
        },
        InstructionOperationInfo {
            name: "movmskpd",
            value: InstructionOperation::MOVMSKPD,
        },
        InstructionOperationInfo {
            name: "movmskps",
            value: InstructionOperation::MOVMSKPS,
        },
        InstructionOperationInfo {
            name: "movntdq",
            value: InstructionOperation::MOVNTDQ,
        },
        InstructionOperationInfo {
            name: "movntdqa",
            value: InstructionOperation::MOVNTDQA,
        },
        InstructionOperationInfo {
            name: "movntpd",
            value: InstructionOperation::MOVNTPD,
        },
        InstructionOperationInfo {
            name: "movntps",
            value: InstructionOperation::MOVNTPS,
        },
        InstructionOperationInfo {
            name: "movntq",
            value: InstructionOperation::MOVNTQ,
        },
        InstructionOperationInfo {
            name: "movq2dq",
            value: InstructionOperation::MOVQ2DQ,
        },
        InstructionOperationInfo {
            name: "mwait",
            value: InstructionOperation::MWAIT,
        },
        InstructionOperationInfo {
            name: "pinsrb",
            value: InstructionOperation::PINSRB,
        },
        InstructionOperationInfo {
            name: "pinsrw",
            value: InstructionOperation::PINSRW,
        },
        InstructionOperationInfo {
            name: "pextrb",
            value: InstructionOperation::PEXTRB,
        },
        InstructionOperationInfo {
            name: "pextrw",
            value: InstructionOperation::PEXTRW,
        },
        InstructionOperationInfo {
            name: "pmovmskb",
            value: InstructionOperation::PMOVMSKB,
        },
        InstructionOperationInfo {
            name: "pmovsxbd",
            value: InstructionOperation::PMOVSXBD,
        },
        InstructionOperationInfo {
            name: "pmovsxbq",
            value: InstructionOperation::PMOVSXBQ,
        },
        InstructionOperationInfo {
            name: "pmovsxdq",
            value: InstructionOperation::PMOVSXDQ,
        },
        InstructionOperationInfo {
            name: "pmovsxbw",
            value: InstructionOperation::PMOVSXBW,
        },
        InstructionOperationInfo {
            name: "pmovsxwd",
            value: InstructionOperation::PMOVSXWD,
        },
        InstructionOperationInfo {
            name: "pmovsxwq",
            value: InstructionOperation::PMOVSXWQ,
        },
        InstructionOperationInfo {
            name: "pmovzxbd",
            value: InstructionOperation::PMOVZXBD,
        },
        InstructionOperationInfo {
            name: "pmovzxbq",
            value: InstructionOperation::PMOVZXBQ,
        },
        InstructionOperationInfo {
            name: "pmovzxdq",
            value: InstructionOperation::PMOVZXDQ,
        },
        InstructionOperationInfo {
            name: "pmovzxbw",
            value: InstructionOperation::PMOVZXBW,
        },
        InstructionOperationInfo {
            name: "pmovzxwd",
            value: InstructionOperation::PMOVZXWD,
        },
        InstructionOperationInfo {
            name: "pmovzxwq",
            value: InstructionOperation::PMOVZXWQ,
        },
        InstructionOperationInfo {
            name: "prefetch",
            value: InstructionOperation::PREFETCH,
        },
        InstructionOperationInfo {
            name: "prefetchnta",
            value: InstructionOperation::PREFETCHNTA,
        },
        InstructionOperationInfo {
            name: "prefetcht0",
            value: InstructionOperation::PREFETCHT0,
        },
        InstructionOperationInfo {
            name: "prefetcht1",
            value: InstructionOperation::PREFETCHT1,
        },
        InstructionOperationInfo {
            name: "prefetcht2",
            value: InstructionOperation::PREFETCHT2,
        },
        InstructionOperationInfo {
            name: "prefetchw",
            value: InstructionOperation::PREFETCHW,
        },
        InstructionOperationInfo {
            name: "pshufd",
            value: InstructionOperation::PSHUFD,
        },
        InstructionOperationInfo {
            name: "pshufhw",
            value: InstructionOperation::PSHUFHW,
        },
        InstructionOperationInfo {
            name: "pshuflw",
            value: InstructionOperation::PSHUFLW,
        },
        InstructionOperationInfo {
            name: "pshufw",
            value: InstructionOperation::PSHUFW,
        },
        InstructionOperationInfo {
            name: "punpcklbw",
            value: InstructionOperation::PUNPCKLBW,
        },
        InstructionOperationInfo {
            name: "punpckldq",
            value: InstructionOperation::PUNPCKLDQ,
        },
        InstructionOperationInfo {
            name: "punpcklwd",
            value: InstructionOperation::PUNPCKLWD,
        },
        InstructionOperationInfo {
            name: "roundsd",
            value: InstructionOperation::ROUNDSD,
        },
        InstructionOperationInfo {
            name: "roundss",
            value: InstructionOperation::ROUNDSS,
        },
        InstructionOperationInfo {
            name: "sgdt",
            value: InstructionOperation::SGDT,
        },
        InstructionOperationInfo {
            name: "sidt",
            value: InstructionOperation::SIDT,
        },
        InstructionOperationInfo {
            name: "sldt",
            value: InstructionOperation::SLDT,
        },
        InstructionOperationInfo {
            name: "shufpd",
            value: InstructionOperation::SHUFPD,
        },
        InstructionOperationInfo {
            name: "shufps",
            value: InstructionOperation::SHUFPS,
        },
        InstructionOperationInfo {
            name: "smsw",
            value: InstructionOperation::SMSW,
        },
        InstructionOperationInfo {
            name: "str",
            value: InstructionOperation::STR,
        },
        InstructionOperationInfo {
            name: "swapgs",
            value: InstructionOperation::SWAPGS,
        },
        InstructionOperationInfo {
            name: "ucomisd",
            value: InstructionOperation::UCOMISD,
        },
        InstructionOperationInfo {
            name: "ucomiss",
            value: InstructionOperation::UCOMISS,
        },
        InstructionOperationInfo {
            name: "unpckhpd",
            value: InstructionOperation::UNPCKHPD,
        },
        InstructionOperationInfo {
            name: "unpckhps",
            value: InstructionOperation::UNPCKHPS,
        },
        InstructionOperationInfo {
            name: "unpcklpd",
            value: InstructionOperation::UNPCKLPD,
        },
        InstructionOperationInfo {
            name: "unpcklps",
            value: InstructionOperation::UNPCKLPS,
        },
        InstructionOperationInfo {
            name: "verr",
            value: InstructionOperation::VERR,
        },
        InstructionOperationInfo {
            name: "verw",
            value: InstructionOperation::VERW,
        },
        InstructionOperationInfo {
            name: "vmcall",
            value: InstructionOperation::VMCALL,
        },
        InstructionOperationInfo {
            name: "vmclear",
            value: InstructionOperation::VMCLEAR,
        },
        InstructionOperationInfo {
            name: "vmlaunch",
            value: InstructionOperation::VMLAUNCH,
        },
        InstructionOperationInfo {
            name: "vmptrld",
            value: InstructionOperation::VMPTRLD,
        },
        InstructionOperationInfo {
            name: "vmptrst",
            value: InstructionOperation::VMPTRST,
        },
        InstructionOperationInfo {
            name: "vmresume",
            value: InstructionOperation::VMRESUME,
        },
        InstructionOperationInfo {
            name: "vmxoff",
            value: InstructionOperation::VMXOFF,
        },
        InstructionOperationInfo {
            name: "vmxon",
            value: InstructionOperation::VMXON,
        },
        InstructionOperationInfo {
            name: "xgetbv",
            value: InstructionOperation::XGETBV,
        },
        InstructionOperationInfo {
            name: "xsetbv",
            value: InstructionOperation::XSETBV,
        },
    ];
