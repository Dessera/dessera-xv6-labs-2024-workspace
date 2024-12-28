#![no_std]
#![no_main]
use core::panic::PanicInfo;

extern "C" {
    // fn printf(s: *const u8);
    fn write(fd: i32, buf: *const u8, count: usize) -> isize;
    fn exit(status: i32) -> i32;
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    // maybe return to xv6, i don't know
    unsafe {
        exit(1_i32);
    }

    loop {}
}

fn print_str(s: &str) {
    unsafe {
        write(1, s.as_bytes().as_ptr(), s.len());
    }
}

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    let foos = xv6_labs_rs::foo();
    print_str(foos);
    unsafe {
        // printf("hello world from Rust!!\n".as_bytes().as_ptr());
        exit(0_i32)
    }
}
