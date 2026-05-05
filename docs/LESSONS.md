# 📚 Krisha OS — Learning Journey

This is my personal log of building Krisha OS from scratch.
Each lesson teaches one core concept of OS development.

## Status

| Lesson | Topic | Status |
|--------|-------|--------|
| [Lesson 1](lessons/01-freestanding.md) | Freestanding Rust Binary | ✅ Done |
| [Lesson 2](lessons/02-first-boot.md) | First Boot in QEMU | ✅ Done |
| [Lesson 3](lessons/03-vga-buffer.md) | VGA Buffer + println! | 🚧 In progress |
| Lesson 4 | CPU Exception Handlers | ⏳ Coming |
| Lesson 5 | Hardware Interrupts + Keyboard | ⏳ Coming |
| Lesson 6 | Memory Paging | ⏳ Coming |
| Lesson 7 | Heap Allocation | ⏳ Coming |

## Quick Reference

### Common commands
```bash
cargo run --package krisha-builder    # Build kernel + disk image
cargo build --target x86_64-unknown-none  # Build just kernel
```

### Key files
- `kernel/src/main.rs` — Kernel entry point
- `kernel/src/vga_buffer.rs` — Screen output driver
- `builder/build.rs` — Wraps kernel into bootable disk

### Useful resources
- [Phil-Opp's blog](https://os.phil-opp.com/) — main reference
- [OSDev Wiki](https://wiki.osdev.org/) — encyclopedia
- [Rust Embedded Book](https://docs.rust-embedded.org/book/)