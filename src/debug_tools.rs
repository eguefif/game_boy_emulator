use std::io::stdin;

use crate::{cpu::registers::combine, cpu::Cpu};

pub const DEBUG_GRAPHIC: bool = false;
const TEST_ROM: bool = true;
const DEBUG_MODE: bool = false;
const DEBUG_STOP: bool = false;
static mut TEST_ROM_MESSAGE: String = String::new();
static mut STOP_CYCLE: u128 = 0;

pub fn handle_debug(opcode: u8, cpu: &mut Cpu) {
    if TEST_ROM {
        handle_test_rom(cpu);
    }
    if !DEBUG_MODE {
        return;
    }
    display_opcode(opcode, cpu);
    if DEBUG_STOP && should_stop(cpu.memory.cycle) {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        if !input.trim().is_empty() {
            unsafe {
                STOP_CYCLE = input.trim().parse().unwrap();
            }
        }
    }
}

fn should_stop(cycle: u128) -> bool {
    unsafe { STOP_CYCLE < cycle }
}

fn handle_test_rom(cpu: &mut Cpu) {
    update_testrom_message(cpu);
    print_testrom_message();
}
fn update_testrom_message(cpu: &mut Cpu) {
    if cpu.memory.read(0xFF02) == 0x81 {
        let c = cpu.memory.read(0xFF01);
        unsafe {
            TEST_ROM_MESSAGE.push(c as char);
        }
        cpu.memory.write(0xFF02, 0);
    }
}

fn print_testrom_message() {
    unsafe {
        if !TEST_ROM_MESSAGE.is_empty() && should_print() {
            println!("Rom result: {}", TEST_ROM_MESSAGE);
            std::process::exit(0);
        }
    }
}

fn should_print() -> bool {
    unsafe {
        if TEST_ROM_MESSAGE.find("Failed").is_some() {
            return true;
        }
        if TEST_ROM_MESSAGE.find("Passed").is_some() {
            return true;
        }
        false
    }
}
fn display_opcode(opcode: u8, cpu: &mut Cpu) {
    let mut opcode_display = opcode;
    if opcode == 0xcb {
        opcode_display = cpu.memory.read(cpu.memory.pc);
        print!("${:<04x}: cb {:02x} |", cpu.memory.pc - 1, opcode_display);
    } else {
        print!("${:<04x}: {:02x}    |", cpu.memory.pc - 1, opcode_display);
    }
    print!(" {:20} |", diassemble(opcode, cpu));
    print!("{}", cpu.reg);
    print!(" cycles: {}", cpu.memory.cycle - 1);
    print!(" | iflag: {:0<5b}", cpu.memory.interrupt.iflag);
    //display_stack(cpu);
    println!();
}

#[allow(dead_code)]
fn display_stack(cpu: &mut Cpu) {
    let mut sp = cpu.reg.sp;
    let lo = cpu.memory.read(sp) as u16;
    sp = sp.wrapping_add(1);
    let hi = cpu.memory.read(sp) as u16;
    let sp_mem = (hi << 8) | lo;
    print!("| memory[sp]: {:x}", sp_mem);
}

