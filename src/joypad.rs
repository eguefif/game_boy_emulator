#![allow(clippy::new_without_default)]
#![allow(dead_code)]

use minifb::{Key, KeyRepeat, Window};

#[derive(Clone, Debug)]
enum Mode {
    Pad,
    Buttons,
    None,
}

#[derive(Clone, Debug)]
pub struct Joypad {
    pad: u8,
    buttons: u8,
    mode: Mode,
    pub interrupt: bool,
}

impl Joypad {
    pub fn new() -> Joypad {
        Joypad {
            pad: 0xF,
            buttons: 0xF,
            mode: Mode::None,
            interrupt: false,
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
        if (value >> 4) & 1 == 0 {
            self.mode = Mode::Pad
        }
        if (value >> 5) & 1 == 0 {
            self.mode = Mode::Buttons;
        }
        if value & 0x30 == 0x30 {
            self.mode = Mode::None;
        }
    }

    pub fn get_joypad(&mut self) -> u8 {
        let retval = 0xC0 + self.get_mode();
        match self.mode {
            Mode::Pad => retval | self.pad,
            Mode::Buttons => retval | self.buttons,
            Mode::None => retval | 0xF,
        }
    }

    fn get_mode(&mut self) -> u8 {
        match self.mode {
            Mode::Pad => 0x10,
            Mode::Buttons => 0x20,
            Mode::None => 0x30,
        }
    }

    pub fn update(&mut self, window: &Window) {
        let before_pad = self.pad & 0xF;
        let before_buttons = self.buttons & 0xF;
        window
            .get_keys_pressed(KeyRepeat::No)
            .iter()
            .for_each(|key| match key {
                Key::J => self.buttons &= 0b1110,
                Key::K => self.buttons &= 0b1101,
                Key::I => self.buttons &= 0b1011,
                Key::U => self.buttons &= 0b0111,

                Key::D => self.pad &= 0b1110,
                Key::A => self.pad &= 0b1101,
                Key::W => self.pad &= 0b1011,
                Key::S => self.pad &= 0b0111,
                _ => {}
            });
        window.get_keys_released().iter().for_each(|key| match key {
            Key::J => self.buttons |= 0b0001,
            Key::K => self.buttons |= 0b0010,
            Key::I => self.buttons |= 0b0100,
            Key::U => self.buttons |= 0b1000,

            Key::D => self.pad |= 0b0001,
            Key::A => self.pad |= 0b0010,
            Key::W => self.pad |= 0b0100,
            Key::S => self.pad |= 0b1000,
            _ => {}
        });
        //if before_pad != self.pad || before_buttons != self.buttons {
        //    println!("PAD: before {:0>4b}, after {:0>4b}", before_pad, self.pad);
        //    println!(
        //        "Buttons: before {:0>4b}, after {:0>4b}",
        //        before_buttons, self.buttons
        //    );
        //}
        if before_pad > self.pad || before_buttons > self.buttons {
            self.interrupt = true;
        }
    }
}
