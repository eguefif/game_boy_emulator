use crate::cpu::Cpu;

pub mod cpu;
pub mod debug_tools;
pub mod execute;
pub mod memorybus;
pub mod read_write_cpu;
pub mod registers;

fn main() {
    let mut cpu = Cpu::new();
    loop {
        cpu.step();
    }
}
