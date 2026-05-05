# krisha-os
AI native kernel programming named after my daughter :)

An AI-native operating system kernel written from scratch in Rust.
What is Krisha?
Krisha OS is an experimental operating system that treats artificial intelligence as a first-class kernel primitive — not an application running on top of the OS, but a fundamental capability woven into the kernel itself.
In Krisha:

🧠 Inference is a syscall — ai_infer() lives alongside read() and write()
🔢 Tensors are kernel types — managed like file descriptors
📦 Models are shared resources — loaded once, used everywhere safely
🔒 AI permissions are capabilities — apps declare what models and data they touch
💬 The shell speaks human — natural language is the command line

🌍 Why Build Another OS?
Every mainstream operating system — Linux, Windows, macOS — was designed before the AI era. They bolt machine learning on as libraries and apps. Krisha asks a different question:

What would an OS look like if it were designed in the age of intelligence?

Krisha is built on the belief that AI should be a primitive, not an afterthought. Just as the 1970s gave us file systems and the 1990s gave us networking stacks as core OS features, the 2020s deserve an OS where intelligent computing is woven into the kernel itself.
🎯 Design Principles
PrincipleWhat it means🦀 Memory-safeWritten entirely in Rust — no buffer overflows, no use-after-free🧠 AI-nativeInference, tensors, and embeddings as kernel primitives🔒 Capability-secureApps explicitly declare AI access; the kernel enforces🌐 Local-firstYour intelligence runs on your hardware, your data stays yours⚡ Hardware-awareRoutes inference to CPU, GPU, or NPU automatically🧩 Microkernel-leaningInference engines isolated; a model crash doesn't crash the kernel
🗺️ Roadmap
Krisha is being built in phases over a multi-year journey.
Phase 1 — Foundation 🌱
Boot a Rust kernel in QEMU, handle keyboard input.

 Freestanding Rust binary
 Custom x86_64 target
 Bootloader integration
 VGA text output
 Serial debug output
 CPU exception handling
 Hardware interrupts
 Keyboard input

Phase 2 — Core OS 🏗️
Memory, processes, IPC.

 Memory paging (4-level x86_64)
 Frame allocator + heap
 Async/await runtime
 Cooperative & preemptive scheduling
 Inter-process communication
 Basic filesystem

Phase 3 — AI Subsystem 🧠 (the hotspot)
Inference becomes a syscall.

 Tensor type as kernel primitive
 Model registry (kernel-managed shared models)
 AI syscall interface (ai_load, ai_infer, ai_embed, ai_stream)
 Integration with candle
 First LLM running via syscall (TinyLlama or Phi quantized)
 Capability-based AI permissions
 Inference scheduler

Phase 4 — Natural Language Shell 💬
Talk to your computer.

 Vector store as kernel primitive
 Context manager (conversations as OS objects)
 krisha-sh — the natural language shell
 Driver framework (async-first, loadable)

Phase 5 — Beyond 🚀

 WASM userspace
 Distributed inference across devices
 Formal verification of AI permissions
 App ecosystem

🏛️ Architecture
┌─────────────────────────────────────────────┐
│  Userspace                                  │
│  ├─ krisha-sh (natural language shell)      │
│  ├─ AI-native applications                  │
│  └─ Traditional applications                │
├─────────────────────────────────────────────┤
│  Syscall Interface                          │
│  ├─ Traditional: read, write, open, fork    │
│  └─ AI-native: ai_load, ai_infer, ai_embed  │
├─────────────────────────────────────────────┤
│  Kernel — AI Subsystem                      │
│  ├─ Model Registry                          │
│  ├─ Tensor Manager                          │
│  ├─ Inference Scheduler                     │
│  ├─ Vector Store                            │
│  ├─ Context Manager                         │
│  └─ Capability Enforcement                  │
├─────────────────────────────────────────────┤
│  Kernel — Traditional                       │
│  Memory │ Processes │ Files │ Drivers       │
├─────────────────────────────────────────────┤
│  Hardware Abstraction Layer                 │
│  CPU │ GPU │ NPU │ I/O                      │
└─────────────────────────────────────────────┘

Quick Start
Prerequisites

Rust nightly (rustup default nightly)
QEMU (qemu-system-x86_64)
Cargo bootimage (cargo install bootimage)

📚 Learning Resources
This project is also a learning journey. Resources guiding the way:

📖 Writing an OS in Rust — Philipp Oppermann's blog
📕 Operating Systems: Three Easy Pieces — free OS textbook
🌐 OSDev Wiki — the bible of OS development
🦀 The Rust Programming Language — official Rust book
🏗️ Redox OS — production Rust OS, source of inspiration
🤗 Candle — pure-Rust ML framework

🤝 Contributing
Krisha is in early development and contributions are welcomed with open arms! 🙏
Especially looking for help with:

🐛 Bug fixes and code reviews
📝 Documentation improvements
🧪 Test coverage
🦀 Rust idiom suggestions
💡 AI-native design ideas

To contribute:

Fork the repo
Create a feature branch (git checkout -b feature/amazing-thing)
Commit your changes (git commit -am 'Add amazing thing')
Push to the branch (git push origin feature/amazing-thing)
Open a Pull Request

📊 Project Status
ComponentStatusBootloader🚧 Setting upVGA output⏳ PlannedMemory management⏳ PlannedProcess scheduler⏳ PlannedAI subsystem⏳ PlannedNatural language shell💭 Dreaming
📜 License
Krisha OS is licensed under either of:

MIT License
Apache License 2.0

at your option.
🙏 Acknowledgments
Krisha stands on the shoulders of giants:

The Rust community for building a language fit for kernels
Philipp Oppermann for his blog that teaches Rust OS development
The Redox OS team for proving Rust can power a full operating system
Andreas Kling and SerenityOS for inspiration on building from scratch
The countless OS researchers and engineers whose decades of work make this possible
Finally thanks to Claude for guiding with all structred materials good emojis and collecting resources under one single window




