#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

mod interrupts;
mod keyboard;
mod memory;
mod serial;
mod storage;
mod syscall;
mod tasks;
mod vga;

use bootloader_api::{entry_point, BootInfo};
use x86_64::instructions::hlt;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    serial::init();
    serial_println!("ORBIT kernel starting");

    memory::init(&boot_info.memory_regions);
    vga::init(boot_info);
    vga::write_line(0, b"ORBIT Kernel v0.2");
    vga::write_line(1, b"Boot successful.");
    vga::write_line(3, b"Architecture: x86_64");
    vga::write_line(4, b"Memory: physical frame allocator");
    vga::write_line(5, b"Interrupts: IDT + PIC + PIT");
    vga::write_line(6, b"Input: PS/2 keyboard");
    vga::write_line(7, b"Execution: scheduler + syscall ABI");
    vga::write_line(8, b"Storage: block-device layer");
    vga::write_line(10, b"Serial console: COM1");

    interrupts::init();
    interrupts::init_pit(100);
    tasks::init();
    storage::init();
    syscall::describe();
    memory::describe();
    tasks::describe();

    serial_println!("ORBIT kernel initialized");
    serial_println!("architecture=x86_64");
    serial_println!("mode=bare-metal");
    serial_println!("memory=frame-allocator");
    serial_println!("interrupts=idt-pic");
    serial_println!("timer=pit-100hz");
    serial_println!("keyboard=ps2");
    serial_println!("scheduler=round-robin-model");
    serial_println!("syscall=abi-foundation");
    serial_println!("storage=block-device-foundation");
    serial_println!("serial=com1");

    interrupts::enable();
    serial_println!("interrupts: enabled");

    loop { hlt(); }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    serial_println!("kernel panic");
    loop { hlt(); }
}
