# ORBIT

**Small x86_64 operating-system kernel written from scratch in Rust.**

ORBIT is a bare-metal systems project focused on the mechanisms beneath application software: bootstrapping, physical memory management, interrupts, timer-driven scheduling, system-call boundaries, storage abstractions, and hardware-facing I/O.

The kernel is `no_std`. A separate host-side crate builds the boot image and launches it under QEMU.

## Boot Path

```text
Host / Cargo
     │
     ▼
Boot image builder
     │
     ▼
BIOS bootloader
     │
     ▼
ORBIT kernel
x86_64 / no_std
     │
 ┌───┼───────────────┐
 ▼   ▼       ▼       ▼
Memory IRQs  Tasks  Syscalls
 │     │      │       │
 └─────┴──────┴───────┘
           │
           ▼
        Storage
           │
           ▼
      Boot self-tests
```

## Implemented Foundations

### Boot and kernel

- Rust `no_std` kernel
- `x86_64-unknown-none` target
- BIOS bootloader integration
- Kernel/host workspace separation
- Framebuffer console
- COM1 serial diagnostics
- QEMU launch path

### Memory and interrupts

- Bootloader memory-map inspection
- Physical 4 KiB frame allocator
- Interrupt Descriptor Table
- Breakpoint, page-fault, and double-fault handlers
- PIC remapping
- PIT timer at 100 Hz
- PS/2 keyboard IRQ path

### Execution and syscalls

- Task control blocks
- Round-robin scheduler model
- Timer-driven scheduler tick path
- Register-based syscall contract
- Syscall dispatcher

### Storage

- Block-device trait abstraction
- Fixed 512-byte block interface
- RAM-disk implementation
- Read/write and bounds-checking paths

## Validation

The kernel runs boot-time self-tests covering:

- physical frame allocation;
- task creation;
- RAM-disk write/read/data integrity;
- RAM-disk bounds checking;
- syscall dispatch; and
- aggregate subsystem state.

A successful run emits deterministic serial output through COM1 while the framebuffer provides human-readable guest state.

## Repository Structure

```text
ORBIT/
├── .cargo/config.toml
├── kernel/
│   ├── src/main.rs
│   ├── src/serial.rs
│   ├── src/vga.rs
│   ├── src/memory.rs
│   ├── src/interrupts.rs
│   ├── src/keyboard.rs
│   ├── src/tasks.rs
│   ├── src/syscall.rs
│   ├── src/storage.rs
│   └── src/selftest.rs
├── os/
│   ├── build.rs
│   └── src/main.rs
├── .github/workflows/ci.yml
├── Cargo.toml
└── rust-toolchain.toml
```

## Tech Stack

| Layer | Technology |
| --- | --- |
| Kernel | Rust `no_std` |
| Architecture | x86_64 |
| Target | `x86_64-unknown-none` |
| Boot | bootloader 0.11.x / BIOS |
| Architecture support | `x86_64` crate |
| Synchronization | `spin` |
| Serial | COM1 / 16550-compatible UART |
| Virtual machine | QEMU |
| CI | GitHub Actions |
| Toolchain | Rust nightly |

## Getting Started

Prerequisites:

- Rust nightly
- QEMU
- Git

The repository includes `rust-toolchain.toml` for the required Rust target/components.

```bash
git clone https://github.com/Scarlet-Twinz/ORBIT.git
cd ORBIT
cargo fmt --all
cargo build --release -p orbit-os
cargo run --release -p orbit-os
```

The host crate builds the kernel, creates a bootable BIOS image, and launches `qemu-system-x86_64` with serial output attached.

## Engineering Boundaries

ORBIT intentionally documents what it **does not yet claim**:

- The scheduler is a model with timer-driven bookkeeping, not hardware context switching.
- The syscall layer is an ABI/dispatcher foundation, not a completed ring-3 userspace transition.
- Storage is RAM-backed, not persistent disk storage.

These boundaries are important because the project is about building kernel foundations incrementally rather than presenting a partial mechanism as a finished operating system.

## Current State

**Functional bare-metal kernel foundation.**

The implemented foundation boots under QEMU, initializes its hardware-facing subsystems, runs the boot-time self-test, and remains active with interrupts enabled.

CI validates formatting and the release build using the repository's nightly toolchain configuration.

## Engineering Principles

- **Mechanism before abstraction** — understand the low-level mechanism before hiding it.
- **Explicit ownership** — shared kernel state has defined synchronization/lifetime boundaries.
- **Incremental validation** — each subsystem should leave the kernel buildable and bootable.
- **Observable execution** — framebuffer output is human-readable; serial output is deterministic.
- **Architecture-first design** — subsystems communicate through narrow interfaces.
- **Failure visibility** — fatal CPU faults are logged and halted instead of silently rebooting.

## License

MIT

## Author

**Anthony Emmanuella Mmasinachi**

Full-stack and systems engineer focused on backend infrastructure, distributed systems, networking, compilers, operating systems, databases, AI integration, and systems programming.

## Project Links

- **Repository:** https://github.com/Scarlet-Twinz/ORBIT
- **Author:** Anthony Emmanuella Mmasinachi
- **GitHub:** https://github.com/Scarlet-Twinz
