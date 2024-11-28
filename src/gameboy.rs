use crate::ppu::config::{DEBUG_HEIGHT, DEBUG_WIDTH, HEIGHT, WIDTH};
use minifb::{Key, Window};
use std::time::Instant;

use crate::cpu::Cpu;
pub fn run_gameboy(window: &mut Window, debug_window: &mut Option<Window>) {
    let mut cpu = Cpu::new();
    let mut speed = 1000;
    let mut counter = 0;
    let mut acc = 0;
    loop {
        let start = Instant::now();
        cpu.step();
        if cpu.memory.ppu.frame_drawn {
            cpu.memory.ppu.frame_drawn = false;
            handle_debug_ctrl(window, &mut cpu);
            cpu.memory.joypad.update(window);
            render(&mut cpu, window, debug_window);
            acc += start.elapsed().as_millis();
            if counter == 10 {
                let tmp = acc / 10;
                if tmp < speed {
                    speed = tmp;
                }
            }
            while start.elapsed().as_millis() < 17 {}
            counter += 1;
        }
        handle_exit(window, speed);
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

fn handle_exit(window: &mut Window, speed: u128) {
    if window.is_key_down(Key::Escape) || !window.is_open() {
        println!("speed max {}", speed);
        std::process::exit(0);
    }
}

fn handle_debug_ctrl(window: &mut Window, cpu: &mut Cpu) {
    if window.is_key_down(Key::Space) {
        cpu.pause = !cpu.pause;
    }
    if window.is_key_down(Key::Enter) {
        cpu.debug = !cpu.debug;
    }
    if window.is_key_down(Key::P) {
        cpu.debug_ppu = !cpu.debug_ppu;
    }
}
