#![no_std]
#![no_main]

use bootloader_api::{entry_point, BootInfo};

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    let message = b"ORBIT Kernel v0.1\nBoot successful.\n";

    unsafe {
        for (index, byte) in message.iter().copied().enumerate() {
            if byte == b'\n' {
                continue;
            }

            let offset = index * 2;
            vga_buffer.add(offset).write_volatile(byte);
            vga_buffer.add(offset + 1).write_volatile(0x0f);
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
