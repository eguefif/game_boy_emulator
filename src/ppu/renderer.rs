use crate::ppu::Ppu;

use crate::ppu::config::WIDTH;
use crate::ppu::{get_u32_color, Tile};

use super::config::HEIGHT;
use super::object::{flip_tile_if_flag, flip_tile_if_flag_16, Object};

impl Ppu {
    pub fn render(&mut self) {
        if self.is_bg_window_active() {
            self.render_back();
            if self.window_visible() {
                self.render_window();
            }
        } else {
            self.paint_white();
        }
        if self.is_obj_active() {
            self.render_obj();
        }
    }

    fn render_back(&mut self) {
        for x in 0..(WIDTH / 8) {
            let offset = self.get_tile_offset(x as u8, self.ly);
            let index = self.get_base_index_data(offset);

            let tile = self.tiles[index];
            self.write_tile_in_video_buffer(&tile, x, self.ly as usize);
        }
    }

    fn get_tile_offset(&mut self, x: u8, y: u8) -> u8 {
        let base = self.get_base_index();
        let x_offset = ((self.scx / 8) + x) & 0x1F;
        let y_offset = y.wrapping_add(self.scy);
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
            128 + (offset.wrapping_add(128) as usize)
        }
    }

    pub fn write_tile_in_video_buffer(&mut self, tile: &Tile, x: usize, y: usize) {
        for xd in 0..8 {
            let pixel = tile[y % 8][xd];
            let color = self.get_color_from_bg_palette(pixel);
            self.video_buffer[y * WIDTH + x * 8 + xd] = get_u32_color(color);
            self.bg_trace[y * WIDTH + x * 8 + xd] = get_u32_color(color);
        }
    }

    fn render_window(&mut self) {
        for x in 0..(WIDTH / 8) {
            let offset = self.get_window_tile_offset(x as u8, self.window_ly);
            let index = self.get_base_index_data(offset);

            let tile = self.tiles[index];
            self.write_tile_in_video_buffer(
                &tile,
                (self.wx as usize + (x * 8)) / 8,
                self.ly as usize,
            );
        }
    }

    fn get_window_tile_offset(&mut self, x: u8, y: u8) -> u8 {
        let base = self.get_window_base_index();
        let offset = base + x as usize + 32 * (y / 8) as usize;
        self.vram[offset]
    }

    fn get_window_base_index(&mut self) -> usize {
        if self.is_window_tilemap2() {
            0x1c00
        } else {
            0x1800
        }
    }

    fn render_obj(&mut self) {
        let mut to_display = self.get_object_to_display();
        to_display.sort();
        for obj in to_display.iter().rev() {
            self.render_object(obj);
        }
    }

    fn render_object(&mut self, obj: &Object) {
        if self.is_obj_16() {
            let tile = self.tiles[obj.index as usize & 0xFE];
            let tile2 = self.tiles[obj.index as usize | 0x01];
            let (sprite1, sprite2) = flip_tile_if_flag_16(tile, tile2, obj.flags);
            self.render_obj_8(sprite2, obj, 8);
            self.render_obj_8(sprite1, obj, 0);
        } else {
            let tile = self.tiles[obj.index as usize];
            let sprite = flip_tile_if_flag(tile, obj.flags);
            self.render_obj_8(sprite, obj, 0);
        }
    }

    fn render_obj_8(&mut self, sprite: Tile, obj: &Object, y_offset: u8) {
        let y: usize =
            obj.y.wrapping_sub(16).wrapping_add(y_offset) as usize + (self.ly as usize % 8);
        let x: usize = obj.x.wrapping_sub(8) as usize;
        for xd in 0..8 {
            if (x + xd) >= WIDTH || y >= HEIGHT {
                continue;
            }
            let pixel = sprite[y % 8][(xd + x) % 8];
            if pixel != 0 {
                let color = self.get_sprite_color(pixel, obj.flags);
                if obj.flags & 0x80 == 0x80 && self.is_bg_window_collision(x + xd, y) {
                    continue;
                }
                self.video_buffer[y * WIDTH + x + xd] = get_u32_color(color);
            }
        }
    }

    fn is_bg_window_collision(&mut self, x: usize, y: usize) -> bool {
        let color = self.bg_trace[y * WIDTH + x];
        color != get_u32_color(0)
    }

    fn get_object_to_display(&mut self) -> Vec<Object> {
        let mut retval: Vec<Object> = vec![];
        let size = self.is_obj_16();
        let mut obj_iter = self.objects.iter();
        let mut counter = 0;
        loop {
            if counter == 10 {
                break;
            }
            let obj = obj_iter.next();
            if let Some(object) = obj {
                let y = object.y;
                if is_object_visible(y, self.ly, size) {
                    retval.push(*object);
                    counter += 1;
                }
            } else {
                break;
            }
        }
        retval
    }

    fn paint_white(&mut self) {
        let color = self.get_color_from_bg_palette(0);
        for x in 0..(WIDTH) {
            self.video_buffer[self.ly as usize * WIDTH + x] = get_u32_color(color);
        }
    }
}
fn is_object_visible(y: u8, ly: u8, big_sprite: bool) -> bool {
    let adjust_ly = ly + 16;
    if !is_sprite_in_visible_frame(y, big_sprite) {
        return false;
    }
    if !big_sprite {
        adjust_ly >= y && adjust_ly < (y + 8)
    } else {
        adjust_ly >= y && adjust_ly < (y + 16)
    }
}

fn is_sprite_in_visible_frame(y: u8, big_sprite: bool) -> bool {
    if y > 160 {
        return false;
    }
    if big_sprite && y.wrapping_add(16) < 16 {
        return false;
    }
    if !big_sprite && y.wrapping_add(16) + 8 < 16 {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_be_visible() {
        let res = is_object_visible(9, 0, false);

        assert!(res);
    }

    #[test]
    fn it_should_not_be_visible() {
        let res = is_object_visible(8, 0, false);

        assert!(!res);
    }

    #[test]
    fn it_should_not_be_visible_for_big_sprite() {
        let res = is_object_visible(0, 0, true);

        assert!(!res);
    }

    #[test]
    fn it_should_be_visible_16() {
        let res = is_object_visible(2, 0, true);

        assert!(res);
    }

    #[test]
    fn it_should_not_be_visible_8() {
        let res = is_object_visible(160, 143, false);

        assert!(!res);
    }

    #[test]
    fn it_should_be_visible_8_lower_part() {
        let res = is_object_visible(159, 143, false);

        assert!(res);
    }

    #[test]
    fn it_should_not_be_visible_16_lower_limit() {
        let res = is_object_visible(40, 23, true);

        assert!(!res);
    }

    #[test]
    fn it_should_be_visible_16_lower_limit() {
        let res = is_object_visible(40, 24, true);

        assert!(res);
    }

    #[test]
    fn it_should_not_be_visible_16_higher_limit() {
        let res = is_object_visible(40, 40, true);

        assert!(!res);
    }

    #[test]
    fn it_should_not_be_visible_8_higher_limit() {
        let res = is_object_visible(40, 32, false);

        assert!(!res);
    }

    #[test]
    fn it_should_be_visible_16_higher_limit() {
        let res = is_object_visible(40, 39, true);

        assert!(res);
    }

    #[test]
    fn it_should_be_visible_8_higher_limit() {
        let res = is_object_visible(40, 31, false);

        assert!(res);
    }
}
