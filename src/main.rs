// Krisha OS — Lesson 2: First Boot 🦚
// Multiboot-compliant kernel that QEMU can boot directly.

#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Multiboot v1 header — required for QEMU's -kernel flag
// Magic: 0x1BADB002, Flags: 0, Checksum: -(magic + flags)
#[link_section = ".multiboot"]
#[used]
static MULTIBOOT_HEADER: [u32; 3] = [
    0x1BADB002,           // Magic number
    0x00000000,           // Flags
    0xE4524FFE,           // Checksum (-(magic + flags))
];

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Print "Krisha OS" via QEMU's debug console (port 0xe9)
    let msg = b"Krisha OS v0.1.0 - Intelligence, by grace. \xF0\x9F\xA6\x9A\n";
    
    for &byte in msg {
        unsafe {
            core::arch::asm!(
                "out dx, al",
                in("dx") 0xe9_u16,
                in("al") byte,
                options(nomem, nostack, preserves_flags)
            );
        }
    }
    
    // Signal success and exit QEMU via isa-debug-exit device
    unsafe {
        core::arch::asm!(
            "out dx, al",
            in("dx") 0xf4_u16,
            in("al") 0x10_u8,
            options(nomem, nostack, preserves_flags)
        );
    }
    
    loop {
        unsafe { core::arch::asm!("hlt", options(nomem, nostack)); }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe { core::arch::asm!("hlt", options(nomem, nostack)); }
    }
}