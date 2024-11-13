pub struct Interrupt {
    pub iflag: u8,
    pub ie: u8,
}

impl Interrupt {
    pub fn new() -> Interrupt {
        Interrupt { iflag: 0, ie: 0 }
    }

    pub fn set_iflag(&mut self, value: u8) {
        self.iflag = value & 0b_0001_1111;
    }

    pub fn set_ie(&mut self, value: u8) {
        self.ie = value & 0b_0001_1111;
    }

    pub fn should_interrupt(&mut self) -> bool {
        let check = self.iflag & self.ie;
        check >= 1
    }

    pub fn reset_if(&mut self) {
        let iflag = self.iflag;
        let vblank = iflag & 0b_0001;
        let lcd = iflag & 0b_0100;
        let timer = iflag & 0b_0100;
        let serial = iflag & 0b_1000;
        let joypad = iflag & 0b1_0000;
        if vblank > 0 {}
    }
}
