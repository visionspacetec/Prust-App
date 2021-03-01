#[allow(non_camel_case_types)]
pub type size_t = usize;

use cty::c_void;

extern "C" {
    fn pvPortMalloc(size: size_t) -> *mut c_void;
    fn vPortFree(p: *mut c_void);	
}

use core::alloc::{GlobalAlloc, Layout};


pub struct FreeRtosAllocator;

unsafe impl GlobalAlloc for FreeRtosAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        pvPortMalloc(layout.size()) as *mut u8
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        vPortFree(ptr as *mut c_void)
    }
}