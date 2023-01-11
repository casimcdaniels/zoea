#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// main entry-point function
#[no_mangle] // don't mangle the name of the function when compiling
pub extern "C" fn _start() -> ! {
    loop {}
}

/// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
