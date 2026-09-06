const SCAN_CODE_RELEASE: u8 = 0x80;

pub fn handle_scancode(scancode: u8) {
    if scancode & SCAN_CODE_RELEASE != 0 { return; }
    match scancode {
        0x1C => serial_println!("keyboard: enter"),
        0x01 => serial_println!("keyboard: escape"),
        0x0E => serial_println!("keyboard: backspace"),
        0x39 => serial_println!("keyboard: space"),
        0x10..=0x19 => serial_println!("keyboard: top-row scancode={:#x}", scancode),
        0x1E..=0x2C => serial_println!("keyboard: home-row scancode={:#x}", scancode),
        0x2D..=0x35 => serial_println!("keyboard: bottom-row scancode={:#x}", scancode),
        _ => {}
    }
}
