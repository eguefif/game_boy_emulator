use crate::ppu::{DEBUG_HEIGHT, DEBUG_WIDTH, HEIGHT, WIDTH};
use debug_tools::DEBUG_SPRITES;
use gameboy::run_gameboy;
use minifb::{Scale, ScaleMode, Window, WindowOptions};

const RESOLUTION_DEBUG: f64 = DEBUG_WIDTH as f64 / DEBUG_HEIGHT as f64;

pub mod apu;
pub mod cartridge;
pub mod cpu;
pub mod debug_tools;
pub mod gameboy;
pub mod joypad;
pub mod memorybus;
pub mod ppu;

fn main() {
    let mut window = get_window();
    let mut debug_window = get_debug_window();
    window.update();
    if let Some(ref mut w) = debug_window {
        w.update();
    }
    run_gameboy(&mut window, &mut debug_window);
}
fn get_window() -> Window {
    let options = WindowOptions {
        borderless: false,
        transparency: false,
        title: true,
        resize: false,
        scale: Scale::FitScreen,
        scale_mode: ScaleMode::Stretch,
        topmost: true,
        none: false,
    };
    let mut window =
        Window::new("Gameboy", 160, 144, options).expect("Error while creating window");
    window.topmost(true);
    window.set_position(755, 20);
    window
}

fn get_debug_window() -> Option<Window> {
    if !DEBUG_SPRITES {
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
