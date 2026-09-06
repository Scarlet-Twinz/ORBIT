const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
const WIDTH: usize = 80;
const HEIGHT: usize = 25;
const DEFAULT_COLOR: u8 = 0x0f;

pub fn clear() {
    unsafe {
        for row in 0..HEIGHT {
            for column in 0..WIDTH {
                let offset = (row * WIDTH + column) * 2;
                VGA_BUFFER.add(offset).write_volatile(b' ');
                VGA_BUFFER.add(offset + 1).write_volatile(DEFAULT_COLOR);
            }
        }
    }
}

pub fn write_line(row: usize, text: &[u8]) {
    if row >= HEIGHT {
        return;
    }

    unsafe {
        for column in 0..WIDTH {
            let byte = text.get(column).copied().unwrap_or(b' ');
            let offset = (row * WIDTH + column) * 2;
            VGA_BUFFER.add(offset).write_volatile(byte);
            VGA_BUFFER.add(offset + 1).write_volatile(DEFAULT_COLOR);
        }
    }
}
