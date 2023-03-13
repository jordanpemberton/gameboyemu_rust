#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unreachable_patterns)]
#![allow(unused_variables)]

use std::collections::HashMap;
use sdl2::keyboard::Keycode::Hash;
use crate::console::instructions::{Instruction};
use crate::console::mmu::{Endianness, Mmu};
use crate::console::registers::{RegIndex, Registers};

pub(crate) const PREFIX_BYTE: u8 = 0xCB;

pub(crate) struct Cpu {
    is_halted: bool,
    pub(crate) registers: Registers,
}

impl Cpu {
    pub(crate) fn new() -> Cpu {
        Cpu {
            is_halted: false,
            registers: Registers::new(),
        }
    }

    pub(crate) fn halt(&mut self) {
        self.is_halted = true;
    }

    pub(crate) fn step(&mut self, mmu: &mut Mmu) -> i16 {
        let mut cycles = 0;

        if !self.is_halted {
            let opcode = self.fetch_opcode(mmu);
            let instruction = Instruction::get_instruction(opcode);
            cycles = self.execute_instruction(instruction, mmu);
        }

        cycles
    }

    pub(crate) fn read_byte_at_pc(&mut self, mmu: &mut Mmu) -> u8 {
        let pc = self.registers.get_word(RegIndex::PC);
        let d8 = mmu.read_byte(pc);
        self.registers.increment(RegIndex::PC, 1);
        d8
    }

    pub(crate) fn read_word_at_pc(&mut self, mmu: &mut Mmu) -> u16 {
        let pc = self.registers.get_word(RegIndex::PC);
        let d16 = mmu.read_word(pc, Endianness::BIG);
        self.registers.increment(RegIndex::PC, 2);
        d16
    }

    fn fetch_opcode(&mut self, mmu: &mut Mmu) -> u16 {
        let mut opcode: u16;

        let mut byte = self.read_byte_at_pc(mmu);

        if byte == PREFIX_BYTE {
            opcode = (byte as u16) << 8;
            byte = self.read_byte_at_pc(mmu);
            opcode |= byte as u16;
        } else {
            opcode = byte as u16;
        }

        opcode
    }

    fn execute_instruction(&mut self, mut instruction: Instruction, mmu: &mut Mmu) -> i16 {
        instruction.execute(self, mmu)
    }
}
