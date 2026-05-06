// kernel/src/time/calendar.rs
//
// Krisha OS — Hindu Calendar (Panchang) 🕉️
//
// Sanskrit/Hindi names for time, days, months, and seasons.

use super::DateTime;
use crate::println;
use crate::print;

/// English month names for cal header
pub const ENGLISH_MONTHS: [&str; 12] = [
    "January", "February", "March", "April",
    "May", "June", "July", "August",
    "September", "October", "November", "December",
];

// ─────────────────────────────────────────────────────────────
// Panchang (Full Hindu Almanac)
// ─────────────────────────────────────────────────────────────

/// Tithi names — 15 lunar days per paksha (waxing/waning half)
pub const TITHI_NAMES: [&str; 15] = [
    "Pratipada", "Dwitiya", "Tritiya", "Chaturthi", "Panchami",
    "Shashthi", "Saptami", "Ashtami", "Navami", "Dashami",
    "Ekadashi", "Dwadashi", "Trayodashi", "Chaturdashi", "Purnima/Amavasya",
];

/// Approximate tithi from day number
pub fn approx_tithi(day: u8) -> (&'static str, &'static str) {
    let tithi_num = (day as usize - 1) % 15;
    let paksha = if day <= 15 { "Shukla" } else { "Krishna" };
    (paksha, TITHI_NAMES[tithi_num])
}

/// What each Vaar is auspicious for (Vedic tradition)
pub fn vaar_significance(vaar_index: u8) -> &'static str {
    match vaar_index {
        0 => "health, leadership, royal matters",
        1 => "peace, water, mother, mind",
        2 => "courage, strength, conflict resolution",
        3 => "learning, communication, business",
        4 => "wisdom, teachers, spirituality",
        5 => "love, beauty, art, comfort",
        6 => "discipline, hard work, justice",
        _ => "general activities",
    }
}

/// Approximate sunrise/sunset for India (IST)
pub fn approx_sunrise_sunset() -> (&'static str, &'static str) {
    ("~05:45 IST", "~18:55 IST")
}

/// Print the full panchang (today's almanac)
pub fn print_panchang(dt: &DateTime) {
    let dow = day_of_week(dt.year, dt.month, dt.day);
    let vaar = VAAR[dow as usize];
    let (paksha, tithi) = approx_tithi(dt.day);
    let significance = vaar_significance(dow);
    let (sunrise, sunset) = approx_sunrise_sunset();
    let samvat = vikram_samvat(dt.year, dt.month);
    let maas = MAAS[maas_index(dt.month)];

    println!();
    println!("  +===== Aaj ka Panchang (Today's Almanac) =====+");
    println!("  |                                              |");
    println!("  |  Vaar:        {} ", vaar);
    println!("  |  Maas:        {} ", maas);
    println!("  |  Paksha:      {} (Diwas {})", paksha, dt.day);
    println!("  |  Tithi:       {} (approx)", tithi);
    println!("  |  Samvat:      Vikram {}", samvat);
    println!("  |  Varsh:       {} CE", dt.year);
    println!("  |                                              |");
    println!("  |  Sunrise:     {} (estimate)", sunrise);
    println!("  |  Sunset:      {} (estimate)", sunset);
    println!("  |                                              |");
    println!("  |  Auspicious for:                             |");
    println!("  |    {}", significance);
    println!("  |                                              |");
    println!("  |  [Note: precise nakshatra needs lunar math]  |");
    println!("  +==============================================+");
    println!();
}

/// How many days in a given month (handles leap years)
pub fn days_in_month(year: u16, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            let y = year as u32;
            if (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

/// Vaar (वार) — Day of the week in Sanskrit
/// Index 0 = Sunday, ..., 6 = Saturday
pub const VAAR: [&str; 7] = [
    "Ravivaar",      // रविवार (Sun-day)
    "Somvaar",       // सोमवार (Moon-day)
    "Mangalvaar",    // मंगलवार (Mars-day)
    "Budhvaar",      // बुधवार (Mercury-day)
    "Guruvaar",      // गुरुवार (Jupiter-day)
    "Shukravaar",    // शुक्रवार (Venus-day)
    "Shanivaar",     // शनिवार (Saturn-day)
];

pub const VAAR_ENGLISH: [&str; 7] = [
    "Sunday", "Monday", "Tuesday", "Wednesday",
    "Thursday", "Friday", "Saturday",
];

/// Maas (मास) — Hindu calendar months
pub const MAAS: [&str; 12] = [
    "Chaitra",       // चैत्र (March-April)
    "Vaishakh",      // वैशाख (April-May)
    "Jyeshtha",      // ज्येष्ठ (May-June)
    "Ashadh",        // आषाढ़ (June-July)
    "Shravan",       // श्रावण (July-August)
    "Bhadrapad",     // भाद्रपद (August-September)
    "Ashwin",        // आश्विन (September-October)
    "Kartik",        // कार्तिक (October-November)
    "Margashirsha",  // मार्गशीर्ष (November-December)
    "Paush",         // पौष (December-January)
    "Magh",          // माघ (January-February)
    "Phalgun",       // फाल्गुन (February-March)
];

/// Kaal (काल) — Time of day greetings
pub fn kaal_greeting(hour: u8) -> &'static str {
    match hour {
        4..=11 => "Shubh Prabhat",   // शुभ प्रभात (Good Morning)
        12..=15 => "Shubh Madhyahn", // शुभ मध्याह्न (Good Noon/Afternoon)
        16..=18 => "Shubh Sandhya",  // शुभ सन्ध्या (Good Evening)
        _ => "Shubh Ratri",          // शुभ रात्रि (Good Night)
    }
}

/// Kaal name for the time period
pub fn kaal_name(hour: u8) -> &'static str {
    match hour {
        4..=11 => "Prabhat-kaal",    // प्रभात काल (Morning Time)
        12..=15 => "Madhyahn-kaal",  // मध्याह्न काल (Noon Time)
        16..=18 => "Sandhya-kaal",   // सन्ध्या काल (Evening Time)
        _ => "Ratri-kaal",           // रात्रि काल (Night Time)
    }
}

