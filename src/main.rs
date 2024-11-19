use crate::ppu::{DEBUG_HEIGHT, DEBUG_WIDTH, HEIGHT, WIDTH};
use debug_tools::DEBUG_SPRITES;
use gameboy::run_gameboy;
use minifb::{Scale, ScaleMode, Window, WindowOptions};

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
        resize: true,
        scale: Scale::X4,
        scale_mode: ScaleMode::Stretch,
        topmost: true,
        none: false,
    };
    let mut window =
        Window::new("Gameboy", WIDTH, HEIGHT, options).expect("Error while creating window");
    window.topmost(true);
    window.set_position(1025, 20);
    window
}

fn get_debug_window() -> Option<Window> {
    if !DEBUG_SPRITES {
        return None;
    }
    let options = WindowOptions {
        borderless: false,
        transparency: false,
        title: true,
        resize: false,
        scale: Scale::X4,
        scale_mode: ScaleMode::Stretch,
        topmost: true,
        none: false,
    };
    let mut window = Window::new("Debug gameboy", DEBUG_WIDTH, DEBUG_HEIGHT, options)
        .expect("Error while creating window");
    window.topmost(true);
    window.set_position(0, 20);
    window.set_background_color(255, 255, 255);
    Some(window)
}
