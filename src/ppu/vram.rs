use crate::ppu::Ppu;

use crate::ppu::config::State;

impl Ppu {
    pub fn read(&mut self, loc: usize) -> u8 {
        match loc {
            0x8000..=0x9FFF => {
                if self.state != State::Mode3 || !self.is_lcd_active() {
                    self.vram[loc - 0x8000]
                } else {
                    0xFF
                }
            }
            0xFE00..=0xFE9F => {
                if self.state == State::Mode0 || self.state == State::Mode1 || !self.is_lcd_active()
                {
                    self.oam[loc - 0xFE00]
                } else {
                    0xFF
                }
            }

            0xFF40 => self.lcdc,
            0xFF41 => self.stat,
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF4A => self.wy,
            0xFF4B => self.wx,

            0xFF44 => self.ly,
            0xFF45 => self.lyc,

            0xFF46 => self.dma,

            0xFF47 => self.bgp,
            0xFF48 => self.obp0,
            0xFF49 => self.obp1,
            _ => 0xFF,
        }
    }
    pub fn write_oam(&mut self, loc: usize, value: u8) {
        self.oam[loc - 0xFE00] = value;
    }

    pub fn write(&mut self, loc: usize, value: u8) {
        match loc {
            0x8000..=0x9FFF => {
                if self.state != State::Mode3 || !self.is_lcd_active() {
                    self.vram[loc - 0x8000] = value;
                    if loc < 0x97FF {
                        self.write_tiles(loc - 0x8000);
                    }
                }
            }
            0xFE00..=0xFE9F => {
                if self.state == State::Mode0 || self.state == State::Mode1 || !self.is_lcd_active()
                {
                    self.oam[loc - 0xFE00] = value
                }
            }

            0xFF40 => self.write_lcdc(value),
            0xFF41 => self.write_stat(value),
            0xFF42 => self.scy = value,
            0xFF43 => self.scx = value,
            0xFF4A => self.wy = value,
            0xFF4B => self.wx = value,
            0xFF44 => {}
            0xFF45 => self.lyc = value,
            0xFF46 => self.dma = value,

            0xFF47 => self.bgp = value,
            0xFF48 => self.obp0 = value,
            0xFF49 => self.obp1 = value,
            _ => {}
        }
    }

    fn write_tiles(&mut self, loc: usize) {
        let normalized_loc = loc & 0xFFFE;
        let tile_loc = loc / 16;
        let row_loc = (loc % 16) / 2;

        let byte1 = self.vram[normalized_loc];
        let byte2 = self.vram[normalized_loc + 1];
        for pixel_index in 0..8 {
            let mask = 1 << (7 - pixel_index);
            let lsb = byte1 & mask;
            let msb = byte2 & mask;
            let value = match (lsb != 0, msb != 0) {
                (false, false) => 0,
                (true, false) => 2,
                (false, true) => 1,
                (true, true) => 3,
            };
            self.tiles[tile_loc][row_loc][pixel_index] = value;
        }
    }
}
