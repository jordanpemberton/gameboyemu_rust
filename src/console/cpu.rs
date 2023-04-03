#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unreachable_patterns)]
#![allow(unused_variables)]

use std::collections::HashMap;
use sdl2::keyboard::Keycode::Hash;
use crate::console::instructions::{Instruction};
use crate::console::mmu::{Endianness, Mmu};
use crate::console::cpu_registers::{CpuRegIndex, CpuRegisters};

pub(crate) const PREFIX_BYTE: u8 = 0xCB;

const DEBUG_PRINT: bool = false;

pub(crate) struct Cpu {
    is_halted: bool,
    pub(crate) registers: CpuRegisters,
}

impl Cpu {
    pub(crate) fn new() -> Cpu {
        Cpu {
            is_halted: false,
            registers: CpuRegisters::new(),
        }
    }

    pub(crate) fn halt(&mut self) {
        self.is_halted = true;
    }

    pub(crate) fn step(&mut self, mmu: &mut Mmu) -> i16 {
        let mut cycles = 0;
        let start_pc = self.registers.get_word(CpuRegIndex::PC);

        if !self.is_halted {
            let opcode = self.fetch_opcode(mmu);
            let instruction = Instruction::get_instruction(opcode);

            if DEBUG_PRINT {
                println!("{:#06X}\t{:#06X}\t{}", start_pc, opcode, instruction.mnemonic);
            }

            cycles = self.execute_instruction(instruction, mmu);
        }

        cycles
    }

    pub(crate) fn read_byte_at_pc(&mut self, mmu: &mut Mmu) -> u8 {
        let pc = self.registers.get_word(CpuRegIndex::PC);
        let d8 = mmu.read_byte(pc);
        self.registers.increment(CpuRegIndex::PC, 1);
        d8
    }

    pub(crate) fn read_word_at_pc(&mut self, mmu: &mut Mmu) -> u16 {
        let pc = self.registers.get_word(CpuRegIndex::PC);
        let d16 = mmu.read_word(pc, Endianness::BIG);
        self.registers.increment(CpuRegIndex::PC, 2);
        d16
    }

    pub(crate) fn check_interrupt(&self) -> i16 {
        // TODO
        0
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
