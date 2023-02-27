#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unreachable_patterns)]
#![allow(unused_variables)]

use crate::console::instructions::{Instruction};
use crate::console::mmu::{Mmu};
use crate::console::registers::{RegIndex, Registers};

pub(crate) const PREFIX_BYTE: u8 = 0xCB;

pub(crate) struct Cpu {
    pub(crate) registers: Registers,
    is_halted: bool,
}

impl Cpu {
    pub(crate) fn new() -> Cpu {
        Cpu {
            registers: Registers::new(),
            is_halted: false,
        }
    }

    pub(crate) fn boot(&mut self, mmu: &mut Mmu) {
        //
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
        let mut opcode: u16;

        let mut source_address = self.registers.get_word(RegIndex::PC);
        let mut byte = mmu.read_byte(source_address);
        self.registers.increment(RegIndex::PC, 1);

        if byte == PREFIX_BYTE {
            opcode = (byte as u16) << 8;
            source_address = self.registers.get_word(RegIndex::PC);
            byte = mmu.read_byte(source_address);
            self.registers.increment(RegIndex::PC, 1);
            opcode |= byte as u16;
        } else {
            opcode = byte as u16;
        }

        opcode
    }

    fn execute_instruction(&mut self, mut instruction: Instruction, mmu: &mut Mmu) -> u16 {
        instruction.execute(self, mmu)
    }
}
