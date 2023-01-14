#![no_std]
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello, world!";

/// main entry-point function
#[no_mangle] // don't mangle the name of the function when compiling
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &b) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = b;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // Set the color to light cyan
        }
    }
    loop {}
}

/// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
