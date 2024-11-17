use std::time::Instant;

use debug_tools::DEBUG_GRAPHIC;
use joypad::Joypad;
use minifb::{Key, Window, WindowOptions};
use ppu::{DEBUG_HEIGHT, DEBUG_WIDTH};

use crate::cpu::Cpu;

pub mod apu;
pub mod cartridge;
pub mod cpu;
pub mod debug_tools;
pub mod joypad;
pub mod memorybus;
pub mod ppu;

const RESOLUTION: f64 = 160.0 / 144.0;
const RESOLUTION_DEBUG: f64 = DEBUG_WIDTH as f64 / DEBUG_HEIGHT as f64;

fn main() {
    let mut cpu = Cpu::new();
    let mut window = get_window();
    let mut debug_window = get_debug_window();
    let mut joypad = Joypad::new();
    window.update();
    if let Some(ref mut w) = debug_window {
        w.update();
    }
    loop {
        joypad.update(&window);
        cpu.joypad = joypad.clone();
        let start = Instant::now();
        cpu.step();
        if cpu.memory.cycle >= 17476 {
            cpu.memory.cycle = 0;
            if let Some(ref mut w) = debug_window {
                let mem = cpu.memory.ppu.get_tiles_memory();
                w.update_with_buffer(mem, DEBUG_WIDTH, DEBUG_HEIGHT)
                    .unwrap();
            }
            while start.elapsed().as_millis() < 17 {}
        }
        handle_exit(&mut window);
    }
}

fn handle_exit(window: &mut Window) {
    if window.is_key_down(Key::Escape) || !window.is_open() {
        std::process::exit(0);
    }
}

fn get_window() -> Window {
    let mut window = Window::new(
        "Gameboy",
        750,
        (750.0 * RESOLUTION) as usize,
        WindowOptions::default(),
    )
    .expect("Error while creating window");
    window.topmost(true);
    window.set_position(755, 20);
    window.set_background_color(255, 0, 0);
    window
}

fn get_debug_window() -> Option<Window> {
    if !DEBUG_GRAPHIC {
        return None;
    }
    let mut window = Window::new(
        "Debug gameboy",
        750,
        750 * RESOLUTION_DEBUG as usize,
        WindowOptions::default(),
    )
    .expect("Error while creating window");
    window.topmost(true);
    window.set_position(0, 20);
    window.set_background_color(255, 255, 255);
    Some(window)
}
