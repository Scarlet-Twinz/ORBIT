use core::fmt;

use spin::Mutex;
use uart_16550::SerialPort;

static SERIAL1: Mutex<SerialPort> = Mutex::new(unsafe { SerialPort::new(0x3F8) });

pub fn init() {
    SERIAL1.lock().init();
}

pub fn _print(args: fmt::Arguments<'_>) {
    use fmt::Write;

    SERIAL1
        .lock()
        .write_fmt(args)
        .expect("serial output failed");
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
