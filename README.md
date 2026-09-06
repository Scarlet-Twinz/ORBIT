# ORBIT

**A small x86_64 operating-system kernel written from scratch in Rust.**

ORBIT is a bare-metal systems project focused on the mechanisms beneath application software: bootstrapping, physical memory management, interrupts, scheduling, system-call boundaries, storage, and low-level hardware interaction.

The repository deliberately separates the `no_std` kernel from the host-side image builder. Kernel code stays platform-focused while the host crate handles image creation and QEMU execution.

## Architecture

```text
                         Host / Cargo
                              │
                         orbit-os crate
                              │
                       BIOS disk image
                              │
                         bootloader
                              │
                    ┌─────────▼─────────┐
                    │    ORBIT Kernel   │
                    │ x86_64 / no_std   │
                    └─────────┬─────────┘
                              │
              ┌───────────────┴────────────────┐
              │                                │
        Framebuffer                         COM1 serial
          console                            diagnostics
              │                                │
              └───────────────┬────────────────┘
                              │
                     Kernel subsystems
                              │
       ┌──────────────┬───────┼────────┬──────────────┐
       │              │       │        │              │
    Memory        Interrupts Tasks   Syscalls      Storage
       │              │       │        │              │
       └──────────────┴───────┼────────┴──────────────┘
                              │
                        Future user space
```

## Repository Layout

```text
ORBIT/
├── .cargo/config.toml       # Bare-metal target configuration
├── .github/workflows/ci.yml # Format/build validation
├── kernel/
│   ├── Cargo.toml            # no_std kernel crate
│   └── src/
│       ├── main.rs           # Kernel entry point and boot sequence
│       ├── serial.rs         # COM1 serial console
│       ├── vga.rs            # Framebuffer console renderer
│       ├── memory.rs         # Physical frame allocator
│       ├── interrupts.rs     # IDT, PIC, PIT, exception/IRQ handlers
│       ├── keyboard.rs       # PS/2 keyboard input path
│       ├── tasks.rs          # Task state and scheduler model
│       ├── syscall.rs       # System-call ABI foundation
│       └── storage.rs       # Block-device abstraction
├── os/
│   ├── Cargo.toml            # Host-side image builder
│   ├── build.rs              # Creates the bootable BIOS image
│   └── src/main.rs           # QEMU launcher
├── Cargo.toml
└── rust-toolchain.toml
```

## Current Milestones

### Boot and kernel foundation — complete

- [x] Rust `no_std` kernel
- [x] `x86_64-unknown-none` target
- [x] BIOS bootloader integration
- [x] Kernel/host workspace separation
- [x] Framebuffer console with pixel-format handling
- [x] Synchronized framebuffer ownership
- [x] COM1 serial diagnostics
- [x] QEMU launch path with no automatic reboot

### Core kernel architecture — implemented foundation

- [x] Bootloader memory-map inspection
- [x] Physical 4 KiB frame allocator
- [x] Interrupt Descriptor Table
- [x] CPU breakpoint/page-fault/double-fault handlers
- [x] PIC remapping
- [x] PIT timer at 100 Hz
- [x] PS/2 keyboard IRQ path
- [x] Task control-block and scheduler model
- [x] Register-based syscall contract and dispatcher
- [x] Block-device interface
- [x] In-memory block device for subsystem testing

These components are intentionally foundations rather than claims of a finished production OS. The next steps require virtual-memory mapping, real context switching, privilege transitions, persistent storage, and user-space execution.

### Next kernel layers

- [ ] Page-table mapper and virtual-memory primitives
- [ ] Kernel heap allocator
- [ ] Real context switching
- [ ] Preemptive task execution
- [ ] `syscall`/`sysret` user-kernel transition
- [ ] Ring-3 user processes
- [ ] Persistent block-device driver
- [ ] Filesystem
- [ ] Executable loader
- [ ] Interactive shell
- [ ] Initial user programs
- [ ] Additional device drivers and networking

## Engineering Principles

- **Mechanism before abstraction** — understand and validate the low-level mechanism before hiding it behind a large framework.
- **Explicit ownership** — shared kernel state has a defined synchronization and lifetime model.
- **Incremental validation** — each subsystem keeps the workspace buildable and the kernel bootable.
- **Observable execution** — framebuffer output is human-readable; COM1 output is deterministic and script-friendly.
- **Architecture-first design** — memory, interrupts, execution, storage, and user space communicate through narrow interfaces.
- **Failure visibility** — fatal CPU faults are logged and halted instead of silently rebooting the guest.

## Toolchain

- Rust nightly
- `x86_64-unknown-none`
- `bootloader` 0.11.x
- `x86_64` architecture crate
- QEMU
- GitHub Actions

## Development

```bash
git clone https://github.com/Scarlet-Twinz/ORBIT.git
cd ORBIT
cargo fmt --all
cargo build --release -p orbit-os
cargo run --release -p orbit-os
```

The host crate builds the kernel, creates a bootable BIOS image, and launches `qemu-system-x86_64` with the serial console attached to the terminal.

## Validation Signals

A successful boot should report the kernel initialization path followed by memory, interrupt, timer, scheduler, syscall, storage, and serial subsystem status. Once interrupts are enabled, the PIT drives the scheduler tick path and keyboard IRQs are routed through the IDT.

## Project Direction

ORBIT is intentionally a kernel project rather than a desktop environment. The goal is to make the machine/software boundary explicit: boot the processor, understand physical memory, establish interrupt handling, schedule execution, define a syscall boundary, add storage, and eventually run isolated user programs.

The project favors a smaller amount of real kernel mechanism over a large collection of superficial features. Each milestone should leave the system easier to inspect, debug, and extend.
