#![allow(dead_code)]
#![allow(clippy::new_without_default)]

use crate::cpu::interrupt::Interrupt;
use crate::cpu::registers::{combine, split_u16};
use crate::cpu::timer::Timer;
use crate::joypad::Joypad;
use std::{env, fs::File, io::Read};

const ROM_B1_END: u16 = 0x3FFF;
const ROM_B1_SIZE: u16 = ROM_B1_END + 1;

const ROM_B2_START: u16 = 0x4000;
const ROM_B2_END: u16 = 0x7FFF;
const ROM_B2_SIZE: u16 = ROM_B2_END - ROM_B2_START + 1;

const VRAM_START: u16 = 0x8000;
const VRAM_END: u16 = 0x9FFF;
const VRAM_SIZE: u16 = VRAM_END - VRAM_START + 1;

const EXTRAM_START: u16 = 0xA000;
const EXTRAM_END: u16 = 0xBFFF;
const EXTRAM_SIZE: u16 = EXTRAM_END - EXTRAM_START + 1;

const HRAM_START: u16 = 0xFF80;
const HRAM_END: u16 = 0xFFFE;
const HRAM_SIZE: u16 = HRAM_END - HRAM_START + 1;

const WRAM_START: u16 = 0xC000;
const WRAM_END: u16 = 0xCFFF;
const WRAM_SIZE: u16 = WRAM_END - WRAM_START + 1;

const W2RAM_START: u16 = 0xD000;
const W2RAM_END: u16 = 0xDFFF;
const W2RAM_SIZE: u16 = W2RAM_END - W2RAM_START + 1;

const ECHO_START: u16 = 0xE000;
const ECHO_END: u16 = 0xFDFF;
const ECHO_SIZE: u16 = ECHO_END - ECHO_START + 1;

const RESCALE_MIRROR: u16 = WRAM_SIZE + W2RAM_SIZE;

const TOTAL_ROM_SIZE: u16 = ROM_B2_SIZE + ROM_B1_SIZE + 1;
const MEM_MAX: u16 = 0xFFFF;

pub struct MemoryBus {
    timer: Timer,
    pub joypad: Joypad,
    pub interrupt: Interrupt,
    rom: [u8; TOTAL_ROM_SIZE as usize],
    vram: [u8; VRAM_SIZE as usize],
    hram: [u8; HRAM_SIZE as usize],
    wram: [u8; WRAM_SIZE as usize],
    w2ram: [u8; W2RAM_SIZE as usize],
    echo: [u8; ECHO_SIZE as usize],
    extram: [u8; EXTRAM_SIZE as usize],
    ie: u8,
    pub pc: u16,
    pub cycle: u128,
    debug: [u8; 2],
}

impl MemoryBus {
    pub fn new() -> MemoryBus {
        MemoryBus {
            timer: Timer::new(),
            joypad: Joypad::new(),
            interrupt: Interrupt::new(),
            rom: get_rom(),
            vram: [0; VRAM_SIZE as usize],
            hram: [0; HRAM_SIZE as usize],
            wram: [0; WRAM_SIZE as usize],
            w2ram: [0; W2RAM_SIZE as usize],
            echo: [0; ECHO_SIZE as usize],
            extram: [0; EXTRAM_SIZE as usize],
            pc: 0x100,
            ie: 0,
            cycle: 0,
            debug: [0; 2],
        }
    }

    pub fn read(&mut self, at: u16) -> u8 {
        let loc = at & MEM_MAX;
        match loc {
            0xFF00 => self.joypad.get_joypad(),
            0xFF01 => self.debug[0],
            0xFF02 => self.debug[1],
            0xFF04 => (self.timer.div >> 8) as u8,
            0xFF05 => self.timer.tima,
            0xFF06 => self.timer.tma,
            0xFF07 => self.timer.tac,
            0xFF0F => self.interrupt.iflag,
            0..=ROM_B2_END => self.rom[loc as usize],
            EXTRAM_START..=EXTRAM_END => self.extram[(loc - EXTRAM_START) as usize],
            VRAM_START..=VRAM_END => self.vram[(loc - VRAM_START) as usize],
            HRAM_START..=HRAM_END => self.hram[(loc - HRAM_START) as usize],
            WRAM_START..=WRAM_END => self.wram[(loc - WRAM_START) as usize],
            W2RAM_START..=W2RAM_END => self.w2ram[(loc - W2RAM_START) as usize],
            ECHO_START..=ECHO_END => match loc - RESCALE_MIRROR {
                WRAM_START..=WRAM_END => self.wram[(loc - RESCALE_MIRROR - WRAM_START) as usize],
                W2RAM_START..=W2RAM_END => {
                    self.w2ram[(loc - RESCALE_MIRROR - W2RAM_START) as usize]
                }
                _ => 0,
            },
            0xFFFF => self.interrupt.ie,
            _ => {
                println!("Read: memory not handled: {:x}", loc);
                0
            }
        }
    }

