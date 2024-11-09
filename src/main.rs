use crate::cpu::Cpu;
use crate::memorybus::MemoryBus;

pub mod cpu;
pub mod memorybus;

fn main() {
    let mut cpu = Cpu::new();
    let mut memory = MemoryBus::new();
    loop {
        cpu.step(&mut memory);
    }
}
