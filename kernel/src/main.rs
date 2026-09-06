#![no_std]
#![no_main]

mod serial;
mod vga;

use bootloader_api::{entry_point, BootInfo};

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    serial::init();
    vga::clear();

    vga::write_line(0, b"ORBIT Kernel v0.1");
    vga::write_line(1, b"Boot successful.");
    vga::write_line(3, b"Architecture: x86_64");
    vga::write_line(4, b"Mode: bare metal");
    vga::write_line(6, b"Serial console: COM1");

    serial_println!("ORBIT kernel initialized");
    serial_println!("architecture=x86_64");
    serial_println!("mode=bare-metal");
    serial_println!("serial=com1");

    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    serial_println!("kernel panic");

    loop {
        core::hint::spin_loop();
    }
}
