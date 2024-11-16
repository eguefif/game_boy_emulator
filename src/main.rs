use debug_tools::DEBUG_GRAPHIC;
use joypad::Joypad;
use minifb::{Window, WindowOptions};

use crate::cpu::Cpu;

pub mod cpu;
pub mod debug_tools;
pub mod joypad;
pub mod memorybus;

const RESOLUTION: f64 = 144.0 / 160.0;

fn main() {
    let mut cpu = Cpu::new();
    let mut window = get_window();
    let mut debug_window = get_debug_window();
    let mut joypad = Joypad::new();
    loop {
        window.update();
        joypad.update(&window);
        if let Some(ref mut w) = debug_window {
            w.update();
        }
        cpu.step(&mut joypad);
    }
}

fn get_window() -> Window {
    let mut window = Window::new(
        "Gameboy",
        500,
        (500.0 * RESOLUTION) as usize,
        WindowOptions::default(),
    )
    .expect("Error while creating window");
    window.topmost(true);
    window.set_background_color(255, 0, 0);
    window
}

fn get_debug_window() -> Option<Window> {
    if !DEBUG_GRAPHIC {
        return None;
    }
    let mut window = Window::new(
        "Debug gameboy",
        500,
        (500.0 * RESOLUTION) as usize,
        WindowOptions::default(),
    )
    .expect("Error while creating window");
    window.topmost(true);
    window.set_position(20, 20);
    window.set_background_color(255, 0, 0);
    Some(window)
}
