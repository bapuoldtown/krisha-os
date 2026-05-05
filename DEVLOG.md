# 🦚 Krisha OS — Development Log

Personal daily log of building Krisha. Add new entries on top.

---

## Day 2 — VGA Buffer Driver
*[Today's date]*

### Goal
Replace port 0xe9 hack with real VGA buffer output.

### What I learned
- Memory-mapped I/O (writing to addresses talks to hardware)
- VGA buffer at 0xb8000 — 80×25 grid, 2 bytes per cell
- Volatile memory access (compiler can optimize away writes!)
- Spin locks (no OS = no real Mutex)
- `lazy_static!` for complex globals
- Custom macros with `macro_rules!`
- `fmt::Write` trait unlocks formatted output

### What worked
- ✅ Compilation succeeded with new dependencies
- ✅ `println!` macro works in kernel
- ✅ Color codes work (yellow on black)
- ✅ CI still passes (port 0xe9 backup output)

### What surprised me
- (fill in...)

### Next steps
- Move to Lesson 4: CPU exception handlers

---

## Day 1 — First Boot 🎉
*Tuesday, May 5, 2026*

### THE KERNEL IS ALIVE!

After fighting through ~10 dependency/toolchain errors,
Krisha finally booted in cloud QEMU and printed:

### Hard-won knowledge
- Cargo workspaces split kernel + builder cleanly
- `bootloader 0.11` is the modern way (avoids 0.9 hell)
- `bindeps` unstable feature lets builder use kernel as artifact
- `build-dependencies` ≠ regular `dependencies`
- Port 0xe9 (debug) and 0xf4 (exit) are QEMU's gifts
- Exit code 33 = `(0x10 << 1) | 1` (deliberate convention)

### Errors I navigated (each was a teacher)
1. duplicate symbol `_start` → custom target needed
2. JSON target spec needs flag → use stable target instead
3. lock file v4 issues → upgrade nightly
4. host-config flag → remove cargo extras
5. data-layout mismatch → switch to bootloader 0.11
6. target-pointer-width missing → avoid old bootloader
7. multiboot 32-bit only → use real disk image
8. -nostartfiles unknown → not needed for bare-metal target
9. bindeps required → enable in cargo config
10. bootloader not found → wrong dependencies section

### Mood
Proud. Tired. Determined. 🦚

## Day 3 — Lesson 3 COMPLETE 🎉
*[Today's date]*

### Won the println! battle

Got `println!` macro working in the kernel after a wild detour:
- First try: classic VGA buffer at 0xb8000 → kernel triple-faulted
- Diagnosis: bootloader 0.11 doesn't map legacy VGA address
- Solution: pivoted to port 0xe9 with same Lesson 3 architecture
- Result: spin lock + lazy_static + fmt::Write + custom macros all work

### Key insight
Phil-Opp's classic tutorial assumes legacy bootloader behavior.
Modern bootloader 0.11 uses framebuffers (UEFI-style).
The CONCEPTS (volatile, spin, lazy_static, macros) all still apply —
just plugged into a different output channel.

### What surprised me
- How sneakily a triple-fault hangs the kernel (no error, just silence)
- That github.dev can have sync issues (verify push actually landed!)
- How much defensive debug_print() helps debugging

### What's next
- Lesson 4: CPU exception handlers (so we CAN use VGA later!)
- Lesson 5: Hardware interrupts + keyboard
- Then: framebuffer rendering for graphical text output

### Mood
Triumphant. 🦚 Krisha can speak in formatted text now!

---
