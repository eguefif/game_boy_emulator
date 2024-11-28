#![allow(clippy::new_without_default)]

pub mod color;
pub mod config;
pub mod lcdc;
pub mod object;
pub mod renderer;
pub mod stat;
pub mod state_handler;
pub mod vram;

use crate::ppu::object::Object;
use config::State;
use config::{Tile, DEBUG_BUFFER, DEBUG_HEIGHT, DEBUG_WIDTH, OAM_SIZE, VIDEO_BUFFER, VRAM_SIZE};

use crate::ppu::color::get_u32_color;
use std::fmt;

pub struct Ppu {
    pub vblank: bool,
    pub stat_int: bool,
    pub dot: u32,
    pub video_buffer: [u32; VIDEO_BUFFER],

    scanline_drawn: bool,
    x: u8,
    ly: u8,
    lyc: u8,
    state: State,

    debug_tiles: [u32; DEBUG_BUFFER],
    vram: [u8; VRAM_SIZE],
    tiles: [Tile; 384],
    pub oam: [u8; OAM_SIZE],
    pub objects: Vec<Object>,

    dma: u8,
    lcdc: u8,
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
            vblank: false,
            stat_int: false,
            video_buffer: [0; VIDEO_BUFFER],
            dot: 0,

            scanline_drawn: false,
            state: State::Mode2,
            ly: 0,
            lyc: 0,
            x: 0,

            debug_tiles: [0; DEBUG_BUFFER],
            vram: [0; VRAM_SIZE],
            tiles: [[[0; 8]; 8]; 384],
            oam: [0; OAM_SIZE],
            objects: vec![],

            dma: 0,
            lcdc: 0,
            stat: 0b0000_0010,

            scy: 0,
            scx: 0,
            wy: 0,
            wx: 0,

            bgp: 0,
            obp0: 0,
            obp1: 0,
        }
    }

    pub fn step(&mut self) {
        if !self.is_lcd_active() {
            return;
        }
        for _ in 0..=4 {
            self.run_ppu();
        }
    }

    pub fn get_video_buffer(&mut self) -> &[u32] {
        &self.video_buffer
    }

    pub fn get_tiles_memory(&mut self) -> &[u32] {
        let mut y: usize = 0;
        let mut x: usize = 0;
        for tile in self.tiles.iter() {
            write_tile_in_debug_buffer(tile, &mut self.debug_tiles, x, y);
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
}

fn write_tile_in_debug_buffer(tile: &Tile, buffer: &mut [u32], x: usize, y: usize) {
    for yd in 0..8 {
        for xd in 0..8 {
            buffer[(y + yd) * DEBUG_WIDTH + xd + x] = get_u32_color(tile[yd][xd]);
        }
    }
}
impl fmt::Display for Ppu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Ppu: | LCDC: {:<30} | Stat: {:<10} | x: {:3} | scx: {:3} | Ly: {:3} | scy: {:3} | Lcy: {:3} | Mode: {:?} | dot: {}",
            get_lcdc(self.lcdc),
            get_stat(self.stat),
            self.x,
            self.scx,
            self.ly,
            self.scy,
            self.lyc,
            self.state,
            self.dot,
        )
    }
}

fn get_lcdc(lcdc: u8) -> String {
    let mut retval = String::new();
    if (lcdc & 0b1000_0000) > 0 {
        retval.push_str("LCD 1 ");
    } else {
        retval.push_str("LCD 0 ");
    }
    if (lcdc & 0b0000_0001) > 0 {
        retval.push_str("W/BG 1 ");
    } else {
        retval.push_str("W/BG 0 ");
    }

    if (lcdc & 0b0010_0000) > 0 {
        retval.push_str("W 1");
    }
    {
        retval.push_str("W 0 ");
    }

    if (lcdc & 0b0100_0000) > 0 {
        retval.push_str("W: W9C00 ");
    } else {
        retval.push_str("W: W9800 ");
    }

    if (lcdc & 0b0001_0000) > 0 {
        retval.push_str("Mode: abs, ");
    } else {
        retval.push_str("Mode: sign, ");
    }

    if (lcdc & 0b0000_1000) > 0 {
        retval.push_str("BG: 9C00, ");
    } else {
        retval.push_str("BG: 9800, ");
    }

    if (lcdc & 0b0000_0010) > 0 {
        retval.push_str("Obj 1, ");
    } else {
        retval.push_str("Obj 0, ");
    }
    if (lcdc & 0b0000_0100) > 0 {
        retval.push_str("size 8, ");
    } else {
        retval.push_str("size 16, ");
    }
    retval
}

fn get_stat(stat: u8) -> String {
    let mut retval = String::new();

    if (stat & 0b0100_0000) > 0 {
        retval.push_str("LYC ");
    }
    if (stat & 0b0010_0000) > 0 {
        retval.push_str("M2 ");
    }
    if (stat & 0b0001_0000) > 0 {
        retval.push_str("M1 ");
    }
    if (stat & 0b0000_1000) > 0 {
        retval.push_str("M0 ");
    }
    if (stat & 0b0000_0100) > 0 {
        retval.push_str("LYC == LY ");
    } else {
        retval.push_str("LYC != LY ");
    }
    match stat & 0b_0000_0011 {
        0b11 => retval.push_str("M3"),
        0b01 => retval.push_str("M1"),
        0b00 => retval.push_str("M0"),
        0b10 => retval.push_str("M2"),
        _ => {}
    }
    retval
}
