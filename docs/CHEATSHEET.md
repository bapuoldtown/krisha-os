# 🧠 Concepts Cheat Sheet

Quick reference for common kernel programming concepts.

## Rust Attributes

| Attribute | Meaning |
|-----------|---------|
| `#![no_std]` | Don't link standard library |
| `#![no_main]` | No conventional main() function |
| `#[no_mangle]` | Don't rename function (linker needs exact name) |
| `#[panic_handler]` | This function handles panics |
| `#[repr(u8)]` | Enum is one byte |
| `#[repr(C)]` | Use C struct layout |
| `#[repr(transparent)]` | Zero-cost wrapper |
| `#[macro_export]` | Make macro available outside module |

## Function Signatures

| Signature | Meaning |
|-----------|---------|
| `extern "C"` | Use C calling convention |
| `-> !` | Function never returns |
| `&'static T` | Reference lives forever |
| `&mut T` | Mutable reference |

## Memory & I/O

| Concept | Use case |
|---------|----------|
| **I/O ports** (`out`, `in`) | Talk to devices like serial, debug exit |
| **Memory-mapped I/O** | Talk to devices like VGA, interrupt controller |
| **Volatile** | Prevent compiler from optimizing reads/writes |
| **Spin lock** | Synchronization without OS |

## Important Memory Addresses

| Address | What lives there |
|---------|------------------|
| `0x000B8000` | VGA text buffer (80×25 chars) |
| `0xFEC00000` | I/O APIC (interrupts) |
| `0xFEE00000` | Local APIC |
| `0xFED00000` | HPET (timer) |

## Important I/O Ports

| Port | Device |
|------|--------|
| `0x20`, `0x21` | PIC1 (master) |
| `0xA0`, `0xA1` | PIC2 (slave) |
| `0x60`, `0x64` | Keyboard controller |
| `0x3F8` | COM1 (serial port) |
| `0xE9` | QEMU debug console (QEMU only!) |
| `0xF4` | QEMU isa-debug-exit (QEMU only!) |

## Kernel Programming Patterns

### Inline assembly
```rust
unsafe {
    core::arch::asm!(
        "out dx, al",
        in("dx") port,
        in("al") value,
        options(nomem, nostack, preserves_flags)
    );
}
```

### Volatile MMIO write
```rust
use volatile::Volatile;
let mut cell: Volatile<u8> = ...;
cell.write(byte);
```

### Spin-locked global
```rust
use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref MY_THING: Mutex<MyType> = Mutex::new(MyType::new());
}

MY_THING.lock().do_something();
```

### Custom macro
```rust
#[macro_export]
macro_rules! my_macro {
    ($($arg:tt)*) => ($crate::module::function(format_args!($($arg)*)));
}
```

## Common Errors → Solutions

| Error | Solution |
|-------|----------|
| `cannot find macro println` | `#![no_std]` removed it — make your own |
| `panic_handler required` | Define `#[panic_handler] fn panic(...)` |
| `eh_personality required` | Set `panic = "abort"` in Cargo.toml |
| `linking with cc failed` | Use bare-metal target like `x86_64-unknown-none` |
| `data-layout differs` | Use stable target, not custom JSON |
| `bindeps required` | Add `[unstable] bindeps = true` to `.cargo/config.toml` |

## QEMU Exit Codes

QEMU formula: `exit = (value << 1) | 1`

| Write to 0xf4 | QEMU exits with |
|---------------|-----------------|
| `0x10` | 33 (success convention) |
| `0x11` | 35 (failure convention) |
| Any value N | `(N<<1) \| 1` |

## Cargo Workspace Magic

| Need | Solution |
|------|----------|
| Multiple crates | `[workspace] members = [...]` |
| Depend on bin artifact | `artifact = "bin"` (needs bindeps) |
| Different targets per crate | Specify in build-dependencies |
| Build script | `build.rs` at crate root |
| Pass data from build.rs | `cargo:rustc-env=NAME=value` |

## Glossary

- **ELF** — Executable Linkable Format (Linux/Unix binary format)
- **PIC** — Programmable Interrupt Controller (legacy, 8259)
- **APIC** — Advanced PIC (modern, used in SMP systems)
- **MMIO** — Memory-Mapped I/O
- **POST** — Power-On Self Test (BIOS startup)
- **TSS** — Task State Segment
- **GDT** — Global Descriptor Table
- **IDT** — Interrupt Descriptor Table
- **MSR** — Model-Specific Register
- **CR0/CR3** — Control Registers
- **VGA** — Video Graphics Array