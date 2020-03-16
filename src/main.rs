#![no_std] // Don't link the Rust standard library
#![no_main] // Disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(ferrox::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use ferrox::println;

/// This function is called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// Panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ferrox::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to Ferrox!");

    #[cfg(test)]
    test_main();

    loop {}
}
