use crate::cpu::Cpu;

pub mod cpu;
pub mod memorybus;
pub mod registers;

fn main() {
    let mut cpu = Cpu::new();
    loop {
        cpu.step();
    }
}
