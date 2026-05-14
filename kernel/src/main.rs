// Krisha OS — Day 4: Shell Scaffolding + Indian Kaal 🦚🕉️

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use bootloader_api::{entry_point, BootInfo};

// Tell Rust about our modules
mod vga_buffer;   // Output driver (port 0xe9)
mod shell;        // Command shell scaffolding
mod time;         // CMOS RTC + Hindu calendar

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    // Boot banner
    println!("=========================================");
    println!("  Krisha OS v0.1.0");
    println!("  Intelligence, by grace.");
    println!("=========================================");
    println!();
    println!("[boot] kernel_main reached");
    println!("[boot] println! macro online");
    println!("[boot] kaal module online (CMOS RTC + Panchang)");
    println!("[boot] shell module online");
    println!("[boot] launching shell demo...");

    // Run the shell demo session
    shell::run_demo();

    println!("[boot] all systems nominal");
    println!("[boot] krisha ready for next lesson");

    qemu_exit_success();

    loop {
        unsafe { core::arch::asm!("hlt", options(nomem, nostack)); }
    }
}

/// Exit QEMU cleanly with success code 33
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
    loop {
        unsafe { core::arch::asm!("hlt", options(nomem, nostack)); }
    }
}