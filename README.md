# ORBIT

**A small x86_64 operating-system kernel written from scratch in Rust.**

ORBIT is a bare-metal systems project focused on the mechanisms beneath application software: bootstrapping, physical memory management, interrupts, timer-driven scheduling, system-call boundaries, storage abstractions, and low-level hardware interaction.

The repository deliberately separates the `no_std` kernel from the host-side image builder. Kernel code stays platform-focused while the host crate handles boot-image creation and QEMU execution.

## Project Preview

ORBIT boots a real kernel image under QEMU and reports subsystem state through a COM1 serial console. A framebuffer console also displays the initial kernel status directly in the guest.

A successful boot currently exercises:

- x86_64 bare-metal kernel entry
- physical memory-map discovery and 4 KiB frame allocation
- IDT exception handling and PIC IRQ routing
- PIT timer interrupts at 100 Hz
- PS/2 keyboard IRQ handling
- scheduler/task-state bookkeeping
- register-based syscall dispatch
- block-device and RAM-disk operations
- boot-time subsystem self-tests

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
                         Self-test layer
                              │
                    ┌─────────▼─────────┐
                    │ Boot-time checks   │
                    │ + QEMU validation  │
                    └────────────────────┘
                              │
                        Future user space
```

## Core Features

### Boot and kernel foundation

- Rust `no_std` kernel
- `x86_64-unknown-none` target
- BIOS bootloader integration
- Kernel/host workspace separation
- Framebuffer console with pixel-format handling
- Synchronized framebuffer ownership
- COM1 serial diagnostics
- QEMU launch path with the guest remaining running after initialization

### Memory and interrupts

- Bootloader memory-map inspection
- Physical 4 KiB frame allocator
- Interrupt Descriptor Table
- CPU breakpoint, page-fault, and double-fault handlers
- PIC remapping
- PIT timer configured for 100 Hz
- PS/2 keyboard IRQ path

### Execution and system calls

- Task control blocks
- Round-robin scheduler model
- Timer-driven scheduler tick path
- Register-based syscall contract
- Syscall dispatcher with tick reporting

### Storage

- Block-device trait abstraction
- Fixed 512-byte block interface
- In-memory RAM-disk implementation
- Read/write and bounds-checking paths

### Validation

The kernel runs a boot-time self-test covering the implemented foundations:

- physical frame allocation
- kernel task creation
- RAM-disk write/read behavior
- RAM-disk data integrity
- RAM-disk range checking
- syscall dispatch
- aggregate subsystem self-test result

## Repository Structure

```text
ORBIT/
├── .cargo/
│   └── config.toml            # Bare-metal target and linker configuration
├── .github/
│   └── workflows/
│       └── ci.yml             # Rust formatting and build validation
├── kernel/
│   ├── Cargo.toml             # no_std kernel crate
│   └── src/
│       ├── main.rs            # Kernel entry point and boot sequence
│       ├── serial.rs          # COM1 serial console
│       ├── vga.rs             # Framebuffer console renderer
│       ├── memory.rs           # Physical frame allocator
│       ├── interrupts.rs      # IDT, PIC, PIT, exceptions, and IRQs
│       ├── keyboard.rs        # PS/2 keyboard input path
│       ├── tasks.rs            # Task state and scheduler model
│       ├── syscall.rs         # System-call ABI foundation
│       ├── storage.rs         # Block-device abstraction and RAM disk
│       └── selftest.rs        # Boot-time subsystem validation
├── os/
│   ├── Cargo.toml             # Host-side image builder and launcher
│   ├── build.rs               # Creates the bootable BIOS image
│   └── src/main.rs            # QEMU launcher
├── Cargo.toml
└── rust-toolchain.toml
```

## Tech Stack

| Layer | Technology |
| --- | --- |
| Kernel language | Rust `no_std` |
| Architecture | x86_64 |
| Target | `x86_64-unknown-none` |
| Boot | `bootloader` 0.11.x / BIOS |
| Architecture support | `x86_64` crate |
| Synchronization | `spin` |
| Serial | COM1 / 16550-compatible UART |
| Virtual machine | QEMU |
| CI | GitHub Actions |
| Toolchain | Rust nightly |

## Getting Started

### Prerequisites

Install:

- Rust nightly
- QEMU
- Git

The repository includes `rust-toolchain.toml`, so the required Rust target and components are selected automatically through rustup.

### Clone

```bash
git clone https://github.com/Scarlet-Twinz/ORBIT.git
cd ORBIT
```

### Format

```bash
cargo fmt --all
```

### Build

```bash
cargo build --release -p orbit-os
```

### Run

```bash
cargo run --release -p orbit-os
```

The host crate builds the kernel, creates a bootable BIOS disk image, and launches `qemu-system-x86_64` with the serial console attached to the terminal.

## Validation Output

A successful run reports the initialization path and then executes the kernel self-test. The expected signals include output similar to:

```text
ORBIT kernel starting
memory: usable_regions=3 remaining_bytes=...
interrupts: IDT loaded, PIC remapped, IRQ0/IRQ1 enabled
timer: PIT configured at 100 Hz
task: created idle task id=1
task: created kernel task id=2
storage: block-device ABI ready block_size=512 blocks=8
selftest: physical-frame-allocation=true
selftest: task-spawn=true
selftest: ramdisk write=true read=true data=true range-check=true
selftest: syscall-dispatch=true
selftest: result=true frames=1 syscall_calls=2 ready_tasks=3
ORBIT kernel initialized
interrupts: enabled
```

The exact memory totals can vary between runs because they depend on the boot environment.

## CI

GitHub Actions installs the pinned nightly toolchain configuration, runs Rust formatting, and builds the OS image on pushes and pull requests targeting `main`.

Local validation should use the same basic commands:

```bash
cargo fmt --all
cargo build --release -p orbit-os
```

## Current Status

**Functional bare-metal kernel foundation.**

The current milestone is complete for the implemented foundation layer: ORBIT boots under QEMU, initializes its core hardware-facing subsystems, runs the boot-time self-test successfully, and remains active with interrupts enabled.

This is intentionally not presented as a finished general-purpose operating system. The scheduler is currently a model rather than a hardware context-switching implementation, the syscall layer is an ABI/dispatcher foundation rather than a completed ring-3 transition, and storage is currently RAM-backed rather than persistent.

## Next Kernel Layers

The architecture leaves a clear path toward a more complete operating system:

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
- **Incremental validation** — each subsystem should leave the workspace buildable and the kernel bootable.
- **Observable execution** — framebuffer output is human-readable while COM1 output is deterministic and script-friendly.
- **Architecture-first design** — memory, interrupts, execution, storage, and future user space communicate through narrow interfaces.
- **Failure visibility** — fatal CPU faults are logged and halted instead of silently rebooting the guest.

## Project Direction

ORBIT is intentionally a kernel project rather than a desktop environment. The goal is to make the machine/software boundary explicit: boot the processor, understand physical memory, establish interrupt handling, schedule execution, define a syscall boundary, add storage, and eventually run isolated user programs.

The project favors a smaller amount of real kernel mechanism over a large collection of superficial features. Each milestone should make the system easier to inspect, debug, validate, and extend.

## Author

**Anthony Emmanuella Mmasinachi**

Software developer focused on frontend engineering, backend systems, APIs, automation, databases, realtime applications, systems programming, and practical software architecture.

**GitHub:** https://github.com/Scarlet-Twinz
