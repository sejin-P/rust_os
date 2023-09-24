#![no_std]
// To tell the Rust compiler that we don’t want to use the normal entry point chain, we add the #![no_main] attribute.
#![no_main]

use core::panic::PanicInfo;

// returning ! means it's diverging func.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// main doesn’t make sense without an underlying runtime that calls it.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}


