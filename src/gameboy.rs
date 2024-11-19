use crate::joypad::Joypad;
use crate::ppu::{DEBUG_HEIGHT, DEBUG_WIDTH, HEIGHT, WIDTH};
use minifb::{Key, Window};
use std::time::Instant;

use crate::cpu::Cpu;
pub fn run_gameboy(window: &mut Window, debug_window: &mut Option<Window>) {
    let mut joypad = Joypad::new();
    let mut cpu = Cpu::new();
    loop {
        joypad.update(window);
        cpu.joypad = joypad.clone();
        let start = Instant::now();
        cpu.step();
        if cpu.memory.cycle % 17556 == 0 {
            render(&mut cpu, window, debug_window);
            while start.elapsed().as_millis() < 17 {}
        }
        handle_exit(window);
    }
}

fn render(cpu: &mut Cpu, window: &mut Window, debug_window: &mut Option<Window>) {
    let video = cpu.memory.ppu.get_video_buffer();
    window.update_with_buffer(video, WIDTH, HEIGHT).unwrap();
    if let Some(ref mut w) = debug_window {
        let mem = cpu.memory.ppu.get_tiles_memory();
        w.update_with_buffer(mem, DEBUG_WIDTH, DEBUG_HEIGHT)
            .unwrap();
    }
}

fn handle_exit(window: &mut Window) {
    if window.is_key_down(Key::Escape) || !window.is_open() {
        std::process::exit(0);
    }
}
