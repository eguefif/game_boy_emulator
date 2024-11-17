#![allow(clippy::new_without_default)]

const TILES_SIZE: usize = 0x97FF - 0x8000 + 1;
const MAP_SIZE: usize = 0x9BFF - 0x9800;
const OAM_SIZE: usize = 0xFE9F - 0xFE00;
const RESOLUTION: usize = 144 * 160;
const WIDTH: usize = 144;

type Tile = [[u8; 8]; 8];

pub struct Ppu {
    debug_tiles: [u32; RESOLUTION + 1],
    tiles_ram: [u8; TILES_SIZE],
    tiles: [Tile; 384],
    map1: [u8; MAP_SIZE],
    map2: [u8; MAP_SIZE],
    oam: [u8; OAM_SIZE],
    dma: u8,
    lcdc: u8,
    ly: u8,
    lyc: u8,
    stat: u8,
    scy: u8,
    scx: u8,
    wy: u8,
    wx: u8,
    bgp: u8,
    obp0: u8,
    obp1: u8,
}

impl Ppu {
    pub fn new() -> Ppu {
        Ppu {
            debug_tiles: [0; RESOLUTION + 1],
            tiles_ram: [0; TILES_SIZE],
            tiles: [[[0; 8]; 8]; 384],
            map1: [0; MAP_SIZE],
            map2: [0; MAP_SIZE],
            oam: [0; OAM_SIZE],
            dma: 0,
            lcdc: 0,
            ly: 0,
            lyc: 0,
            stat: 0,
            scy: 0,
            scx: 0,
            wy: 0,
            wx: 0,
            bgp: 0,
            obp0: 0,
            obp1: 0,
        }
    }

    pub fn read(&mut self, loc: usize) -> u8 {
        match loc {
            0x8000..=0x97FF => self.tiles_ram[loc - 0x8000],
            0x9800..=0x9BFF => self.map1[loc - 0x9800],
            0x9C00..=0x9FFF => self.map2[loc - 0x9C00],
            0xFE00..=0xFE9F => self.oam[loc - 0x8FFF],

            0xFF40 => self.lcdc,
            0xFF41 => self.stat,
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.ly,
            0xFF45 => self.lyc,
            0xFF46 => self.dma,
            0xFF47 => self.bgp,
            0xFF48 => self.obp0,
            0xFF49 => self.obp1,
            0xFF4A => self.wy,
            0xFF4B => self.wx,
            _ => 0xFF,
        }
    }
    pub fn write(&mut self, loc: usize, value: u8) {
        match loc {
            0x8000..=0x97FF => {
                self.tiles_ram[loc - 0x8000] = value;
                self.write_tile(loc - 0x8000);
            }
            0x9800..=0x9BFF => self.map1[loc - 0x9800] = value,
            0x9C00..=0x9FFF => self.map2[loc - 0x9C00] = value,
            0xFE00..=0xFE9F => self.oam[loc - 0xFE00] = value,

            0xFF40 => self.lcdc = value,
            0xFF41 => self.stat = value,
            0xFF42 => self.scy = value,
            0xFF43 => self.scx = value,
            0xFF44 => self.ly = value,
            0xFF45 => self.lyc = value,
            0xFF46 => self.dma = value,
            0xFF47 => self.bgp = value,
            0xFF48 => self.obp0 = value,
            0xFF49 => self.obp1 = value,
            0xFF4A => self.wy = value,
            0xFF4B => self.wx = value,
            _ => {}
        }
    }

    fn write_tile(&mut self, loc: usize) {
        let normalized_loc = loc & 0xFFFE;
        let tile_loc = loc / 16;
        let row_loc = loc % 16 / 2;

        let byte1 = self.tiles_ram[normalized_loc];
        let byte2 = self.tiles_ram[normalized_loc + 1];
        for pixel_index in 0..8 {
            let mask = 1 << pixel_index;
            let lsb = byte1 & mask;
            let msb = byte2 & mask;
            let value = match (lsb != 0, msb != 0) {
                (false, false) => 0,
                (true, false) => 1,
                (false, true) => 2,
                (true, true) => 3,
            };
            self.tiles[tile_loc][row_loc][pixel_index] = value;
        }
    }

    pub fn get_tiles_memory(&mut self) -> &[u32] {
        let mut y = 0;
        let mut x = 0;
        for tile in self.tiles.iter() {
            write_tile_in_buffer(tile, &mut self.debug_tiles, x, y);
            x += 8;
            y += 8;
            if x >= 144 {
                x = 0;
            }
            if y >= 160 {
                y = 0;
            }
        }
        &self.debug_tiles
    }
}

fn write_tile_in_buffer(tile: &Tile, buffer: &mut [u32], x: u8, y: u8) {
    for yd in 0..8 {
        for xd in 0..8 {
            buffer[(y + yd) as usize * WIDTH + xd + x as usize] =
                get_u32_color(tile[yd as usize][xd]);
        }
    }
}

fn get_u32_color(value: u8) -> u32 {
    match value {
        0b00 => from_u8_rgb(15, 56, 15),
        0b01 => from_u8_rgb(48, 98, 48),
        0b10 => from_u8_rgb(139, 172, 15),
        0b11 => from_u8_rgb(155, 188, 15),
        _ => 0,
    }
}

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}
