pub fn get_u32_color(value: u8) -> u32 {
    match value {
        0b00 => from_u8_rgb(10, 10, 10),
        0b01 => from_u8_rgb(130, 130, 130),
        0b10 => from_u8_rgb(210, 210, 210),
        0b11 => from_u8_rgb(255, 255, 255),
        _ => 0,
    }
}

pub fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}