/// Day of week using Zeller's congruence (real algorithm!)
pub fn day_of_week(year: u16, month: u8, day: u8) -> u8 {
    let mut y = year as i32;
    let mut m = month as i32;

    if m < 3 {
        m += 12;
        y -= 1;
    }

    let k = y % 100;
    let j = y / 100;
    let h = (day as i32 + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 + 5 * j) % 7;
    ((h + 6) % 7) as u8
}

/// Vikram Samvat year (Hindu era year)
/// Vikram = Gregorian + 56 (before April) or +57 (April onwards)
pub fn vikram_samvat(year: u16, month: u8) -> u16 {
    if month >= 4 {
        year + 57
    } else {
        year + 56
    }
}

/// Map Gregorian month to Hindu month index
pub fn maas_index(gregorian_month: u8) -> usize {
    match gregorian_month {
        3 => 0, 4 => 1, 5 => 2, 6 => 3,
        7 => 4, 8 => 5, 9 => 6, 10 => 7,
        11 => 8, 12 => 9, 1 => 10, 2 => 11,
        _ => 0,
    }
}

/// Print the beautiful Indian-style date
pub fn print_kaal(dt: &DateTime) {
    let dow = day_of_week(dt.year, dt.month, dt.day);
    let vaar = VAAR[dow as usize];
    let vaar_en = VAAR_ENGLISH[dow as usize];
    let samvat = vikram_samvat(dt.year, dt.month);
    let maas = MAAS[maas_index(dt.month)];
    let greeting = kaal_greeting(dt.hour);
    let kaal = kaal_name(dt.hour);

    println!();
    println!("  +-- Krisha Kaal --------------------------+");
    println!("  |                                         |");
    println!("  |  {}, brother! 🙏", greeting);
    println!("  |  ({})                       ", kaal);
    println!("  |                                         |");
    println!("  |  Vaar:    {} ({})", vaar, vaar_en);
    println!("  |  Tithi:   {} (Diwas {})", maas, dt.day);
    println!("  |  Samvat:  Vikram {}", samvat);
    println!("  |  Varsh:   {} CE", dt.year);
    println!("  |  Samay:   {:02}:{:02}:{:02} UTC", dt.hour, dt.minute, dt.second);
    println!("  |                                         |");
    println!("  +-----------------------------------------+");
    println!();
}

/// Print month calendar with Hindu overlay
pub fn print_cal(dt: &DateTime) {
    let total_days = days_in_month(dt.year, dt.month);
    let first_day = day_of_week(dt.year, dt.month, 1);
    let english_month = ENGLISH_MONTHS[(dt.month - 1) as usize];
    let hindu_month = MAAS[maas_index(dt.month)];
    let samvat = vikram_samvat(dt.year, dt.month);
    let today = dt.day;

    println!();
    println!("  +-------------------------------------------+");
    println!("  |  {} {}  /  {} {}",
        english_month, dt.year, hindu_month, samvat);
    println!("  +-------------------------------------------+");
    println!("  |                                           |");
    println!("  |  Ravi Som  Mang Budh Guru Shuk Shan       |");
    println!("  |                                           |");

    print!("  |  ");
    // Blank cells before day 1
    for _ in 0..first_day {
        print!("     ");
    }

    let mut current_col = first_day;
    for day in 1..=total_days {
        if day == today {
            print!("[{:2}] ", day);
        } else {
            print!(" {:2}  ", day);
        }
        current_col += 1;
        if current_col == 7 {
            println!(" |");
            if day < total_days {
                print!("  |  ");
            }
            current_col = 0;
        }
    }
    if current_col != 0 {
        for _ in current_col..7 {
            print!("     ");
        }
        println!(" |");
    }

    let today_vaar = VAAR[day_of_week(dt.year, dt.month, today) as usize];
    println!("  |                                           |");
    println!("  |  Today: Diwas {} - {}", today, today_vaar);
    println!("  +-------------------------------------------+");
    println!();
}