#![allow(clippy::new_without_default)]

pub struct Apu {
    master: u8,
    master_volume: u8,
    panning: u8,
    ch1_sweep: u8,
    ch1_length: u8,
    ch1_vol: u8,
    ch1_lo: u8,
    ch1_hi: u8,
    ch2_length: u8,
    ch2_vol: u8,
    ch2_lo: u8,
    ch2_hi: u8,
    ch3_dac: u8,
    ch3_length: u8,
    ch3_level: u8,
    ch3_lo: u8,
    ch3_hi: u8,
    ch4_length: u8,
    ch4_vol: u8,
    ch4_freq: u8,
    ch4_ctrl: u8,
    wave_ram: [u8; 16],
}

impl Apu {
    pub fn new() -> Apu {
        Apu {
            master: 0,
            master_volume: 0,
            panning: 0,
            ch1_sweep: 0,
            ch1_length: 0,
            ch1_vol: 0,
            ch1_lo: 0,
            ch1_hi: 0,
            ch2_length: 0,
            ch2_vol: 0,
            ch2_lo: 0,
            ch2_hi: 0,
            ch3_dac: 0,
            ch3_length: 0,
            ch3_level: 0,
            ch3_lo: 0,
            ch3_hi: 0,
            ch4_length: 0,
            ch4_vol: 0,
            ch4_freq: 0,
            ch4_ctrl: 0,
            wave_ram: [0; 16],
        }
    }

    pub fn read(&mut self, loc: u16) -> u8 {
        match loc {
            0xFF26 => self.master,
            0xFF25 => self.panning,
            0xFF24 => self.master_volume,

            0xFF10 => self.ch1_sweep,
            0xFF11 => self.ch1_length,
            0xFF12 => self.ch1_vol,
            0xFF13 => self.ch1_lo,
            0xFF14 => self.ch1_hi,

            0xFF16 => self.ch2_length,
            0xFF17 => self.ch2_vol,
            0xFF18 => self.ch2_lo,
            0xFF19 => self.ch2_hi,

            0xFF1A => self.ch3_dac,
            0xFF1B => self.ch3_length,
            0xFF1C => self.ch3_level,
            0xFF1D => self.ch3_lo,
            0xFF1E => self.ch3_hi,

            0xFF30..0xFF3F => self.wave_ram[(loc - 0xFF30) as usize],

            0xFF20 => self.ch4_length,
            0xFF21 => self.ch4_vol,
            0xFF22 => self.ch4_freq,
            0xFF23 => self.ch4_ctrl,
            _ => {
                println!("PPU read: not handled");
                0xFF
            }
        }
    }

    pub fn write(&mut self, loc: u16, value: u8) {
        match loc {
            0xFF26 => self.master = value,
            0xFF25 => self.panning = value,
            0xFF24 => self.master_volume = value,

            0xFF10 => self.ch1_sweep = value,
            0xFF11 => self.ch1_length = value,
            0xFF12 => self.ch1_vol = value,
            0xFF13 => self.ch1_lo = value,
            0xFF14 => self.ch1_hi = value,

            0xFF16 => self.ch2_length = value,
            0xFF17 => self.ch2_vol = value,
            0xFF18 => self.ch2_lo = value,
            0xFF19 => self.ch2_hi = value,

            0xFF1A => self.ch3_dac = value,
            0xFF1B => self.ch3_length = value,
            0xFF1C => self.ch3_level = value,
            0xFF1D => self.ch3_lo = value,
            0xFF1E => self.ch3_hi = value,

            0xFF30..0xFF3F => self.wave_ram[(loc - 0xFF30) as usize] = value,

            0xFF20 => self.ch4_length = value,
            0xFF21 => self.ch4_vol = value,
            0xFF22 => self.ch4_freq = value,
            0xFF23 => self.ch4_ctrl = value,
            _ => {
                println!("PPU write: not handled")
            }
        }
    }
}
