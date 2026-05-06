// kernel/src/time/mod.rs
//
// Krisha OS — Kaal Module 🕉️
// Reads the Real-Time Clock (RTC) from CMOS hardware

pub mod calendar;

const RTC_SECONDS: u8 = 0x00;
const RTC_MINUTES: u8 = 0x02;
const RTC_HOURS: u8 = 0x04;
const RTC_DAY: u8 = 0x07;
const RTC_MONTH: u8 = 0x08;
const RTC_YEAR: u8 = 0x09;
const RTC_STATUS_B: u8 = 0x0B;

const CMOS_ADDRESS: u16 = 0x70;
const CMOS_DATA: u16 = 0x71;

#[derive(Debug, Clone, Copy)]
pub struct DateTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

unsafe fn cmos_read(reg: u8) -> u8 {
    let mut value: u8;
    core::arch::asm!(
        "out dx, al",
        in("dx") CMOS_ADDRESS,
        in("al") (reg | 0x80),
        options(nomem, nostack, preserves_flags)
    );
    core::arch::asm!(
        "in al, dx",
        out("al") value,
        in("dx") CMOS_DATA,
        options(nomem, nostack, preserves_flags)
    );
    value
}

fn bcd_to_dec(value: u8) -> u8 {
    (value & 0x0F) + ((value >> 4) * 10)
}

unsafe fn wait_for_rtc() {
    loop {
        core::arch::asm!(
            "out dx, al",
            in("dx") CMOS_ADDRESS,
            in("al") 0x8A_u8,
            options(nomem, nostack, preserves_flags)
        );
        let mut status: u8;
        core::arch::asm!(
            "in al, dx",
            out("al") status,
            in("dx") CMOS_DATA,
            options(nomem, nostack, preserves_flags)
        );
        if status & 0x80 == 0 {
            break;
        }
    }
}

pub fn now() -> DateTime {
    unsafe {
        wait_for_rtc();

        let second_raw = cmos_read(RTC_SECONDS);
        let minute_raw = cmos_read(RTC_MINUTES);
        let hour_raw = cmos_read(RTC_HOURS);
        let day_raw = cmos_read(RTC_DAY);
        let month_raw = cmos_read(RTC_MONTH);
        let year_raw = cmos_read(RTC_YEAR);
        let status_b = cmos_read(RTC_STATUS_B);

        let is_binary = status_b & 0x04 != 0;

        let (second, minute, hour, day, month, year) = if is_binary {
            (second_raw, minute_raw, hour_raw, day_raw, month_raw, year_raw)
        } else {
            (
                bcd_to_dec(second_raw),
                bcd_to_dec(minute_raw),
                bcd_to_dec(hour_raw & 0x7F),
                bcd_to_dec(day_raw),
                bcd_to_dec(month_raw),
                bcd_to_dec(year_raw),
            )
        };

        DateTime {
            year: 2000 + year as u16,
            month,
            day,
            hour,
            minute,
            second,
        }
    }
}

pub fn uptime_ticks() -> u64 {
    unsafe {
        let mut high: u32;
        let mut low: u32;
        core::arch::asm!(
            "rdtsc",
            out("edx") high,
            out("eax") low,
            options(nomem, nostack, preserves_flags)
        );
        ((high as u64) << 32) | (low as u64)
    }
}