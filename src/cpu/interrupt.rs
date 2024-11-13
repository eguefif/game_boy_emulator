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

    pub fn reset_iflag(&mut self) -> u16 {
        let mut check = 1;
        let mut counter = 0;
        loop {
            let save = self.iflag;
            self.iflag &= !check & 0b0001_1111;
            if save != self.iflag {
                return (0x40 + counter * 8) as u16;
            }
            check <<= 1;
            counter += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_reset_vblank_only() {
        let mut int = Interrupt::new();
        int.iflag = 0b_0000_1101;

        let addr = int.reset_iflag();

        assert_eq!(int.iflag, 0b_0000_1100);
        assert_eq!(addr, 0x40);
    }

    #[test]
    fn it_should_reset_joypad() {
        let mut int = Interrupt::new();
        int.iflag = 0b_0001_0000;

        let addr = int.reset_iflag();

        assert_eq!(int.iflag, 0b_0000_0000);
        assert_eq!(addr, 0x60);
    }

    #[test]
    fn it_should_return_interrupt_true() {
        let mut int = Interrupt::new();
        int.iflag = 0b_0001_0000;
        int.ie = 0b_0001_0000;

        assert!(int.should_interrupt());
    }

    #[test]
    fn it_should_return_interrupt_true_2() {
        let mut int = Interrupt::new();
        int.iflag = 0b_0001_0000;
        int.ie = 0b_0001_0001;

        assert!(int.should_interrupt());
    }

    #[test]
    fn it_should_return_interrupt_false() {
        let mut int = Interrupt::new();
        int.iflag = 0b_0001_0000;
        int.ie = 0b_0000_0001;

        assert!(!int.should_interrupt());
    }
}
