use crate::{cpu::registers::combine, cpu::Cpu};

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

        //******* Arithmetic Logic Unit (ALU)
        0x80 => String::from("add a, b"),
        0x81 => String::from("add a, c"),
        0x82 => String::from("add a, d"),
        0x83 => String::from("add a, e"),
        0x84 => String::from("add a, h"),
        0x85 => String::from("add a, l"),
        0x86 => String::from("add a, (hl)"),
        0x87 => String::from("add a, a"),
        0x88 => String::from("adc a, b"),
        0x89 => String::from("adc a, c"),
        0x8a => String::from("adc a, d"),
        0x8b => String::from("adc a, e"),
        0x8c => String::from("adc a, h"),
        0x8d => String::from("adc a, l"),
        0x8e => String::from("adc a, (hl)"),
        0x8f => String::from("adc a, a"),

        0xC6 => format!("add a, #${:02x}", imm8),
        0xD6 => format!("sub a, #${:02x}", imm8),
        0xE6 => format!("and a, #${:02x}", imm8),
        0xF6 => format!("or a, #${:02x}", imm8),
        0xCE => format!("adc a, #${:02x}", imm8),
        0xDE => format!("sbc a, #${:02x}", imm8),
        0xEE => format!("xor a, #${:02x}", imm8),
        0xFE => format!("cp a, #${:02x}", imm8),

        0x90 => String::from("sub a, b"),
        0x91 => String::from("sub a, c"),
        0x92 => String::from("sub a, d"),
        0x93 => String::from("sub a, e"),
        0x94 => String::from("sub a, h"),
        0x95 => String::from("sub a, l"),
        0x96 => String::from("sub a, (hl)"),
        0x97 => String::from("sub a, a"),
        0x98 => String::from("sbc a, b"),
        0x99 => String::from("sbc a, c"),
        0x9a => String::from("sbc a, d"),
        0x9b => String::from("sbc a, e"),
        0x9c => String::from("sbc a, h"),
        0x9d => String::from("sbc a, l"),
        0x9e => String::from("sbc a, (hl)"),
        0x9f => String::from("sbc a, a"),

        0xA0 => String::from("and a, b"),
        0xA1 => String::from("and a, c"),
        0xA2 => String::from("and a, d"),
        0xA3 => String::from("and a, e"),
        0xA4 => String::from("and a, h"),
        0xA5 => String::from("and a, l"),
        0xA6 => String::from("and a, (hl)"),
        0xA7 => String::from("and a, a"),
        0xA8 => String::from("xor a, b"),
        0xA9 => String::from("xor a, c"),
        0xAA => String::from("xor a, d"),
        0xAB => String::from("xor a, e"),
        0xAC => String::from("xor a, h"),
        0xAD => String::from("xor a, l"),
        0xAE => String::from("xor a, (hl)"),
        0xAF => String::from("xor a, a"),

        0xB0 => String::from("or a, b"),
        0xB1 => String::from("or a, c"),
        0xB2 => String::from("or a, d"),
        0xB3 => String::from("or a, e"),
        0xB4 => String::from("or a, h"),
        0xB5 => String::from("or a, l"),
        0xB6 => String::from("or a, (hl)"),
        0xB7 => String::from("or a, a"),
        0xB8 => String::from("cp a, b"),
        0xB9 => String::from("cp a, c"),
        0xBA => String::from("cp a, d"),
        0xBB => String::from("cp a, e"),
        0xBC => String::from("cp a, h"),
        0xBD => String::from("cp a, l"),
        0xBE => String::from("cp a, (hl)"),
        0xBF => String::from("cp a, a"),

        //******* Flow control
        0x76 => String::from("halt"),

        //***** Load section
        //Ld sp
        0x08 => format!("ld (${:04x}), sp", imm16),
        0xF8 => format!("ld hl, sp+#${:02x} ({})", imm8 as i8, imm8 as i8),
        0xF9 => String::from("ld sp, hl"),
        //
        // ld imm16
        0x01 => format!("ld bc, ${:04x}", imm16),
        0x11 => format!("ld de, ${:04x}", imm16),
        0x21 => format!("ld hl, ${:04x}", imm16),
        0x31 => format!("ld sp, ${:04x}", imm16),

        // Ld addr16
        0x02 => String::from("ld (bc), a"),
        0x12 => String::from("ld (de), a"),
        0x22 => String::from("ld (hl+), a"),
        0x32 => String::from("ld (hl-), a"),
        0x0A => String::from("ld a, (bc)"),
        0x1A => String::from("ld a, (de)"),
        0x2A => String::from("ld a, (hl+)"),
        0x3A => String::from("ld a, (hl-)"),

        // Ld imm8
        0x06 => format!("ld b, #${:02x}", imm8),
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

        //Ld a16
        0xEA => format!("ld (${:04x}), a", imm16),
        0xFA => format!("ld a, (${:04x})", imm16),

        // Ld regular
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

        0x50 => String::from("ld d, b"),
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
