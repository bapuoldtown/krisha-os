// Krisha OS — Lesson 3: VGA Buffer (Defensive Mode) 🦚

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use bootloader_api::{entry_point, BootInfo};

mod vga_buffer;

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    // PHASE 1: Confirm we're alive via port 0xe9 (this WORKS in CI)
    debug_print(b"[boot] kernel_main reached\n");
    debug_print(b"[boot] about to test VGA buffer\n");

    // PHASE 2: Try VGA buffer (might triple-fault if not mapped!)
    println!("Krisha OS v0.1.0 - VGA test");
    println!("Intelligence, by grace.");

    // PHASE 3: If we got here, VGA worked!
    debug_print(b"[boot] VGA buffer OK!\n");
    debug_print(b"Krisha OS booted via VGA buffer.\n");

    // Exit QEMU with success
    qemu_exit_success();

    loop {
        unsafe { core::arch::asm!("hlt", options(nomem, nostack)); }
    }
}

/// Send bytes to QEMU debug console (port 0xe9).
fn debug_print(bytes: &[u8]) {
    for &byte in bytes {
        unsafe {
            core::arch::asm!(
                "out dx, al",
                in("dx") 0xe9_u16,
                in("al") byte,
                options(nomem, nostack, preserves_flags)
            );
        }
    }
}

/// Exit QEMU with success code 33.
fn qemu_exit_success() {
    unsafe {
        core::arch::asm!(
            "out dx, al",
            in("dx") 0xf4_u16,
            in("al") 0x10_u8,
            options(nomem, nostack, preserves_flags)
        );
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Try debug print first (safer than VGA)
    debug_print(b"\n[KERNEL PANIC]\n");
    
    loop {
        unsafe { core::arch::asm!("hlt", options(nomem, nostack)); }
    }
}