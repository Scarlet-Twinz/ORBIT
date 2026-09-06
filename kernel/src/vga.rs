use bootloader_api::{info::FrameBufferInfo, BootInfo};
use spin::Mutex;

struct Framebuffer {
    buffer: &'static mut [u8],
    info: FrameBufferInfo,
}

static FRAMEBUFFER: Mutex<Option<Framebuffer>> = Mutex::new(None);

const GLYPH_W: usize = 5;
const GLYPH_H: usize = 7;
const SCALE: usize = 3;
const LINE_HEIGHT: usize = 28;

pub fn init(boot_info: &'static mut BootInfo) {
    let Some(framebuffer) = boot_info.framebuffer.take() else {
        return;
    };

    let info = framebuffer.info();
    let buffer = framebuffer.into_buffer();

    *FRAMEBUFFER.lock() = Some(Framebuffer { buffer, info });

    clear();
}

pub fn clear() {
    let mut framebuffer = FRAMEBUFFER.lock();
    let Some(framebuffer) = framebuffer.as_mut() else {
        return;
    };

    for pixel in framebuffer
        .buffer
        .chunks_exact_mut(framebuffer.info.bytes_per_pixel)
    {
        for byte in pixel.iter_mut() {
            *byte = 0;
        }
    }
}

pub fn write_line(row: usize, text: &[u8]) {
    let y = 24 + row * LINE_HEIGHT;
    let mut x = 24;

    for &byte in text {
        if x + GLYPH_W * SCALE >= width() {
            break;
        }
        draw_char(x, y, byte);
        x += (GLYPH_W + 1) * SCALE;
    }
}

fn width() -> usize {
    FRAMEBUFFER
        .lock()
        .as_ref()
        .map(|framebuffer| framebuffer.info.width)
        .unwrap_or(0)
}

fn draw_char(x: usize, y: usize, byte: u8) {
    let glyph = glyph(byte);

    for (gy, bits) in glyph.iter().enumerate() {
        for gx in 0..GLYPH_W {
            if bits & (1 << (GLYPH_W - 1 - gx)) == 0 {
                continue;
            }
            for sy in 0..SCALE {
                for sx in 0..SCALE {
                    put_pixel(x + gx * SCALE + sx, y + gy * SCALE + sy, 255, 255, 255);
                }
            }
        }
    }
}

fn put_pixel(x: usize, y: usize, r: u8, g: u8, b: u8) {
    let mut framebuffer = FRAMEBUFFER.lock();
    let Some(framebuffer) = framebuffer.as_mut() else {
        return;
    };

    let info = framebuffer.info;
    if x >= info.width || y >= info.height {
        return;
    }

    let offset = (y * info.stride + x) * info.bytes_per_pixel;
    if offset + info.bytes_per_pixel > framebuffer.buffer.len() {
        return;
    }

    match info.pixel_format {
        bootloader_api::info::PixelFormat::Rgb => {
            framebuffer.buffer[offset] = r;
            framebuffer.buffer[offset + 1] = g;
            framebuffer.buffer[offset + 2] = b;
        }
        bootloader_api::info::PixelFormat::Bgr => {
            framebuffer.buffer[offset] = b;
            framebuffer.buffer[offset + 1] = g;
            framebuffer.buffer[offset + 2] = r;
        }
        bootloader_api::info::PixelFormat::U8 => {
            framebuffer.buffer[offset] = r;
        }
        _ => {
            for byte in &mut framebuffer.buffer[offset..offset + info.bytes_per_pixel] {
                *byte = 255;
            }
        }
    }
}

#[rustfmt::skip]
fn glyph(byte: u8) -> [u8; GLYPH_H] {
    let c = byte.to_ascii_uppercase();
    match c {
        b'A' => [0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001],
        b'B' => [0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110],
        b'C' => [0b01111, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b01111],
        b'D' => [0b11110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11110],
        b'E' => [0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111],
        b'F' => [0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000],
        b'G' => [0b01111, 0b10000, 0b10000, 0b10111, 0b10001, 0b10001, 0b01111],
        b'H' => [0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001],
        b'I' => [0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b11111],
        b'J' => [0b00111, 0b00010, 0b00010, 0b00010, 0b00010, 0b10010, 0b01100],
        b'K' => [0b10001, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010, 0b10001],
        b'L' => [0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111],
        b'M' => [0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001, 0b10001],
        b'N' => [0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001, 0b10001],
        b'O' => [0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110],
        b'P' => [0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000],
        b'Q' => [0b01110, 0b10001, 0b10001, 0b10001, 0b10101, 0b10010, 0b01101],
        b'R' => [0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001],
        b'S' => [0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110],
        b'T' => [0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100],
        b'U' => [0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110],
        b'V' => [0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100],
        b'W' => [0b10001, 0b10001, 0b10001, 0b10101, 0b10101, 0b11011, 0b10001],
        b'X' => [0b10001, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001, 0b10001],
        b'Y' => [0b10001, 0b10001, 0b01010, 0b00100, 0b00100, 0b00100, 0b00100],
        b'Z' => [0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b11111],
        b'0' => [0b01110, 0b10001, 0b10011, 0b10101, 0b11001, 0b10001, 0b01110],
        b'1' => [0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110],
        b'2' => [0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b01000, 0b11111],
        b'3' => [0b11110, 0b00001, 0b00001, 0b01110, 0b00001, 0b00001, 0b11110],
        b'4' => [0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010],
        b'5' => [0b11111, 0b10000, 0b10000, 0b11110, 0b00001, 0b00001, 0b11110],
        b'6' => [0b01110, 0b10000, 0b10000, 0b11110, 0b10001, 0b10001, 0b01110],
        b'7' => [0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000],
        b'8' => [0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110],
        b'9' => [0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00001, 0b01110],
        b'.' => [0, 0, 0, 0, 0, 0b00100, 0b00100],
        b':' => [0, 0b00100, 0b00100, 0, 0b00100, 0b00100, 0],
        b'-' => [0, 0, 0, 0b11111, 0, 0, 0],
        b'_' => [0, 0, 0, 0, 0, 0, 0b11111],
        _ => [0; GLYPH_H],
    }
}
