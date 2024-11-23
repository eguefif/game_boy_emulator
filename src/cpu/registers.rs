#![allow(dead_code)]
#![allow(clippy::new_without_default)]

use std::fmt;

const ZERO: u8 = 0b_1000_0000;
const N_FLAG: u8 = 0b_0100_0000;
const HALF_CARRY: u8 = 0b_0010_0000;
const CARRY: u8 = 0b_0001_0000;

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
pub enum Reg16 {
    BC,
    DE,
    HL,
    SP,
    AF,
}

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

    pub fn inc_sp(&mut self) {
        let sp = self.sp;
        self.sp = sp.wrapping_add(1);
    }

    pub fn dec_sp(&mut self) {
        let sp = self.sp;
        self.sp = sp.wrapping_sub(1);
    }

    pub fn af(&mut self) -> u16 {
        combine(self.a as u16, self.f as u16)
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

    pub fn set_bc(&mut self, value: u16) {
        (self.b, self.c) = split_u16(value);
    }

    pub fn set_de(&mut self, value: u16) {
        (self.d, self.e) = split_u16(value)
    }

    pub fn set_hl(&mut self, value: u16) {
        (self.h, self.l) = split_u16(value);
    }

    pub fn set_af(&mut self, value: u16) {
        (self.a, self.f) = split_u16(value);
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
            Flags::ZERO => self.set_flag_value(ZERO, value),
            Flags::N => self.set_flag_value(N_FLAG, value),
            Flags::CARRY => self.set_flag_value(CARRY, value),
            Flags::HALF => self.set_flag_value(HALF_CARRY, value),
        }
    }

    fn set_flag_value(&mut self, set: u8, value: bool) {
        if value {
            self.f |= set;
        } else {
            self.f &= !set;
        }
    }
}

pub fn test_half_carry_8(value: u8, addend: u8, carry: u8) -> bool {
    let mask = 0b_0000_1111;
    ((value as u16 & mask) + (addend as u16 & mask) + carry as u16) > mask
}

pub fn test_carry_8(value: u8, addend: u8, carry: u8) -> bool {
    ((value as u16 & 0xFF) + (addend as u16 & 0xFF) + carry as u16) > 0xFF
}

pub fn combine(high: u16, low: u16) -> u16 {
    (high << 8) | low
}

pub fn split_u16(value: u16) -> (u8, u8) {
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
            " flags: {} | a: ${:02x} | bc: ${:04x} | de: ${:04x} | hl: ${:04x} | sp: ${:04x}",
            get_flag_str(self.f),
            self.a,
            bc,
            de,
            hl,
            self.sp
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_set_carry() {
        let mut reg = Registers::new();

        reg.f = 0b_0100_0000;
        reg.set_flag(Flags::CARRY, true);

        assert_eq!(reg.f, 0b_0101_0000)
    }

    #[test]
    fn it_should_unset_carry() {
        let mut reg = Registers::new();

        reg.f = 0b_0101_0000;
        reg.set_flag(Flags::CARRY, false);

        assert_eq!(reg.f, 0b_0100_0000)
    }

    #[test]
    fn it_should_return_zero_flag_is_set() {
        let mut reg = Registers::new();

        reg.f = 0b_1001_0000;
        let res = reg.is_flag(Flags::ZERO);

        assert!(res)
    }

    #[test]
    fn it_should_return_a_format() {
        let mut reg = Registers::new();

        reg.f = 0b_1111_0000;
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

    #[test]
    fn it_should_test_half_carry_8_true() {
        let value = 0xF;
        let addend = 1;
        let test = test_half_carry_8(value, addend, 0);

        assert!(test);
    }

    #[test]
    fn it_should_test_carry_8_true() {
        let value = 0xFF;
        let addend = 1;
        let test = test_carry_8(value, addend, 0);

        assert!(test);
    }

    #[test]
    fn it_should_test_half_carry_8_false() {
        let value = 0xF;
        let addend = 1;
        let test = test_half_carry_8(value, addend, 0);

        assert!(test);
    }

    #[test]
    fn it_should_test_carry_8_false() {
        let value = 0xFF;
        let addend = 1;
        let test = test_carry_8(value, addend, 0);

        assert!(test);
    }
}
