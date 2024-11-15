pub struct Interrupt {
    pub iflag: u8,
    pub ie: u8,
}

const TIMER: u8 = 0b_0000_0100;

impl Interrupt {
    pub fn new() -> Interrupt {
        Interrupt { iflag: 0, ie: 0 }
    }

    pub fn require_timer(&mut self) {
        self.iflag |= TIMER;
    }

    pub fn set_iflag(&mut self, value: u8) {
        self.iflag = value & 0b_0001_1111;
    }

    pub fn set_ie(&mut self, value: u8) {
        self.ie = value & 0b_0001_1111;
    }

    pub fn should_interrupt(&mut self) -> bool {
        self.iflag & self.ie != 0
    }

    pub fn get_next_interrupt(&mut self) -> u16 {
        let int = self.ie & self.iflag;
        int.trailing_zeros() as u16
    }

    pub fn get_interrupt_addr(&mut self) -> u16 {
        let int = self.get_next_interrupt();
        self.reset_int(int);
        0x40 + (int * 8)
    }

    fn reset_int(&mut self, int: u16) {
        self.iflag &= !(1 << int);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_get_next_interrupt_timer() {
        let mut int = Interrupt::new();
        int.iflag = 0b_0001_0101;
        int.ie = 0b_0001_1100;

        let check = int.get_next_interrupt();

        assert_eq!(check, 2);
    }

    #[test]
    fn it_get_next_interrupt_joypad() {
        let mut int = Interrupt::new();
        int.iflag = 0b_0001_0000;
        int.ie = 0b_0001_1111;

        let check = int.get_next_interrupt();

        assert_eq!(check, 4);
    }

    #[test]
    fn it_get_next_interrupt_vblank() {
        let mut int = Interrupt::new();
        int.iflag = 0b_0001_1111;
        int.ie = 0b_0001_1111;

        let check = int.get_next_interrupt();

        assert_eq!(check, 0);
    }

    #[test]
    fn it_should_reset_return_addr_joypad() {
        let mut int = Interrupt::new();
        int.iflag = 0b_0001_1100;
        int.ie = 0b_0001_0000;

        let addr = int.get_interrupt_addr();

        assert_eq!(int.iflag, 0b_0000_1100);
        assert_eq!(addr, 0x60);
    }

    #[test]
    fn it_should_reset_return_addr_vblanc() {
        let mut int = Interrupt::new();
        int.iflag = 0b_0000_1101;
        int.ie = 0b_0001_1111;

        let addr = int.get_interrupt_addr();

        assert_eq!(int.iflag, 0b_0000_1100);
        assert_eq!(addr, 0x40);
    }

    #[test]
    fn it_should_reset_vblank_only() {
        let mut int = Interrupt::new();
        int.iflag = 0b_0000_1101;
        int.ie = 0b_0001_1111;

        let _ = int.get_interrupt_addr();

        assert_eq!(int.iflag, 0b_0000_1100);
    }

    #[test]
    fn it_should_reset_joypad() {
        let mut int = Interrupt::new();
        int.iflag = 0b_0001_0000;
        int.ie = 0b_0001_1111;

        let _ = int.get_interrupt_addr();

        assert_eq!(int.iflag, 0b_0000_0000);
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
