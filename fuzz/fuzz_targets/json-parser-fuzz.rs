#![no_main]
use libfuzzer_sys::fuzz_target;
use json_typegen_shared::parse::*;
use std::str;

fuzz_target!(|data: &[u8]| {
    if data.len() > 0 {
        let opt = data[0] & 2;
        match str::from_utf8(&data[1..]) {
            Ok(in_string) => {
                if opt == 0 {
                    full_macro(in_string);
                } else if opt == 1 {
                    macro_input(in_string);
                }
            }
            Err(..) => (),
        }
    }
});
