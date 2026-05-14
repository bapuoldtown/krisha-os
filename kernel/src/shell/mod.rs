// kernel/src/shell/mod.rs
//
// Krisha OS Shell — the user-facing command interpreter 🦚

use crate::println;

pub mod commands;
pub mod parser;
pub mod art;        // ← NEW: ASCII art module

pub const PROMPT: &str = "krisha> ";

/// Run a static demo of what the shell will look like.
pub fn run_demo() {
    println!();
    println!("+----------------------------------------------+");
    println!("|    Krisha Shell - Scaffolding Demo 🦚        |");
    println!("|    (Real input coming in Lesson 5)           |");
    println!("+----------------------------------------------+");
    println!();

    // Greeting commands
    demo_command("hello");
    demo_command("namaste");

    // System info
    demo_command("info");
    demo_command("krisha");

    // Time & calendar
    demo_command("date");
    demo_command("kaal");
    demo_command("cal");
    demo_command("panjika");
    demo_command("panchang");
    demo_command("uptime");

    // Wisdom
    demo_command("fortune");

    // Misc
    demo_command("echo Krisha lives");
    demo_command("ls /vfs");
    demo_command("mem");
    demo_command("ai status");
    demo_command("help");

    println!();
    println!("[shell] demo session complete");
    println!("[shell] in Lesson 5, you will type commands yourself!");
    println!();
}

fn demo_command(cmd: &str) {
    println!("{}{}", PROMPT, cmd);
    commands::run(cmd);
    println!();
}