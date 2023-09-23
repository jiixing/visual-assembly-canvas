const R_SIZE: usize = 0xF;

#[derive(Debug)]
pub struct Registers {
    pub buffer: [u8; R_SIZE],
}

#[derive(Copy, Clone)]
pub enum Register {
    // General Purpose Registers
    R01 = 0x01,
    R02 = 0x02,
    R03 = 0x03,
    R04 = 0x04,
    R05 = 0x05,
    R06 = 0x06,
    R07 = 0x07,
    R08 = 0x08,
    R09 = 0x09,
    R10 = 0x0A,

    /// Frame Pointer
    FP = 0x0B,

    /// Stack Pointer
    SP = 0x0C,

    /// Program Counter
    PC = 0x0D,

    /// Status Register
    SR = 0x0E,
}

type R = Register;

impl Registers {
    pub fn new() -> Registers {
        Registers {
            buffer: [0; R_SIZE],
        }
    }

    pub fn set(&mut self, r: R, val: u8) {
        self.buffer[r as usize] = val;
    }

    pub fn get(&self, r: R) -> u8 {
        self.buffer[r as usize]
    }

    pub fn inc(&mut self, r: R) {
        self.set(r, self.get(r) + 1);
    }

    pub fn dec(&mut self, r: R) {
        self.set(r, self.get(r) - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::Register::{FP, PC};
    use super::*;

    #[test]
    fn test_set_register() {
        let mut r = Registers::new();
        r.set(FP, 0x10);
        assert_eq!(r.get(FP), 0x10, "FP should be set to 0x10");

        r.set(PC, 0xFF);
        assert_eq!(r.get(PC), 0xFF, "PC should be set to 0xFF")
    }

    #[test]
    fn test_inc_dec() {
        let mut r = Registers::new();

        r.inc(PC);
        r.inc(PC);
        assert_eq!(r.get(PC), 2, "PC should be incremented");

        r.dec(PC);
        assert_eq!(r.get(PC), 1, "PC should be decremented");
    }
}