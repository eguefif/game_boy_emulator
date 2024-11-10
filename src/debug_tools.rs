use crate::cpu::Cpu;

pub fn handle_debug(opcode: u8, cpu: &Cpu) {
    print!("${:0<4}: {:2x}    |", cpu.memory.pc, opcode);
    print!(" {:20} |", diasemble(opcode, cpu));
    print!("{} |", cpu.reg);
}

fn diasemble(opcode: u8, cpu: &Cpu) -> String {
    String::new()
}
