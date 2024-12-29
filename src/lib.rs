#![no_std]

pub mod _cunistd;
pub mod alloc;
pub mod io;

pub mod main {
    use crate::_cunistd;
    use core::panic::PanicInfo;

    #[panic_handler]
    pub fn panic(_panic: &PanicInfo<'_>) -> ! {
        unsafe { _cunistd::exit(1_i32) }
    }

    pub fn exit(status: i32) -> ! {
        unsafe { _cunistd::exit(status) }
    }
}
