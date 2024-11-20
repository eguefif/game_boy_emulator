use crate::ppu::Ppu;

use crate::ppu::config::{TILEMAP_SIZE, WIDTH};
use crate::ppu::{get_u32_color, Tile};

use crate::ppu::color::from_u8_rgb;

impl Ppu {
    pub fn render(&mut self) {
        for _ in 0..6 {
            if self.is_bg_window_active() {
                if self.is_window() {
                    self.render_window();
                } else {
                    self.render_bg();
                }
            }
            self.x += 1;
        }
    }

    fn render_bg(&mut self) {
        let index = self.get_tile_map_index();
        let tile = self.tiles[index];
        if self.is_lcd_active() {
            self.write_tile_in_video_buffer(&tile, self.x as usize, self.ly as usize);
        } else {
            write_white_in_video_buffer(&mut self.video_buffer, self.x as usize, self.ly as usize);
        }
    }

    fn get_tile_map_index(&mut self) -> usize {
        let base_index = self.get_tile_data_base_index();
        if base_index == 0 {
            let offset = self.get_bg_offset();
            base_index + offset
        } else {
            let offset = self.get_bg_offset() as i8;
            if offset < 0 {
                (base_index as i16 - offset as i16) as usize
            } else {
                base_index + offset as usize
            }
        }
    }

    fn get_tile_data_base_index(&mut self) -> usize {
        if self.lcdc & 0b_0001_0000 > 0 {
            0
        } else {
            384 / 2
        }
    }

    fn get_bg_offset(&mut self) -> usize {
        let index = (self.ly / 8) as usize * 32 + (self.x / 8) as usize;
        if self.is_bg_tile_data1() {
            self.vram[0x1800 + index] as usize
        } else {
            self.vram[0x1000 + index] as usize
        }
    }

    pub fn write_tile_in_video_buffer(&mut self, tile: &Tile, x: usize, y: usize) {
        let pixel = tile[y % 8][x % 8];
        let color = self.get_color_from_bg_palette(pixel);
        self.video_buffer[y * WIDTH + x] = get_u32_color(color);
    }

    fn render_window(&mut self) {}
}

fn wrapping_tilemap_add(x1: u8, x2: u8) -> u16 {
    if x1 as u16 + x2 as u16 > TILEMAP_SIZE {
        x1 as u16 + x2 as u16 - TILEMAP_SIZE
    } else {
        x1 as u16 + x2 as u16
    }
}

pub fn write_white_in_video_buffer(buffer: &mut [u32], x: usize, y: usize) {
    buffer[y * WIDTH + x] = from_u8_rgb(255, 255, 255);
}
