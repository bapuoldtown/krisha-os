// kernel/src/shell/commands.rs
//
// Krisha Shell — built-in commands 🦚

use crate::println;
use crate::time;

pub fn run(cmd: &str) {
    let trimmed = cmd.trim();

    if trimmed == "hello" {
        cmd_hello();
    } else if trimmed == "info" {
        cmd_info();
    } else if trimmed == "date" {
        cmd_date();
    } else if trimmed == "uptime" {
        cmd_uptime();
    } else if trimmed.starts_with("color") {
        cmd_color(trimmed);
    } else if trimmed.starts_with("echo") {
        cmd_echo(trimmed);
    } else if trimmed.starts_with("ls") {
        cmd_ls(trimmed);
    } else if trimmed.starts_with("cat") {
        cmd_cat(trimmed);
    } else if trimmed.starts_with("mkdir") {
        cmd_mkdir(trimmed);
    } else if trimmed == "mem" {
        cmd_mem();
    } else if trimmed.starts_with("ai") {
        cmd_ai(trimmed);
    } else if trimmed == "help" {
        cmd_help();
    } else if trimmed == "shutdown" {
        cmd_shutdown();
    } else {
        println!("krisha: command not found: {}", trimmed);
        println!("        type 'help' for available commands");
    }
}

fn cmd_hello() {
    println!("Namaste, brother! 🙏");
    println!("Krisha greets you with grace.");
}

fn cmd_info() {
    println!("Krisha OS v0.1.0 - Intelligence, by grace.");
    println!("  Architecture: x86_64");
    println!("  Memory:       256 MB");
    println!("  Kernel:       Rust no_std");
    println!("  Bootloader:   bootloader 0.11");
    println!("  Status:       Day 4 - shell scaffolding");
}

/// Beautiful date command with Hindu calendar (Panchang)!
fn cmd_date() {
    println!("[rtc] reading from CMOS chip...");
    let now = time::now();
    time::calendar::print_kaal(&now);
}

fn cmd_uptime() {
    let ticks = time::uptime_ticks();
    println!("[tsc] CPU cycles since reset: {}", ticks);
    println!("       (precise wall-clock uptime needs Lesson 5 timer)");
}

fn cmd_color(cmd: &str) {
    let arg = cmd.strip_prefix("color").unwrap_or("").trim();
    if arg.is_empty() {
        println!("usage: color <name>");
        println!("       names: yellow, white, green, red, blue, cyan");
    } else {
        println!("[color stub] would set color to: {}", arg);
        println!("[color stub] Lesson 5+ will make this work for real");
    }
}

fn cmd_echo(cmd: &str) {
    let text = cmd.strip_prefix("echo").unwrap_or("").trim();
    println!("{}", text);
}

fn cmd_ls(cmd: &str) {
    let path = cmd.strip_prefix("ls").unwrap_or("").trim();
    let display_path = if path.is_empty() { "/vfs" } else { path };

    println!("[vfs] listing {}:", display_path);

    if display_path == "/vfs" || path.is_empty() {
        println!("  greeting.txt        37 bytes");
        println!("  notes.txt           512 bytes");
        println!("  todo.md             128 bytes");
        println!("  myproject/          (directory)");
    } else {
        println!("  (empty or path not found)");
    }

    println!("[vfs stub] real filesystem coming in Lesson 9");
}

fn cmd_cat(cmd: &str) {
    let path = cmd.strip_prefix("cat").unwrap_or("").trim();
    if path.is_empty() {
        println!("usage: cat <file>");
        return;
    }

    if path == "/vfs/greeting.txt" {
        println!("Hello from Krisha OS!");
        println!("Intelligence, by grace. 🦚");
    } else {
        println!("[vfs stub] cat {} - not yet implemented", path);
    }
}

fn cmd_mkdir(cmd: &str) {
    let path = cmd.strip_prefix("mkdir").unwrap_or("").trim();
    if path.is_empty() {
        println!("usage: mkdir <path>");
        return;
    }

    println!("[vfs stub] would create directory: {}", path);
    println!("[vfs stub] Lesson 9 will make this work");
}

fn cmd_mem() {
    println!("Memory status:");
    println!("  Heap:     0 KB / 0 KB used (heap not yet allocated)");
    println!("  Stack:    ~32 KB available");
    println!("  Total:    256 MB (from BootInfo)");
    println!("[mem stub] Lesson 6 will provide real heap stats");
}

fn cmd_ai(cmd: &str) {
    let arg = cmd.strip_prefix("ai").unwrap_or("").trim();

    if arg == "status" || arg.is_empty() {
        println!("AI Subsystem Status:");
        println!("  State:        not loaded");
        println!("  Models:       0 loaded");
        println!("  Vector store: not initialized");
        println!("  Accelerator:  none detected");
        println!();
        println!("AI subsystem coming in Lessons 12+");
    } else if arg == "models" {
        println!("[ai stub] Lesson 13: model registry");
    } else if arg.starts_with("infer") {
        println!("[ai stub] Lesson 14: inference syscalls");
    } else {
        println!("ai: unknown subcommand: {}", arg);
    }
}

fn cmd_help() {
    println!("Krisha Shell - available commands:");
    println!();
    println!("  hello              Namaste from Krisha");
    println!("  info               Show system info");
    println!("  date               Show kaal (Indian + Gregorian)");
    println!("  uptime             Show CPU ticks since boot");
    println!("  echo <text>        Print text");
    println!("  color <name>       Change output color [stub]");
    println!("  ls [path]          List directory [stub]");
    println!("  cat <file>         Display file [stub]");
    println!("  mkdir <path>       Create directory [stub]");
    println!("  mem                Show memory usage");
    println!("  ai <subcommand>    AI subsystem (status, models, infer)");
    println!("  shutdown           Exit Krisha");
    println!("  help               This message");
    println!();
    println!("Real commands: hello, info, date, uptime, echo, help");
    println!("Stub commands: color, ls, cat, mkdir, mem, ai");
}

fn cmd_shutdown() {
    println!("Alvida, brother! 🙏");
    println!("Krisha is signing off.");
}