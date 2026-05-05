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