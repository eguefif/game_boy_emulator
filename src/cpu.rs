#![allow(dead_code)]
#![allow(clippy::new_without_default)]

use crate::memorybus::MemoryBus;

pub struct Cpu {
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,
    pc: u16,
    cycle: u128,
}
impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            af: 0,
            bc: 0,
            de: 0,
            hl: 0,
            sp: 0,
            pc: 0,
            cycle: 0,
        }
    }
    pub fn step(&mut self, memory: &mut MemoryBus) {}
}