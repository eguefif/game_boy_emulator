#![allow(clippy::new_without_default)]

use std::{env, fs::File, io::Read};

const TOTAL_ROM_SIZE: u16 = 0x7FFF + 1;

pub struct Cartridge {
    rom: [u8; TOTAL_ROM_SIZE as usize],
}

impl Cartridge {
    pub fn new() -> Cartridge {
        Cartridge { rom: get_rom() }
    }

    pub fn read(&mut self, at: u16) -> u8 {
        self.rom[at as usize]
    }
    pub fn write(&mut self, at: u16, value: u8) {
        self.rom[at as usize] = value;
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
