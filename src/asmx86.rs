// Licensed under the 2-Clause BSD license <LICENSE or
// https://opensource.org/licenses/BSD-2-Clause>. This
// file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_camel_case_types,
         non_snake_case)]

use std::ptr;

static OPERATION_STRINGS: [&str; 621] = [
    "",
    "aaa",
    "aad",
    "aam",
    "aas",
    "add",
    "adc",
    "and",
    "arpl",
    "blendpd",
    "blendps",
    "blendvpd",
    "blendvps",
    "bound",
    "bsf",
    "bsr",
    "bswap",
    "bt",
    "btc",
    "btr",
    "bts",
    "callf",
    "call",
    "clc",
    "cld",
    "clflush",
    "cli",
    "clts",
    "cmc",
    "cmp",
    "cmpxch16b",
    "cmpxch8b",
    "cmpxchg",
    "cpuid",
    "crc32",
    "daa",
    "das",
    "dec",
    "div",
    "dppd",
    "dpps",
    "emms",
    "enter",
    "f2xm1",
    "fabs",
    "fadd",
    "faddp",
    "fbld",
    "fbstp",
    "fchs",
    "fclex",
    "fcmovb",
    "fcmovbe",
    "fcmove",
    "fcmovnb",
    "fcmovnbe",
    "fcmovne",
    "fcmovnu",
    "fcmovu",
    "fcom",
    "fcomi",
    "fcomip",
    "fcomp",
    "fcompp",
    "fcos",
    "fdecstp",
    "fdisi",
    "fdiv",
    "fdivp",
    "fdivr",
    "fdivrp",
    "femms",
    "feni",
    "ffree",
    "ffreep",
    "fiadd",
    "ficom",
    "ficomp",
    "fidiv",
    "fidivr",
    "fild",
    "fimul",
    "fincstp",
    "finit",
    "fist",
    "fistp",
    "fisttp",
    "fisub",
    "fisubr",
    "fld",
    "fld1",
    "fldcw",
    "fldenv",
    "fldl2e",
    "fldl2t",
    "fldlg2",
    "fldln2",
    "fldpi",
    "fldz",
    "fmul",
    "fmulp",
    "fnop",
    "fpatan",
    "fprem",
    "fprem1",
    "fptan",
    "frichop",
    "frinear",
    "frint2",
    "frndint",
    "frstor",
    "frstpm",
    "fsave",
    "fscale",
    "fsetpm",
    "fsin",
    "fsincos",
    "fsqrt",
    "fst",
    "fstcw",
    "fstdw",
    "fstenv",
    "fstp",
    "fstsg",
    "fstsw",
    "fsub",
    "fsubp",
    "fsubr",
    "fsubrp",
    "ftst",
    "fucom",
    "fucomi",
    "fucomip",
    "fucomp",
    "fucompp",
    "fwait",
    "fxam",
    "fxch",
    "fxrstor",
    "fxsave",
    "fxtract",
    "fyl2x",
    "fyl2xp1",
    "getsec",
    "hlt",
    "idiv",
    "imul",
    "in",
    "inc",
    "int",
    "int1",
    "int3",
    "into",
    "invd",
    "invlpg",
    "iret",
    "jmpf",
    "jmp",
    "lahf",
    "lar",
    "ldmxcsr",
    "lds",
    "lea",
    "leave",
    "les",
    "lfence",
    "lfs",
    "lgs",
    "loop",
    "loope",
    "loopne",
    "lsl",
    "lss",
    "mfence",
    "mov",
    "movnti",
    "movss",
    "movsx",
    "movsxd",
    "movupd",
    "movups",
    "movzx",
    "mpsadbw",
    "mul",
    "neg",
    "nop",
    "not",
    "or",
    "out",
    "packssdw",
    "packsswb",
    "packusdw",
    "packuswb",
    "pabsb",
    "pabsd",
    "pabsw",
    "paddb",
    "paddd",
    "paddq",
    "paddw",
    "paddsb",
    "paddsw",
    "paddusb",
    "paddusw",
    "palignr",
    "pand",
    "pandn",
    "pause",
    "pavgb",
    "pavgusb",
    "pavgw",
    "pblendvb",
    "pblendw",
    "pcmpeqb",
    "pcmpeqd",
    "pcmpeqq",
    "pcmpeqw",
    "pcmpestri",
    "pcmpestrm",
    "pcmpgtb",
    "pcmpgtd",
    "pcmpgtq",
    "pcmpgtw",
    "pcmpistri",
    "pcmpistrm",
    "pf2id",
    "pf2iw",
    "pfacc",
    "pfadd",
    "pfcmpeq",
    "pfcmpge",
    "pfcmpgt",
    "pfmax",
    "pfmin",
    "pfmul",
    "pfnacc",
    "pfpnacc",
    "pfrcp",
    "pfrcpit1",
    "pfrcpit2",
    "pfrcpv",
    "pfrsqit1",
    "pfrsqrt",
    "pfrsqrtv",
    "pfsub",
    "pfsubr",
    "phaddd",
    "phaddsw",
    "phaddw",
    "phminposuw",
    "phsubd",
    "phsubsw",
    "phsubw",
    "pi2fd",
    "pi2fw",
    "pmaddwd",
    "pmaddubsw",
    "pmaxsb",
    "pmaxsd",
    "pmaxsw",
    "pmaxub",
    "pmaxud",
    "pmaxuw",
    "pminsb",
    "pminsd",
    "pminsw",
    "pminub",
    "pminud",
    "pminuw",
    "pmuldq",
    "pmulhrsw",
    "pmulhrw",
    "pmulhuw",
    "pmulhw",
    "pmulld",
    "pmullw",
    "pmuludq",
    "pop",
    "popcnt",
    "por",
    "psadbw",
    "pshufb",
    "psignb",
    "psignd",
    "psignw",
    "pslld",
    "pslldq",
    "psllq",
    "psllw",
    "psrad",
    "psraw",
    "psrld",
    "psrldq",
    "psrlq",
    "psrlw",
    "psubb",
    "psubd",
    "psubq",
    "psubw",
    "psubsb",
    "psubsw",
    "psubusb",
    "psubusw",
    "pswapd",
    "ptest",
    "punpckhbw",
    "punpckhdq",
    "punpckhqdq",
    "punpckhwd",
    "punpcklqdq",
    "push",
    "pxor",
    "rdmsr",
    "rdpmc",
    "rdtsc",
    "retf",
    "retn",
    "rcl",
    "rcr",
    "rol",
    "ror",
    "roundps",
    "roundpd",
    "rsm",
    "sahf",
    "salc",
    "sar",
    "sbb",
    "sfence",
    "shl",
    "shld",
    "shr",
    "shrd",
    "sub",
    "stc",
    "std",
    "sti",
    "stmxcsr",
    "syscall",
    "sysenter",
    "sysexit",
    "sysret",
    "test",
    "ud2",
    "vmread",
    "vmwrite",
    "wbinvd",
    "wrmsr",
    "xchg",
    "xlat",
    "xadd",
    "xor",
    "xrstor",
    "xsave",
    "addps",
    "addpd",
    "addsd",
    "addss",
    "addsubpd",
    "addsubps",
    "andnps",
    "andnpd",
    "andps",
    "andpd",
    "cbw",
    "cwde",
    "cdqe",
    "cmpsb",
    "cmpsw",
    "cmpsd",
    "cmpsq",
    "cmovo",
    "cmovno",
    "cmovb",
    "cmovae",
    "cmove",
    "cmovne",
    "cmovbe",
    "cmova",
    "cmovs",
    "cmovns",
    "cmovpe",
    "cmovpo",
    "cmovl",
    "cmovge",
    "cmovle",
    "cmovg",
    "cwd",
    "cdq",
    "cqo",
    "divps",
    "divpd",
    "divsd",
    "divss",
    "insb",
    "insw",
    "insd",
    "insq",
    "jcxz",
    "jecxz",
    "jrcxz",
    "jo",
    "jno",
    "jb",
    "jae",
    "je",
    "jne",
    "jbe",
    "ja",
    "js",
    "jns",
    "jpe",
    "jpo",
    "jl",
    "jge",
    "jle",
    "jg",
    "lodsb",
    "lodsw",
    "lodsd",
    "lodsq",
    "maxps",
    "maxpd",
    "maxsd",
    "maxss",
    "minps",
    "minpd",
    "minsd",
    "minss",
    "movd",
    "movq",
    "movsb",
    "movsw",
    "movsd",
    "movsq",
    "mulps",
    "mulpd",
    "mulsd",
    "mulss",
    "orps",
    "orpd",
    "outsb",
    "outsw",
    "outsd",
    "outsq",
    "pextrd",
    "pextrq",
    "pinsrd",
    "pinsrq",
    "popa",
    "popad",
    "popf",
    "popfd",
    "popfq",
    "pusha",
    "pushad",
    "pushf",
    "pushfd",
    "pushfq",
    "rcpps",
    "rcpss",
    "rsqrtps",
    "rsqrtss",
    "scasb",
    "scasw",
    "scasd",
    "scasq",
    "seto",
    "setno",
    "setb",
    "setae",
    "sete",
    "setne",
    "setbe",
    "seta",
    "sets",
    "setns",
    "setpe",
    "setpo",
    "setl",
    "setge",
    "setle",
    "setg",
    "sqrtps",
    "sqrtpd",
    "sqrtsd",
    "sqrtss",
    "stosb",
    "stosw",
    "stosd",
    "stosq",
    "subps",
    "subpd",
    "subsd",
    "subss",
    "xorps",
    "xorpd",
    "cmppd",
    "cmpps",
    "cmpss",
    "comisd",
    "comiss",
    "cvtdq2pd",
    "cvtdq2ps",
    "cvtpd2dq",
    "cvtpd2pi",
    "cvtpd2ps",
    "cvtpi2pd",
    "cvtpi2ps",
    "cvtps2dq",
    "cvtps2pd",
    "cvtps2pi",
    "cvtsd2si",
    "cvtsd2ss",
    "cvtsi2sd",
    "cvtsi2ss",
    "cvtss2sd",
    "cvtss2si",
    "cvttpd2dq",
    "cvttpd2pi",
    "cvttps2dq",
    "cvttps2pi",
    "cvttsd2si",
    "cvttss2si",
    "extractps",
    "haddpd",
    "haddps",
    "hsubpd",
    "hsubps",
    "insertps",
    "lddqu",
    "lgdt",
    "lidt",
    "lldt",
    "lmsw",
    "ltr",
    "maskmovq",
    "maskmovdqu",
    "mmxnop",
    "monitor",
    "movapd",
    "movaps",
    "movddup",
    "movdq2q",
    "movdqa",
    "movdqu",
    "movhlps",
    "movhpd",
    "movhps",
    "movshdup",
    "movsldup",
    "movlhps",
    "movlpd",
    "movlps",
    "movmskpd",
    "movmskps",
    "movntdq",
    "movntdqa",
    "movntpd",
    "movntps",
    "movntq",
    "movq2dq",
    "mwait",
    "pinsrb",
    "pinsrw",
    "pextrb",
    "pextrw",
    "pmovmskb",
    "pmovsxbd",
    "pmovsxbq",
    "pmovsxdq",
    "pmovsxbw",
    "pmovsxwd",
    "pmovsxwq",
    "pmovzxbd",
    "pmovzxbq",
    "pmovzxdq",
    "pmovzxbw",
    "pmovzxwd",
    "pmovzxwq",
    "prefetch",
    "prefetchnta",
    "prefetcht0",
    "prefetcht1",
    "prefetcht2",
    "prefetchw",
    "pshufd",
    "pshufhw",
    "pshuflw",
    "pshufw",
    "punpcklbw",
    "punpckldq",
    "punpcklwd",
    "roundsd",
    "roundss",
    "sgdt",
    "sidt",
    "sldt",
    "shufpd",
    "shufps",
    "smsw",
    "str",
    "swapgs",
    "ucomisd",
    "ucomiss",
    "unpckhpd",
    "unpckhps",
    "unpcklpd",
    "unpcklps",
    "verr",
    "verw",
    "vmcall",
    "vmclear",
    "vmlaunch",
    "vmptrld",
    "vmptrst",
    "vmresume",
    "vmxoff",
    "vmxon",
    "xgetbv",
    "xsetbv",
];

static OPERAND_STRING: [&str; 158] = [
    "",
    "",
    "",
    "al",
    "cl",
    "dl",
    "bl",
    "ah",
    "ch",
    "dh",
    "bh",
    "spl",
    "bpl",
    "sil",
    "dil",
    "r8b",
    "r9b",
    "r10b",
    "r11b",
    "r12b",
    "r13b",
    "r14b",
    "r15b",
    "ax",
    "cx",
    "dx",
    "bx",
    "sp",
    "bp",
    "si",
    "di",
    "r8w",
    "r9w",
    "r10w",
    "r11w",
    "r12w",
    "r13w",
    "r14w",
    "r15w",
    "eax",
    "ecx",
    "edx",
    "ebx",
    "esp",
    "ebp",
    "esi",
    "edi",
    "r8d",
    "r9d",
    "r10d",
    "r11d",
    "r12d",
    "r13d",
    "r14d",
    "r15d",
    "rax",
    "rcx",
    "rdx",
    "rbx",
    "rsp",
    "rbp",
    "rsi",
    "rdi",
    "r8",
    "r9",
    "r10",
    "r11",
    "r12",
    "r13",
    "r14",
    "r15",
    "st0",
    "st1",
    "st2",
    "st3",
    "st4",
    "st5",
    "st6",
    "st7",
    "mm0",
    "mm1",
    "mm2",
    "mm3",
    "mm4",
    "mm5",
    "mm6",
    "mm7",
    "xmm0",
    "xmm1",
    "xmm2",
    "xmm3",
    "xmm4",
    "xmm5",
    "xmm6",
    "xmm7",
    "xmm8",
    "xmm9",
    "xmm10",
    "xmm11",
    "xmm12",
    "xmm13",
    "xmm14",
    "xmm15",
    "cr0",
    "cr1",
    "cr2",
    "cr3",
    "cr4",
    "cr5",
    "cr6",
    "cr7",
    "cr8",
    "cr9",
    "cr10",
    "cr11",
    "cr12",
    "cr13",
    "cr14",
    "cr15",
    "dr0",
    "dr1",
    "dr2",
    "dr3",
    "dr4",
    "dr5",
    "dr6",
    "dr7",
    "dr8",
    "dr9",
    "dr10",
    "dr11",
    "dr12",
    "dr13",
    "dr14",
    "dr15",
    "tr0",
    "tr1",
    "tr2",
    "tr3",
    "tr4",
    "tr5",
    "tr6",
    "tr7",
    "tr8",
    "tr9",
    "tr10",
    "tr11",
    "tr12",
    "tr13",
    "tr14",
    "tr15",
    "es",
    "cs",
    "ss",
    "ds",
    "fs",
    "gs",
    "rip",
];

#[derive(Clone, Copy, PartialEq, PartialOrd)]
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
    pub fn from_i32(_i: i32) -> Self {
        unimplemented!();
    }
}

impl Default for InstructionOperation {
    fn default() -> Self {
        InstructionOperation::INVALID
    }
}


#[derive(Clone, Copy, PartialEq, PartialOrd)]
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
    pub fn from_i32(_i: i32) -> Self {
        unimplemented!();
    }
}