fn diassemble(opcode: u8, cpu: &mut Cpu) -> String {
    let pc = cpu.memory.pc;
    let imm8 = cpu.memory.read(pc);
    let low = cpu.memory.read(pc);
    let high = cpu.memory.read(pc + 1);
    let imm16 = combine(high as u16, low as u16);
    match opcode {
        0x0 => String::from("nop"),
        0xCB => diassemble_cb(cpu),
        0xF3 => String::from("di"),
        0xFB => String::from("ei"),

        //******* Bit operations
        0x07 => String::from("rlca"),
        0x17 => String::from("rla"),
        0x0F => String::from("rrca"),
        0x1F => String::from("rra"),

        //******* Arithmetic Logic Unit (ALU)
        0xE8 => format!("add sp, #${:02x} ({})", imm8, imm8 as i8),
        0x27 => String::from("daa"),

        0x09 => String::from("add hl, bc"),
        0x19 => String::from("add hl, de"),
        0x29 => String::from("add hl, hl"),
        0x39 => String::from("add hl, sp"),

        0x03 => String::from("inc bc"),
        0x13 => String::from("inc de"),
        0x23 => String::from("inc hl"),
        0x33 => String::from("inc sp"),
        0x0B => String::from("dec bc"),
        0x1B => String::from("dec de"),
        0x2B => String::from("dec hl"),
        0x3B => String::from("dec sp"),

        0x04 => String::from("inc b"),
        0x14 => String::from("inc d"),
        0x24 => String::from("inc h"),
        0x34 => String::from("inc (hl)"),
        0x0C => String::from("inc c"),
        0x1C => String::from("inc e"),
        0x2C => String::from("inc l"),
        0x3C => String::from("inc a"),

        0x05 => String::from("dec b"),
        0x15 => String::from("dec d"),
        0x25 => String::from("dec h"),
        0x35 => String::from("dec (hl)"),
        0x0D => String::from("dec c"),
        0x1D => String::from("dec e"),
        0x2D => String::from("dec l"),
        0x3D => String::from("dec a"),

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
        0xC7 => String::from("rst 0"),
        0xD7 => String::from("rst 2"),
        0xE7 => String::from("rst 4"),
        0xF7 => String::from("rst 6"),
        0xCF => String::from("rst 1"),
        0xDF => String::from("rst 3"),
        0xEF => String::from("rst 5"),
        0xFF => String::from("rst 7"),

        0xC4 => format!("call nz, (${:02x})", imm16),
        0xD4 => format!("call nc, (${:02x})", imm16),
        0xCC => format!("call z, (${:02x})", imm16),
        0xDC => format!("call c, (${:02x})", imm16),
        0xCD => format!("call (${:02x})", imm16),

        0xC0 => String::from("ret nz"),
        0xD0 => String::from("ret nc"),
        0xC8 => String::from("ret z"),
        0xD8 => String::from("ret c"),
        0xC9 => String::from("ret"),
        0xD9 => String::from("reti"),

        0xC1 => String::from("pop bc"),
        0xD1 => String::from("pop de"),
        0xE1 => String::from("pop hl"),
        0xF1 => String::from("pop af"),

        0xC5 => String::from("push bc"),
        0xD5 => String::from("push de"),
        0xE5 => String::from("push hl"),
        0xF5 => String::from("push af"),

        0x76 => String::from("halt"),
        0x37 => String::from("scf"),
        0x2F => String::from("cpl"),
        0x3F => String::from("ccf"),

        0x20 => format!("jr nz, #${:02x} ({})", imm8, imm8 as i8),
        0x30 => format!("jr nc, #${:02x} ({})", imm8, imm8 as i8),
        0x18 => format!("jr #${:02x} ({})", imm8, imm8 as i8),
        0x28 => format!("jr z, #${:02x} ({})", imm8, imm8 as i8),
        0x38 => format!("jr c, #${:02x} ({})", imm8, imm8 as i8),

        0xC2 => format!("jp nz, (${:04x})", imm16),
        0xD2 => format!("jp nc, (${:04x})", imm16),
        0xC3 => format!("jp (${:04x})", imm16),
        0xCA => format!("jp z, (${:04x})", imm16),
        0xDA => format!("jp c, (${:04x})", imm16),
        0xE9 => String::from("jp hl"),

        //***** Load section
        //Ld sp
        0x08 => format!("ld (${:04x}), sp", imm16),
        0xF8 => format!("ld hl, sp+#${:02x} ({})", imm8 as i8, imm8 as i8),
        0xF9 => String::from("ld sp, hl"),

        0x01 => format!("ld bc, ${:04x}", imm16),
        0x11 => format!("ld de, ${:04x}", imm16),
        0x21 => format!("ld hl, ${:04x}", imm16),
        0x31 => format!("ld sp, ${:04x}", imm16),

        0x02 => String::from("ld (bc), a"),
        0x12 => String::from("ld (de), a"),
        0x22 => String::from("ld (hl+), a"),
        0x32 => String::from("ld (hl-), a"),
        0x0A => String::from("ld a, (bc)"),
        0x1A => String::from("ld a, (de)"),
        0x2A => String::from("ld a, (hl+)"),
        0x3A => String::from("ld a, (hl-)"),

        0x06 => format!("ld b, #${:02x}", imm8),
        0x16 => format!("ld d, #${:02x}", imm8),
        0x26 => format!("ld h, #${:02x}", imm8),
        0x36 => format!("ld (hl), #${:02x}", imm8),
        0x0E => format!("ld c, #${:02x}", imm8),
        0x1E => format!("ld e, #${:02x}", imm8),
        0x2E => format!("ld l, #${:02x}", imm8),
        0x3E => format!("ld a, #${:02x}", imm8),

        0xE0 => format!("ld (${:02x}), a", combine(0xFF, imm8 as u16)),
        0xF0 => format!("ld a, (${:02x})", combine(0xFF, imm8 as u16)),
        0xE2 => format!("ld (${:02x}), a", combine(0xFF, cpu.reg.c as u16)),
        0xF2 => format!("ld a, (${:02x})", combine(0xFF, cpu.reg.c as u16)),

        0xEA => format!("ld (${:04x}), a", imm16),
        0xFA => format!("ld a, (${:04x})", imm16),

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

fn diassemble_cb(cpu: &mut Cpu) -> String {
    let opcode = cpu.memory.read(cpu.memory.pc);

    match opcode {
        0x0..=0x7 => format!("rlc {}", get_target_debug(opcode)),
        0x8..=0xF => format!("rrc {}", get_target_debug(opcode)),
        0x10..=0x17 => format!("rl {}", get_target_debug(opcode)),
        0x18..=0x1F => format!("rr {}", get_target_debug(opcode)),
        0x20..=0x27 => format!("sla {}", get_target_debug(opcode)),
        0x28..=0x2F => format!("sra {}", get_target_debug(opcode)),
        0x30..=0x37 => format!("swap {}", get_target_debug(opcode)),
        0x38..=0x3F => format!("srl {}", get_target_debug(opcode)),
        0x40..=0x7F => format!(
            "bit {}, {}",
            get_value_debug(opcode),
            get_target_debug(opcode)
        ),
        0x80..=0xBF => format!(
            "res {}, {}",
            get_value_debug(opcode),
            get_target_debug(opcode)
        ),
        0xC0..=0xFF => format!(
            "set {}, {}",
            get_value_debug(opcode),
            get_target_debug(opcode)
        ),
    }
}

fn get_target_debug(opcode: u8) -> String {
    match opcode & 0b_0000_0111 {
        0b0000 => String::from("b"),
        0b0001 => String::from("c"),
        0b0010 => String::from("d"),
        0b0011 => String::from("e"),
        0b0100 => String::from("h"),
        0b0101 => String::from("l"),
        0b0110 => String::from("hl"),
        0b0111 => String::from("a"),
        _ => String::from("N"),
    }
}

fn get_value_debug(opcode: u8) -> String {
    match (opcode >> 3) & 0b_0000_0111 {
        0b000 => String::from("0"),
        0b001 => String::from("1"),
        0b010 => String::from("2"),
        0b011 => String::from("3"),
        0b100 => String::from("4"),
        0b101 => String::from("5"),
        0b110 => String::from("6"),
        0b111 => String::from("7"),
        _ => String::from("N"),
    }
}
