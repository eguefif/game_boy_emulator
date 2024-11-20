use crate::ppu::config::State;
use crate::ppu::config::State::{Mode0, Mode1, Mode2, Mode3};
use crate::ppu::Ppu;

impl Ppu {
    pub fn write_stat(&mut self, value: u8) {
        let before = self.stat & 0b0000_0111;
        self.stat = (value & 0b1111_1000) | before;
    }

    pub fn update_stat(&mut self, new_state: State) {
        //let before = self.stat;
        match new_state {
            Mode3 => {
                self.stat |= 0b11;
            }
            Mode1 => {
                self.stat |= 0b01;
                self.stat &= 0b1111_1101;
            }
            Mode2 => {
                self.stat |= 0b10;
                self.stat &= 0b1111_1110;
            }
            Mode0 => self.stat &= 0b1111_1100,
        }
        //self.handle_stat_interrupt(before, value);
    }

    //fn handle_stat_interrupt(&mut self, before: u8, value: u8){
    //    
    //}
}
