#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate burst;

use burst::x86::*;

fuzz_target!(|data: &[u8]| {
    let mut result = Instruction::default();
    unsafe {
        Disassemble64(data.as_ptr(), 0, data.len(), &mut result);
    }
});
