#![no_std]
#![no_main]

use std_xv6::alloc::format;
use std_xv6::io::{BufWriter, File, Write};
use std_xv6::main::exit;

#[no_mangle]
pub extern "C" fn main(argc: i32, argv: *const *const u8) -> i32 {
    let mut stdout = File::new(1);

    let some_str = format!("hello, {}!\n", "world");

    stdout.write(some_str.as_bytes()).unwrap();

    exit(0);
}
