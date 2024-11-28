use crate::ppu::Ppu;

use crate::ppu::config::WIDTH;
use crate::ppu::{get_u32_color, Tile};

use super::object::Object;

impl Ppu {
    pub fn render(&mut self) {
        if self.is_bg_window_active() {
            self.render_back();
            if self.is_window() {
                self.render_window();
            }
        }
        if self.is_obj_active() {
            self.render_obj();
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

    fn render_obj(&mut self) {
        let to_display = self.get_object_to_display();
        for obj in to_display.iter() {
            self.render_object(obj);
        }
    }

    fn render_object(&mut self, obj: &Object) {
        let sprite = self.tiles[obj.index as usize];
        let height: usize;
        if self.is_obj_16() {
            height = 16;
        } else {
            height = 8;
        }
        let y: usize = obj.y.wrapping_sub(16) as usize + (self.ly as usize % height);
        let x: usize = obj.x.wrapping_sub(8) as usize;
        for xd in 0..8 {
            if (x + xd) > 159 || y > 143 {
                continue;
            }
            let color = self.get_sprite_color(sprite[y as usize % height][(xd + x) % 8], obj.flags);
            if color != 0 {
                self.video_buffer[y as usize * WIDTH + x + xd] = get_u32_color(color);
            }
        }
    }

    fn get_object_to_display(&mut self) -> Vec<Object> {
        let mut retval: Vec<Object> = vec![];
        let mut counter = 0;
        let size = self.is_obj_16();
        for object in self.objects.iter() {
            if counter == 10 {
                break;
            }
            let y = object.y;
            if is_object_visible(y, self.ly, size) {
                retval.push(*object);
            }

            counter += 1;
        }

        retval
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
    if big_sprite && y + 16 < 16 {
        return false;
    }
    if !big_sprite && y + 8 < 16 {
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
