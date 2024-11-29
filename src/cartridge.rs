#![allow(clippy::new_without_default)]
#![allow(unused_variables)]

use std::{env, fs::File, io::Read};

use crate::cartridge::header::Header;

const TOTAL_ROM_SIZE: u16 = 0x7FFF + 1;

pub mod header;

pub struct Cartridge {
    rom: [u8; TOTAL_ROM_SIZE as usize],
    bootrom: [u8; 0x100],
    header: Header,
    pub bootstrap: bool,
}

impl Cartridge {
    pub fn new() -> Cartridge {
        let rom = get_rom();
        let retval = Cartridge {
            rom,
            header: Header::new(rom),
            bootrom: get_bootrom(),
            bootstrap: true,
        };
        println!("{}", retval.header);
        retval
    }

    pub fn read(&mut self, at: u16) -> u8 {
        if self.bootstrap {
            match at {
                0..0x100 => self.bootrom[at as usize],
                _ => self.rom[at as usize],
            }
        } else {
            self.rom[at as usize]
        }
    }
    pub fn write(&mut self, at: u16, value: u8) {
        match at {
            _ => {}
        }
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

fn get_bootrom() -> [u8; 0x100 as usize] {
    let filename = "./rom/dmg_boot.bin";
    if filename == "error" {
        return [0; 0x100 as usize];
    }
    let mut rom = [0; 0x100 as usize];
    match File::open(filename) {
        Ok(mut file) => {
            let _ = file.read(&mut rom).unwrap();
            rom
        }
        Err(_) => rom,
    }
}
