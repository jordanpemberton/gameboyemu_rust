#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unreachable_patterns)]
#![allow(unused_variables)]

use crate::console::instructions::{Instruction};
use crate::console::mmu::Mmu;
use crate::console::registers::Registers;

pub(crate) const PREFIX_BYTE: u8 = 0xCB;

pub(crate) struct Cpu {
    is_halted: bool,
    pub(crate) pc: u16,
    pub(crate) sp: u16,
    pub(crate) registers: Registers,
}

impl Cpu {
    pub(crate) fn new() -> Cpu {
        Cpu {
            is_halted: false,
            pc: 0,
            sp: 0,
            registers: Registers::new(),
        }
    }

    pub(crate) fn increment_pc(&mut self, increment_by: u16) {
        self.pc = self.pc.wrapping_add(increment_by);
    }

    pub(crate) fn step(&mut self, mmu: &mut Mmu) -> u16 {
        let mut cycles = 0;

        if !self.is_halted {
            let opcode = self.fetch_opcode(mmu);
            let instruction = Instruction::get_instruction(opcode);
            cycles = self.execute_instruction(instruction, mmu);
        }

        cycles
    }

    fn fetch_opcode(&mut self, mmu: &mut Mmu) -> u16 {
        let opcode: u16;

        let byte = mmu.read_byte(self.pc);
        self.increment_pc(1);

        if byte == PREFIX_BYTE {
            opcode = mmu.read_word(self.pc - 1);
            self.increment_pc(1);
        } else {
            opcode = byte as u16;
        }

        opcode
    }

    fn execute_instruction(&mut self, mut instruction: Instruction, mmu: &mut Mmu) -> u16 {
        instruction.execute(self, mmu)
    }
}
