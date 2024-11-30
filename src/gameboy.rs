use crate::{
    debug_tools::DEBUG_SPRITES,
    ppu::config::{DEBUG_HEIGHT, DEBUG_WIDTH, HEIGHT, WIDTH},
};
use minifb::{Key, Window};
use std::time::Instant;

use crate::cpu::Cpu;
pub fn run_gameboy(window: &mut Window, debug_window: &mut Option<Window>) {
    let mut cpu = Cpu::new();
    loop {
        let start = Instant::now();
        cpu.step();
        if cpu.memory.ppu.frame_drawn {
            cpu.memory.ppu.frame_drawn = false;
            cpu.memory.joypad.update(window);
            render(&mut cpu, window, debug_window);
            while start.elapsed().as_millis() < 1 {}
        }
        handle_exit(window);
    }
}

fn render(cpu: &mut Cpu, window: &mut Window, debug_window: &mut Option<Window>) {
    let video = cpu.memory.ppu.get_video_buffer();
    window.update_with_buffer(video, WIDTH, HEIGHT).unwrap();
    if !DEBUG_SPRITES {
        return;
    }
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
