// kernel/src/time/calendar.rs
//
// Krisha OS — Hindu Calendar (Panchang) 🕉️
//
// Sanskrit/Hindi names for time, days, months, and seasons.

use super::DateTime;
use crate::println;

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