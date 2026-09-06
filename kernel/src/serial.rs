use core::arch::asm;
use core::fmt;

pub fn init() {
    unsafe {
        outb(0x3F8 + 1, 0x00); // disable interrupts
        outb(0x3F8 + 3, 0x80); // enable DLAB
        outb(0x3F8, 0x03); // 38400 baud divisor low byte
        outb(0x3F8 + 1, 0x00); // divisor high byte
        outb(0x3F8 + 3, 0x03); // 8 bits, no parity, one stop bit
        outb(0x3F8 + 2, 0xC7); // enable FIFO, clear, 14-byte threshold
        outb(0x3F8 + 4, 0x0B); // IRQs enabled, RTS/DSR set
    }
}

pub fn _print(args: fmt::Arguments<'_>) {
    use fmt::Write;
    let mut writer = SerialWriter;
    let _ = writer.write_fmt(args);
}

struct SerialWriter;

impl fmt::Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            unsafe {
                while (inb(0x3F8 + 5) & 0x20) == 0 {
                    core::hint::spin_loop();
                }
                outb(0x3F8, byte);
            }
        }
        Ok(())
    }
}

unsafe fn outb(port: u16, value: u8) {
    asm!("out dx, al", in("dx") port, in("al") value, options(nostack, preserves_flags));
}

unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    asm!("in al, dx", in("dx") port, out("al") value, options(nostack, preserves_flags));
    value
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! serial_println {
    () => {
        $crate::serial::_print(format_args!("\n"))
    };
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!("{}\n", format_args!($($arg)*)))
    };
}
