use crate::ppu::Ppu;

use super::write_tile_in_buffer;

impl Ppu {
    pub fn render(&mut self) {
        for _ in 0..4 {
            if self.is_window() {
                self.render_window();
            } else {
                self.render_bg();
            }
            self.x += 1;
            if self.x >= 144 {
                return;
            }
        }
    }

    fn render_bg(&mut self) {
        let base_index = self.get_tile_data_base_index();
        let index;
        if base_index == 0 {
            let offset = self.get_bg_index();
            index = base_index + offset;
        } else {
            let offset = self.get_bg_index() as i8;
            index = (base_index + offset as u8 as usize) % 384;
        }
        let tile = self.tiles[index];
        let x = (self.scx.wrapping_add(self.x) % 8) as usize;
        let y = (self.scy.wrapping_add(self.ly) % 8) as usize;
        write_tile_in_buffer(&tile, &mut self.video_buffer, x, y);
    }

    fn get_tile_data_base_index(&mut self) -> usize {
        if self.lcdc & 0b_0001_0000 > 0 {
            0
        } else {
            384 / 2
        }
    }

    fn get_bg_index(&mut self) -> usize {
        ((self.scy + self.ly) / 8) as usize * 32 + ((self.scx + self.x) / 8) as usize
    }

    fn render_window(&mut self) {}

    fn is_window(&mut self) -> bool {
        false
    }
}
