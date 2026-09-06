#![no_std]
#![no_main]

use bootloader_api::{entry_point, BootInfo};

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    let message = b"ORBIT Kernel v0.1\nBoot successful.";
    let mut row = 0usize;
    let mut column = 0usize;

    unsafe {
        for byte in message.iter().copied() {
            if byte == b'\n' {
                row += 1;
                column = 0;
                continue;
            }

            let offset = (row * 80 + column) * 2;
            vga_buffer.add(offset).write_volatile(byte);
            vga_buffer.add(offset + 1).write_volatile(0x0f);
            column += 1;
        }
    }

    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
