# Lesson 2 — First Boot in QEMU

## 🎯 Goal
Get Krisha OS to actually boot in QEMU and print "Krisha OS" — verified by CI.

## 🏗️ The Architecture We Built

We use a Cargo **workspace** with TWO sub-projects:
### Why two projects?
- **Kernel** runs on bare CPU with NO OS
- **Builder** runs on Linux to create the disk image
- They have different compile targets and dependencies
- Can't mix them in one project

### The flow
## 🔑 Key Concepts

### Cargo workspace
Root `Cargo.toml` is just:
```toml
[workspace]
resolver = "2"
members = ["kernel", "builder"]
```
This groups multiple crates into one project.

### Binary dependencies (`bindeps`)
The builder depends on the kernel **as a built binary**, not source:
```toml
[build-dependencies]
krisha-kernel = { path = "../kernel", artifact = "bin", target = "x86_64-unknown-none" }
```
This is unstable Rust feature — must enable in `.cargo/config.toml`:
```toml
[unstable]
bindeps = true
```

### `bootloader_api` crate
Provides the contract between bootloader and kernel:
```rust
use bootloader_api::{entry_point, BootInfo};
entry_point!(kernel_main);
fn kernel_main(_boot_info: &'static mut BootInfo) -> ! { ... }
```
The `entry_point!` macro generates code that tells the bootloader where to jump.

### `bootloader` crate (in builder)
Used at build time to wrap the kernel into a bootable image:
```rust
bootloader::BiosBoot::new(&kernel)
    .create_disk_image(&bios_path)
    .unwrap();
```

## 📡 The Two Magic Ports

We can't use `printf` (no OS). Instead we talk to QEMU via I/O ports:

### Port 0xe9 — Debug Console
Whatever bytes we write to this port appear in QEMU's stdout.
QEMU forwards them to CI logs.

```rust
unsafe {
    core::arch::asm!(
        "out dx, al",
        in("dx") 0xe9_u16,
        in("al") byte,
    );
}
```

### Port 0xf4 — ISA Debug Exit
Writing to this port makes QEMU exit cleanly.
The exit code follows the formula: `(value << 1) | 1`
- We write `0x10` → QEMU exits with code 33
- Convention: 33 = success

```rust
unsafe {
    core::arch::asm!(
        "out dx, al",
        in("dx") 0xf4_u16,
        in("al") 0x10_u8,    // 0x10 << 1 | 1 = 33 (success)
    );
}
```

## 🦀 What `unsafe` Means
Rust forces us to wrap dangerous operations in `unsafe`:
- Direct memory access via raw pointers
- Inline assembly (`asm!`)
- Calling C code

It's not "this is bad" — it's "I, the programmer, take responsibility."

## 🔄 The Boot Sequence

When `qemu-system-x86_64 -drive file=krisha.img` runs:

1. **BIOS POST** — virtual PC powers on
2. **BIOS reads boot sector** — finds magic bytes 0x55AA
3. **Stage 1 bootloader** (16-bit) — switches to 32-bit
4. **Stage 2 bootloader** (32-bit) — switches to 64-bit, sets up paging
5. **Loads kernel ELF** into memory
6. **Calls kernel_main(boot_info)**
7. **Our code runs!** — prints to port 0xe9, exits via port 0xf4

## 🐛 Errors We Fought (in order)

1. **`duplicate symbol _start`** — Rust wanted to create Linux executable. Fixed with custom target.
2. **`-Zjson-target-spec required`** — Modern Rust needs flag for JSON targets. Fixed by switching approaches.
3. **`lock file version 4 requires -Znext-lockfile-bump`** — Pinned Rust too old. Fixed by upgrading.
4. **`-Zhost-config requires -Ztarget-applies-to-host`** — Bad cargo config. Removed extras.
5. **`data-layout differs from LLVM target`** — Phil-Opp's old data-layout incompatible. Fixed by switching to bootloader 0.11.
6. **`Field target-pointer-width required`** — bootloader 0.9's old target. Avoided by upgrading.
7. **`Cannot load x86-64 image, give a 32bit one`** — Multiboot v1 only supports 32-bit. Pivoted to bootloader 0.11.
8. **`-nostartfiles unknown argument`** — rust-lld doesn't use GCC flags. Removed flag.
9. **`bindeps required`** — Unstable feature. Enabled in cargo config.
10. **`bootloader not found`** — Was in wrong dependencies section. Moved to build-dependencies.

**Each error was a lesson.** Real OS dev navigates dependency hell daily.

## ✅ What Worked Final Setup

- **Rust nightly** (latest, no pinning)
- **bootloader_api 0.11** in kernel
- **bootloader 0.11** in builder build-dependencies
- **Workspace structure** (kernel + builder)
- **Standard `x86_64-unknown-none` target** (no custom JSON)
- **Real BIOS disk image** (not multiboot)

## 🎉 First Output