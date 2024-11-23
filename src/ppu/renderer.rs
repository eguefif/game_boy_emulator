use crate::ppu::Ppu;

use crate::ppu::config::WIDTH;
use crate::ppu::{get_u32_color, Tile};

impl Ppu {
    pub fn render(&mut self) {
        if self.is_bg_window_active() {
            self.render_back();
            if self.is_window() {
                self.render_window();
            }
        }
    }

    fn render_back(&mut self) {
        for x in 0..(WIDTH / 8) {
            let offset = self.get_tile_offset(x as u8);
            let index = self.get_base_index_data(offset);

            let tile = self.tiles[index];
            self.write_tile_in_video_buffer(&tile, x, self.ly as usize);
        }
    }

    fn get_tile_offset(&mut self, x: u8) -> u8 {
        let base = self.get_base_index();
        let x_offset = ((self.scx / 8) + x) & 0x1F;
        let y_offset = self.ly.wrapping_add(self.scy);
        let offset = base + x_offset as usize + 32 * (y_offset / 8) as usize;
        self.vram[offset]
    }

    fn get_base_index(&mut self) -> usize {
        if self.is_bg_tilemap2() {
            0x1c00
        } else {
            0x1800
        }
    }

    fn get_base_index_data(&mut self, offset: u8) -> usize {
        if self.is_tiledata1() {
            offset as usize
        } else {
            128 + ((offset as i8 as i16) + 128) as usize
        }
    }

    pub fn write_tile_in_video_buffer(&mut self, tile: &Tile, x: usize, y: usize) {
        for xd in 0..8 {
            let pixel = tile[y % 8][xd];
            let color = self.get_color_from_bg_palette(pixel);
            self.video_buffer[y * WIDTH + x * 8 + xd] = get_u32_color(color);
        }
    }

    fn render_window(&mut self) {}
}
