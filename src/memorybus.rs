#![allow(dead_code)]
#![allow(clippy::new_without_default)]

use crate::apu::Apu;
use crate::cartridge::Cartridge;
use crate::cpu::interrupt::Interrupt;
use crate::cpu::registers::{combine, split_u16};
use crate::cpu::timer::Timer;
use crate::joypad::Joypad;
use crate::ppu::Ppu;

const VRAM_SIZE: u16 = 0x9FFF - 0x8000 + 1;
const HRAM_SIZE: u16 = 0xFFFE - 0xFF80 + 1;
const WRAM_SIZE: u16 = 0xDFFF - 0xC000 + 1;
const MEM_MAX: u16 = 0xFFFF;

pub struct MemoryBus {
    pub ppu: Ppu,
    pub joypad: Joypad,
    pub interrupt: Interrupt,
    pub pc: u16,
    pub cycle: u128,
    apu: Apu,
    timer: Timer,
    cartridge: Cartridge,
    vram: [u8; VRAM_SIZE as usize],
    hram: [u8; HRAM_SIZE as usize],
    wram: [u8; WRAM_SIZE as usize],
    ie: u8,
    debug: [u8; 2],

    dma: bool,
    dma_addr: u16,
    dma_target: u16,
}

impl MemoryBus {
    pub fn new() -> MemoryBus {
        MemoryBus {
            ppu: Ppu::new(),
            apu: Apu::new(),
            cartridge: Cartridge::new(),
            timer: Timer::new(),
            joypad: Joypad::new(),
            interrupt: Interrupt::new(),
            vram: [0; VRAM_SIZE as usize],
            hram: [0; HRAM_SIZE as usize],
            wram: [0; WRAM_SIZE as usize],
            pc: 0x100,
            ie: 0,
            cycle: 0,
            debug: [0; 2],
            dma: false,
            dma_addr: 0,
            dma_target: 0xFE00,
        }
    }

    pub fn read(&mut self, at: u16) -> u8 {
        let loc = at & MEM_MAX;
        match loc {
            0..=0x7FFF => self.cartridge.read(loc),

            0xFF00 => self.joypad.get_joypad(),
            0xFF01 => self.debug[0],
            0xFF02 => self.debug[1],
            0xFF04 => (self.timer.div >> 8) as u8,
            0xFF05 => self.timer.tima,
            0xFF06 => self.timer.tma,
            0xFF07 => self.timer.tac,
            0xFF0F => self.interrupt.iflag,

            0xFF40..=0xFF4B => self.ppu.read(loc as usize),
            0xFF51..=0xFF55 => self.ppu.read(loc as usize),
            0x8000..=0x9FFF => self.ppu.read(loc as usize),
            0xFE00..=0xFE9F => self.ppu.read(loc as usize),

            0xFF80..=0xFFFE => self.hram[(loc - 0xFF80) as usize],
            0xC000..=0xDFFF => self.wram[(loc - 0xC000) as usize],
            0xE000..=0xFDFF => {
                let new_loc = loc & (WRAM_SIZE - 1);
                self.wram[(new_loc) as usize]
            }
            0xFFFF => self.interrupt.ie,
            _ => {
                eprintln!("Read: memory not handled: {:x}", loc);
                0
            }
        }
    }

    pub fn write(&mut self, at: u16, value: u8) {
        let loc = at & MEM_MAX;

        match loc {
            0..=0x7FFF => self.cartridge.write(loc, value),

            0xFF00 => self.joypad.set_joypad(value),
            0xFF01 => self.debug[0] = value,
            0xFF02 => self.debug[1] = value,
            0xFF04 => self.timer.div = 0,
            0xFF05 => self.timer.tima = value,
            0xFF06 => self.timer.tma = value,
            0xFF07 => self.timer.tac = value | 0xF8,
            0xFF0F => self.interrupt.set_iflag(value),
            0xFF46 => self.handle_dma(value),
            0xFF80..=0xFFFE => self.hram[(loc - 0xFF80) as usize] = value,

            0xFF10..=0xFF3F => self.apu.write(loc, value),

            0xFF40..=0xFF4B => self.ppu.write(loc as usize, value),
            0xFF51..=0xFF55 => self.ppu.write(loc as usize, value),
            0x8000..=0x9FFF => self.ppu.write(loc as usize, value),
            0xFE00..=0xFE9F => self.ppu.write(loc as usize, value),

            0xC000..=0xDFFF => self.wram[(loc - 0xC000) as usize] = value,
            0xE000..=0xFDFF => {
                let new_loc = loc & (WRAM_SIZE - 1);
                self.wram[(new_loc) as usize] = value;
            }
            0xFFFF => self.interrupt.set_ie(value),
            _ => eprintln!("Write: memory not handled: {:x}", loc),
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

    fn handle_dma(&mut self, value: u8) {
        let addr = (value as u16) << 8;
        self.dma = true;
        self.dma_addr = addr;
        self.dma_target = 0xFE00;
    }

    pub fn tick(&mut self) {
        self.cycle += 1;
        if self.timer.handle_timer() {
            self.interrupt.require_timer();
        }
        self.ppu.step();
        if self.ppu.vblank {
            self.interrupt.require_vblank();
            self.ppu.vblank = false;
        }
        if self.ppu.stat_int {
            self.interrupt.require_stat();
            self.ppu.stat_int = false;
        }

        if self.dma {
            let value = self.read(self.dma_addr);
            self.ppu.write_oam(self.dma_target as usize, value);
            self.dma_addr = self.dma_addr.wrapping_add(1);
            self.dma_target = self.dma_target.wrapping_add(1);
            if self.dma_target == 0xFE9F {
                self.dma = false;
            }
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
