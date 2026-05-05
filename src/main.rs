// Krisha OS — Lesson 1: Freestanding Binary
//
// 🦚 Intelligence, by grace.
// This is the very first version — minimal, almost nothing.
// Just proves we can build Rust code without an OS.

#![no_std]      // No standard library
#![no_main]     // No normal main() entry

use core::panic::PanicInfo;

/// Called on panic. Since we have no OS, just loop.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Our entry point — replaces main().
/// The linker looks for a function called `_start`.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Krisha is alive! (But silent for now.)
    loop {}
}
