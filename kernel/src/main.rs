// Krisha OS — Kernel 🦚

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use bootloader_api::{entry_point, BootInfo};

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    let msg = b"Krisha OS v0.1.0 - Intelligence, by grace.\n";
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