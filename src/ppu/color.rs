use crate::ppu::Ppu;

impl Ppu {
    pub fn get_color_from_bg_palette(&mut self, value: u8) -> u8 {
        match value & 0b0000_0011 {
            0 => self.bgp & 0b0000_0011,
            1 => (self.bgp >> 2) & 0b0000_0011,
            2 => (self.bgp >> 4) & 0b0000_0011,
            3 => (self.bgp >> 6) & 0b0000_0011,
            _ => 0,
        }
    }
    pub fn get_sprite_color(&mut self, value: u8, flags: u8) -> u8 {
        let mut palette = self.obp0;
        if flags & 0x10 == 0x10 {
            palette = self.obp1;
        }
        match value & 0b0000_0011 {
            0 => palette & 0b0000_0011,
            1 => (palette >> 2) & 0b0000_0011,
            2 => (palette >> 4) & 0b0000_0011,
            3 => (palette >> 6) & 0b0000_0011,
            _ => 0,
        }
    }
}

pub fn get_u32_color(value: u8) -> u32 {
    match value {
        0b00 => from_u8_rgb(0x2A, 0x45, 0x3B),
        0b01 => from_u8_rgb(0x36, 0x5D, 0x48),
        0b10 => from_u8_rgb(0x57, 0x7C, 0x44),
        0b11 => from_u8_rgb(0x7F, 0x86, 0x0F),
        _ => 0,
    }
}

pub fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}
