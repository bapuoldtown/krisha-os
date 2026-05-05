// Krisha OS — Lesson 3: Real Screen Output 🦚
//
// Now using a proper VGA buffer driver with println!()

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use bootloader_api::{entry_point, BootInfo};

// Tell Rust about our submodule (vga_buffer.rs)
mod vga_buffer;

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    // The first time println!() is called, it auto-initializes the WRITER
    println!("=========================================");
    println!("  Krisha OS v0.1.0");
    println!("  Intelligence, by grace.  ");
    println!("=========================================");
    println!();
    println!("[boot] kernel_main() reached");
    println!("[boot] VGA buffer driver online");
    println!("[boot] println! macro working");
    println!();
    println!("Hello from Krisha! ");
    println!();
    println!("This text is being written directly to");
    println!("the VGA text buffer at memory 0xb8000");
    println!("Each character takes 2 bytes:");
    println!("  - Byte 1: the ASCII character");
    println!("  - Byte 2: foreground/background color");
    println!();

    // Also send to the QEMU debug port so we see it in CI logs
    debug_print(b"Krisha OS booted via VGA buffer.\n");

    // Exit QEMU with success (so CI can verify)
    qemu_exit_success();

    loop {
        unsafe { core::arch::asm!("hlt", options(nomem, nostack)); }
    }
}

/// Send bytes to QEMU debug console (port 0xe9).
/// Useful for CI verification since CI can't see VGA buffer.
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
fn panic(info: &PanicInfo) -> ! {
    // Try to print the panic info to screen
    println!("\n[KERNEL PANIC] {}", info);

    loop {
        unsafe { core::arch::asm!("hlt", options(nomem, nostack)); }
    }
}