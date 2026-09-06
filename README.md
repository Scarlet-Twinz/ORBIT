# ORBIT

**A small x86_64 operating-system kernel written from scratch in Rust.**

ORBIT explores the foundations beneath application software: bootstrapping, memory management, interrupts, task scheduling, system calls, and low-level hardware interaction.

The project is intentionally small. Each subsystem is introduced incrementally, documented, and validated before the next layer is added.

## Architecture

```text
Firmware / Bootloader
        ↓
   ORBIT Kernel
        ↓
  Memory Management
        ↓
 Interrupt Handling
        ↓
   Task Scheduling
        ↓
   System Calls
        ↓
 Filesystem / Drivers
        ↓
    User Programs
```

## Current Status

### Phase 1 — Bootstrapping

- [x] Rust kernel binary
- [x] x86_64 bare-metal target
- [x] Bootloader integration
- [x] Kernel entry point
- [x] Initial VGA text output
- [x] CI build validation

### Planned Milestones

- [ ] Serial output and structured kernel logging
- [ ] Physical frame allocator
- [ ] Page tables and virtual memory
- [ ] Kernel heap allocator
- [ ] Interrupt Descriptor Table
- [ ] CPU exception handling
- [ ] Programmable timer
- [ ] Keyboard driver
- [ ] Task abstraction
- [ ] Context switching
- [ ] Scheduler
- [ ] System-call interface
- [ ] User/kernel boundary
- [ ] Simple filesystem
- [ ] Executable loading
- [ ] Interactive kernel shell

Networking and additional device support will be considered only after the core kernel is stable.

## Design Principles

- **Small surface area** — build the minimum needed for each subsystem.
- **Explicit mechanisms** — prefer understandable low-level code over abstraction for its own sake.
- **Incremental validation** — keep the kernel buildable as new subsystems are introduced.
- **Document the machine** — explain why the kernel works, not only how to compile it.

## Toolchain

- Rust nightly
- x86_64 bare-metal target
- bootloader
- QEMU for virtual hardware during development
- GitHub Actions for CI

## Development

Install Rust through [rustup](https://rustup.rs/) and install QEMU for your platform.

Then clone the repository and build the kernel:

```bash
git clone https://github.com/Scarlet-Twinz/ORBIT.git
cd ORBIT
cargo build --release
```

The repository pins the required Rust toolchain and target through `rust-toolchain.toml`.

## Project Direction

ORBIT is deliberately being developed as a kernel rather than a desktop environment. The goal is to understand and implement the mechanisms that connect software to the underlying machine.
