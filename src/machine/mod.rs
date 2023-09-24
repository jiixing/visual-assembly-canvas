use crate::mem::Memory;
use crate::register::Registers;
use crate::instructions::Load;
use crate::instructions::Instruction::{Push, Pop, Add};

#[derive(Debug)]
pub struct Machine {
    pub mem: Memory,
    pub reg: Registers,
}

impl Machine {
    pub fn new() -> Machine {
        let mut mem = Memory::new();
        let reg = Registers::new();

        mem.load_code(vec![Push(5), Push(10), Add, Pop]);

        Machine {
            mem,
            reg,
        }
    }


}
