#![no_std]
#![no_main]

extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};
use core::panic::PanicInfo;
use xv6_labs_rs::unistd::{exit, free, malloc, write};

struct XV6Allocator;

unsafe impl GlobalAlloc for XV6Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        malloc(layout.size() as u32)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        free(ptr);
    }
}

#[global_allocator]
static ALLOCATOR: XV6Allocator = XV6Allocator;

fn print(s: &str) {
    unsafe {
        write(1, s.as_ptr(), s.len() as i32);
    }
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    // maybe return to xv6, i don't know
    unsafe { exit(1_i32) }
}

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    let mut hello = b"hello world\n".to_vec();

    // print(core::str::from_utf8(&hello).unwrap());
    if let Ok(s) = core::str::from_utf8(&hello) {
        print(s);
    }

    hello.push(b'1');
    hello.push(b'2');
    hello.push(b'3');
    hello.push(b'\n');

    // print(core::str::from_utf8(&hello).unwrap());
    if let Ok(s) = core::str::from_utf8(&hello) {
        print(s);
    }

    unsafe { exit(0_i32) }
}
