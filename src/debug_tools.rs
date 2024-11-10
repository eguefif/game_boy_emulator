use crate::cpu::Cpu;

pub fn handle_debug(opcode: u8, cpu: &Cpu) {
    print!("${:<04x}: {:2x}    |", cpu.memory.pc - 1, opcode);
    print!(" {:20} |", diassemble(opcode, cpu));
    print!("{} |", cpu.reg);
    print!(" cycles: {}", cpu.memory.cycle);
    println!();
}

fn diassemble(opcode: u8, _cpu: &Cpu) -> String {
    match opcode {
        0x0 => String::from("nop"),
        0x40 => String::from("ld b, b"),
        0x41 => String::from("ld b, c"),
        0x42 => String::from("ld b, d"),
        0x43 => String::from("ld b, e"),
        0x44 => String::from("ld b, h"),
        0x45 => String::from("ld b, l"),
        0x46 => String::from("ld b, (hl)"),
        0x47 => String::from("ld b, a"),
        0x48 => String::from("ld c, b"),
        0x49 => String::from("ld c, c"),
        0x4a => String::from("ld c, d"),
        0x4b => String::from("ld c, e"),
        0x4c => String::from("ld c, h"),
        0x4d => String::from("ld c, l"),
        0x4e => String::from("ld c, (hl)"),
        0x4f => String::from("ld c, a"),

        0x50 => String::from("ld b, b"),
        0x51 => String::from("ld d, c"),
        0x52 => String::from("ld d, d"),
        0x53 => String::from("ld d, e"),
        0x54 => String::from("ld d, h"),
        0x55 => String::from("ld d, l"),
        0x56 => String::from("ld d, (hl)"),
        0x57 => String::from("ld d, a"),
        0x58 => String::from("ld e, b"),
        0x59 => String::from("ld e, c"),
        0x5a => String::from("ld e, d"),
        0x5b => String::from("ld e, e"),
        0x5c => String::from("ld e, h"),
        0x5d => String::from("ld e, l"),
        0x5e => String::from("ld e, (hl)"),
        0x5f => String::from("ld e, a"),

        0x60 => String::from("ld h, b"),
        0x61 => String::from("ld h, c"),
        0x62 => String::from("ld h, d"),
        0x63 => String::from("ld h, e"),
        0x64 => String::from("ld h, h"),
        0x65 => String::from("ld h, l"),
        0x66 => String::from("ld h, (hl)"),
        0x67 => String::from("ld h, a"),
        0x68 => String::from("ld l, b"),
        0x69 => String::from("ld l, c"),
        0x6a => String::from("ld l, d"),
        0x6b => String::from("ld l, e"),
        0x6c => String::from("ld l, h"),
        0x6d => String::from("ld l, l"),
        0x6e => String::from("ld l, (hl)"),
        0x6f => String::from("ld l, a"),

        0x70 => String::from("ld (hl), b"),
        0x71 => String::from("ld (hl), c"),
        0x72 => String::from("ld (hl), d"),
        0x73 => String::from("ld (hl), e"),
        0x74 => String::from("ld (hl), h"),
        0x75 => String::from("ld (hl), l"),
        0x76 => String::from("ld (hl), (hl)"),
        0x77 => String::from("ld (hl), a"),
        0x78 => String::from("ld a, b"),
        0x79 => String::from("ld a, c"),
        0x7a => String::from("ld a, d"),
        0x7b => String::from("ld a, e"),
        0x7c => String::from("ld a, h"),
        0x7d => String::from("ld a, l"),
        0x7e => String::from("ld a, (hl)"),
        0x7f => String::from("ld a, a"),

        _ => String::from("unknown opcode"),
    }
}
