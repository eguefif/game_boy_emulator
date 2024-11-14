use crate::cpu::Cpu;

pub mod cpu;
pub mod debug_tools;
pub mod memorybus;

fn main() {
    let mut cpu = Cpu::new();
    loop {
        if !cpu.step() {
            break;
        }
    }
}
