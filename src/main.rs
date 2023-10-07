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
static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // start address of buffer
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        // why unsafe? -> rust compiler doesn't know whether its pointer is valid or not
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}


