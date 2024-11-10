#![allow(dead_code)]
#![allow(clippy::new_without_default)]

use std::fmt;

const ZERO: u8 = 0b_0000_1000;
const N_FLAG: u8 = 0b_0000_0100;
const HALF_CARRY: u8 = 0b_0000_0010;
const CARRY: u8 = 0b_0000_0001;

pub enum Flags {
    ZERO,
    N,
    CARRY,
    HALF,
}

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0x01,
            f: 0x00,
            b: 0xff,
            c: 0x13,
            d: 0x00,
            e: 0xc1,
            h: 0x84,
            l: 0x03,
            sp: 0xfffe,
        }
    }

    pub fn hl(&mut self) -> u16 {
        combine(self.h as u16, self.l as u16)
    }

    pub fn bc(&mut self) -> u16 {
        combine(self.b as u16, self.c as u16)
    }

    pub fn de(&mut self) -> u16 {
        combine(self.d as u16, self.e as u16)
    }

    pub fn set_hl(&mut self, value: u16) {
        (self.h, self.l) = split_u8(value);
    }

    pub fn is_flag(&mut self, flag: Flags) -> bool {
        match flag {
            Flags::ZERO => (self.f & ZERO) >= 1,
            Flags::N => (self.f & N_FLAG) >= 1,
            Flags::CARRY => (self.f & CARRY) >= 1,
            Flags::HALF => (self.f & HALF_CARRY) >= 1,
        }
    }

    pub fn set_flag(&mut self, flag: Flags, value: bool) {
        match flag {
            Flags::ZERO => self.make_set_flag(ZERO, value),
            Flags::N => self.make_set_flag(N_FLAG, value),
            Flags::CARRY => self.make_set_flag(CARRY, value),
            Flags::HALF => self.make_set_flag(HALF_CARRY, value),
        }
    }
    fn make_set_flag(&mut self, set: u8, value: bool) {
        if value {
            self.f |= set;
        } else {
            self.f &= !set;
        }
    }
}

fn combine(high: u16, low: u16) -> u16 {
    (high << 8) | low
}

fn split_u8(value: u16) -> (u8, u8) {
    let high = (value >> 8) & 0xFF;
    let low = value & 0xFF;
    (high as u8, low as u8)
}

fn get_flag_str(f: u8) -> String {
    let mut retval = String::new();
    if (f & ZERO) >= 1 {
        retval.push('z');
    } else {
        retval.push('-');
    }
    if (f & N_FLAG) >= 1 {
        retval.push('n');
    } else {
        retval.push('-');
    }
    if (f & HALF_CARRY) >= 1 {
        retval.push('h');
    } else {
        retval.push('-');
    }
    if (f & CARRY) >= 1 {
        retval.push('c');
    } else {
        retval.push('-');
    }
    retval
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bc = combine(self.b as u16, self.c as u16);
        let de = combine(self.d as u16, self.e as u16);
        let hl = combine(self.h as u16, self.l as u16);
        write!(
            f,
            " flags: {} | a: ${:02x} | bc: ${:04x} | de: ${:04x} | hl: ${:04x} | sp: ${:04x} |",
            get_flag_str(self.f),
            self.a,
            bc,
            de,
            hl,
            self.sp
        )
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Reg8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Copy, Clone, Debug)]
pub enum Addr {
    BC,
    DE,
    HL,
}

#[derive(Copy, Clone, Debug)]
pub enum Imm8 {}

#[derive(Copy, Clone, Debug)]
pub enum Reg16 {
    BC,
    DE,
    HL,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_set_carry() {
        let mut reg = Registers::new();

        reg.f = 0b_0000_0100;
        reg.set_flag(Flags::CARRY, true);

        assert_eq!(reg.f, 0b_0000_0101)
    }

    #[test]
    fn it_should_unset_carry() {
        let mut reg = Registers::new();

        reg.f = 0b_0000_0101;
        reg.set_flag(Flags::CARRY, false);

        assert_eq!(reg.f, 0b_0000_0100)
    }

    #[test]
    fn it_should_return_zero_flag_is_set() {
        let mut reg = Registers::new();

        reg.f = 0b_0000_1001;
        let res = reg.is_flag(Flags::ZERO);

        assert!(res)
    }

    #[test]
    fn it_should_return_a_format() {
        let mut reg = Registers::new();

        reg.f = 0b_0000_1111;
        let str = format!("{}", reg);

        assert_eq!(
            str,
            " flags: znhc | a: $01 | bc: $ff13 | de: $00c1 | hl: $8403 | sp: $fffe |"
        )
    }

    #[test]
    fn it_should_return_a_format_with_no_flags() {
        let mut reg = Registers::new();

        reg.f = 0b_0000_0000;
        let str = format!("{}", reg);

        assert_eq!(
            str,
            " flags: ---- | a: $01 | bc: $ff13 | de: $00c1 | hl: $8403 | sp: $fffe |"
        )
    }
}
