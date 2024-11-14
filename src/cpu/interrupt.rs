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

    pub fn get_next_interrupt(&mut self) -> Result<u32, bool> {
        let mut position = 0;
        loop {
            if self.is_interrupt_enabled(position) {
                self.iflag &= !(1 << position);
                return Ok(position);
            }
            if position > 32 {
                break;
            }
            position += 1;
        }
        Err(false)
    }

    pub fn get_interrupt_addr(&mut self) -> Result<u16, bool> {
        match self.get_next_interrupt() {
            Ok(interrupt) => Ok(0x40 + (interrupt as u16 * 8)),
            Err(e) => Err(e),
        }
    }

    fn is_interrupt_enabled(&mut self, position: u32) -> bool {
        (self.ie >> position) & 1 == 1 && (self.iflag >> position) & 1 == 1
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

        let check = int.get_next_interrupt().unwrap();

        assert_eq!(check, 2);
    }

    #[test]
    fn it_get_next_interrupt_joypad() {
        let mut int = Interrupt::new();
        int.iflag = 0b_0001_0000;
        int.ie = 0b_0001_1111;

        let check = int.get_next_interrupt().unwrap();

        assert_eq!(check, 4);
    }

    #[test]
    fn it_get_next_interrupt_vblank() {
        let mut int = Interrupt::new();
        int.iflag = 0b_0001_1111;
        int.ie = 0b_0001_1111;

        let check = int.get_next_interrupt().unwrap();

        assert_eq!(check, 0);
    }

    #[test]
    fn it_check_interrupt_enabled_true() {
        let mut int = Interrupt::new();
        int.ie = 0b_0000_1000;
        int.iflag = 0b_0000_1000;

        let check = int.is_interrupt_enabled(3);

        assert!(check);
    }

    #[test]
    fn it_check_interrupt_enabled_false2() {
        let mut int = Interrupt::new();
        int.ie = 0b_0000_0010;
        int.iflag = 0b_0000_0010;

        let check = int.is_interrupt_enabled(3);

        assert!(!check);
    }

    #[test]
    fn it_check_interrupt_enabled_false() {
        let mut int = Interrupt::new();
        int.ie = 0b_0001_0000;
        int.iflag = 0b_0001_0000;

        let check = int.is_interrupt_enabled(3);

        assert!(!check);
    }

    #[test]
    fn it_should_reset_return_addr_joypad() {
        let mut int = Interrupt::new();
        int.iflag = 0b_0001_1100;
        int.ie = 0b_0001_0000;

        let addr = int.get_interrupt_addr().unwrap();

        println!("addr {:x}", addr);
        assert_eq!(int.iflag, 0b_0000_1100);
        assert_eq!(addr, 0x60);
    }

    #[test]
    fn it_should_reset_return_addr_vblanc() {
        let mut int = Interrupt::new();
        int.iflag = 0b_0000_1101;
        int.ie = 0b_0001_1111;

        let addr = int.get_interrupt_addr().unwrap();

        assert_eq!(int.iflag, 0b_0000_1100);
        println!("addr {:x}", addr);
        assert_eq!(addr, 0x40);
    }

    #[test]
    fn it_should_reset_vblank_only() {
        let mut int = Interrupt::new();
        int.iflag = 0b_0000_1101;
        int.ie = 0b_0001_1111;

        let _ = int.get_interrupt_addr().unwrap();

        assert_eq!(int.iflag, 0b_0000_1100);
    }

    #[test]
    fn it_should_reset_joypad() {
        let mut int = Interrupt::new();
        int.iflag = 0b_0001_0000;
        int.ie = 0b_0001_1111;

        let _ = int.get_interrupt_addr().unwrap();

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
