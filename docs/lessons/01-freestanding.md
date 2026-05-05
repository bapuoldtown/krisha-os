# Lesson 1 — The Freestanding Rust Binary

## 🎯 Goal
Make Rust code that compiles WITHOUT an operating system underneath.

## 🤔 The Problem
Normal Rust programs use `std` (standard library) which needs an OS for:
- File I/O, networking, threads
- Memory allocation (heap)
- Panic unwinding

But our kernel IS the OS, so we can't have these.

## 🔑 Key Concepts

### `#![no_std]`
Tells Rust: "Don't link the standard library."
We only get `core` (basic types: integers, slices, references).

### `#![no_main]`
Tells Rust: "I don't have a normal `main()` function."
Why? Normal `main()` is called by Rust's runtime, which needs an OS.

### `#[panic_handler]`
A function Rust calls when code panics.
Normally provided by `std`. We must define our own.

### `_start` entry point
Linkers look for a function called `_start` as the program's start.
We provide this with:
```rust
#[no_mangle]                    // Don't rename it
pub extern "C" fn _start() -> ! {
    loop {}
}
```

### `panic = "abort"`
Rust normally "unwinds the stack" on panic — needs OS support.
We say "just abort" instead in `Cargo.toml`:
```toml
[profile.dev]
panic = "abort"
```

## ✅ What worked
A minimal kernel that compiled but didn't boot anywhere yet.

## 💡 Lessons learned
- The standard library is a HUGE abstraction over OS features
- `core` is OS-independent and what kernels use
- Compiler errors teach you what `no_std` removes
- Linker still tries to make a Linux executable — Lesson 2 fixes this

## 📖 Phil-Opp reference
[A Freestanding Rust Binary](https://os.phil-opp.com/freestanding-rust-binary/)