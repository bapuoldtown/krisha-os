// Krisha OS — Lesson 3 🦚

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use bootloader_api::{entry_point, BootInfo};

mod vga_buffer;

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    println!("=========================================");
    println!("  Krisha OS v0.1.0");
    println!("  Intelligence, by grace.");
    println!("=========================================");
    println!();
    println!("[boot] kernel_main reached");
    println!("[boot] println! macro working");
    println!("Hello from Krisha!");
    println!();
    println!("Numbers work: {}, {}, {}", 1, 42, 100);
    println!("Krisha OS booted successfully via println!");

    qemu_exit_success();

    loop {
        unsafe { core::arch::asm!("hlt", options(nomem, nostack)); }
    }
}

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