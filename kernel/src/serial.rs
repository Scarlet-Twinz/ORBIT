use core::fmt;

use spin::Mutex;
use uart_16550::{Config, Uart16550Tty};

static SERIAL1: Mutex<Option<Uart16550Tty>> = Mutex::new(None);

pub fn init() {
    let mut serial = SERIAL1.lock();

    if serial.is_none() {
        let uart = unsafe {
            Uart16550Tty::new_port(0x3F8, Config::default())
                .expect("failed to create COM1 UART")
        };
        *serial = Some(uart);
    }
}

pub fn _print(args: fmt::Arguments<'_>) {
    use fmt::Write;

    let mut serial = SERIAL1.lock();
    if let Some(uart) = serial.as_mut() {
        uart.write_fmt(args).expect("serial output failed");
    }
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
