// kernel/src/time/fortune.rs
//
// Krisha OS — Sanskrit Shloka Fortune 📿
// Picks a shloka based on CPU uptime ticks

use crate::println;

/// Collection of inspiring Sanskrit shlokas
/// Tuple: (Sanskrit transliteration, English meaning, Source)
pub const SHLOKAS: [(&str, &str, &str); 10] = [
    (
        "Yog karmasu kaushalam",
        "Yoga is skill in action",
        "Bhagavad Gita 2.50",
    ),
    (
        "Vasudhaiva kutumbakam",
        "The world is one family",
        "Maha Upanishad 6.71",
    ),
    (
        "Satyameva jayate",
        "Truth alone triumphs",
        "Mundaka Upanishad 3.1.6",
    ),
    (
        "Aham brahmasmi",
        "I am the universe",
        "Brihadaranyaka Upanishad",
    ),
    (
        "Tat tvam asi",
        "Thou art that",
        "Chandogya Upanishad 6.8.7",
    ),
    (
        "Karmanye vadhikaraste, ma phaleshu kadachana",
        "You have right to action, not to its fruits",
        "Bhagavad Gita 2.47",
    ),
    (
        "Sarve bhavantu sukhinah",
        "May all beings be happy",
        "Universal Prayer",
    ),
    (
        "Asato ma sad gamaya",
        "Lead me from untruth to truth",
        "Brihadaranyaka Upanishad 1.3.28",
    ),
    (
        "Tamaso ma jyotir gamaya",
        "Lead me from darkness to light",
        "Brihadaranyaka Upanishad 1.3.28",
    ),
    (
        "Vidya dadati vinayam",
        "Knowledge gives humility",
        "Hitopadesha",
    ),
];

/// Pick a shloka using CPU uptime as randomness
pub fn random_shloka() -> &'static (&'static str, &'static str, &'static str) {
    let ticks = super::uptime_ticks();
    let index = (ticks as usize) % SHLOKAS.len();
    &SHLOKAS[index]
}

/// Print a beautiful fortune
pub fn print_fortune() {
    let (sanskrit, english, source) = random_shloka();

    println!();
    println!("  +-- || Shloka || ---------------------------+");
    println!("  |                                            |");
    println!("  |  \"{}\"", sanskrit);
    println!("  |     ({})", english);
    println!("  |                                            |");
    println!("  |              -- {}", source);
    println!("  |                                            |");
    println!("  +--------------------------------------------+");
    println!();
}