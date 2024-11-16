#![allow(clippy::new_without_default)]
#![allow(dead_code)]

use minifb::{Key, Window};

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
    interrupt: bool,
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
            retval &= 0b_0001_0000;
        }
        if self.button_active {
            retval &= 0b_0010_0000;
        }
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
            return !retval;
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
            return retval;
        }
        0x0
    }

    pub fn update(&mut self, window: &Window) {
        self.reset();
        if window.is_key_down(Key::W) && !window.is_key_down(Key::S) {
            if !self.top {
                self.interrupt = true;
            }
            self.top = true;
        }
        if !window.is_key_down(Key::W) && window.is_key_down(Key::S) {
            if !self.down {
                self.interrupt = true;
            }
            self.down = true;
        }
        if window.is_key_down(Key::A) && !window.is_key_down(Key::D) {
            if !self.left {
                self.interrupt = true;
            }
            self.left = true;
        }
        if !window.is_key_down(Key::A) && window.is_key_down(Key::D) {
            if !self.right {
                self.interrupt = true;
            }
            self.right = true;
        }
        if window.is_key_down(Key::J) {
            if !self.a {
                self.interrupt = true;
            }
            self.a = true;
        }
        if window.is_key_down(Key::K) {
            if !self.b {
                self.interrupt = true;
            }
            self.b = true;
        }
        if window.is_key_down(Key::U) {
            if !self.start {
                self.interrupt = true;
            }
            self.start = true;
        }
        if window.is_key_down(Key::I) {
            if !self.select {
                self.interrupt = true;
            }
            self.select = true;
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
