#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate burst;

use burst::x86::*;

fuzz_target!(|data: &[u8]| {
    let result = &mut Instruction::default() as *mut Instruction;
    assert!(unsafe {
        Disassemble64(data.as_ptr(), 0, data.len(), result)
    });
    let mut out = String::new();
    let fmt = "%a %b %i %o";
    unsafe {
        assert!(
            DisassembleToString64(&mut out, fmt.as_ptr(), data.as_ptr(), 0, data.len(), result)
                .is_ok()
        );
    }
});
