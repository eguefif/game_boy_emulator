#![allow(clippy::new_without_default)]
#![allow(dead_code)]

use minifb::{Key, Window};
use std::fmt;

#[derive(Clone)]
pub struct Joypad {
    down: bool,
    top: bool,
    left: bool,
    right: bool,
    a: bool,
    b: bool,
    start: bool,
    select: bool,
    pub interrupt: bool,
    pad_active: bool,
    button_active: bool,
}

impl Joypad {
    pub fn new() -> Joypad {
        Joypad {
            down: false,
            top: false,
            left: false,
            right: false,
            a: false,
            b: false,
            start: false,
            select: false,
            interrupt: false,
            pad_active: false,
            button_active: false,
        }
    }

    pub fn is_interrupt(&mut self) -> bool {
        if self.interrupt {
            self.interrupt = false;
            return true;
        }
        false
    }

    pub fn set_joypad(&mut self, value: u8) {
        self.button_active = value & 0b_0010_0000 > 0;

        self.pad_active = value & 0b_0001_0000 > 0;
    }

    pub fn get_joypad(&mut self) -> u8 {
        let mut retval = 0b_1100_1111;
        if self.pad_active {
            if self.right {
                retval &= !0b1;
            }
            if self.left {
                retval &= !0b10;
            }
            if self.top {
                retval &= !0b100;
            }
            if self.down {
                retval &= !0b1000;
            }
            retval = self.set_joypad_bits(retval);
            return retval;
        } else if self.button_active {
            if self.a {
                retval &= !0b1;
            }
            if self.b {
                retval &= !0b10;
            }
            if self.select {
                retval &= !0b100;
            }
            if self.start {
                retval &= !0b1000;
            }
            retval = self.set_joypad_bits(retval);
            return retval;
        }
        0xFF
    }

    fn set_joypad_bits(&mut self, input: u8) -> u8 {
        let mut retval = input;
        if self.pad_active {
            retval |= 0b_0001_0000;
        }
        if self.button_active {
            retval |= 0b_0010_0000;
        }
        retval
    }

    pub fn update(&mut self, window: &Window) {
        if window.is_key_down(Key::W) {
            //&& !window.is_key_down(Key::S) {
            if !self.top {
                self.interrupt = true;
            }
            self.top = true;
        } else {
            self.top = false;
        }
        if !window.is_key_down(Key::W) {
            //&& window.is_key_down(Key::S) {
            if !self.down {
                self.interrupt = true;
            }
            self.down = true;
        } else {
            self.down = false;
        }
        if window.is_key_down(Key::A) {
            //&& !window.is_key_down(Key::D) {
            if !self.left {
                self.interrupt = true;
            }
            self.left = true;
        } else {
            self.left = false;
        }
        if !window.is_key_down(Key::A) {
            //&& window.is_key_down(Key::D) {
            if !self.right {
                self.interrupt = true;
            }
            self.right = true;
        } else {
            self.right = false;
        }
        if window.is_key_down(Key::J) {
            if !self.a {
                self.interrupt = true;
            }
            self.a = true;
        } else {
            self.a = false;
        }
        if window.is_key_down(Key::K) {
            if !self.b {
                self.interrupt = true;
            }
            self.b = true;
        } else {
            self.b = false;
        }
        if window.is_key_down(Key::U) {
            if !self.start {
                self.interrupt = true;
            }
            self.start = true;
        } else {
            self.start = false;
        }
        if window.is_key_down(Key::I) {
            if !self.select {
                self.interrupt = true;
            }
            self.select = true;
        } else {
            self.select = false;
        }
    }

    fn reset(&mut self) {
        self.down = false;
        self.top = false;
        self.right = false;
        self.left = false;
        self.a = false;
        self.b = false;
        self.select = false;
        self.start = false;
    }
}

impl fmt::Display for Joypad {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Joypad: SA: {}, SE: {}, A: {}, B: {}, UP: {}, D: {}, L: {}, R: {}",
            self.start, self.select, self.a, self.b, self.top, self.down, self.left, self.right
        )
    }
}
