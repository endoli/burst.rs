#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate burst;

use burst::x86::*;

fuzz_target!(|data: &[u8]| {
    if let Ok(instr) = disassemble_64(data, 0, data.len()) {
        let mut out = String::new();
        format_instruction_string(&mut out, "%a %b %i %o", Some(data), 0, &instr).unwrap();
    };
});
