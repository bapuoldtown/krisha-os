// Krisha OS — Lesson 2: The First Boot 🦚

#![no_std]
#![no_main]

use core::panic::PanicInfo;

static GREETING: &[u8] = b"Krisha OS v0.1.0 - Intelligence, by grace. We are booting a simple micro kernel via QEMU enulator in x86 architecture. This is the first step to build a simple OS from scratch. Welcome to the world of OS development!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in GREETING.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0x0e;
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}