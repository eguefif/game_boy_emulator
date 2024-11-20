use crate::ppu::Ppu;
use crate::ppu::State;

impl Ppu {
    pub fn write_lcdc(&mut self, value: u8) {
        if self.state == State::Mode1 && (value & 0b_1000_0000) == 0 && self.stat & 0b_1000_0000 > 0
        {
            println!("forbiden tried to turn of lcd");
            return;
        }
        self.lcdc = value;
    }

    pub fn is_lcd_active(&mut self) -> bool {
        self.lcdc & 0b1000_0000 >= 1
    }

    pub fn is_bg_window_active(&mut self) -> bool {
        self.lcdc & 0b0000_0001 >= 1
    }

    pub fn is_window(&mut self) -> bool {
        self.lcdc & 0b0010_0000 >= 1
    }

    pub fn is_bg_tile_data1(&mut self) -> bool {
        self.lcdc & 0b0000_1000 == 0
    }

    pub fn is_window_tile_data1(&mut self) -> bool {
        self.lcdc & 0b0100_0000 == 0
    }
}
