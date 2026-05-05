// kernel/src/vga_buffer.rs - Krisha OS Output Driver 🦚
// Uses QEMU's port 0xe9 for output (legacy VGA buffer not mapped)

use core::fmt;
use spin::Mutex;
use lazy_static::lazy_static;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Yellow = 14,
    White = 15,
}

pub struct Writer {
    foreground: Color,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        unsafe {
            core::arch::asm!(
                "out dx, al",
                in("dx") 0xe9_u16,
                in("al") byte,
                options(nomem, nostack, preserves_flags)
            );
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        foreground: Color::Yellow,
    });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}