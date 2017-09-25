#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate burst;

use burst::x86::*;

fuzz_target!(|data: &[u8]| {
    let result = &mut Instruction::default() as *mut Instruction;
    unsafe {
        Disassemble64(data.as_ptr(), 0, data.len(), result);
    }
});
