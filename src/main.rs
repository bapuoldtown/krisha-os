// Krisha OS — Lesson 2 (Simplified): Bare metal kernel
// 🦚 Intelligence, by grace.

#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Write to QEMU's debug exit port to signal "we're alive!"
    // QEMU port 0xe9 prints to stdout when -debugcon is enabled.
    let msg = b"Krisha OS v0.1.0 - Intelligence, by grace.\n";
    for &byte in msg {
        unsafe {
            core::arch::asm!(
                "out dx, al",
                in("dx") 0xe9_u16,
                in("al") byte,
            );
        }
    }
    
    // Exit QEMU cleanly with success code
    unsafe {
        core::arch::asm!(
            "out dx, al",
            in("dx") 0xf4_u16,
            in("al") 0x10_u8,  // Success exit code
        );
    }
    
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}