impl Default for OperandType {
    fn default() -> Self {
        OperandType::NONE
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
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

#[derive(Copy, Default)]
#[repr(C)]
pub struct InstructionOperand {
    pub operand: OperandType,
    pub components: [OperandType; 2],
    pub scale: u8,
    pub size: u16,
    pub immediate: isize,
    pub segment: SegmentRegister,
}

impl Clone for InstructionOperand {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy, Default)]
#[repr(C)]
pub struct Instruction {
    pub operation: InstructionOperation,
    pub operands: [InstructionOperand; 3],
    pub flags: u32,
    pub segment: SegmentRegister,
    pub length: usize,
}

impl Clone for Instruction {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
#[repr(i32)]
pub enum RepPrefix {
    REP_PREFIX_NONE = 0i32,
    REP_PREFIX_REPNE,
    REP_PREFIX_REPE,
}

#[derive(Copy)]
#[repr(C)]
pub struct DecodeState {
    pub result: *mut Instruction,
    pub operand0: *mut InstructionOperand,
    pub operand1: *mut InstructionOperand,
    pub opcodeStart: *const u8,
    pub opcode: *const u8,
    pub addr: usize,
    pub len: usize,
    pub origLen: usize,
    pub opSize: u16,
    pub finalOpSize: u16,
    pub addrSize: u16,
    pub flags: u32,
    pub invalid: bool,
    pub insufficientLength: bool,
    pub opPrefix: bool,
    pub rep: RepPrefix,
    pub using64: bool,
    pub rex: bool,
    pub rexRM1: bool,
    pub rexRM2: bool,
    pub rexReg: bool,
    pub ripRelFixup: *mut isize,
}

impl Clone for DecodeState {
    fn clone(&self) -> Self {
        *self
    }
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

#[derive(Copy)]
#[repr(C)]
pub struct InstructionEncoding {
    pub operation: u16,
    pub flags: u16,
    pub func: unsafe extern "C" fn(*mut DecodeState),
}

impl Clone for InstructionEncoding {
    fn clone(&self) -> Self {
        *self
    }
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

#[derive(Copy)]
#[repr(C)]
pub struct SparseInstructionEncoding {
    pub opcode: u8,
    pub encoding: InstructionEncoding,
}

impl Clone for SparseInstructionEncoding {
    fn clone(&self) -> Self {
        *self
    }
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

static GROUP_OPERATIONS: [[u16; 8]; 26] = [
    [
        InstructionOperation::ADD as (u16),
        InstructionOperation::OR as (u16),
        InstructionOperation::ADC as (u16),
        InstructionOperation::SBB as (u16),
        InstructionOperation::AND as (u16),
        InstructionOperation::SUB as (u16),
        InstructionOperation::XOR as (u16),
        InstructionOperation::CMP as (u16),
    ],
    [
        InstructionOperation::ROL as (u16),
        InstructionOperation::ROR as (u16),
        InstructionOperation::RCL as (u16),
        InstructionOperation::RCR as (u16),
        InstructionOperation::SHL as (u16),
        InstructionOperation::SHR as (u16),
        InstructionOperation::SHL as (u16),
        InstructionOperation::SAR as (u16),
    ],
    [
        InstructionOperation::MOV as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
    ],
    [
        InstructionOperation::TEST as (u16),
        InstructionOperation::TEST as (u16),
        InstructionOperation::NOT as (u16),
        InstructionOperation::NEG as (u16),
        InstructionOperation::MUL as (u16),
        InstructionOperation::IMUL as (u16),
        InstructionOperation::DIV as (u16),
        InstructionOperation::IDIV as (u16),
    ],
    [
        InstructionOperation::INC as (u16),
        InstructionOperation::DEC as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
    ],
    [
        InstructionOperation::INC as (u16),
        InstructionOperation::DEC as (u16),
        InstructionOperation::CALL as (u16),
        InstructionOperation::CALLF as (u16),
        InstructionOperation::JMP as (u16),
        InstructionOperation::JMPF as (u16),
        InstructionOperation::PUSH as (u16),
        InstructionOperation::INVALID as (u16),
    ],
    [
        InstructionOperation::SLDT as (u16),
        InstructionOperation::STR as (u16),
        InstructionOperation::LLDT as (u16),
        InstructionOperation::LTR as (u16),
        InstructionOperation::VERR as (u16),
        InstructionOperation::VERW as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
    ],
    [
        InstructionOperation::SGDT as (u16),
        InstructionOperation::SIDT as (u16),
        InstructionOperation::LGDT as (u16),
        InstructionOperation::LIDT as (u16),
        InstructionOperation::SMSW as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::LMSW as (u16),
        InstructionOperation::INVLPG as (u16),
    ],
    [
        InstructionOperation::PREFETCH as (u16),
        InstructionOperation::PREFETCHW as (u16),
        InstructionOperation::PREFETCH as (u16),
        InstructionOperation::PREFETCH as (u16),
        InstructionOperation::PREFETCH as (u16),
        InstructionOperation::PREFETCH as (u16),
        InstructionOperation::PREFETCH as (u16),
        InstructionOperation::PREFETCH as (u16),
    ],
    [
        InstructionOperation::PREFETCHNTA as (u16),
        InstructionOperation::PREFETCHT0 as (u16),
        InstructionOperation::PREFETCHT1 as (u16),
        InstructionOperation::PREFETCHT2 as (u16),
        InstructionOperation::MMXNOP as (u16),
        InstructionOperation::MMXNOP as (u16),
        InstructionOperation::MMXNOP as (u16),
        InstructionOperation::MMXNOP as (u16),
    ],
    [
        InstructionOperation::MMXNOP as (u16),
        InstructionOperation::MMXNOP as (u16),
        InstructionOperation::MMXNOP as (u16),
        InstructionOperation::MMXNOP as (u16),
        InstructionOperation::MMXNOP as (u16),
        InstructionOperation::MMXNOP as (u16),
        InstructionOperation::MMXNOP as (u16),
        InstructionOperation::MMXNOP as (u16),
    ],
    [
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::BT as (u16),
        InstructionOperation::BTS as (u16),
        InstructionOperation::BTR as (u16),
        InstructionOperation::BTC as (u16),
    ],
    [
        InstructionOperation::FNOP as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
    ],
    [
        InstructionOperation::FCHS as (u16),
        InstructionOperation::FABS as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::FTST as (u16),
        InstructionOperation::FXAM as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
    ],
    [
        InstructionOperation::FLD1 as (u16),
        InstructionOperation::FLDL2T as (u16),
        InstructionOperation::FLDL2E as (u16),
        InstructionOperation::FLDPI as (u16),
        InstructionOperation::FLDLG2 as (u16),
        InstructionOperation::FLDLN2 as (u16),
        InstructionOperation::FLDZ as (u16),
        InstructionOperation::INVALID as (u16),
    ],
    [
        InstructionOperation::F2XM1 as (u16),
        InstructionOperation::FYL2X as (u16),
        InstructionOperation::FPTAN as (u16),
        InstructionOperation::FPATAN as (u16),
        InstructionOperation::FXTRACT as (u16),
        InstructionOperation::FPREM1 as (u16),
        InstructionOperation::FDECSTP as (u16),
        InstructionOperation::FINCSTP as (u16),
    ],
    [
        InstructionOperation::FPREM as (u16),
        InstructionOperation::FYL2XP1 as (u16),
        InstructionOperation::FSQRT as (u16),
        InstructionOperation::FSINCOS as (u16),
        InstructionOperation::FRNDINT as (u16),
        InstructionOperation::FSCALE as (u16),
        InstructionOperation::FSIN as (u16),
        InstructionOperation::FCOS as (u16),
    ],
    [
        InstructionOperation::INVALID as (u16),
        InstructionOperation::FUCOMPP as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
    ],
    [
        InstructionOperation::FENI as (u16),
        InstructionOperation::FDISI as (u16),
        InstructionOperation::FCLEX as (u16),
        InstructionOperation::FINIT as (u16),
        InstructionOperation::FSETPM as (u16),
        InstructionOperation::FRSTPM as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
    ],
    [
        InstructionOperation::INVALID as (u16),
        InstructionOperation::FCOMPP as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
    ],
    [
        InstructionOperation::FSTSW as (u16),
        InstructionOperation::FSTDW as (u16),
        InstructionOperation::FSTSG as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
    ],
    [
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::FRINT2 as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
    ],
    [
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::FRICHOP as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
    ],
    [
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::FRINEAR as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
    ],
    [
        InstructionOperation::FXSAVE as (u16),
        InstructionOperation::FXRSTOR as (u16),
        InstructionOperation::LDMXCSR as (u16),
        InstructionOperation::STMXCSR as (u16),
        InstructionOperation::XSAVE as (u16),
        InstructionOperation::XRSTOR as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::CLFLUSH as (u16),
    ],
    [
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::LFENCE as (u16),
        InstructionOperation::MFENCE as (u16),
        InstructionOperation::SFENCE as (u16),
    ],
];

static GROUP_0F01_REG_OPERATIONS: [[u16; 8]; 8] = [
    [
        InstructionOperation::INVALID as (u16),
        InstructionOperation::VMCALL as (u16),
        InstructionOperation::VMLAUNCH as (u16),
        InstructionOperation::VMRESUME as (u16),
        InstructionOperation::VMXOFF as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
    ],
    [
        InstructionOperation::MONITOR as (u16),
        InstructionOperation::MWAIT as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
    ],
    [
        InstructionOperation::XGETBV as (u16),
        InstructionOperation::XSETBV as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
    ],
    [
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
    ],
    [
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
    ],
    [
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
    ],
    [
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
    ],
    [
        InstructionOperation::SWAPGS as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
        InstructionOperation::INVALID as (u16),
    ],
];

static MMX_GROUP_OPERATIONS: [[[u16; 2]; 8]; 3] = [
    [
        [
            InstructionOperation::INVALID as (u16),
            InstructionOperation::INVALID as (u16),
        ],
        [
            InstructionOperation::INVALID as (u16),
            InstructionOperation::INVALID as (u16),
        ],
        [
            InstructionOperation::PSRLW as (u16),
            InstructionOperation::PSRLW as (u16),
        ],
        [
            InstructionOperation::INVALID as (u16),
            InstructionOperation::INVALID as (u16),
        ],
        [
            InstructionOperation::PSRAW as (u16),
            InstructionOperation::PSRAW as (u16),
        ],
        [
            InstructionOperation::INVALID as (u16),
            InstructionOperation::INVALID as (u16),
        ],
        [
            InstructionOperation::PSLLW as (u16),
            InstructionOperation::PSLLW as (u16),
        ],
        [
            InstructionOperation::INVALID as (u16),
            InstructionOperation::INVALID as (u16),
        ],
    ],
    [
        [
            InstructionOperation::INVALID as (u16),
            InstructionOperation::INVALID as (u16),
        ],
        [
            InstructionOperation::INVALID as (u16),
            InstructionOperation::INVALID as (u16),
        ],
        [
            InstructionOperation::PSRLD as (u16),
            InstructionOperation::PSRLD as (u16),
        ],
        [
            InstructionOperation::INVALID as (u16),
            InstructionOperation::INVALID as (u16),
        ],
        [
            InstructionOperation::PSRAD as (u16),
            InstructionOperation::PSRAD as (u16),
        ],
        [
            InstructionOperation::INVALID as (u16),
            InstructionOperation::INVALID as (u16),
        ],
        [
            InstructionOperation::PSLLD as (u16),
            InstructionOperation::PSLLD as (u16),
        ],
        [
            InstructionOperation::INVALID as (u16),
            InstructionOperation::INVALID as (u16),
        ],
    ],
    [
        [
            InstructionOperation::INVALID as (u16),
            InstructionOperation::INVALID as (u16),
        ],
        [
            InstructionOperation::INVALID as (u16),
            InstructionOperation::INVALID as (u16),
        ],
        [
            InstructionOperation::PSRLQ as (u16),
            InstructionOperation::PSRLQ as (u16),
        ],
        [
            InstructionOperation::INVALID as (u16),
            InstructionOperation::PSRLDQ as (u16),
        ],
        [
            InstructionOperation::INVALID as (u16),
            InstructionOperation::INVALID as (u16),
        ],
        [
            InstructionOperation::INVALID as (u16),
            InstructionOperation::INVALID as (u16),
        ],
        [
            InstructionOperation::PSLLQ as (u16),
            InstructionOperation::PSLLQ as (u16),
        ],
        [
            InstructionOperation::INVALID as (u16),
            InstructionOperation::PSLLDQ as (u16),
        ],
    ],
];

#[derive(Copy)]
#[repr(C)]
pub struct SSETableOperationEntry {
    pub operation: u16,
    pub regType: u8,
    pub rmType: u8,
}

impl Clone for SSETableOperationEntry {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct SSETableEntry {
    pub regOps: [SSETableOperationEntry; 4],
    pub memOps: [SSETableOperationEntry; 4],
}

impl Clone for SSETableEntry {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
#[repr(i32)]
pub enum SSETableOperandType {
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
                operation: InstructionOperation::MOVUPS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVUPD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVSD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVSS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVUPS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVUPD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVSD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVSS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_32 as (u8),
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVHLPS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVDDUP as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVSLDUP as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVLPS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVLPD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVDDUP as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVSLDUP as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVLPS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVLPD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::UNPCKLPS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::UNPCKLPD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::UNPCKLPS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::UNPCKLPD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::UNPCKHPS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::UNPCKHPD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::UNPCKHPS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::UNPCKHPD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVLHPS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVSHDUP as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVHPS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVHPD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVSHDUP as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVHPS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVHPD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVAPS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVAPD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVAPS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVAPD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPI2PS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::MMX_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPI2PD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::MMX_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTSI2SD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::GPR_32_OR_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTSI2SS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::GPR_32_OR_64 as (u8),
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPI2PS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::MMX_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPI2PD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::MMX_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTSI2SD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::GPR_32_OR_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTSI2SS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::GPR_32_OR_64 as (u8),
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVNTPS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVNTPD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::CVTTPS2PI as (u16),
                regType: SSETableOperandType::MMX_64 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTTPD2PI as (u16),
                regType: SSETableOperandType::MMX_64 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTTSD2SI as (u16),
                regType: SSETableOperandType::GPR_32_OR_64 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTTSS2SI as (u16),
                regType: SSETableOperandType::GPR_32_OR_64 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::CVTTPS2PI as (u16),
                regType: SSETableOperandType::MMX_64 as (u8),
                rmType: SSETableOperandType::SSE_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTTPD2PI as (u16),
                regType: SSETableOperandType::MMX_64 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTTSD2SI as (u16),
                regType: SSETableOperandType::GPR_32_OR_64 as (u8),
                rmType: SSETableOperandType::SSE_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTTSS2SI as (u16),
                regType: SSETableOperandType::GPR_32_OR_64 as (u8),
                rmType: SSETableOperandType::SSE_32 as (u8),
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPS2PI as (u16),
                regType: SSETableOperandType::MMX_64 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPD2PI as (u16),
                regType: SSETableOperandType::MMX_64 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTSD2SI as (u16),
                regType: SSETableOperandType::GPR_32_OR_64 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTSS2SI as (u16),
                regType: SSETableOperandType::GPR_32_OR_64 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPS2PI as (u16),
                regType: SSETableOperandType::MMX_64 as (u8),
                rmType: SSETableOperandType::SSE_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPD2PI as (u16),
                regType: SSETableOperandType::MMX_64 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTSD2SI as (u16),
                regType: SSETableOperandType::GPR_32_OR_64 as (u8),
                rmType: SSETableOperandType::SSE_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTSS2SI as (u16),
                regType: SSETableOperandType::GPR_32_OR_64 as (u8),
                rmType: SSETableOperandType::SSE_32 as (u8),
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::UCOMISS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::UCOMISD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::UCOMISS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_32 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::UCOMISD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::COMISS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::COMISD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::COMISS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_32 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::COMISD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVMSKPS as (u16),
                regType: SSETableOperandType::GPR_32_OR_64 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVMSKPD as (u16),
                regType: SSETableOperandType::GPR_32_OR_64 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPS2PD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPD2PS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTSD2SS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTSS2SD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPS2PD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPD2PS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTSD2SS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTSS2SD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_32 as (u8),
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::CVTDQ2PS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPS2DQ as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTTPS2DQ as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::CVTDQ2PS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPS2DQ as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTTPS2DQ as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::PUNPCKLBW as (u16),
                regType: SSETableOperandType::MMX_64 as (u8),
                rmType: SSETableOperandType::MMX_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PUNPCKLBW as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::PUNPCKLBW as (u16),
                regType: SSETableOperandType::MMX_64 as (u8),
                rmType: SSETableOperandType::MMX_32 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PUNPCKLBW as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::PUNPCKLWD as (u16),
                regType: SSETableOperandType::MMX_64 as (u8),
                rmType: SSETableOperandType::MMX_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PUNPCKLWD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::PUNPCKLWD as (u16),
                regType: SSETableOperandType::MMX_64 as (u8),
                rmType: SSETableOperandType::MMX_32 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PUNPCKLWD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::PUNPCKLDQ as (u16),
                regType: SSETableOperandType::MMX_64 as (u8),
                rmType: SSETableOperandType::MMX_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PUNPCKLDQ as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::PUNPCKLDQ as (u16),
                regType: SSETableOperandType::MMX_64 as (u8),
                rmType: SSETableOperandType::MMX_32 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PUNPCKLDQ as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVD as (u16),
                regType: SSETableOperandType::MMX_64 as (u8),
                rmType: SSETableOperandType::GPR_32_OR_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::GPR_32_OR_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVD as (u16),
                regType: SSETableOperandType::MMX_64 as (u8),
                rmType: SSETableOperandType::GPR_32_OR_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::GPR_32_OR_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVQ as (u16),
                regType: SSETableOperandType::MMX_64 as (u8),
                rmType: SSETableOperandType::MMX_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVDQA as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVDQU as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVQ as (u16),
                regType: SSETableOperandType::MMX_64 as (u8),
                rmType: SSETableOperandType::MMX_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVDQA as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVDQU as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::PSHUFW as (u16),
                regType: SSETableOperandType::MMX_64 as (u8),
                rmType: SSETableOperandType::MMX_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PSHUFD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PSHUFLW as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PSHUFHW as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::PSHUFW as (u16),
                regType: SSETableOperandType::MMX_64 as (u8),
                rmType: SSETableOperandType::MMX_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PSHUFD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PSHUFLW as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PSHUFHW as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::HADDPD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::HADDPS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::HADDPD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::HADDPS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::HSUBPD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::HSUBPS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::HSUBPD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::HSUBPS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVD as (u16),
                regType: SSETableOperandType::MMX_64 as (u8),
                rmType: SSETableOperandType::GPR_32_OR_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::GPR_32_OR_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVQ as (u16),
                regType: SSETableOperandType::SSE_128_FLIP as (u8),
                rmType: SSETableOperandType::SSE_128_FLIP as (u8),
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVD as (u16),
                regType: SSETableOperandType::MMX_64 as (u8),
                rmType: SSETableOperandType::GPR_32_OR_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::GPR_32_OR_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVQ as (u16),
                regType: SSETableOperandType::SSE_128_FLIP as (u8),
                rmType: SSETableOperandType::SSE_128_FLIP as (u8),
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::CMPPS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CMPPD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CMPSD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CMPSS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::CMPPS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CMPPD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CMPSD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CMPSS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_32 as (u8),
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::PINSRW as (u16),
                regType: SSETableOperandType::MMX_64 as (u8),
                rmType: SSETableOperandType::GPR_32_OR_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PINSRW as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::GPR_32_OR_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::PINSRW as (u16),
                regType: SSETableOperandType::MMX_64 as (u8),
                rmType: SSETableOperandType::GPR_32_OR_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PINSRW as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::GPR_32_OR_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::PEXTRW as (u16),
                regType: SSETableOperandType::MMX_64 as (u8),
                rmType: SSETableOperandType::GPR_32_OR_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PEXTRW as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::GPR_32_OR_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::PEXTRW as (u16),
                regType: SSETableOperandType::MMX_64 as (u8),
                rmType: SSETableOperandType::GPR_32_OR_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PEXTRW as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::GPR_32_OR_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::SHUFPS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::SHUFPD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::SHUFPS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::SHUFPD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::ADDSUBPD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::ADDSUBPS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::ADDSUBPD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::ADDSUBPS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVQ as (u16),
                regType: SSETableOperandType::SSE_128_FLIP as (u8),
                rmType: SSETableOperandType::SSE_128_FLIP as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVDQ2Q as (u16),
                regType: SSETableOperandType::MMX_64 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVQ2DQ as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::MMX_64 as (u8),
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVQ as (u16),
                regType: SSETableOperandType::SSE_128_FLIP as (u8),
                rmType: SSETableOperandType::SSE_128_FLIP as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVMSKB as (u16),
                regType: SSETableOperandType::GPR_32_OR_64 as (u8),
                rmType: SSETableOperandType::MMX_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVMSKB as (u16),
                regType: SSETableOperandType::GPR_32_OR_64 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTTPD2DQ as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPD2DQ as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTDQ2PD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTTPD2DQ as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTPD2DQ as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::CVTDQ2PD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::MOVNTQ as (u16),
                regType: SSETableOperandType::MMX_64 as (u8),
                rmType: SSETableOperandType::MMX_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVNTDQ as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::LDDQU as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::MASKMOVQ as (u16),
                regType: SSETableOperandType::MMX_64 as (u8),
                rmType: SSETableOperandType::MMX_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MASKMOVDQU as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVSXBW as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVSXBW as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVSXBD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVSXBD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_32 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVSXBQ as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVSXBQ as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_16 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVSXWD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVSXWD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVSXWQ as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVSXWQ as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_32 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVSXDQ as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVSXDQ as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::MOVNTDQA as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVZXBW as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVZXBW as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVZXBD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVZXBD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_32 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVZXBQ as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVZXBQ as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_16 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVZXWD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVZXWD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVZXWQ as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVZXWQ as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_32 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVZXDQ as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PMOVZXDQ as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::ROUNDSS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::ROUNDSS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_32 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::ROUNDSD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::ROUNDSD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PEXTRB as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::GPR_32_OR_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PEXTRB as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::GPR_32_OR_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PEXTRW as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::GPR_32_OR_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PEXTRW as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_16 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PEXTRD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::GPR_32_OR_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PEXTRD as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::GPR_32_OR_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::EXTRACTPS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::GPR_32_OR_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::EXTRACTPS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_32 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PINSRB as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::GPR_32_OR_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::PINSRB as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::GPR_32_OR_64 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
    SSETableEntry {
        regOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INSERTPS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_128 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
        memOps: [
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INSERTPS as (u16),
                regType: SSETableOperandType::SSE_128 as (u8),
                rmType: SSETableOperandType::SSE_32 as (u8),
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
            SSETableOperationEntry {
                operation: InstructionOperation::INVALID as (u16),
                regType: 0u8,
                rmType: 0u8,
            },
        ],
    },
];

#[derive(Copy)]
#[repr(C)]
pub struct SparseOpEntry {
    pub opcode: u8,
    pub operation: u16,
}

impl Clone for SparseOpEntry {
    fn clone(&self) -> Self {
        *self
    }
}

static SPARSE_3DNOW_OPCODES: [SparseOpEntry; 26] = [
    SparseOpEntry {
        opcode: 0xcu8,
        operation: InstructionOperation::PI2FW as (u16),
    },
    SparseOpEntry {
        opcode: 0xdu8,
        operation: InstructionOperation::PI2FD as (u16),
    },
    SparseOpEntry {
        opcode: 0x1cu8,
        operation: InstructionOperation::PF2IW as (u16),
    },
    SparseOpEntry {
        opcode: 0x1du8,
        operation: InstructionOperation::PF2ID as (u16),
    },
    SparseOpEntry {
        opcode: 0x86u8,
        operation: InstructionOperation::PFRCPV as (u16),
    },
    SparseOpEntry {
        opcode: 0x87u8,
        operation: InstructionOperation::PFRSQRTV as (u16),
    },
    SparseOpEntry {
        opcode: 0x8au8,
        operation: InstructionOperation::PFNACC as (u16),
    },
    SparseOpEntry {
        opcode: 0x8eu8,
        operation: InstructionOperation::PFPNACC as (u16),
    },
    SparseOpEntry {
        opcode: 0x90u8,
        operation: InstructionOperation::PFCMPGE as (u16),
    },
    SparseOpEntry {
        opcode: 0x94u8,
        operation: InstructionOperation::PFMIN as (u16),
    },
    SparseOpEntry {
        opcode: 0x96u8,
        operation: InstructionOperation::PFRCP as (u16),
    },
    SparseOpEntry {
        opcode: 0x97u8,
        operation: InstructionOperation::PFRSQRT as (u16),
    },
    SparseOpEntry {
        opcode: 0x9au8,
        operation: InstructionOperation::PFSUB as (u16),
    },
    SparseOpEntry {
        opcode: 0x9eu8,
        operation: InstructionOperation::PFADD as (u16),
    },
    SparseOpEntry {
        opcode: 0xa0u8,
        operation: InstructionOperation::PFCMPGT as (u16),
    },
    SparseOpEntry {
        opcode: 0xa4u8,
        operation: InstructionOperation::PFMAX as (u16),
    },
    SparseOpEntry {
        opcode: 0xa6u8,
        operation: InstructionOperation::PFRCPIT1 as (u16),
    },
    SparseOpEntry {
        opcode: 0xa7u8,
        operation: InstructionOperation::PFRSQIT1 as (u16),
    },
    SparseOpEntry {
        opcode: 0xaau8,
        operation: InstructionOperation::PFSUBR as (u16),
    },
    SparseOpEntry {
        opcode: 0xaeu8,
        operation: InstructionOperation::PFACC as (u16),
    },
    SparseOpEntry {
        opcode: 0xb0u8,
        operation: InstructionOperation::PFCMPEQ as (u16),
    },
    SparseOpEntry {
        opcode: 0xb4u8,
        operation: InstructionOperation::PFMUL as (u16),
    },
    SparseOpEntry {
        opcode: 0xb6u8,
        operation: InstructionOperation::PFRCPIT2 as (u16),
    },
    SparseOpEntry {
        opcode: 0xb7u8,
        operation: InstructionOperation::PMULHRW as (u16),
    },
    SparseOpEntry {
        opcode: 0xbbu8,
        operation: InstructionOperation::PSWAPD as (u16),
    },
    SparseOpEntry {
        opcode: 0xbfu8,
        operation: InstructionOperation::PAVGUSB as (u16),
    },
];

static REG8_LIST: [u8; 8] = [
    OperandType::REG_AL as (u8),
    OperandType::REG_CL as (u8),
    OperandType::REG_DL as (u8),
    OperandType::REG_BL as (u8),
    OperandType::REG_AH as (u8),
    OperandType::REG_CH as (u8),
    OperandType::REG_DH as (u8),
    OperandType::REG_BH as (u8),
];

static REG8_LIST64: [u8; 16] = [
    OperandType::REG_AL as (u8),
    OperandType::REG_CL as (u8),
    OperandType::REG_DL as (u8),
    OperandType::REG_BL as (u8),
    OperandType::REG_SPL as (u8),
    OperandType::REG_BPL as (u8),
    OperandType::REG_SIL as (u8),
    OperandType::REG_DIL as (u8),
    OperandType::REG_R8B as (u8),
    OperandType::REG_R9B as (u8),
    OperandType::REG_R10B as (u8),
    OperandType::REG_R11B as (u8),
    OperandType::REG_R12B as (u8),
    OperandType::REG_R13B as (u8),
    OperandType::REG_R14B as (u8),
    OperandType::REG_R15B as (u8),
];

static REG16_LIST: [u8; 16] = [
    OperandType::REG_AX as (u8),
    OperandType::REG_CX as (u8),
    OperandType::REG_DX as (u8),
    OperandType::REG_BX as (u8),
    OperandType::REG_SP as (u8),
    OperandType::REG_BP as (u8),
    OperandType::REG_SI as (u8),
    OperandType::REG_DI as (u8),
    OperandType::REG_R8W as (u8),
    OperandType::REG_R9W as (u8),
    OperandType::REG_R10W as (u8),
    OperandType::REG_R11W as (u8),
    OperandType::REG_R12W as (u8),
    OperandType::REG_R13W as (u8),
    OperandType::REG_R14W as (u8),
    OperandType::REG_R15W as (u8),
];

static REG32_LIST: [u8; 16] = [
    OperandType::REG_EAX as (u8),
    OperandType::REG_ECX as (u8),
    OperandType::REG_EDX as (u8),
    OperandType::REG_EBX as (u8),
    OperandType::REG_ESP as (u8),
    OperandType::REG_EBP as (u8),
    OperandType::REG_ESI as (u8),
    OperandType::REG_EDI as (u8),
    OperandType::REG_R8D as (u8),
    OperandType::REG_R9D as (u8),
    OperandType::REG_R10D as (u8),
    OperandType::REG_R11D as (u8),
    OperandType::REG_R12D as (u8),
    OperandType::REG_R13D as (u8),
    OperandType::REG_R14D as (u8),
    OperandType::REG_R15D as (u8),
];

static REG64_LIST: [u8; 16] = [
    OperandType::REG_RAX as (u8),
    OperandType::REG_RCX as (u8),
    OperandType::REG_RDX as (u8),
    OperandType::REG_RBX as (u8),
    OperandType::REG_RSP as (u8),
    OperandType::REG_RBP as (u8),
    OperandType::REG_RSI as (u8),
    OperandType::REG_RDI as (u8),
    OperandType::REG_R8 as (u8),
    OperandType::REG_R9 as (u8),
    OperandType::REG_R10 as (u8),
    OperandType::REG_R11 as (u8),
    OperandType::REG_R12 as (u8),
    OperandType::REG_R13 as (u8),
    OperandType::REG_R14 as (u8),
    OperandType::REG_R15 as (u8),
];

static MMX_REG_LIST: [u8; 16] = [
    OperandType::REG_MM0 as (u8),
    OperandType::REG_MM1 as (u8),
    OperandType::REG_MM2 as (u8),
    OperandType::REG_MM3 as (u8),
    OperandType::REG_MM4 as (u8),
    OperandType::REG_MM5 as (u8),
    OperandType::REG_MM6 as (u8),
    OperandType::REG_MM7 as (u8),
    OperandType::REG_MM0 as (u8),
    OperandType::REG_MM1 as (u8),
    OperandType::REG_MM2 as (u8),
    OperandType::REG_MM3 as (u8),
    OperandType::REG_MM4 as (u8),
    OperandType::REG_MM5 as (u8),
    OperandType::REG_MM6 as (u8),
    OperandType::REG_MM7 as (u8),
];

static XMM_REG_LIST: [u8; 16] = [
    OperandType::REG_XMM0 as (u8),
    OperandType::REG_XMM1 as (u8),
    OperandType::REG_XMM2 as (u8),
    OperandType::REG_XMM3 as (u8),
    OperandType::REG_XMM4 as (u8),
    OperandType::REG_XMM5 as (u8),
    OperandType::REG_XMM6 as (u8),
    OperandType::REG_XMM7 as (u8),
    OperandType::REG_XMM8 as (u8),
    OperandType::REG_XMM9 as (u8),
    OperandType::REG_XMM10 as (u8),
    OperandType::REG_XMM11 as (u8),
    OperandType::REG_XMM12 as (u8),
    OperandType::REG_XMM13 as (u8),
    OperandType::REG_XMM14 as (u8),
    OperandType::REG_XMM15 as (u8),
];

static FPU_REG_LIST: [u8; 16] = [
    OperandType::REG_ST0 as (u8),
    OperandType::REG_ST1 as (u8),
    OperandType::REG_ST2 as (u8),
    OperandType::REG_ST3 as (u8),
    OperandType::REG_ST4 as (u8),
    OperandType::REG_ST5 as (u8),
    OperandType::REG_ST6 as (u8),
    OperandType::REG_ST7 as (u8),
    OperandType::REG_ST0 as (u8),
    OperandType::REG_ST1 as (u8),
    OperandType::REG_ST2 as (u8),
    OperandType::REG_ST3 as (u8),
    OperandType::REG_ST4 as (u8),
    OperandType::REG_ST5 as (u8),
    OperandType::REG_ST6 as (u8),
    OperandType::REG_ST7 as (u8),
];

unsafe extern "C" fn InvalidDecode(state: *mut DecodeState) {
    (*state).invalid = true;
}

unsafe extern "C" fn Read8(state: *mut DecodeState) -> u8 {
    let val: u8;
    if (*state).len < 1usize {
        (*state).invalid = true;
        (*state).insufficientLength = true;
        (*state).len = 0usize;
        0xccu8
    } else {
        val = *{
            let _old = (*state).opcode;
            (*state).opcode = (*state).opcode.offset(1isize);
            _old
        };
        (*state).len = (*state).len.wrapping_sub(1usize);
        val
    }
}

unsafe extern "C" fn GetFinalOpSize(state: *mut DecodeState) -> u16 {
    if (*state).flags & 0x100u32 != 0 {
        1u16
    } else {
        (*state).opSize
    }
}

unsafe extern "C" fn ProcessEncoding(
    state: *mut DecodeState,
    encoding: *const InstructionEncoding,
) {
    (*(*state).result).operation = InstructionOperation::from_i32((*encoding).operation as (i32));
    (*state).flags = (*encoding).flags as (u32);
    if (*state).using64 && ((*state).flags & 0x4000u32 != 0) {
        (*state).invalid = true;
    } else {
        if (*state).using64 && ((*state).flags & 0x8000u32 != 0) {
            (*state).opSize = if (*state).opPrefix { 4i32 } else { 8i32 } as (u16);
        }
        (*state).finalOpSize = GetFinalOpSize(state);
        if (*state).flags & 0x200u32 != 0 {
            (*state).operand0 = &mut (*(*state).result).operands[1usize] as
                (*mut InstructionOperand);
            (*state).operand1 = &mut (*(*state).result).operands[0usize] as
                (*mut InstructionOperand);
        } else {
            (*state).operand0 = &mut (*(*state).result).operands[0usize] as
                (*mut InstructionOperand);
            (*state).operand1 = &mut (*(*state).result).operands[1usize] as
                (*mut InstructionOperand);
        }
        if (*state).flags & 0x2000u32 != 0 {
            (*state).finalOpSize = 2u16;
        }
        if (*state).flags & 0x1000u32 != 0 {
            if (*state).finalOpSize as (i32) == 4i32 {
                (*(*state).result).operation = InstructionOperation::from_i32(
                    ((*(*state).result).operation as (i32) + 1i32) as
                        (i32),
                );
            } else if (*state).finalOpSize as (i32) == 8i32 {
                (*(*state).result).operation = InstructionOperation::from_i32(
                    ((*(*state).result).operation as (i32) + 2i32) as
                        (i32),
                );
            }
        }
        if (*state).flags & 0x40u32 != 0 {
            if (*state).rep as (i32) != RepPrefix::REP_PREFIX_NONE as (i32) {
                (*(*state).result).flags = (*(*state).result).flags | 2u32;
            }
        } else if (*state).flags & 0x80u32 != 0 {
            if (*state).rep as (i32) == RepPrefix::REP_PREFIX_REPNE as (i32) {
                (*(*state).result).flags = (*(*state).result).flags | 4u32;
            } else if (*state).rep as (i32) == RepPrefix::REP_PREFIX_REPE as (i32) {
                (*(*state).result).flags = (*(*state).result).flags | 8u32;
            }
        }
        ((*encoding).func)(state);
        if (*(*state).result).operation as (i32) == InstructionOperation::INVALID as (i32) {
            (*state).invalid = true;
        }
        if (*(*state).result).flags & 1u32 != 0 {
            if (*state).flags & 0x20u32 == 0 {
                (*state).invalid = true;
            } else if (*(*state).result).operation as (i32) == InstructionOperation::CMP as (i32) {
                (*state).invalid = true;
            } else if (*(*state).result).operands[0usize].operand as (i32) !=
                       OperandType::MEM as (i32) &&
                       ((*(*state).result).operands[1usize].operand as (i32) !=
                            OperandType::MEM as (i32))
            {
                (*state).invalid = true;
            }
        }
    }
}

unsafe extern "C" fn ProcessOpcode(
    state: *mut DecodeState,
    map: *const InstructionEncoding,
    opcode: u8,
) {
    ProcessEncoding(
        state,
        &*map.offset(opcode as (isize)) as (*const InstructionEncoding),
    );
}

unsafe extern "C" fn ProcessSparseOpcode(
    state: *mut DecodeState,
    map: *const SparseInstructionEncoding,
    mapSize: usize,
    opcode: u8,
) {
    let mut _currentBlock;
    let mut i: i32;
    let mut min: i32;
    let mut max: i32;
    (*(*state).result).operation = InstructionOperation::INVALID;
    min = 0i32;
    max = mapSize as (i32) - 1i32;
    i = (min + max) / 2i32;
    'loop1: loop {
        if !(min <= max) {
            _currentBlock = 5;
            break;
        }
        if opcode as (i32) > (*map.offset(i as (isize))).opcode as (i32) {
            min = i + 1i32;
        } else {
            if !(opcode as (i32) < (*map.offset(i as (isize))).opcode as (i32)) {
                _currentBlock = 4;
                break;
            }
            max = i - 1i32;
        }
        i = (min + max) / 2i32;
    }
    if _currentBlock == 4 {
        ProcessEncoding(
            state,
            &(*map.offset(i as (isize))).encoding as (*const InstructionEncoding),
        );
    }
}

unsafe extern "C" fn SetOperandToImm8(state: *mut DecodeState, oper: *mut InstructionOperand) {
    (*oper).operand = OperandType::IMM;
    (*oper).size = 1u16;
    (*oper).immediate = Read8(state) as (isize);
}

unsafe extern "C" fn DecodeTwoByte(state: *mut DecodeState) {
    let opcode: u8 = Read8(state);
    if opcode as (i32) == 0x38i32 {
        ProcessSparseOpcode(
            state,
            THREE_BYTE_0F38_MAP.as_ptr(),
            ::std::mem::size_of::<[SparseInstructionEncoding; 48]>()
                .wrapping_div(::std::mem::size_of::<SparseInstructionEncoding>()),
            Read8(state),
        );
    } else if opcode as (i32) == 0x3ai32 {
        ProcessSparseOpcode(
            state,
            THREE_BYTE_0F3A_MAP.as_ptr(),
            ::std::mem::size_of::<[SparseInstructionEncoding; 22]>()
                .wrapping_div(::std::mem::size_of::<SparseInstructionEncoding>()),
            Read8(state),
        );
        SetOperandToImm8(
            state,
            &mut (*(*state).result).operands[2usize] as (*mut InstructionOperand),
        );
    } else {
        ProcessOpcode(state, TWO_BYTE_OPCODE_MAP.as_ptr(), opcode);
    }
}

unsafe extern "C" fn Peek8(state: *mut DecodeState) -> u8 {
    if (*state).len < 1usize {
        (*state).invalid = true;
        (*state).insufficientLength = true;
        (*state).len = 0usize;
        0xccu8
    } else {
        *(*state).opcode
    }
}

unsafe extern "C" fn DecodeFpu(state: *mut DecodeState) {
    let modRM: u8 = Peek8(state);
    let reg: u8 = (modRM as (i32) >> 3i32 & 7i32) as (u8);
    let op: u8 = (*(*state).result).operation as (u8);
    let map: *const InstructionEncoding;
    if modRM as (i32) & 0xc0i32 == 0xc0i32 {
        map = FPU_REG_OPCODE_MAP[op as (usize)].as_ptr();
    } else {
        map = FPU_MEM_OPCODE_MAP[op as (usize)].as_ptr();
    }
    ProcessEncoding(
        state,
        &*map.offset(reg as (isize)) as (*const InstructionEncoding),
    );
}

unsafe extern "C" fn DecodeNoOperands(_state: *mut DecodeState) {}

unsafe extern "C" fn GetByteRegList(state: *mut DecodeState) -> *const u8 {
    if (*state).rex {
        REG8_LIST64.as_ptr()
    } else {
        REG8_LIST.as_ptr()
    }
}

unsafe extern "C" fn GetRegListForFinalOpSize(state: *mut DecodeState) -> *const u8 {
    let switch1 = (*state).finalOpSize;
    if switch1 as (i32) == 8i32 {
        REG64_LIST.as_ptr()
    } else if switch1 as (i32) == 4i32 {
        REG32_LIST.as_ptr()
    } else if switch1 as (i32) == 2i32 {
        REG16_LIST.as_ptr()
    } else if switch1 as (i32) == 1i32 {
        GetByteRegList(state)
    } else {
        0i32 as (*mut ::std::os::raw::c_void) as (*const u8)
    }
}

unsafe extern "C" fn GetRegListForAddrSize(state: *mut DecodeState) -> *const u8 {
    let switch3 = (*state).addrSize;
    if switch3 as (i32) == 8i32 {
        REG64_LIST.as_ptr()
    } else if switch3 as (i32) == 4i32 {
        REG32_LIST.as_ptr()
    } else if switch3 as (i32) == 2i32 {
        REG16_LIST.as_ptr()
    } else {
        0i32 as (*mut ::std::os::raw::c_void) as (*const u8)
    }
}

unsafe extern "C" fn Read32(state: *mut DecodeState) -> u32 {
    let val: u32;
    if (*state).len < 4usize {
        (*state).invalid = true;
        (*state).insufficientLength = true;
        (*state).len = 0usize;
        0u32
    } else {
        val = *((*state).opcode as (*mut u32));
        (*state).opcode = (*state).opcode.offset(4isize);
        (*state).len = (*state).len.wrapping_sub(4usize);
        val
    }
}

unsafe extern "C" fn ReadSigned32(state: *mut DecodeState) -> isize {
    Read32(state) as (i32) as (isize)
}

unsafe extern "C" fn ReadSigned8(state: *mut DecodeState) -> isize {
    Read8(state) as (i8) as (isize)
}

unsafe extern "C" fn GetFinalSegment(
    state: *mut DecodeState,
    seg: SegmentRegister,
) -> SegmentRegister {
    if (*(*state).result).segment == SegmentRegister::SEG_DEFAULT {
        seg
    } else {
        (*(*state).result).segment
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct RMDef {
    pub first: OperandType,
    pub second: OperandType,
    pub segment: SegmentRegister,
}

impl Clone for RMDef {
    fn clone(&self) -> Self {
        *self
    }
}

unsafe extern "C" fn SetMemOperand(
    state: *mut DecodeState,
    oper: *mut InstructionOperand,
    def: *const RMDef,
    immed: isize,
) {
    (*oper).operand = OperandType::MEM;
    (*oper).components[0usize] = (*def).first;
    (*oper).components[1usize] = (*def).second;
    (*oper).immediate = immed;
    (*oper).segment = GetFinalSegment(state, (*def).segment);
}

unsafe extern "C" fn Read16(state: *mut DecodeState) -> u16 {
    let val: u16;
    if (*state).len < 2usize {
        (*state).invalid = true;
        (*state).insufficientLength = true;
        (*state).len = 0usize;
        0u16
    } else {
        val = *((*state).opcode as (*mut u16));
        (*state).opcode = (*state).opcode.offset(2isize);
        (*state).len = (*state).len.wrapping_sub(2usize);
        val
    }
}

unsafe extern "C" fn ReadSigned16(state: *mut DecodeState) -> isize {
    Read16(state) as (i16) as (isize)
}

unsafe extern "C" fn DecodeRM(
    state: *mut DecodeState,
    mut rmOper: *mut InstructionOperand,
    regList: *const u8,
    rmSize: u16,
    regOper: *mut u8,
) {
    let rmByte: u8 = Read8(state);
    let mod_: u8 = (rmByte as (i32) >> 6i32) as (u8);
    let mut rm: u8 = (rmByte as (i32) & 7i32) as (u8);
    let mut temp = InstructionOperand::default();
    if !regOper.is_null() {
        *regOper = (rmByte as (i32) >> 3i32 & 7i32) as (u8);
    }
    if rmOper.is_null() {
        rmOper = &mut temp as (*mut InstructionOperand);
    }
    (*rmOper).size = rmSize;
    if (*state).addrSize as (i32) == 2i32 {
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
        if mod_ as (i32) == 3i32 {
            (*rmOper).operand = OperandType::from_i32(*regList.offset(rm as (isize)) as (i32));
        } else if mod_ as (i32) == 2i32 {
            SetMemOperand(
                state,
                rmOper,
                &RM16_COMPONENTS[rm as (usize)] as (*const RMDef),
                ReadSigned16(state),
            );
        } else if mod_ as (i32) == 1i32 {
            SetMemOperand(
                state,
                rmOper,
                &RM16_COMPONENTS[rm as (usize)] as (*const RMDef),
                ReadSigned8(state),
            );
        } else if mod_ as (i32) == 0i32 {
            if rm as (i32) == 6i32 {
                rm = 8u8;
                SetMemOperand(
                    state,
                    rmOper,
                    &RM16_COMPONENTS[rm as (usize)] as (*const RMDef),
                    Read16(state) as (isize),
                );
            } else {
                SetMemOperand(
                    state,
                    rmOper,
                    &RM16_COMPONENTS[rm as (usize)] as (*const RMDef),
                    0isize,
                );
            }
        }
        if (*rmOper).components[0usize] as (i32) == OperandType::NONE as (i32) {
            (*rmOper).immediate = (*rmOper).immediate & 0xffffisize;
        }
    } else {
        let addrRegList: *const u8 = GetRegListForAddrSize(state);
        let rmReg1Offset: u8 = (if (*state).rexRM1 { 8i32 } else { 0i32 }) as (u8);
        let rmReg2Offset: u8 = (if (*state).rexRM2 { 8i32 } else { 0i32 }) as (u8);
        let mut seg: SegmentRegister = SegmentRegister::SEG_DEFAULT;
        (*rmOper).operand = OperandType::MEM;
        if mod_ as (i32) != 3i32 && (rm as (i32) == 4i32) {
            let sibByte: u8 = Read8(state);
            let base: u8 = (sibByte as (i32) & 7i32) as (u8);
            let index: u8 = (sibByte as (i32) >> 3i32 & 7i32) as (u8);
            (*rmOper).scale = (1i32 << (sibByte as (i32) >> 6i32)) as (u8);
            if mod_ as (i32) != 0i32 || base as (i32) != 5i32 {
                (*rmOper).components[0usize] = OperandType::from_i32(*addrRegList.offset(
                    (base as (i32) + rmReg1Offset as (i32)) as
                        (isize),
                ) as (i32));
            }
            if index as (i32) + rmReg2Offset as (i32) != 4i32 {
                (*rmOper).components[1usize] = OperandType::from_i32(*addrRegList.offset(
                    (index as (i32) + rmReg2Offset as (i32)) as
                        (isize),
                ) as (i32));
            }
            if mod_ as (i32) == 2i32 {
                (*rmOper).immediate = ReadSigned32(state);
            } else if mod_ as (i32) == 1i32 {
                (*rmOper).immediate = ReadSigned8(state);
            } else if mod_ as (i32) == 0i32 {
                if base as (i32) == 5i32 {
                    (*rmOper).immediate = ReadSigned32(state);
                }
            }
            if base as (i32) + rmReg1Offset as (i32) == 4i32 ||
                base as (i32) + rmReg1Offset as (i32) == 5i32
            {
                seg = SegmentRegister::SEG_SS;
            } else {
                seg = SegmentRegister::SEG_DS;
            }
        } else if mod_ as (i32) == 3i32 {
            (*rmOper).operand = OperandType::from_i32(*regList.offset(
                (rm as (i32) + rmReg1Offset as (i32)) as (isize),
            ) as (i32));
        } else if mod_ as (i32) == 2i32 {
            (*rmOper).components[0usize] = OperandType::from_i32(*addrRegList.offset(
                (rm as (i32) + rmReg1Offset as (i32)) as
                    (isize),
            ) as (i32));
            (*rmOper).immediate = ReadSigned32(state);
            seg = if rm as (i32) == 5i32 {
                SegmentRegister::SEG_SS
            } else {
                SegmentRegister::SEG_DS
            };
        } else if mod_ as (i32) == 1i32 {
            (*rmOper).components[0usize] = OperandType::from_i32(*addrRegList.offset(
                (rm as (i32) + rmReg1Offset as (i32)) as
                    (isize),
            ) as (i32));
            (*rmOper).immediate = ReadSigned8(state);
            seg = if rm as (i32) == 5i32 {
                SegmentRegister::SEG_SS
            } else {
                SegmentRegister::SEG_DS
            };
        } else if mod_ as (i32) == 0i32 {
            if rm as (i32) == 5i32 {
                (*rmOper).immediate = ReadSigned32(state);
                if (*state).addrSize as (i32) == 8i32 {
                    (*state).ripRelFixup = &mut (*rmOper).immediate as (*mut isize);
                }
            } else {
                (*rmOper).components[0usize] = OperandType::from_i32(*addrRegList.offset(
                    (rm as (i32) + rmReg1Offset as (i32)) as
                        (isize),
                ) as (i32));
            }
            seg = SegmentRegister::SEG_DS;
        }
        if seg as (i32) != SegmentRegister::SEG_DEFAULT as (i32) {
            (*rmOper).segment = GetFinalSegment(state, seg);
        }
    }
}

unsafe extern "C" fn DecodeRMReg(
    state: *mut DecodeState,
    rmOper: *mut InstructionOperand,
    rmRegList: *const u8,
    rmSize: u16,
    regOper: *mut InstructionOperand,
    regList: *const u8,
    regSize: u16,
) {
    let mut reg: u8 = 0;
    DecodeRM(state, rmOper, rmRegList, rmSize, &mut reg as (*mut u8));
    if !regOper.is_null() {
        let regOffset: u8 = (if (*state).rexReg { 8i32 } else { 0i32 }) as (u8);
        (*regOper).size = regSize;
        (*regOper).operand = OperandType::from_i32(*regList.offset(
            (reg as (i32) + regOffset as (i32)) as (isize),
        ) as (i32));
    }
}

unsafe extern "C" fn DecodeRegRM(state: *mut DecodeState) {
    let mut size: u16 = (*state).finalOpSize;
    let regList: *const u8 = GetRegListForFinalOpSize(state);
    let switch2 = (*state).flags & 0x3u32;
    if !(switch2 == 0u32) {
        if switch2 == 0x3u32 {
            size = 0u16;
        } else if switch2 == 0x2u32 {
            size = (size as (i32) + 2i32) as (u16);
        } else if switch2 == 0x1u32 {
            size = (size as (i32) * 2i32) as (u16);
        }
    }
    DecodeRMReg(
        state,
        (*state).operand1,
        regList,
        size,
        (*state).operand0,
        regList,
        (*state).finalOpSize,
    );
    if size as (i32) != (*state).finalOpSize as (i32) &&
        ((*(*state).operand1).operand as (i32) != OperandType::MEM as (i32))
    {
        (*state).invalid = true;
    }
}

unsafe extern "C" fn ReadFinalOpSize(state: *mut DecodeState) -> isize {
    if (*state).flags & 0x400u32 != 0 {
        ReadSigned8(state)
    } else {
        let switch4 = (*state).finalOpSize;
        (if switch4 as (i32) == 8i32 {
             ReadSigned32(state)
         } else if switch4 as (i32) == 4i32 {
             Read32(state) as (isize)
         } else if switch4 as (i32) == 2i32 {
             Read16(state) as (isize)
         } else if switch4 as (i32) == 1i32 {
             Read8(state) as (isize)
         } else {
             0isize
         })
    }
}

unsafe extern "C" fn SetOperandToImm(state: *mut DecodeState, oper: *mut InstructionOperand) {
    (*oper).operand = OperandType::IMM;
    (*oper).size = (*state).finalOpSize;
    (*oper).immediate = ReadFinalOpSize(state);
}

unsafe extern "C" fn DecodeRegRMImm(state: *mut DecodeState) {
    let regList: *const u8 = GetRegListForFinalOpSize(state);
    DecodeRMReg(
        state,
        (*state).operand1,
        regList,
        (*state).finalOpSize,
        (*state).operand0,
        regList,
        (*state).finalOpSize,
    );
    SetOperandToImm(
        state,
        &mut (*(*state).result).operands[2usize] as (*mut InstructionOperand),
    );
}

unsafe extern "C" fn DecodeRMRegImm8(state: *mut DecodeState) {
    let regList: *const u8 = GetRegListForFinalOpSize(state);
    DecodeRMReg(
        state,
        (*state).operand0,
        regList,
        (*state).finalOpSize,
        (*state).operand1,
        regList,
        (*state).finalOpSize,
    );
    SetOperandToImm8(
        state,
        &mut (*(*state).result).operands[2usize] as (*mut InstructionOperand),
    );
}

unsafe extern "C" fn DecodeRMRegCL(state: *mut DecodeState) {
    let regList: *const u8 = GetRegListForFinalOpSize(state);
    DecodeRMReg(
        state,
        (*state).operand0,
        regList,
        (*state).finalOpSize,
        (*state).operand1,
        regList,
        (*state).finalOpSize,
    );
    (*(*state).result).operands[2usize].operand = OperandType::REG_CL;
    (*(*state).result).operands[2usize].size = 1u16;
}

unsafe extern "C" fn SetOperandToEaxFinalOpSize(
    state: *mut DecodeState,
    oper: *mut InstructionOperand,
) {
    let regList: *const u8 = GetRegListForFinalOpSize(state);
    (*oper).operand = OperandType::from_i32(*regList.offset(0isize) as (i32));
    (*oper).size = (*state).finalOpSize;
}

unsafe extern "C" fn DecodeEaxImm(state: *mut DecodeState) {
    SetOperandToEaxFinalOpSize(state, (*state).operand0);
    SetOperandToImm(state, (*state).operand1);
}

unsafe extern "C" fn DecodePushPopSeg(state: *mut DecodeState) {
    let mut offset: i8 = 0i8;
    if *(*state).opcode.offset(-1isize) as (i32) >= 0xa0i32 {
        offset = -16i8;
    }
    (*(*state).operand0).operand = OperandType::from_i32(
        (OperandType::REG_ES as (i32) + (*(*state).opcode.offset(-1isize) as (i32) >> 3i32) +
             offset as (i32)) as (i32),
    );
    (*(*state).operand0).size = (*state).opSize;
}

unsafe extern "C" fn SetOperandToOpReg(state: *mut DecodeState, oper: *mut InstructionOperand) {
    let regList: *const u8 = GetRegListForFinalOpSize(state);
    let regOffset: u8 = (if (*state).rexRM1 { 8i32 } else { 0i32 }) as (u8);
    (*oper).operand = OperandType::from_i32(*regList.offset(
        ((*(*state).opcode.offset(-1isize) as (i32) & 7i32) +
             regOffset as (i32)) as (isize),
    ) as (i32));
    (*oper).size = (*state).finalOpSize;
}

unsafe extern "C" fn DecodeOpReg(state: *mut DecodeState) {
    SetOperandToOpReg(state, (*state).operand0);
}

unsafe extern "C" fn DecodeEaxOpReg(state: *mut DecodeState) {
    SetOperandToEaxFinalOpSize(state, (*state).operand0);
    SetOperandToOpReg(state, (*state).operand1);
}

unsafe extern "C" fn Read64(state: *mut DecodeState) -> usize {
    if (*state).len < 8usize {
        (*state).invalid = true;
        (*state).insufficientLength = true;
        (*state).len = 0usize;
        0usize
    } else {
        let old_val = (*(*state).opcode) as usize;
        (*state).opcode = (*state).opcode.offset(8isize);
        (*state).len = (*state).len.wrapping_sub(8usize);
        old_val
    }
}

unsafe extern "C" fn DecodeOpRegImm(state: *mut DecodeState) {
    SetOperandToOpReg(state, (*state).operand0);
    (*(*state).operand1).operand = OperandType::IMM;
    (*(*state).operand1).size = (*state).finalOpSize;
    (*(*state).operand1).immediate = if (*state).opSize as (i32) == 8i32 {
        Read64(state)
    } else {
        ReadFinalOpSize(state) as (usize)
    } as (isize);
}

unsafe extern "C" fn DecodeNop(state: *mut DecodeState) {
    if (*state).rexRM1 {
        (*(*state).result).operation = InstructionOperation::XCHG;
        DecodeEaxOpReg(state);
    }
}

unsafe extern "C" fn DecodeImm(state: *mut DecodeState) {
    SetOperandToImm(state, (*state).operand0);
}

unsafe extern "C" fn SetOperandToImm16(state: *mut DecodeState, oper: *mut InstructionOperand) {
    (*oper).operand = OperandType::IMM;
    (*oper).size = 2u16;
    (*oper).immediate = Read16(state) as (isize);
}

unsafe extern "C" fn DecodeImm16Imm8(state: *mut DecodeState) {
    SetOperandToImm16(state, (*state).operand0);
    SetOperandToImm8(state, (*state).operand1);
}

unsafe extern "C" fn SetOperandToEsEdi(
    state: *mut DecodeState,
    oper: *mut InstructionOperand,
    size: u16,
) {
    let addrRegList: *const u8 = GetRegListForAddrSize(state);
    (*oper).operand = OperandType::MEM;
    (*oper).components[0usize] = OperandType::from_i32(*addrRegList.offset(7isize) as (i32));
    (*oper).size = size;
    (*oper).segment = SegmentRegister::SEG_ES;
}

unsafe extern "C" fn DecodeEdiDx(state: *mut DecodeState) {
    SetOperandToEsEdi(state, (*state).operand0, (*state).finalOpSize);
    (*(*state).operand1).operand = OperandType::REG_DX;
    (*(*state).operand1).size = 2u16;
}

unsafe extern "C" fn SetOperandToDsEsi(
    state: *mut DecodeState,
    oper: *mut InstructionOperand,
    size: u16,
) {
    let addrRegList: *const u8 = GetRegListForAddrSize(state);
    (*oper).operand = OperandType::MEM;
    (*oper).components[0usize] = OperandType::from_i32(*addrRegList.offset(6isize) as (i32));
    (*oper).size = size;
    (*oper).segment = GetFinalSegment(state, SegmentRegister::SEG_DS);
}

unsafe extern "C" fn DecodeDxEsi(state: *mut DecodeState) {
    (*(*state).operand0).operand = OperandType::REG_DX;
    (*(*state).operand0).size = 2u16;
    SetOperandToDsEsi(state, (*state).operand1, (*state).finalOpSize);
}

unsafe extern "C" fn ReadSignedFinalOpSize(state: *mut DecodeState) -> isize {
    let switch5 = (*state).finalOpSize;
    if switch5 as (i32) == 8i32 || switch5 as (i32) == 4i32 {
        ReadSigned32(state)
    } else if switch5 as (i32) == 2i32 {
        ReadSigned16(state)
    } else if switch5 as (i32) == 1i32 {
        ReadSigned8(state)
    } else {
        0isize
    }
}

unsafe extern "C" fn DecodeRelImm(state: *mut DecodeState) {
    (*(*state).operand0).operand = OperandType::IMM;
    (*(*state).operand0).size = (*state).opSize;
    (*(*state).operand0).immediate = ReadSignedFinalOpSize(state);
    (*(*state).operand0).immediate =
        ((*(*state).operand0).immediate as (usize)).wrapping_add((*state).addr.wrapping_add(
            (((*state).opcode as (isize)).wrapping_sub((*state).opcodeStart as (isize)) /
                 ::std::mem::size_of::<u8>() as (isize)) as
                (usize),
        )) as (isize);
}

unsafe extern "C" fn UpdateOperationForAddrSize(state: *mut DecodeState) {
    if (*state).addrSize as (i32) == 4i32 {
        (*(*state).result).operation =
            InstructionOperation::from_i32(((*(*state).result).operation as (i32) + 1i32) as (i32));
    } else if (*state).addrSize as (i32) == 8i32 {
        (*(*state).result).operation =
            InstructionOperation::from_i32(((*(*state).result).operation as (i32) + 2i32) as (i32));
    }
}

unsafe extern "C" fn DecodeRelImmAddrSize(state: *mut DecodeState) {
    DecodeRelImm(state);
    UpdateOperationForAddrSize(state);
}

unsafe extern "C" fn DecodeGroupRM(state: *mut DecodeState) {
    let regList: *const u8 = GetRegListForFinalOpSize(state);
    let mut regField: u8 = 0;
    DecodeRM(
        state,
        (*state).operand0,
        regList,
        (*state).finalOpSize,
        &mut regField as (*mut u8),
    );
    (*(*state).result).operation = InstructionOperation::from_i32(
        GROUP_OPERATIONS[(*(*state).result).operation as (i32) as
                             (usize)]
            [regField as (usize)] as (i32),
    );
}

unsafe extern "C" fn DecodeGroupRMImm(state: *mut DecodeState) {
    DecodeGroupRM(state);
    SetOperandToImm(state, (*state).operand1);
}

unsafe extern "C" fn DecodeGroupRMImm8V(state: *mut DecodeState) {
    DecodeGroupRM(state);
    SetOperandToImm8(state, (*state).operand1);
}

unsafe extern "C" fn DecodeGroupRMOne(state: *mut DecodeState) {
    DecodeGroupRM(state);
    (*(*state).operand1).operand = OperandType::IMM;
    (*(*state).operand1).size = 1u16;
    (*(*state).operand1).immediate = 1isize;
}

unsafe extern "C" fn DecodeGroupRMCl(state: *mut DecodeState) {
    DecodeGroupRM(state);
    (*(*state).operand1).operand = OperandType::REG_CL;
    (*(*state).operand1).size = 1u16;
}

unsafe extern "C" fn DecodeGroupF6F7(state: *mut DecodeState) {
    DecodeGroupRM(state);
    if (*(*state).result).operation as (i32) == InstructionOperation::TEST as (i32) {
        SetOperandToImm(state, (*state).operand1);
    }
    if (*(*state).result).flags & 1u32 != 0 &&
        ((*(*state).result).operation as (i32) != InstructionOperation::NOT as (i32)) &&
        ((*(*state).result).operation as (i32) != InstructionOperation::NEG as (i32))
    {
        (*state).invalid = true;
    }
}

unsafe extern "C" fn DecodeGroupFF(state: *mut DecodeState) {
    if (*state).using64 {
        let rm: u8 = Peek8(state);
        let regField: u8 = (rm as (i32) >> 3i32 & 7i32) as (u8);
        if regField as (i32) >= 2i32 && (regField as (i32) <= 5i32) {
            (*state).finalOpSize = {
                (*state).opSize = if (*state).opPrefix { 4i32 } else { 8i32 } as (u16);
                (*state).opSize
            };
        }
    }
    DecodeGroupRM(state);
    if (*(*state).result).operation as (i32) == InstructionOperation::CALLF as (i32) ||
        (*(*state).result).operation as (i32) == InstructionOperation::JMPF as (i32)
    {
        if (*(*state).operand0).operand as (i32) != OperandType::MEM as (i32) {
            (*state).invalid = true;
        }
        (*(*state).operand0).size = ((*(*state).operand0).size as (i32) + 2i32) as (u16);
    }
    if (*(*state).result).flags & 1u32 != 0 &&
        ((*(*state).result).operation as (i32) != InstructionOperation::INC as (i32)) &&
        ((*(*state).result).operation as (i32) != InstructionOperation::DEC as (i32))
    {
        (*state).invalid = true;
    }
}

unsafe extern "C" fn DecodeGroup0F00(state: *mut DecodeState) {
    let rm: u8 = Peek8(state);
    let regField: u8 = (rm as (i32) >> 3i32 & 7i32) as (u8);
    if regField as (i32) >= 2i32 {
        (*state).opSize = 2u16;
    }
    DecodeGroupRM(state);
}

unsafe extern "C" fn DecodeGroup0F01(state: *mut DecodeState) {
    let rm: u8 = Peek8(state);
    let modField: u8 = (rm as (i32) >> 6i32 & 3i32) as (u8);
    let regField: u8 = (rm as (i32) >> 3i32 & 7i32) as (u8);
    let rmField: u8 = (rm as (i32) & 7i32) as (u8);
    if modField as (i32) == 3i32 && (regField as (i32) != 4i32) && (regField as (i32) != 6i32) {
        (*(*state).result).operation = InstructionOperation::from_i32(
            GROUP_0F01_REG_OPERATIONS[rmField as (usize)]
                [regField as (usize)] as (i32),
        );
    } else {
        if regField as (i32) < 4i32 {
            (*state).opSize = if (*state).using64 { 10i32 } else { 6i32 } as (u16);
        } else if regField as (i32) != 7i32 {
            (*state).opSize = 2u16;
        } else {
            (*state).opSize = 1u16;
        }
        DecodeGroupRM(state);
    }
}

unsafe extern "C" fn DecodeGroup0FAE(state: *mut DecodeState) {
    let rm: u8 = Peek8(state);
    let modField: u8 = (rm as (i32) >> 6i32 & 3i32) as (u8);
    let regField: u8 = (rm as (i32) >> 3i32 & 7i32) as (u8);
    if modField as (i32) == 3i32 {
        (*(*state).result).operation = InstructionOperation::from_i32(
            GROUP_OPERATIONS[((*(*state).result).operation as (i32) + 1i32) as
                                 (usize)]
                [regField as (usize)] as (i32),
        );
    } else {
        if regField as (i32) & 2i32 == 0i32 {
            (*state).opSize = 512u16;
        } else if regField as (i32) & 6i32 == 2i32 {
            (*state).opSize = 4u16;
        } else {
            (*state).opSize = 1u16;
        }
        DecodeGroupRM(state);
    }
}

unsafe extern "C" fn Decode0FB8(state: *mut DecodeState) {
    if (*state).rep as (i32) != RepPrefix::REP_PREFIX_REPE as (i32) {
        if (*state).using64 {
            (*state).opSize = if (*state).opPrefix { 4i32 } else { 8i32 } as (u16);
        }
        (*state).finalOpSize = GetFinalOpSize(state);
        DecodeRelImm(state);
    } else {
        DecodeRegRM(state);
    }
}

unsafe extern "C" fn GetRegListForOpSize(state: *mut DecodeState) -> *const u8 {
    let switch6 = (*state).opSize;
    if switch6 as (i32) == 8i32 {
        REG64_LIST.as_ptr()
    } else if switch6 as (i32) == 4i32 {
        REG32_LIST.as_ptr()
    } else if switch6 as (i32) == 2i32 {
        REG16_LIST.as_ptr()
    } else {
        0i32 as (*mut ::std::os::raw::c_void) as (*const u8)
    }
}

unsafe extern "C" fn DecodeRMSRegV(state: *mut DecodeState) {
    let regList: *const u8 = GetRegListForOpSize(state);
    let mut regField: u8 = 0;
    DecodeRM(
        state,
        (*state).operand0,
        regList,
        (*state).opSize,
        &mut regField as (*mut u8),
    );
    if regField as (i32) >= 6i32 {
        (*state).invalid = true;
    }
    (*(*state).operand1).operand =
        OperandType::from_i32((OperandType::REG_ES as (i32) + regField as (i32)) as (i32));
    (*(*state).operand1).size = 2u16;
    if (*(*state).result).operands[0usize].operand as (i32) == OperandType::REG_CS as (i32) {
        (*state).invalid = true;
    }
}

unsafe extern "C" fn DecodeRM8(state: *mut DecodeState) {
    let regList: *const u8 = GetByteRegList(state);
    DecodeRM(
        state,
        (*state).operand0,
        regList,
        1u16,
        0i32 as (*mut ::std::os::raw::c_void) as (*mut u8),
    );
}

unsafe extern "C" fn DecodeRMV(state: *mut DecodeState) {
    let regList: *const u8 = GetRegListForOpSize(state);
    DecodeRM(
        state,
        (*state).operand0,
        regList,
        (*state).opSize,
        0i32 as (*mut ::std::os::raw::c_void) as (*mut u8),
    );
}

unsafe extern "C" fn DecodeFarImm(state: *mut DecodeState) {
    SetOperandToImm(state, (*state).operand1);
    SetOperandToImm16(state, (*state).operand0);
}

unsafe extern "C" fn ReadAddrSize(state: *mut DecodeState) -> isize {
    let switch7 = (*state).addrSize;
    if switch7 as (i32) == 8i32 || switch7 as (i32) == 4i32 {
        Read32(state) as (isize)
    } else if switch7 as (i32) == 2i32 {
        Read16(state) as (isize)
    } else {
        0isize
    }
}

unsafe extern "C" fn SetOperandToImmAddr(state: *mut DecodeState, oper: *mut InstructionOperand) {
    (*oper).operand = OperandType::MEM;
    (*oper).immediate = ReadAddrSize(state);
    (*oper).segment = GetFinalSegment(state, SegmentRegister::SEG_DS);
    (*oper).size = (*state).finalOpSize;
}

unsafe extern "C" fn DecodeEaxAddr(state: *mut DecodeState) {
    SetOperandToEaxFinalOpSize(state, (*state).operand0);
    SetOperandToImmAddr(state, (*state).operand1);
}

unsafe extern "C" fn DecodeEdiEsi(state: *mut DecodeState) {
    SetOperandToEsEdi(state, (*state).operand0, (*state).finalOpSize);
    SetOperandToDsEsi(state, (*state).operand1, (*state).finalOpSize);
}

unsafe extern "C" fn DecodeEdiEax(state: *mut DecodeState) {
    SetOperandToEsEdi(state, (*state).operand0, (*state).finalOpSize);
    SetOperandToEaxFinalOpSize(state, (*state).operand1);
}

unsafe extern "C" fn DecodeEaxEsi(state: *mut DecodeState) {
    SetOperandToEaxFinalOpSize(state, (*state).operand0);
    SetOperandToDsEsi(state, (*state).operand1, (*state).finalOpSize);
}

unsafe extern "C" fn DecodeAlEbxAl(state: *mut DecodeState) {
    let regList: *const u8 = GetRegListForAddrSize(state);
    (*(*state).operand0).operand = OperandType::REG_AL;
    (*(*state).operand0).size = 1u16;
    (*(*state).operand1).operand = OperandType::MEM;
    (*(*state).operand1).components[0usize] =
        OperandType::from_i32(*regList.offset(3isize) as (i32));
    (*(*state).operand1).components[1usize] = OperandType::REG_AL;
    (*(*state).operand1).size = 1u16;
    (*(*state).operand1).segment = GetFinalSegment(state, SegmentRegister::SEG_DS);
}

unsafe extern "C" fn DecodeEaxImm8(state: *mut DecodeState) {
    SetOperandToEaxFinalOpSize(state, (*state).operand0);
    SetOperandToImm8(state, (*state).operand1);
}

unsafe extern "C" fn DecodeEaxDx(state: *mut DecodeState) {
    SetOperandToEaxFinalOpSize(state, (*state).operand0);
    (*(*state).operand1).operand = OperandType::REG_DX;
    (*(*state).operand1).size = 2u16;
}

unsafe extern "C" fn Decode3DNow(state: *mut DecodeState) {
    let mut _currentBlock;
    let op: u8;
    let mut i: i32;
    let mut min: i32;
    let mut max: i32;
    DecodeRMReg(
        state,
        (*state).operand1,
        MMX_REG_LIST.as_ptr(),
        8u16,
        (*state).operand0,
        MMX_REG_LIST.as_ptr(),
        8u16,
    );
    op = Read8(state);
    (*(*state).result).operation = InstructionOperation::INVALID;
    min = 0i32;
    max = ::std::mem::size_of::<[SparseOpEntry; 26]>().wrapping_div(
        ::std::mem::size_of::<SparseOpEntry>(),
    ) as (i32) - 1i32;
    i = (min + max) / 2i32;
    'loop1: loop {
        if !(min <= max) {
            _currentBlock = 5;
            break;
        }
        if op as (i32) > SPARSE_3DNOW_OPCODES[i as (usize)].opcode as (i32) {
            min = i + 1i32;
        } else {
            if !(op as (i32) < SPARSE_3DNOW_OPCODES[i as (usize)].opcode as (i32)) {
                _currentBlock = 4;
                break;
            }
            max = i - 1i32;
        }
        i = (min + max) / 2i32;
    }
    if _currentBlock == 4 {
        (*(*state).result).operation =
            InstructionOperation::from_i32(SPARSE_3DNOW_OPCODES[i as (usize)].operation as (i32));
    }
}

unsafe extern "C" fn DecodeSSEPrefix(state: *mut DecodeState) -> u8 {
    if (*state).opPrefix {
        (*state).opPrefix = false;
        1u8
    } else if (*state).rep as (i32) == RepPrefix::REP_PREFIX_REPNE as (i32) {
        (*state).rep = RepPrefix::REP_PREFIX_NONE;
        2u8
    } else if (*state).rep as (i32) == RepPrefix::REP_PREFIX_REPE as (i32) {
        (*state).rep = RepPrefix::REP_PREFIX_NONE;
        3u8
    } else {
        0u8
    }
}

unsafe extern "C" fn GetOperandForSSEEntryType(
    state: *mut DecodeState,
    type_: u16,
    mut operandIndex: u8,
) -> *mut InstructionOperand {
    if type_ as (i32) == SSETableOperandType::SSE_128_FLIP as (i32) {
        operandIndex = (1i32 - operandIndex as (i32)) as (u8);
    }
    if operandIndex as (i32) == 0i32 {
        (*state).operand0
    } else {
        (*state).operand1
    }
}

unsafe extern "C" fn GetRegListForSSEEntryType(state: *mut DecodeState, type_: u16) -> *const u8 {
    if type_ as (i32) == SSETableOperandType::GPR_32_OR_64 as (i32) {
        (if (*state).opSize as (i32) == 8i32 {
             REG64_LIST.as_ptr()
         } else {
             REG32_LIST.as_ptr()
         })
    } else if type_ as (i32) == SSETableOperandType::MMX_64 as (i32) ||
               type_ as (i32) == SSETableOperandType::MMX_32 as (i32)
    {
        MMX_REG_LIST.as_ptr()
    } else {
        XMM_REG_LIST.as_ptr()
    }
}

unsafe extern "C" fn GetSizeForSSEEntryType(state: *mut DecodeState, type_: u16) -> u16 {
    if type_ as (i32) == SSETableOperandType::GPR_32_OR_64 as (i32) {
        (if (*state).opSize as (i32) == 8i32 {
             8i32
         } else {
             4i32
         }) as (u16)
    } else if type_ as (i32) == SSETableOperandType::MMX_64 as (i32) ||
               type_ as (i32) == SSETableOperandType::SSE_64 as (i32)
    {
        8u16
    } else if type_ as (i32) == SSETableOperandType::MMX_32 as (i32) ||
               type_ as (i32) == SSETableOperandType::SSE_32 as (i32)
    {
        4u16
    } else if type_ as (i32) == SSETableOperandType::SSE_16 as (i32) {
        2u16
    } else {
        16u16
    }
}

unsafe extern "C" fn UpdateOperationForSSEEntryType(state: *mut DecodeState, type_: u16) {
    if type_ as (i32) == SSETableOperandType::GPR_32_OR_64 as (i32) &&
        ((*state).opSize as (i32) == 8i32)
    {
        (*(*state).result).operation =
            InstructionOperation::from_i32(((*(*state).result).operation as (i32) + 1i32) as (i32));
    }
}

unsafe extern "C" fn DecodeSSETable(state: *mut DecodeState) {
    let type_: u8 = DecodeSSEPrefix(state);
    let rm: u8 = Peek8(state);
    let modField: u8 = (rm as (i32) >> 6i32 & 3i32) as (u8);
    let entry: *const SSETableEntry = &SSE_TABLE[(*(*state).result).operation as (i32) as (usize)];
    let opEntry: *const SSETableOperationEntry;
    if modField as (i32) == 3i32 {
        opEntry = &(*entry).regOps[type_ as (usize)];
    } else {
        opEntry = &(*entry).memOps[type_ as (usize)];
    }
    (*(*state).result).operation = InstructionOperation::from_i32((*opEntry).operation as (i32));
    DecodeRMReg(
        state,
        GetOperandForSSEEntryType(state, (*opEntry).rmType as (u16), 1u8),
        GetRegListForSSEEntryType(state, (*opEntry).rmType as (u16)),
        GetSizeForSSEEntryType(state, (*opEntry).rmType as (u16)),
        GetOperandForSSEEntryType(state, (*opEntry).regType as (u16), 0u8),
        GetRegListForSSEEntryType(state, (*opEntry).regType as (u16)),
        GetSizeForSSEEntryType(state, (*opEntry).regType as (u16)),
    );
    if (*state).flags & 0x800u32 != 0 {
        UpdateOperationForSSEEntryType(state, (*opEntry).regType as (u16));
        UpdateOperationForSSEEntryType(state, (*opEntry).rmType as (u16));
    }
}

unsafe extern "C" fn DecodeSSETableImm8(state: *mut DecodeState) {
    DecodeSSETable(state);
    SetOperandToImm8(
        state,
        &mut (*(*state).result).operands[2usize] as (*mut InstructionOperand),
    );
}

unsafe extern "C" fn DecodeSSETableMem8(state: *mut DecodeState) {
    DecodeSSETable(state);
    if (*(*state).operand0).operand as (i32) == OperandType::MEM as (i32) {
        (*(*state).operand0).size = 1u16;
    }
    if (*(*state).operand1).operand as (i32) == OperandType::MEM as (i32) {
        (*(*state).operand1).size = 1u16;
    }
}

unsafe extern "C" fn GetSizeForSSEType(type_: u8) -> u16 {
    if type_ as (i32) == 2i32 {
        8u16
    } else if type_ as (i32) == 3i32 {
        4u16
    } else {
        16u16
    }
}

unsafe extern "C" fn DecodeSSE(state: *mut DecodeState) {
    let type_: u8 = DecodeSSEPrefix(state);
    let rm: u8 = Peek8(state);
    let modField: u8 = (rm as (i32) >> 6i32 & 3i32) as (u8);
    let size: u16;
    (*(*state).result).operation = InstructionOperation::from_i32(
        ((*(*state).result).operation as (i32) + type_ as (i32)) as
            (i32),
    );
    if modField as (i32) == 3i32 {
        size = 16u16;
    } else {
        size = GetSizeForSSEType(type_);
    }
    DecodeRMReg(
        state,
        (*state).operand1,
        XMM_REG_LIST.as_ptr(),
        size,
        (*state).operand0,
        XMM_REG_LIST.as_ptr(),
        16u16,
    );
}

unsafe extern "C" fn DecodeSSESingle(state: *mut DecodeState) {
    let type_: u8 = DecodeSSEPrefix(state);
    if type_ as (i32) == 1i32 || type_ as (i32) == 2i32 {
        (*state).invalid = true;
    } else {
        (*(*state).result).operation = InstructionOperation::from_i32(
            ((*(*state).result).operation as (i32) +
                 (type_ as (i32) & 1i32)) as (i32),
        );
        DecodeRMReg(
            state,
            (*state).operand1,
            XMM_REG_LIST.as_ptr(),
            16u16,
            (*state).operand0,
            XMM_REG_LIST.as_ptr(),
            16u16,
        );
    }
}

unsafe extern "C" fn DecodeSSEPacked(state: *mut DecodeState) {
    let type_: u8 = DecodeSSEPrefix(state);
    if type_ as (i32) == 2i32 || type_ as (i32) == 3i32 {
        (*state).invalid = true;
    } else {
        (*(*state).result).operation = InstructionOperation::from_i32(
            ((*(*state).result).operation as (i32) +
                 (type_ as (i32) & 1i32)) as (i32),
        );
        DecodeRMReg(
            state,
            (*state).operand1,
            XMM_REG_LIST.as_ptr(),
            16u16,
            (*state).operand0,
            XMM_REG_LIST.as_ptr(),
            16u16,
        );
    }
}

unsafe extern "C" fn DecodeMMX(state: *mut DecodeState) {
    if (*state).opPrefix {
        DecodeRMReg(
            state,
            (*state).operand1,
            XMM_REG_LIST.as_ptr(),
            16u16,
            (*state).operand0,
            XMM_REG_LIST.as_ptr(),
            16u16,
        );
    } else {
        DecodeRMReg(
            state,
            (*state).operand1,
            MMX_REG_LIST.as_ptr(),
            8u16,
            (*state).operand0,
            MMX_REG_LIST.as_ptr(),
            8u16,
        );
    }
}

unsafe extern "C" fn DecodeMMXSSEOnly(state: *mut DecodeState) {
    if (*state).opPrefix {
        DecodeRMReg(
            state,
            (*state).operand1,
            XMM_REG_LIST.as_ptr(),
            16u16,
            (*state).operand0,
            XMM_REG_LIST.as_ptr(),
            16u16,
        );
    } else {
        (*state).invalid = true;
    }
}

unsafe extern "C" fn DecodeMMXGroup(state: *mut DecodeState) {
    let mut regField: u8 = 0;
    if (*state).opPrefix {
        DecodeRM(
            state,
            (*state).operand0,
            XMM_REG_LIST.as_ptr(),
            16u16,
            &mut regField as (*mut u8),
        );
        (*(*state).result).operation = InstructionOperation::from_i32(
            MMX_GROUP_OPERATIONS[(*(*state).result).operation as (i32) as
                                     (usize)]
                [regField as (usize)]
                [1usize] as (i32),
        );
    } else {
        DecodeRM(
            state,
            (*state).operand0,
            MMX_REG_LIST.as_ptr(),
            8u16,
            &mut regField as (*mut u8),
        );
        (*(*state).result).operation = InstructionOperation::from_i32(
            MMX_GROUP_OPERATIONS[(*(*state).result).operation as (i32) as
                                     (usize)]
                [regField as (usize)]
                [0usize] as (i32),
        );
    }
    SetOperandToImm8(state, (*state).operand1);
}

unsafe extern "C" fn DecodePinsrw(state: *mut DecodeState) {
    DecodeSSETableImm8(state);
    if (*(*state).operand1).operand as (i32) == OperandType::MEM as (i32) {
        (*(*state).operand1).size = 2u16;
    }
}

unsafe extern "C" fn DecodeRegCR(state: *mut DecodeState) {
    if (*state).opSize as (i32) == 2i32 {
        (*state).opSize = 4u16;
    }
    let regList = GetRegListForOpSize(state);
    let reg = Read8(state);
    if (*(*state).result).flags & 1u32 != 0 {
        (*(*state).result).flags = (*(*state).result).flags & !1i32 as (u32);
        (*state).rexReg = true;
    }
    (*(*state).operand0).operand = OperandType::from_i32(*regList.offset(
        ((reg as (i32) & 7i32) +
             if (*state).rexRM1 {
                 8i32
             } else {
                 0i32
             }) as (isize),
    ) as (i32));
    (*(*state).operand0).size = (*state).opSize;
    (*(*state).operand1).operand = OperandType::from_i32(
        ((*(*state).result).operation as (i32) + (reg as (i32) >> 3i32 & 7i32) +
             if (*state).rexReg { 8i32 } else { 0i32 }) as (i32),
    );
    (*(*state).operand1).size = (*state).opSize;
    (*(*state).result).operation = InstructionOperation::MOV;
}

unsafe extern "C" fn DecodeMovSXZX8(state: *mut DecodeState) {
    DecodeRMReg(
        state,
        (*state).operand1,
        GetByteRegList(state),
        1u16,
        (*state).operand0,
        GetRegListForOpSize(state),
        (*state).opSize,
    );
}

unsafe extern "C" fn DecodeMovSXZX16(state: *mut DecodeState) {
    DecodeRMReg(
        state,
        (*state).operand1,
        REG16_LIST.as_ptr(),
        2u16,
        (*state).operand0,
        GetRegListForOpSize(state),
        (*state).opSize,
    );
}

unsafe extern "C" fn DecodeMem16(state: *mut DecodeState) {
    DecodeRM(
        state,
        (*state).operand0,
        REG32_LIST.as_ptr(),
        2u16,
        0i32 as (*mut ::std::os::raw::c_void) as (*mut u8),
    );
    if (*(*state).operand0).operand as (i32) != OperandType::MEM as (i32) {
        (*state).invalid = true;
    }
}

unsafe extern "C" fn DecodeMem32(state: *mut DecodeState) {
    DecodeRM(
        state,
        (*state).operand0,
        REG32_LIST.as_ptr(),
        4u16,
        0i32 as (*mut ::std::os::raw::c_void) as (*mut u8),
    );
    if (*(*state).operand0).operand as (i32) != OperandType::MEM as (i32) {
        (*state).invalid = true;
    }
}

unsafe extern "C" fn DecodeMem64(state: *mut DecodeState) {
    DecodeRM(
        state,
        (*state).operand0,
        REG32_LIST.as_ptr(),
        8u16,
        0i32 as (*mut ::std::os::raw::c_void) as (*mut u8),
    );
    if (*(*state).operand0).operand as (i32) != OperandType::MEM as (i32) {
        (*state).invalid = true;
    }
}

unsafe extern "C" fn DecodeMem80(state: *mut DecodeState) {
    DecodeRM(
        state,
        (*state).operand0,
        REG32_LIST.as_ptr(),
        10u16,
        0i32 as (*mut ::std::os::raw::c_void) as (*mut u8),
    );
    if (*(*state).operand0).operand as (i32) != OperandType::MEM as (i32) {
        (*state).invalid = true;
    }
}

unsafe extern "C" fn DecodeMemFloatEnv(state: *mut DecodeState) {
    DecodeRM(
        state,
        (*state).operand0,
        REG32_LIST.as_ptr(),
        if (*state).opSize as (i32) == 2i32 {
            14i32
        } else {
            28i32
        } as (u16),
        0i32 as (*mut ::std::os::raw::c_void) as (*mut u8),
    );
    if (*(*state).operand0).operand as (i32) != OperandType::MEM as (i32) {
        (*state).invalid = true;
    }
}

unsafe extern "C" fn DecodeMemFloatSave(state: *mut DecodeState) {
    DecodeRM(
        state,
        (*state).operand0,
        REG32_LIST.as_ptr(),
        if (*state).opSize as (i32) == 2i32 {
            94i32
        } else {
            108i32
        } as (u16),
        0i32 as (*mut ::std::os::raw::c_void) as (*mut u8),
    );
    if (*(*state).operand0).operand as (i32) != OperandType::MEM as (i32) {
        (*state).invalid = true;
    }
}

unsafe extern "C" fn DecodeFPUReg(state: *mut DecodeState) {
    DecodeRM(
        state,
        (*state).operand0,
        FPU_REG_LIST.as_ptr(),
        10u16,
        0i32 as (*mut ::std::os::raw::c_void) as (*mut u8),
    );
}

unsafe extern "C" fn DecodeFPURegST0(state: *mut DecodeState) {
    DecodeFPUReg(state);
    (*(*state).operand1).operand = OperandType::REG_ST0;
    (*(*state).operand1).size = 10u16;
}

unsafe extern "C" fn DecodeRegGroupNoOperands(state: *mut DecodeState) {
    let rmByte: u8 = Read8(state);
    (*(*state).result).operation = InstructionOperation::from_i32(
        GROUP_OPERATIONS[(*(*state).result).operation as (i32) as
                             (usize)]
            [(rmByte as (i32) & 7i32) as (usize)] as (i32),
    );
}

unsafe extern "C" fn DecodeRegGroupAX(state: *mut DecodeState) {
    DecodeRegGroupNoOperands(state);
    (*(*state).operand0).operand = OperandType::REG_AX;
    (*(*state).operand0).size = 2u16;
}

unsafe extern "C" fn DecodeCmpXch8B(state: *mut DecodeState) {
    let rm: u8 = Peek8(state);
    let regField: u8 = (rm as (i32) >> 3i32 & 7i32) as (u8);
    if regField as (i32) == 1i32 {
        if (*state).opSize as (i32) == 2i32 {
            (*state).opSize = 4u16;
        } else if (*state).opSize as (i32) == 8i32 {
            (*(*state).result).operation = InstructionOperation::CMPXCH16B;
        }
        DecodeRM(
            state,
            (*state).operand0,
            GetRegListForOpSize(state),
            ((*state).opSize as (i32) * 2i32) as (u16),
            0i32 as (*mut ::std::os::raw::c_void) as (*mut u8),
        );
    } else if regField as (i32) == 6i32 {
        if (*state).opPrefix {
            (*(*state).result).operation = InstructionOperation::VMCLEAR;
        } else if (*state).rep as (i32) == RepPrefix::REP_PREFIX_REPE as (i32) {
            (*(*state).result).operation = InstructionOperation::VMXON;
        } else {
            (*(*state).result).operation = InstructionOperation::VMPTRLD;
        }
        DecodeRM(
            state,
            (*state).operand0,
            REG64_LIST.as_ptr(),
            8u16,
            0i32 as (*mut ::std::os::raw::c_void) as (*mut u8),
        );
    } else if regField as (i32) == 7i32 {
        (*(*state).result).operation = InstructionOperation::VMPTRST;
        DecodeRM(
            state,
            (*state).operand0,
            REG64_LIST.as_ptr(),
            8u16,
            0i32 as (*mut ::std::os::raw::c_void) as (*mut u8),
        );
    } else {
        (*state).invalid = true;
    }
    if (*(*state).operand0).operand as (i32) != OperandType::MEM as (i32) {
        (*state).invalid = true;
    }
}

unsafe extern "C" fn DecodeMovNti(state: *mut DecodeState) {
    if (*state).opSize as (i32) == 2i32 {
        (*state).opSize = 4u16;
    }
    DecodeRMReg(
        state,
        (*state).operand0,
        GetRegListForOpSize(state),
        (*state).opSize,
        (*state).operand1,
        GetRegListForOpSize(state),
        (*state).opSize,
    );
    if (*(*state).operand0).operand as (i32) != OperandType::MEM as (i32) {
        (*state).invalid = true;
    }
}

unsafe extern "C" fn DecodeCrc32(state: *mut DecodeState) {
    let srcRegList: *const u8 = GetRegListForFinalOpSize(state);
    let destRegList: *const u8 = if (*state).opSize as (i32) == 8i32 {
        REG64_LIST.as_ptr()
    } else {
        REG32_LIST.as_ptr()
    };
    let destSize: u16 = (if (*state).opSize as (i32) == 8i32 {
                             8i32
                         } else {
                             4i32
                         }) as (u16);
    DecodeRMReg(
        state,
        (*state).operand1,
        srcRegList,
        (*state).finalOpSize,
        (*state).operand0,
        destRegList,
        destSize,
    );
}

unsafe extern "C" fn DecodeArpl(state: *mut DecodeState) {
    if (*state).using64 {
        let regList: *const u8 = GetRegListForFinalOpSize(state);
        (*(*state).result).operation = InstructionOperation::MOVSXD;
        DecodeRMReg(
            state,
            (*state).operand1,
            REG32_LIST.as_ptr(),
            4u16,
            (*state).operand0,
            regList,
            (*state).finalOpSize,
        );
    } else {
        (*state).operand0 = &mut (*(*state).result).operands[1usize] as (*mut InstructionOperand);
        (*state).operand1 = &mut (*(*state).result).operands[0usize] as (*mut InstructionOperand);
        (*state).finalOpSize = 2u16;
        DecodeRegRM(state);
    }
}

unsafe extern "C" fn ClearOperand(oper: *mut InstructionOperand) {
    (*oper).operand = OperandType::NONE;
    (*oper).components[0usize] = OperandType::NONE;
    (*oper).components[1usize] = OperandType::NONE;
    (*oper).scale = 1u8;
    (*oper).immediate = 0isize;
}

unsafe extern "C" fn InitDisassemble(state: *mut DecodeState) {
    ClearOperand(
        &mut (*(*state).result).operands[0usize] as (*mut InstructionOperand),
    );
    ClearOperand(
        &mut (*(*state).result).operands[1usize] as (*mut InstructionOperand),
    );
    ClearOperand(
        &mut (*(*state).result).operands[2usize] as (*mut InstructionOperand),
    );
    (*(*state).result).operation = InstructionOperation::INVALID;
    (*(*state).result).flags = 0u32;
    (*(*state).result).segment = SegmentRegister::SEG_DEFAULT;
    (*state).invalid = false;
    (*state).insufficientLength = false;
    (*state).opPrefix = false;
    (*state).rep = RepPrefix::REP_PREFIX_NONE;
    (*state).ripRelFixup = 0i32 as (*mut ::std::os::raw::c_void) as (*mut isize);
    (*state).rex = false;
    (*state).rexReg = false;
    (*state).rexRM1 = false;
    (*state).rexRM2 = false;
    (*state).origLen = (*state).len;
}

unsafe extern "C" fn ProcessPrefixes(state: *mut DecodeState) {
    let mut _currentBlock;
    let mut rex: u8 = 0u8;
    let mut addrPrefix: bool = false;
    'loop1: loop {
        if !!(*state).invalid {
            _currentBlock = 11;
            break;
        }
        let prefix: u8 = Read8(state);
        if prefix as (i32) >= 0x26i32 && (prefix as (i32) <= 0x3ei32) &&
            (prefix as (i32) & 7i32 == 6i32)
        {
            (*(*state).result).segment = SegmentRegister::from_i32(
                (SegmentRegister::SEG_ES as (i32) + ((prefix as (i32) >> 3i32) - 4i32)) as
                    (i32),
            );
        } else if prefix as (i32) == 0x64i32 || prefix as (i32) == 0x65i32 {
            (*(*state).result).segment = SegmentRegister::from_i32(
                (SegmentRegister::SEG_ES as (i32) +
                     (prefix as (i32) - 0x60i32)) as (i32),
            );
        } else if prefix as (i32) == 0x66i32 {
            (*state).opPrefix = true;
            (*(*state).result).flags = (*(*state).result).flags | 16u32;
        } else if prefix as (i32) == 0x67i32 {
            addrPrefix = true;
            (*(*state).result).flags = (*(*state).result).flags | 32u32;
        } else if prefix as (i32) == 0xf0i32 {
            (*(*state).result).flags = (*(*state).result).flags | 1u32;
        } else if prefix as (i32) == 0xf2i32 {
            (*state).rep = RepPrefix::REP_PREFIX_REPNE;
        } else if prefix as (i32) == 0xf3i32 {
            (*state).rep = RepPrefix::REP_PREFIX_REPE;
        } else {
            if !((*state).using64 && (prefix as (i32) >= 0x40i32) && (prefix as (i32) <= 0x4fi32)) {
                _currentBlock = 10;
                break;
            }
            rex = prefix;
            continue;
        }
        rex = 0u8;
    }
    if _currentBlock == 10 {
        (*state).opcode = (*state).opcode.offset(-1isize);
        (*state).len = (*state).len.wrapping_add(1usize);
    }
    if (*state).opPrefix {
        (*state).opSize = if (*state).opSize as (i32) == 2i32 {
            4i32
        } else {
            2i32
        } as (u16);
    }
    if addrPrefix {
        (*state).addrSize = if (*state).addrSize as (i32) == 4i32 {
            2i32
        } else {
            4i32
        } as (u16);
    }
    if rex != 0 {
        (*state).rex = true;
        (*state).rexRM1 = rex as (i32) & 1i32 != 0i32;
        (*state).rexRM2 = rex as (i32) & 2i32 != 0i32;
        (*state).rexReg = rex as (i32) & 4i32 != 0i32;
        if rex as (i32) & 8i32 != 0 {
            (*state).opSize = 8u16;
        }
    }
}

unsafe extern "C" fn FinishDisassemble(state: *mut DecodeState) {
    (*(*state).result).length = (((*state).opcode as (isize)).wrapping_sub(
        (*state).opcodeStart as (isize),
    ) / ::std::mem::size_of::<u8>() as (isize)) as (usize);
    if !(*state).ripRelFixup.is_null() {
        *(*state).ripRelFixup = (*(*state).ripRelFixup as (usize)).wrapping_add(
            (*state).addr.wrapping_add((*(*state).result).length),
        ) as (isize);
    }
    if (*state).insufficientLength && ((*state).origLen < 15usize) {
        (*(*state).result).flags = (*(*state).result).flags | 0x80000000u32;
    }
}

#[no_mangle]
pub unsafe extern "C" fn Disassemble16(
    opcode: *const u8,
    addr: usize,
    maxLen: usize,
    result: *mut Instruction,
) -> bool {
    let mut state = DecodeState::default();
    state.result = result;
    state.opcodeStart = opcode;
    state.opcode = opcode;
    state.addr = addr;
    state.len = if maxLen > 15usize { 15usize } else { maxLen };
    state.addrSize = 2u16;
    state.opSize = 2u16;
    state.using64 = false;
    InitDisassemble(&mut state as (*mut DecodeState));
    ProcessPrefixes(&mut state as (*mut DecodeState));
    ProcessOpcode(
        &mut state as (*mut DecodeState),
        MAIN_OPCODE_MAP.as_ptr(),
        Read8(&mut state as (*mut DecodeState)),
    );
    FinishDisassemble(&mut state as (*mut DecodeState));
    !state.invalid
}

#[no_mangle]
pub unsafe extern "C" fn Disassemble32(
    opcode: *const u8,
    addr: usize,
    maxLen: usize,
    result: *mut Instruction,
) -> bool {
    let mut state = DecodeState::default();
    state.result = result;
    state.opcodeStart = opcode;
    state.opcode = opcode;
    state.addr = addr;
    state.len = if maxLen > 15usize { 15usize } else { maxLen };
    state.addrSize = 4u16;
    state.opSize = 4u16;
    state.using64 = false;
    InitDisassemble(&mut state as (*mut DecodeState));
    ProcessPrefixes(&mut state as (*mut DecodeState));
    ProcessOpcode(
        &mut state as (*mut DecodeState),
        MAIN_OPCODE_MAP.as_ptr(),
        Read8(&mut state as (*mut DecodeState)),
    );
    FinishDisassemble(&mut state as (*mut DecodeState));
    !state.invalid
}

#[no_mangle]
pub unsafe extern "C" fn Disassemble64(
    opcode: *const u8,
    addr: usize,
    maxLen: usize,
    result: *mut Instruction,
) -> bool {
    let mut state = DecodeState::default();
    state.result = result;
    state.opcodeStart = opcode;
    state.opcode = opcode;
    state.addr = addr;
    state.len = if maxLen > 15usize { 15usize } else { maxLen };
    state.addrSize = 8u16;
    state.opSize = 4u16;
    state.using64 = true;
    InitDisassemble(&mut state as (*mut DecodeState));
    ProcessPrefixes(&mut state as (*mut DecodeState));
    ProcessOpcode(
        &mut state as (*mut DecodeState),
        MAIN_OPCODE_MAP.as_ptr(),
        Read8(&mut state as (*mut DecodeState)),
    );
    FinishDisassemble(&mut state as (*mut DecodeState));
    !state.invalid
}

unsafe extern "C" fn WriteChar(out: *mut *mut u8, outMaxLen: *mut usize, ch: u8) {
    if *outMaxLen > 1usize {
        *{
            let _old = *out;
            *out = (*out).offset(1isize);
            _old
        } = ch;
        *outMaxLen = (*outMaxLen).wrapping_sub(1usize);
    }
}

unsafe extern "C" fn WriteString(out: *mut *mut u8, outMaxLen: *mut usize, mut str: *const u8) {
    'loop0: loop {
        if *str == 0 {
            break;
        }
        WriteChar(out, outMaxLen, *str);
        str = str.offset(1isize);
    }
}

unsafe extern "C" fn WriteOperand(
    out: *mut *mut u8,
    outMaxLen: *mut usize,
    type_: OperandType,
    scale: u8,
    plus: bool,
) {
    if plus {
        WriteString(out, outMaxLen, (*b"+\0").as_ptr());
    }
    WriteString(out, outMaxLen, OPERAND_STRING[type_ as (usize)].as_ptr());
    if scale as (i32) != 1i32 {
        WriteChar(out, outMaxLen, b'*');
        WriteChar(out, outMaxLen, (scale as (i32) + b'0' as (i32)) as (u8));
    }
}

unsafe extern "C" fn GetSizeString(size: u16) -> *const u8 {
    if size as (i32) == 16i32 {
        (*b"oword \0").as_ptr()
    } else if size as (i32) == 10i32 {
        (*b"tword \0").as_ptr()
    } else if size as (i32) == 8i32 {
        (*b"qword \0").as_ptr()
    } else if size as (i32) == 6i32 {
        (*b"fword \0").as_ptr()
    } else if size as (i32) == 4i32 {
        (*b"dword \0").as_ptr()
    } else if size as (i32) == 2i32 {
        (*b"word \0").as_ptr()
    } else if size as (i32) == 1i32 {
        (*b"byte \0").as_ptr()
    } else {
        (*b"\0").as_ptr()
    }
}

unsafe extern "C" fn WriteHex(
    out: *mut *mut u8,
    outMaxLen: *mut usize,
    mut val: usize,
    mut width: u32,
    prefix: bool,
) {
    let mut temp = [0u8; 17];
    let mut i: i32;
    if prefix {
        WriteString(out, outMaxLen, (*b"0x\0").as_ptr());
    }
    if width > 16u32 {
        width = 16u32;
    }
    i = width.wrapping_sub(1u32) as (i32);
    'loop5: loop {
        if !(i >= 0i32) {
            break;
        }
        let digit: u8 = (val & 0xfusize) as (u8);
        if digit as (i32) < 10i32 {
            temp[i as (usize)] = (digit as (i32) + b'0' as (i32)) as (u8);
        } else {
            temp[i as (usize)] = (digit as (i32) + b'a' as (i32) - 10i32) as (u8);
        }
        i = i - 1;
        val = val >> 4i32;
    }
    temp[width as (usize)] = 0u8;
    WriteString(out, outMaxLen, temp.as_mut_ptr() as (*const u8));
}

#[no_mangle]
pub unsafe extern "C" fn FormatInstructionString(
    mut out: *mut u8,
    mut outMaxLen: usize,
    mut fmt: *const u8,
    opcode: *const u8,
    addr: usize,
    instr: *const Instruction,
) -> usize {
    let mut _currentBlock;
    let start: *mut u8 = out;
    let len: usize;
    'loop1: loop {
        if *fmt == 0 {
            break;
        }
        if *fmt as (i32) == b'%' as (i32) {
            let mut width: u32 = 0u32;
            fmt = fmt.offset(1isize);
            'loop8: loop {
                if *fmt == 0 {
                    _currentBlock = 63;
                    break;
                }
                if *fmt as (i32) == b'a' as (i32) {
                    _currentBlock = 60;
                    break;
                }
                if *fmt as (i32) == b'b' as (i32) {
                    _currentBlock = 53;
                    break;
                }
                if *fmt as (i32) == b'i' as (i32) {
                    _currentBlock = 42;
                    break;
                }
                if *fmt as (i32) == b'o' as (i32) {
                    _currentBlock = 17;
                    break;
                }
                if !(*fmt as (i32) >= b'0' as (i32) && (*fmt as (i32) <= b'9' as (i32))) {
                    _currentBlock = 14;
                    break;
                }
                width = width.wrapping_mul(10u32).wrapping_add(
                    (*fmt as (i32) - b'0' as (i32)) as
                        (u32),
                );
                fmt = fmt.offset(1isize);
            }
            if _currentBlock == 63 {
            } else if _currentBlock == 14 {
                WriteChar(
                    &mut out as (*mut *mut u8),
                    &mut outMaxLen as (*mut usize),
                    *fmt,
                );
            } else if _currentBlock == 17 {
                let mut i: u32;
                i = 0u32;
                'loop18: loop {
                    if !(i < 3u32) {
                        break;
                    }
                    if (*instr).operands[i as (usize)].operand as (i32) ==
                        OperandType::NONE as (i32)
                    {
                        break;
                    }
                    if i != 0u32 {
                        WriteString(
                            &mut out as (*mut *mut u8),
                            &mut outMaxLen as (*mut usize),
                            (*b", \0").as_ptr(),
                        );
                    }
                    if (*instr).operands[i as (usize)].operand as (i32) ==
                        OperandType::IMM as (i32)
                    {
                        WriteHex(
                            &mut out as (*mut *mut u8),
                            &mut outMaxLen as (*mut usize),
                            (*instr).operands[i as (usize)].immediate as (usize),
                            ((*instr).operands[i as (usize)].size as (i32) * 2i32) as (u32),
                            true,
                        );
                    } else if (*instr).operands[i as (usize)].operand as (i32) ==
                               OperandType::MEM as (i32)
                    {
                        let mut plus: bool = false;
                        WriteString(
                            &mut out as (*mut *mut u8),
                            &mut outMaxLen as (*mut usize),
                            GetSizeString((*instr).operands[i as (usize)].size),
                        );
                        if (*instr).segment as (i32) != SegmentRegister::SEG_DEFAULT as (i32) ||
                            (*instr).operands[i as (usize)].segment as (i32) ==
                                SegmentRegister::SEG_ES as (i32)
                        {
                            WriteOperand(
                                &mut out as (*mut *mut u8),
                                &mut outMaxLen as (*mut usize),
                                OperandType::from_i32(
                                    ((*instr).operands[i as (usize)].segment as (i32) +
                                         OperandType::REG_ES as (i32)) as
                                        (i32),
                                ),
                                1u8,
                                false,
                            );
                            WriteChar(
                                &mut out as (*mut *mut u8),
                                &mut outMaxLen as (*mut usize),
                                b':',
                            );
                        }
                        WriteChar(
                            &mut out as (*mut *mut u8),
                            &mut outMaxLen as (*mut usize),
                            b'[',
                        );
                        if (*instr).operands[i as (usize)].components[0usize] as (i32) !=
                            OperandType::NONE as (i32)
                        {
                            WriteOperand(
                                &mut out as (*mut *mut u8),
                                &mut outMaxLen as (*mut usize),
                                (*instr).operands[i as (usize)].components[0usize],
                                1u8,
                                false,
                            );
                            plus = true;
                        }
                        if (*instr).operands[i as (usize)].components[1usize] as (i32) !=
                            OperandType::NONE as (i32)
                        {
                            WriteOperand(
                                &mut out as (*mut *mut u8),
                                &mut outMaxLen as (*mut usize),
                                (*instr).operands[i as (usize)].components[1usize],
                                (*instr).operands[i as (usize)].scale,
                                plus,
                            );
                            plus = true;
                        }
                        if (*instr).operands[i as (usize)].immediate != 0isize ||
                            (*instr).operands[i as (usize)].components[0usize] as (i32) ==
                                OperandType::NONE as (i32) &&
                                ((*instr).operands[i as (usize)].components[1usize] as (i32) ==
                                     OperandType::NONE as (i32))
                        {
                            if plus && ((*instr).operands[i as (usize)].immediate >= -0x80isize) &&
                                ((*instr).operands[i as (usize)].immediate < 0isize)
                            {
                                WriteChar(
                                    &mut out as (*mut *mut u8),
                                    &mut outMaxLen as (*mut usize),
                                    b'-',
                                );
                                WriteHex(
                                    &mut out as (*mut *mut u8),
                                    &mut outMaxLen as (*mut usize),
                                    -(*instr).operands[i as (usize)].immediate as (usize),
                                    2u32,
                                    true,
                                );
                            } else if plus &&
                                       ((*instr).operands[i as (usize)].immediate > 0isize) &&
                                       ((*instr).operands[i as (usize)].immediate <= 0x7fisize)
                            {
                                WriteChar(
                                    &mut out as (*mut *mut u8),
                                    &mut outMaxLen as (*mut usize),
                                    b'+',
                                );
                                WriteHex(
                                    &mut out as (*mut *mut u8),
                                    &mut outMaxLen as (*mut usize),
                                    (*instr).operands[i as (usize)].immediate as (usize),
                                    2u32,
                                    true,
                                );
                            } else {
                                if plus {
                                    WriteChar(
                                        &mut out as (*mut *mut u8),
                                        &mut outMaxLen as (*mut usize),
                                        b'+',
                                    );
                                }
                                WriteHex(
                                    &mut out as (*mut *mut u8),
                                    &mut outMaxLen as (*mut usize),
                                    (*instr).operands[i as (usize)].immediate as (usize),
                                    8u32,
                                    true,
                                );
                            }
                        }
                        WriteChar(
                            &mut out as (*mut *mut u8),
                            &mut outMaxLen as (*mut usize),
                            b']',
                        );
                    } else {
                        WriteOperand(
                            &mut out as (*mut *mut u8),
                            &mut outMaxLen as (*mut usize),
                            (*instr).operands[i as (usize)].operand,
                            1u8,
                            false,
                        );
                    }
                    i = i.wrapping_add(1u32);
                }
            } else if _currentBlock == 42 {
                let operationStart: *mut u8 = out;
                if (*instr).flags & (2i32 | 8i32 | 4i32) as (u32) != 0 {
                    WriteString(
                        &mut out as (*mut *mut u8),
                        &mut outMaxLen as (*mut usize),
                        (*b"rep\0").as_ptr(),
                    );
                    if (*instr).flags & 4u32 != 0 {
                        WriteChar(
                            &mut out as (*mut *mut u8),
                            &mut outMaxLen as (*mut usize),
                            b'n',
                        );
                    }
                    if (*instr).flags & (4i32 | 8i32) as (u32) != 0 {
                        WriteChar(
                            &mut out as (*mut *mut u8),
                            &mut outMaxLen as (*mut usize),
                            b'e',
                        );
                    }
                    WriteChar(
                        &mut out as (*mut *mut u8),
                        &mut outMaxLen as (*mut usize),
                        b' ',
                    );
                }
                if (*instr).flags & 1u32 != 0 {
                    WriteString(
                        &mut out as (*mut *mut u8),
                        &mut outMaxLen as (*mut usize),
                        (*b"lock \0").as_ptr(),
                    );
                }
                WriteString(
                    &mut out as (*mut *mut u8),
                    &mut outMaxLen as (*mut usize),
                    OPERATION_STRINGS[(*instr).operation as (usize)].as_ptr(),
                );
                'loop51: loop {
                    if !(((out as (isize)).wrapping_sub(operationStart as (isize)) /
                              ::std::mem::size_of::<u8>() as (isize)) as
                             (usize) < width as (usize) &&
                             (outMaxLen > 1usize))
                    {
                        break;
                    }
                    WriteChar(
                        &mut out as (*mut *mut u8),
                        &mut outMaxLen as (*mut usize),
                        b' ',
                    );
                }
            } else if _currentBlock == 53 {
                let mut i: usize;
                i = 0usize;
                'loop54: loop {
                    if !(i < (*instr).length) {
                        break;
                    }
                    WriteHex(
                        &mut out as (*mut *mut u8),
                        &mut outMaxLen as (*mut usize),
                        *opcode.offset(i as (isize)) as (usize),
                        2u32,
                        false,
                    );
                    i = i.wrapping_add(1usize);
                }
                'loop55: loop {
                    if !(i < width as (usize)) {
                        break;
                    }
                    WriteString(
                        &mut out as (*mut *mut u8),
                        &mut outMaxLen as (*mut usize),
                        (*b"  \0").as_ptr(),
                    );
                    i = i.wrapping_add(1usize);
                }
            } else {
                if width == 0u32 {
                    width = ::std::mem::size_of::<*mut ::std::os::raw::c_void>()
                        .wrapping_mul(2usize) as (u32);
                }
                WriteHex(
                    &mut out as (*mut *mut u8),
                    &mut outMaxLen as (*mut usize),
                    addr,
                    width,
                    false,
                );
            }
        } else {
            WriteChar(
                &mut out as (*mut *mut u8),
                &mut outMaxLen as (*mut usize),
                *fmt,
            );
        }
        fmt = fmt.offset(1isize);
    }
    len = ((out as (isize)).wrapping_sub(start as (isize)) /
               ::std::mem::size_of::<u8>() as (isize)) as (usize);
    if outMaxLen > 0usize {
        *{
            let _old = out;
            let _out = out.offset(1isize);
            _old
        } = 0u8;
    }
    len
}

