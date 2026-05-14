// kernel/src/shell/commands.rs
//
// Krisha Shell — built-in commands 🦚🕉️
//
// Each command supports both English AND Sanskrit/Hindi aliases.
// E.g., "date" and "kaal" do the same thing.

use crate::println;
use crate::time;
use crate::shell::art;

/// Main command dispatcher — matches command string to function.
pub fn run(cmd: &str) {
    let trimmed = cmd.trim();

    // ─────────────── Greetings ───────────────
    if trimmed == "hello" || trimmed == "namaste" {
        cmd_namaste();
    }
    // ─────────────── System Info ─────────────
    else if trimmed == "info" || trimmed == "vivran" {
        cmd_info();
    }
    else if trimmed == "krisha" {
        art::print_krisha_logo();
    }
    // ─────────────── Time & Calendar ─────────
    else if trimmed == "date" || trimmed == "kaal" {
        cmd_date();
    }
    else if trimmed == "cal" || trimmed == "panjika" {
        cmd_cal();
    }
    else if trimmed == "panchang" {
        cmd_panchang();
    }
    else if trimmed == "uptime" {
        cmd_uptime();
    }
    // ─────────────── Wisdom ──────────────────
    else if trimmed == "fortune" || trimmed == "shloka" {
        cmd_fortune();
    }
    // ─────────────── Misc ────────────────────
    else if trimmed.starts_with("color") {
        cmd_color(trimmed);
    }
    else if trimmed.starts_with("echo") {
        cmd_echo(trimmed);
    }
    else if trimmed.starts_with("ls") {
        cmd_ls(trimmed);
    }
    else if trimmed.starts_with("cat") {
        cmd_cat(trimmed);
    }
    else if trimmed.starts_with("mkdir") {
        cmd_mkdir(trimmed);
    }
    else if trimmed == "mem" || trimmed == "smriti" {
        cmd_mem();
    }
    else if trimmed.starts_with("ai") || trimmed == "gyaan" {
        cmd_ai(trimmed);
    }
    else if trimmed == "help" || trimmed == "sahayata" {
        cmd_help();
    }
    else if trimmed == "shutdown" || trimmed == "alvida" {
        cmd_shutdown();
    }
    else {
        println!("krisha: command not found: {}", trimmed);
        println!("        type 'help' (or 'sahayata') for available commands");
    }
}

// ──────────────────────────────────────────────────────────
// Greetings
// ──────────────────────────────────────────────────────────

fn cmd_namaste() {
    art::print_namaste();
}

// ──────────────────────────────────────────────────────────
// System Info
// ──────────────────────────────────────────────────────────

fn cmd_info() {
    println!("Krisha OS v0.1.0 - Intelligence, by grace.");
    println!("  Architecture: x86_64");
    println!("  Memory:       256 MB");
    println!("  Kernel:       Rust no_std");
    println!("  Bootloader:   bootloader 0.11");
    println!("  Status:       Day 4 - shell scaffolding");
    println!("  Languages:    English + Sanskrit aliases");
}

// ──────────────────────────────────────────────────────────
// Time & Calendar
// ──────────────────────────────────────────────────────────

/// Date with Hindu calendar info (alias: kaal)
fn cmd_date() {
    println!("[rtc] reading from CMOS chip...");
    let now = time::now();
    time::calendar::print_kaal(&now);
}

/// Month calendar with Hindu overlay (alias: panjika)
fn cmd_cal() {
    println!("[rtc] generating calendar...");
    let now = time::now();
    time::calendar::print_cal(&now);
}

/// Full Hindu almanac (panchang)
fn cmd_panchang() {
    println!("[rtc] reading panchang...");
    let now = time::now();
    time::calendar::print_panchang(&now);
}

fn cmd_uptime() {
    let ticks = time::uptime_ticks();
    println!("[tsc] CPU cycles since reset: {}", ticks);
    println!("       (precise wall-clock uptime needs Lesson 5 timer)");
}

// ──────────────────────────────────────────────────────────
// Wisdom
// ──────────────────────────────────────────────────────────

/// Random Sanskrit shloka (alias: shloka)
fn cmd_fortune() {
    time::fortune::print_fortune();
}

// ──────────────────────────────────────────────────────────
// Misc commands (mostly stubs for now)
// ──────────────────────────────────────────────────────────

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
    println!("Memory status (Smriti):");
    println!("  Heap:     0 KB / 0 KB used (heap not yet allocated)");
    println!("  Stack:    ~32 KB available");
    println!("  Total:    256 MB (from BootInfo)");
    println!("[mem stub] Lesson 6 will provide real heap stats");
}

fn cmd_ai(cmd: &str) {
    let arg = cmd.strip_prefix("ai").unwrap_or("").trim();

    if arg == "status" || arg.is_empty() {
        println!("AI Subsystem Status (Gyaan):");
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
    println!("  (English / Sanskrit alias)");
    println!();
    println!("  Greetings:");
    println!("    hello / namaste     Greet Krisha");
    println!();
    println!("  System:");
    println!("    info / vivran       Show system info");
    println!("    krisha              Show Krisha OS logo");
    println!("    mem / smriti        Show memory usage");
    println!();
    println!("  Time & Calendar:");
    println!("    date / kaal         Show date/time with kaal info");
    println!("    cal / panjika       Show month calendar");
    println!("    panchang            Show full Hindu almanac");
    println!("    uptime              Show CPU ticks since boot");
    println!();
    println!("  Wisdom:");
    println!("    fortune / shloka    Random Sanskrit shloka");
    println!();
    println!("  Misc:");
    println!("    echo <text>         Print text");
    println!("    color <name>        Change output color [stub]");
    println!("    ls [path]           List directory [stub]");
    println!("    cat <file>          Display file [stub]");
    println!("    mkdir <path>        Create directory [stub]");
    println!("    ai <subcmd> / gyaan AI subsystem (status, models)");
    println!();
    println!("  Exit:");
    println!("    shutdown / alvida   Exit Krisha");
    println!("    help / sahayata     This message");
}

fn cmd_shutdown() {
    println!("Alvida, brother! 🙏");
    println!("Krisha is signing off.");
}