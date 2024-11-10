#![allow(dead_code)]
#![allow(clippy::new_without_default)]

use crate::registers::{combine, split_u8};
use std::{env, fs::File, io::Read};

const ROM_B1_END: u16 = 0x3FFF;
const ROM_B1_SIZE: u16 = ROM_B1_END + 1;

const ROM_B2_START: u16 = 0x4000;
const ROM_B2_END: u16 = 0x7FFF;
const ROM_B2_SIZE: u16 = ROM_B2_END - ROM_B2_START + 1;

const TOTAL_ROM_SIZE: u16 = ROM_B2_SIZE + ROM_B1_SIZE + 1;

const MEM_MAX: u16 = 0xFFFF;

pub struct MemoryBus {
    rom: [u8; TOTAL_ROM_SIZE as usize],
    pub pc: u16,
    pub cycle: u128,
}

impl MemoryBus {
    pub fn new() -> MemoryBus {
        MemoryBus {
            rom: get_rom(),
            pc: 0x100,
            cycle: 0,
        }
    }

    pub fn fetch_next_byte(&mut self) -> u8 {
        let retval = self.read(self.pc);
        self.inc_pc();
        self.tick();
        retval
    }

    pub fn read(&mut self, at: u16) -> u8 {
        let loc = at & MEM_MAX;

        match loc {
            0..=ROM_B2_END => self.rom[loc as usize],
            _ => {
                println!("Read: memory not handled: {}", loc);
                0
            }
        }
    }

    fn inc_pc(&mut self) {
        let pc = self.pc;
        self.pc = pc.wrapping_add(1);
    }

    pub fn fetch_byte(&mut self, at: u16) -> u8 {
        let retval = self.read(at);
        self.tick();
        retval
    }

    pub fn write_byte(&mut self, at: u16, value: u8) {
        self.tick();
        self.write(at, value);
    }

    pub fn tick(&mut self) {
        self.cycle += 1;
    }

    fn write(&mut self, at: u16, value: u8) {
        let loc = at & MEM_MAX;

        match loc {
            0..=ROM_B2_END => self.rom[loc as usize] = value,
            _ => println!("Write: memory not handled: {}", loc),
        }
    }

    pub fn fetch_next_word(&mut self) -> u16 {
        let low = self.fetch_next_byte();
        let high = self.fetch_next_byte();
        combine(high as u16, low as u16)
    }

    pub fn write_word(&mut self, at: u16, value: u16) {
        let (high, low) = split_u8(value);
        self.write_byte(at, high);
        self.write_byte(at, low);
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
