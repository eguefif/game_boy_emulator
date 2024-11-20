pub const VRAM_SIZE: usize = 0x9FFF - 0x8000 + 1;
pub const OAM_SIZE: usize = 0xFE9F - 0xFE00 + 1;

pub type Tile = [[u8; 8]; 8];

pub const DEBUG_WIDTH: usize = 256;
pub const DEBUG_HEIGHT: usize = 192;
pub const DEBUG_BUFFER: usize = DEBUG_WIDTH * DEBUG_HEIGHT;

pub const WIDTH: usize = 32 * 8;
pub const HEIGHT: usize = 32 * 8;
pub const VIDEO_BUFFER: usize = WIDTH * HEIGHT;

pub const TILEMAP_SIZE: u16 = 32 * 32;

#[derive(PartialEq, Debug)]
pub enum State {
    Mode2,
    Mode3,
    Mode0,
    Mode1,
}

#[derive(PartialEq, Debug)]
pub enum PpuInterrupt {
    Vblank,
    Stat,
    None,
}