    pub fn write(&mut self, at: u16, value: u8) {
        let loc = at & MEM_MAX;

        match loc {
            0xFF00 => self.joypad.set_joypad(value),
            0xFF01 => self.debug[0] = value,
            0xFF02 => self.debug[1] = value,
            0xFF04 => self.timer.div = 0,
            0xFF05 => self.timer.tima = value,
            0xFF06 => self.timer.tma = value,
            0xFF07 => self.timer.tac = value | 0xF8,
            0xFF0F => self.interrupt.set_iflag(value),
            0..=ROM_B2_END => self.rom[loc as usize] = value,
            EXTRAM_START..=EXTRAM_END => self.extram[(loc - EXTRAM_START) as usize] = value,
            VRAM_START..=VRAM_END => self.vram[(loc - VRAM_START) as usize] = value,
            HRAM_START..=HRAM_END => self.hram[(loc - HRAM_START) as usize] = value,
            WRAM_START..=WRAM_END => self.wram[(loc - WRAM_START) as usize] = value,
            W2RAM_START..=W2RAM_END => self.w2ram[(loc - W2RAM_START) as usize] = value,
            ECHO_START..=ECHO_END => match loc - RESCALE_MIRROR {
                WRAM_START..=WRAM_END => {
                    self.wram[(loc - RESCALE_MIRROR - WRAM_START) as usize] = value
                }
                W2RAM_START..=W2RAM_END => {
                    self.w2ram[(loc - RESCALE_MIRROR - WRAM_START) as usize] = value
                }
                _ => {}
            },
            0xFFFF => self.interrupt.set_ie(value),
            _ => println!("Write: memory not handled: {:x}", loc),
        }
    }

    pub fn fetch_next_byte(&mut self) -> u8 {
        self.tick();
        let retval = self.read(self.pc);
        self.inc_pc();
        retval
    }

    fn inc_pc(&mut self) {
        let pc = self.pc;
        self.pc = pc.wrapping_add(1);
    }

    pub fn fetch_byte(&mut self, at: u16) -> u8 {
        self.tick();
        self.read(at)
    }

    pub fn write_byte(&mut self, at: u16, value: u8) {
        self.tick();
        self.write(at, value);
    }

    pub fn tick(&mut self) {
        self.cycle += 1;
        if self.timer.handle_timer() {
            self.interrupt.require_timer();
        }
    }

    pub fn fetch_next_word(&mut self) -> u16 {
        let low = self.fetch_next_byte();
        let high = self.fetch_next_byte();
        combine(high as u16, low as u16)
    }

    pub fn write_word(&mut self, at: u16, value: u16) {
        let (high, low) = split_u16(value);
        self.write_byte(at, low);
        self.write_byte(at.wrapping_add(1), high);
    }
}

fn get_rom() -> [u8; TOTAL_ROM_SIZE as usize] {
    let filename = get_filename();
    if filename == "error" {
        return [0; TOTAL_ROM_SIZE as usize];
    }
    let mut rom = [0; TOTAL_ROM_SIZE as usize];
    match File::open(filename) {
        Ok(mut file) => {
            let _ = file.read(&mut rom).unwrap();
            rom
        }
        Err(_) => rom,
    }
}

fn get_filename() -> String {
    let arg: Vec<String> = env::args().collect();
    if arg.len() != 2 {
        return String::from("error");
    }
    String::from(&arg[1])
}
