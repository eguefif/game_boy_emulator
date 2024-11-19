use crate::ppu::{from_u8_rgb, Ppu, VRAM_SIZE};

use crate::ppu::{get_u32_color, Tile, WIDTH};

const TILEMAP_SIZE: u16 = 32 * 32;

impl Ppu {
    pub fn render(&mut self) {
        for _ in 0..10 {
            //if self.is_bg_window_active() {
            if self.is_window() {
                self.render_window();
            } else {
                self.render_bg();
            }
            //}
            self.x += 1;
        }
    }

    fn render_bg(&mut self) {
        let index = self.get_tile_map_index();
        let tile = self.tiles[index];
        //if self.is_lcd_active() {
        write_tile_in_video_buffer(
            &tile,
            &mut self.video_buffer,
            self.x as usize,
            self.ly as usize,
        );
        //} else {
        //    write_white_in_video_buffer(&mut self.video_buffer, self.x as usize, self.ly as usize);
        //}
    }

    fn get_tile_map_index(&mut self) -> usize {
        let base_index = self.get_tile_data_base_index();
        if base_index == 0 {
            let offset = self.get_bg_index();
            base_index + offset
        } else {
            let offset = self.get_bg_index() as i8;
            base_index + offset as u8 as usize
        }
    }

    fn get_tile_data_base_index(&mut self) -> usize {
        if self.lcdc & 0b_0001_0000 > 0 {
            0
        } else {
            384 / 2
        }
    }

    fn get_bg_index(&mut self) -> usize {
        let index = (wrapping_tilemap_add(self.scy, self.ly) / 8) as usize * 32
            + (wrapping_tilemap_add(self.scx, self.x) / 8) as usize;
        if 0x1800 + index >= VRAM_SIZE || 0x1c00 + index >= VRAM_SIZE {
            return 0;
        }
        if self.lcdc & 0b_0000_1000 == 0 {
            self.vram[0x1800 + index] as usize
        } else {
            self.vram[0x1c00 + index] as usize
        }
    }

    fn render_window(&mut self) {}

    fn is_window(&mut self) -> bool {
        false
    }
}

fn wrapping_tilemap_add(x1: u8, x2: u8) -> u16 {
    if x1 as u16 + x2 as u16 > TILEMAP_SIZE {
        x1 as u16 + x2 as u16 - TILEMAP_SIZE
    } else {
        x1 as u16 + x2 as u16
    }
}

pub fn write_tile_in_video_buffer(tile: &Tile, buffer: &mut [u32], x: usize, y: usize) {
    buffer[y * WIDTH + x] = get_u32_color(tile[y % 8][x % 8]);
}

pub fn write_white_in_video_buffer(buffer: &mut [u32], x: usize, y: usize) {
    buffer[y * WIDTH + x] = from_u8_rgb(255, 255, 255);
}
