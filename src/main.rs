#![no_std] // Don't link the Rust standard library
#![no_main] // Disable all Rust-level entry points

mod vga_buffer;

use core::panic::PanicInfo;

/// This function is called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to Ferrox!");
    loop {}
}
