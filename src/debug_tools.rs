use crate::{cpu::Cpu, registers::combine};

pub fn handle_debug(opcode: u8, cpu: &mut Cpu) {
    print!("${:<04x}: {:02x}    |", cpu.memory.pc - 1, opcode);
    print!(" {:20} |", diassemble(opcode, cpu));
    print!("{} |", cpu.reg);
    print!(" cycles: {}", cpu.memory.cycle);
    println!();
}

fn diassemble(opcode: u8, cpu: &mut Cpu) -> String {
    let pc = cpu.memory.pc;
    let imm8 = cpu.memory.read(pc);
    let low = cpu.memory.read(pc);
    let high = cpu.memory.read(pc + 1);
    let imm16 = combine(high as u16, low as u16);
    match opcode {
        0x0 => String::from("nop"),

        0x02 => String::from("ld (bc), a"),
        0x12 => String::from("ld (de), a"),
        0x22 => String::from("ld (hl+), a"),
        0x32 => String::from("ld ,(hl-), a"),
        0x0A => String::from("ld a, (bc)"),
        0x1A => String::from("ld a, (de)"),
        0x2A => String::from("ld a, (hl+)"),
        0x3A => String::from("ld a, (hl-)"),

        0x06 => format!("ld b, #${:2x}", imm8),
        0x16 => format!("ld d, #${:02x}", imm8),
        0x26 => format!("ld h, #${:02x}", imm8),
        0x36 => format!("ld (hl), #${:02x}", imm8),
        0x0E => format!("ld c, #${:02x}", imm8),
        0x1E => format!("ld e, #${:02x}", imm8),
        0x2E => format!("ld l, #${:02x}", imm8),
        0x3E => format!("ld a, #${:02x}", imm8),

        //Ld ZeroPage
        0xE0 => format!("ld (${:02x}), a", combine(0xFF, imm8 as u16)),
        0xF0 => format!("ld a, (${:02x})", combine(0xFF, imm8 as u16)),
        0xE2 => format!("ld (${:02x}), a", combine(0xFF, cpu.reg.c as u16)),
        0xF2 => format!("ld a, (${:02x})", combine(0xFF, cpu.reg.c as u16)),

        0xEA => format!("ld (${:04x}), a", imm16),
        0xFA => format!("ld a, (${:04x})", imm16),

        // ld imm16
        0x01 => format!("ld bc, ${:04x}", imm16),
        0x11 => format!("ld de, ${:04x}", imm16),
        0x21 => format!("ld hl, ${:04x}", imm16),
        0x31 => format!("ld sp, ${:04x}", imm16),

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
        0x76 => String::from("halt"),
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
