# Lesson 3 — VGA Text Buffer + `println!` Macro

## 🎯 Goal
Replace the port-0xe9 hack with proper screen output by writing
directly to VGA memory. Build our own `println!` macro.

## 🤔 Why Replace Port 0xe9?
- Port 0xe9 only works in QEMU
- Real PCs don't have it
- We need REAL screen output to progress toward a real OS

## 🧠 The Core Concept: Memory-Mapped I/O

CPUs talk to hardware in two ways:
1. **I/O ports** (Lesson 2 — `out dx, al`)
2. **Memory-mapped I/O** (THIS lesson — write to special addresses)

In MMIO, certain memory addresses don't go to RAM — they go to hardware:
- `0xb8000` → VGA text screen
- `0xfee00000` → CPU interrupt controller
- `0xfed00000` → High Precision Timer

Writing to `0xb8000` makes characters appear on screen!

## 🖼️ VGA Text Buffer Layout

- **Size:** 80 columns × 25 rows = 2000 character cells
- **Each cell:** 2 bytes (1 char + 1 color)
- **Total:** 4000 bytes from `0xb8000` to `0xb8FA0`

### Cell format
### Position math
Position `(row, col)` → memory offset `(row × 80 + col) × 2`

### Color byte structure
Example: Yellow on black = `0000_1110` = `0x0e`
- Background 0 = Black
- Foreground 14 = Yellow

## 🚨 Critical Concept: Volatile Memory

### The problem
Rust compilers optimize aggressively. If you write:
```rust
*buffer = b'H';   // Write 'H'
*buffer = b'I';   // Write 'I'
```
Compiler thinks: "Why write 'H' if it's overwritten? Skip it!"

### Why this is bad for hardware
Every write to VGA memory matters — the screen sees it!
Skipping writes = invisible bugs.

### Solution: `Volatile` wrapper
```rust
use volatile::Volatile;

struct Buffer {
    chars: [[Volatile<ScreenChar>; 80]; 25],
}

cell.write(value);  // Compiler MUST emit this write
```

**Use volatile for ALL memory-mapped I/O.**

## 🔒 Critical Concept: Spin Locks

### The problem
Multi-core CPU: two threads writing to screen simultaneously
→ garbled output.

Normal Rust uses `std::sync::Mutex` — but `Mutex` needs OS!

### Solution: Spin Lock
It "spins" instead of sleeping. Wasteful but works without OS.

```rust
use spin::Mutex;

static WRITER: Mutex<Writer> = Mutex::new(...);
WRITER.lock().write_string("hello");
```

## 🦀 Rust Concepts Used

### `#[repr(u8)]`
Forces enum to be a single byte:
```rust
#[repr(u8)]
enum Color { Black = 0, ... }
```

### `#[repr(transparent)]`
Wrapper has zero overhead — same memory layout as inner type:
```rust
#[repr(transparent)]
struct ColorCode(u8);
```

### `#[repr(C)]`
Use C struct layout (predictable byte ordering):
```rust
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}
```

### `lazy_static!`
Initialize globals on first use:
```rust
lazy_static! {
    static ref WRITER: Mutex<Writer> = Mutex::new(...);
}
```
Why? Rust forbids complex globals (Mutex needs runtime init).

### Rust macros
```rust
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
```
Defines `println!` for our kernel.

### `fmt::Write` trait
Implementing this lets `format!` machinery work:
```rust
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
```
Now we can do: `println!("count: {}", num)` with full formatting!

## 📦 New Dependencies

```toml
[dependencies]
volatile = "0.4.6"      # Safe MMIO
spin = "0.9"            # Spin lock
lazy_static = { version = "1.4", features = ["spin_no_std"] }
```

## 🏗️ The Writer Architecture
## ✅ What This Enables

```rust
println!("Krisha OS v{}", "0.1.0");          // Formatted output
println!("CPU: {}, Memory: {} MB", 1, 256);  // Multiple args
WRITER.lock().set_color(Color::Red, Color::Black);
println!("ERROR!");                          // Different colors
```

## 🐛 Possible Issues

| Issue | Cause | Fix |
|-------|-------|-----|
| No output visible | Volatile not used | Wrap in `Volatile<T>` |
| Garbled when threading | Missing lock | Use `spin::Mutex` |
| `lazy_static` not found | Wrong feature | Add `spin_no_std` feature |
| Can't lock in interrupt | Deadlock | Disable interrupts during lock |

## 💡 Key Takeaways

1. **Memory-mapped I/O** is how kernels talk to most hardware
2. **Volatile** is essential for MMIO (compiler optimizations otherwise)
3. **Spin locks** replace OS-based mutexes
4. **`lazy_static`** enables complex globals
5. **Custom macros** make kernel APIs ergonomic
6. **`fmt::Write`** unlocks all of Rust's formatting

## 📖 References
- [Phil-Opp VGA Text Mode](https://os.phil-opp.com/vga-text-mode/)
- [OSDev Wiki - Printing To Screen](https://wiki.osdev.org/Printing_To_Screen)
- [VGA Hardware Reference](https://wiki.osdev.org/VGA_Hardware)