#[no_mangle]
pub unsafe extern "C" fn DisassembleToString16(
    out: *mut u8,
    outMaxLen: usize,
    fmt: *const u8,
    opcode: *const u8,
    addr: usize,
    maxLen: usize,
    instr: *mut Instruction,
) -> usize {
    if !Disassemble16(opcode, addr, maxLen, instr) {
        0usize
    } else {
        FormatInstructionString(
            out,
            outMaxLen,
            fmt,
            opcode,
            addr,
            instr as (*const Instruction),
        )
    }
}

#[no_mangle]
pub unsafe extern "C" fn DisassembleToString32(
    out: *mut u8,
    outMaxLen: usize,
    fmt: *const u8,
    opcode: *const u8,
    addr: usize,
    maxLen: usize,
    instr: *mut Instruction,
) -> usize {
    if !Disassemble32(opcode, addr, maxLen, instr) {
        0usize
    } else {
        FormatInstructionString(
            out,
            outMaxLen,
            fmt,
            opcode,
            addr,
            instr as (*const Instruction),
        )
    }
}

#[no_mangle]
pub unsafe extern "C" fn DisassembleToString64(
    out: *mut u8,
    outMaxLen: usize,
    fmt: *const u8,
    opcode: *const u8,
    addr: usize,
    maxLen: usize,
    instr: *mut Instruction,
) -> usize {
    if !Disassemble64(opcode, addr, maxLen, instr) {
        0usize
    } else {
        FormatInstructionString(
            out,
            outMaxLen,
            fmt,
            opcode,
            addr,
            instr as (*const Instruction),
        )
    }
}
