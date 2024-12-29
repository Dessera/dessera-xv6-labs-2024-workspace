extern crate alloc as alloc_crate;
use crate::_cunistd;
use core::alloc::{GlobalAlloc, Layout};

pub use alloc_crate::*;

pub struct XV6Allocator;

unsafe impl GlobalAlloc for XV6Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        _cunistd::malloc(layout.size() as u32)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        _cunistd::free(ptr);
    }
}

#[global_allocator]
pub static ALLOCATOR: XV6Allocator = XV6Allocator;
