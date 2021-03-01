#![no_std]
#![no_builtins]
#![feature(alloc_error_handler)]
/* Modules defined here */
pub mod hal;
pub mod freertos;
pub mod app;

/* Link lazy_static*/ 
#[macro_use]
extern crate lazy_static;

/* Linking FreeRTOS Allocator*/
extern crate alloc;
use freertos::alloc::FreeRtosAllocator;
#[global_allocator] // set the global allocator
static ALLOCATOR: FreeRtosAllocator = FreeRtosAllocator;
/* Linking Wrapper Crate */
extern crate freertos_rs;
use freertos_rs::*;



#[no_mangle]
pub extern "C" fn prust_main(){
    loop{
        app::handle_packets();
    };
}


use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[alloc_error_handler]
fn oom_handler(_: core::alloc::Layout) -> ! {
    panic!("OOM!");
}
