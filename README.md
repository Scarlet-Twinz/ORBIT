# ORBIT

**A small x86_64 operating-system kernel written from scratch in Rust.**

ORBIT explores the foundations beneath application software: bootstrapping, memory management, interrupts, task scheduling, system calls, and low-level hardware interaction.

The repository separates the bare-metal kernel from the host-side image builder so the kernel can remain a `no_std` target while the build tooling runs normally on the development machine.

## Architecture

```text
                 ┌─────────────────────┐
                 │     Host / Cargo     │
                 │      orbit-os       │
                 └──────────┬──────────┘
                            │
                     bootloader image
                            │
                 ┌──────────▼──────────┐
                 │      Bootloader      │
                 └──────────┬──────────┘
                            │
                 ┌──────────▼──────────┐
                 │     ORBIT Kernel     │
                 │     x86_64/no_std    │
                 └──────────┬──────────┘
                            │
              ┌─────────────┴─────────────┐
              │                           │
         VGA text output            COM1 serial
              │                           │
              └─────────────┬─────────────┘
                            │
                    Future subsystems
                            │
        ┌───────────────────┼───────────────────┐
        │                   │                   │
     Memory              Interrupts          Scheduler
        │                   │                   │
        └───────────────────┼───────────────────┘
                            │
                       System Calls
                            │
                     User / Kernel Mode
                            │
                    Filesystem / Drivers
                            │
                       User Programs
```

## Repository Layout

```text
ORBIT/
├── .cargo/
│   └── config.toml       # Bare-metal target configuration
├── .github/
│   └── workflows/
│       └── ci.yml        # Formatting and kernel build checks
├── kernel/
│   ├── Cargo.toml        # no_std kernel crate
│   └── src/
│       ├── main.rs       # Kernel entry point and boot sequence
│       ├── serial.rs     # COM1 serial console
│       └── vga.rs        # VGA text-mode output
├── os/
│   ├── Cargo.toml        # Host-side image builder
│   ├── build.rs          # Creates the bootable BIOS image
│   └── src/
│       └── main.rs       # Builds and launches ORBIT with QEMU
├── Cargo.toml            # Workspace definition
└── rust-toolchain.toml   # Pinned nightly toolchain components
```

## Current Status

### Phase 1 — Bootstrapping

- [x] Rust `no_std` kernel binary
- [x] x86_64 bare-metal target
- [x] Bootloader integration
- [x] Workspace separation between kernel and host builder
- [x] Kernel entry point
- [x] VGA text-mode output
- [x] COM1 serial console
- [x] Boot status output
- [x] QEMU launch path
- [x] CI build validation

### Phase 2 — Memory

- [ ] Bootloader memory-map inspection
- [ ] Physical frame allocator
- [ ] Page-table inspection
- [ ] Virtual-memory primitives
- [ ] Kernel heap allocator

### Phase 3 — Interrupts and Hardware

- [ ] Interrupt Descriptor Table
- [ ] CPU exception handling
- [ ] Programmable interval timer
- [ ] Keyboard driver
- [ ] Hardware abstraction modules

### Phase 4 — Execution

- [ ] Task abstraction
- [ ] Context switching
- [ ] Scheduler
- [ ] System-call interface
- [ ] User/kernel boundary

### Phase 5 — Storage and User Space

- [ ] Simple filesystem
- [ ] Executable loading
- [ ] Interactive kernel shell
- [ ] Initial user programs

Networking and additional device support will be considered after the core kernel is stable.

## Design Principles

- **Small surface area** — introduce only the mechanism needed for the current subsystem.
- **Explicit mechanisms** — prefer understandable low-level code over abstraction for its own sake.
- **Incremental validation** — keep the workspace buildable as each subsystem is introduced.
- **Observable boot** — use VGA for local visual output and COM1 for machine-readable serial logs.
- **Document the machine** — explain why the kernel works, not only how to compile it.

## Toolchain

- Rust nightly
- `x86_64-unknown-none`
- `bootloader`
- QEMU
- GitHub Actions

The repository declares the required Rust target and components in `rust-toolchain.toml`.

## Development

Install Rust through [rustup](https://rustup.rs/) and install QEMU for your platform.

Clone the repository and open it in your editor:

```bash
git clone https://github.com/Scarlet-Twinz/ORBIT.git
cd ORBIT
```

Build the bootable BIOS image:

```bash
cargo build --release -p orbit-os
```

Run ORBIT in QEMU:

```bash
cargo run --release -p orbit-os
```

`cargo run` builds the kernel, creates the BIOS disk image, and launches `qemu-system-x86_64` with the COM1 serial console attached to the terminal. If QEMU is not installed, the build still succeeds and the generated image path is printed.

## Project Direction

ORBIT is deliberately being developed as a kernel rather than a desktop environment. The goal is to implement and understand the mechanisms that connect software to the underlying machine, progressing from bootstrapping through memory, interrupts, scheduling, system calls, storage, and user space.
