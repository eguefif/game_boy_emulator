#![allow(clippy::new_without_default)]

const VRAM_SIZE: usize = 0x9FFF - 0x8000 + 1;
const OAM_SIZE: usize = 0xFE9F - 0xFE00 + 1;

type Tile = [[u8; 8]; 8];

pub mod renderer;
pub mod state_handler;

pub const DEBUG_WIDTH: usize = 256;
pub const DEBUG_HEIGHT: usize = 192;
const DEBUG_BUFFER: usize = DEBUG_WIDTH * DEBUG_HEIGHT;

pub const WIDTH: usize = 160;
pub const HEIGHT: usize = 144;
const VIDEO_BUFFER: usize = WIDTH * HEIGHT;

#[derive(PartialEq, Debug)]
enum State {
    Mode2,
    Mode3,
    Mode0,
    Mode1,
}

#[derive(PartialEq)]
pub enum PpuInterrupt {
    Vblank,
    Stat,
    None,
}

pub struct Ppu {
    x: u8,
    pub interrupt: PpuInterrupt,
    state: State,
    dot: u32,
    pub video_buffer: [u32; VIDEO_BUFFER],
    debug_tiles: [u32; DEBUG_BUFFER],
    vram: [u8; VRAM_SIZE],
    tiles: [Tile; 384],
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
            x: 0,
            dot: 0,
            state: State::Mode2,
            debug_tiles: [0; DEBUG_BUFFER],
            video_buffer: [0; VIDEO_BUFFER],
            vram: [0; VRAM_SIZE],
            tiles: [[[0; 8]; 8]; 384],
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
            interrupt: PpuInterrupt::None,
        }
    }

    pub fn step(&mut self) {
        self.update_state();
        self.run_ppu();
    }

    pub fn read(&mut self, loc: usize) -> u8 {
        match loc {
            0x8000..=0x9FFF => {
                if self.state != State::Mode3 {
                    self.vram[loc - 0x8000]
                } else {
                    0xFF
                }
            }
            0xFE00..=0xFE9F => {
                if self.state == State::Mode0 || self.state == State::Mode1 {
                    self.oam[loc - 0xFE00]
                } else {
                    0xFF
                }
            }

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
            0x8000..=0x9FFF => {
                //if true {
                if self.state != State::Mode3 {
                    self.vram[loc - 0x8000] = value;
                    if loc < 0x97FF {
                        self.update_tiles(loc - 0x8000);
                    }
                } else {
                    println!(
                        "Try to access vram. Mode: {:?}, dot: {:?}, stat: {:b}, lcdc: {:b}",
                        self.state, self.dot, self.stat, self.lcdc
                    );
                }
            }
            0xFE00..=0xFE9F => {
                if self.state == State::Mode0 || self.state == State::Mode1 {
                    self.oam[loc - 0xFE00] = value
                }
            }

            0xFF40 => self.lcdc = value,
            0xFF41 => self.stat = value & 0b_0111_1100,
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

    fn update_tiles(&mut self, loc: usize) {
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
                (true, false) => 1,
                (false, true) => 2,
                (true, true) => 3,
            };
            self.tiles[tile_loc][row_loc][pixel_index] = value;
        }
    }

    pub fn get_tiles_memory(&mut self) -> &[u32] {
        let mut y: usize = 0;
        let mut x: usize = 0;
        for tile in self.tiles.iter() {
            write_tile_in_buffer(tile, &mut self.debug_tiles, x, y);
            x += 8;
            if x >= DEBUG_WIDTH {
                x = 0;
                y += 8
            }
            if y >= DEBUG_HEIGHT {
                break;
            }
        }
        &self.debug_tiles
    }

    pub fn get_video_buffer(&mut self) -> &[u32] {
        &self.video_buffer
    }
}

pub fn write_tile_in_buffer(tile: &Tile, buffer: &mut [u32], x: usize, y: usize) {
    for yd in 0..8 {
        for xd in 0..8 {
            buffer[(y + yd) * DEBUG_WIDTH + xd + x] = get_u32_color(tile[yd][xd]);
        }
    }
}

fn get_u32_color(value: u8) -> u32 {
    match value {
        0b00 => from_u8_rgb(15, 15, 15),
        0b01 => from_u8_rgb(75, 75, 75),
        0b10 => from_u8_rgb(150, 150, 150),
        0b11 => from_u8_rgb(255, 255, 255),
        _ => 0,
    }
}

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}
