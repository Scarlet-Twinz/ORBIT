# ORBIT

**A small x86_64 operating-system kernel written from scratch in Rust.**

ORBIT is a bare-metal systems project focused on the mechanisms beneath application software: bootstrapping, memory management, interrupt handling, scheduling, system calls, storage, and low-level hardware interaction.

The repository deliberately separates the `no_std` kernel from the host-side image builder. This keeps platform-specific kernel code isolated while allowing the development tooling to use the normal Rust standard library.

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
              ┌──────────────┴──────────────┐
              │                             │
        Framebuffer                    COM1 serial
          console                        console
              │                             │
              └──────────────┬──────────────┘
                             │
                     Kernel subsystems
                             │
        ┌────────────────────┼────────────────────┐
        │                    │                    │
     Memory              Interrupts           Execution
        │                    │                    │
        └────────────────────┼────────────────────┘
                             │
                      System-call ABI
                             │
                     Storage / Drivers
                             │
                        User space
```

## Repository Layout

```text
ORBIT/
├── .cargo/
│   └── config.toml          # Bare-metal target configuration
├── .github/
│   └── workflows/
│       └── ci.yml           # Formatting and build validation
├── kernel/
│   ├── Cargo.toml           # no_std kernel crate
│   └── src/
│       ├── main.rs          # Kernel entry point and boot sequence
│       ├── serial.rs        # COM1 serial console
│       └── vga.rs           # Framebuffer console renderer
├── os/
│   ├── Cargo.toml           # Host-side image builder
│   ├── build.rs             # Creates the bootable BIOS image
│   └── src/
│       └── main.rs          # QEMU launcher
├── Cargo.toml               # Workspace definition
└── rust-toolchain.toml      # Nightly toolchain and components
```

## Current Milestone

### Boot and kernel foundation — complete

- [x] Rust `no_std` kernel binary
- [x] `x86_64-unknown-none` bare-metal target
- [x] BIOS bootloader integration
- [x] Separate kernel and host tooling crates
- [x] Kernel entry point
- [x] Boot-time framebuffer acquisition
- [x] Pixel-format-aware framebuffer console
- [x] Synchronized framebuffer access through `spin::Mutex`
- [x] COM1 serial console
- [x] Structured kernel boot messages
- [x] QEMU launch path
- [x] CI formatting/build validation

### Memory subsystem — planned

- [ ] Bootloader memory-map abstraction
- [ ] Physical frame allocator
- [ ] Page-table inspection and manipulation
- [ ] Virtual-memory primitives
- [ ] Kernel heap allocator

### Interrupts and hardware — planned

- [ ] Interrupt Descriptor Table
- [ ] CPU exception handlers
- [ ] Programmable interval timer
- [ ] Keyboard driver
- [ ] Hardware abstraction layer

### Execution — planned

- [ ] Kernel task abstraction
- [ ] Context switching
- [ ] Preemptive scheduler
- [ ] System-call ABI
- [ ] User/kernel privilege boundary

### Storage and user space — planned

- [ ] Block-device abstraction
- [ ] Filesystem layer
- [ ] Executable loader
- [ ] Interactive shell
- [ ] Initial user programs

Networking and additional device support will follow once the core execution and memory model is stable.

## Engineering Principles

- **Mechanism before abstraction** — kernel abstractions are introduced when the underlying mechanism is understood and testable.
- **Explicit ownership** — shared kernel state should have a clear synchronization and lifetime model.
- **Incremental validation** — every subsystem should keep the workspace buildable and bootable.
- **Observable execution** — the framebuffer provides local visual output while COM1 provides deterministic machine-readable diagnostics.
- **Architecture-first design** — interfaces are kept narrow so memory, interrupts, execution, drivers, and user space can evolve independently.
- **Failure visibility** — QEMU is configured not to automatically reboot after a fatal guest failure, making kernel faults diagnosable instead of silently restarting.

## Toolchain

- Rust nightly
- `x86_64-unknown-none`
- `bootloader` 0.11.x
- QEMU
- GitHub Actions

The repository declares the required Rust target and components in `rust-toolchain.toml`.

## Development

Install Rust through [rustup](https://rustup.rs/) and QEMU for your platform.

Clone the repository:

```bash
git clone https://github.com/Scarlet-Twinz/ORBIT.git
cd ORBIT
```

Format the workspace:

```bash
cargo fmt --all
```

Build the bootable BIOS image:

```bash
cargo build --release -p orbit-os
```

Run ORBIT in QEMU:

```bash
cargo run --release -p orbit-os
```

The host crate builds the kernel, creates a BIOS disk image, and launches `qemu-system-x86_64`. The guest exposes its early boot diagnostics through the COM1 serial console and renders its framebuffer console through the bootloader-provided graphics buffer.

## Project Direction

ORBIT is intentionally a kernel project rather than a desktop environment. The objective is to implement the layers that connect software to the machine and to make each layer observable, testable, and replaceable.

The long-term architecture moves from bootstrapping into memory management, interrupts, execution, system calls, storage, drivers, and user space. The current boot milestone establishes the foundation on which those subsystems can be built without hiding the machine behind a high-level runtime.
