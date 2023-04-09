#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unreachable_patterns)]
#![allow(unused_variables)]

use std::collections::HashMap;
use sdl2::keyboard::Keycode::Hash;
use crate::console::instructions::{Instruction};
use crate::console::mmu::{Endianness, Mmu};
use crate::console::cpu_registers::{CpuRegIndex, CpuRegisters};
use crate::console::interrupts::Interrupts;

pub(crate) const PREFIX_BYTE: u8 = 0xCB;

const DEBUG_PRINT: bool = false;

pub(crate) struct Cpu {
    is_halted: bool,
    pub(crate) registers: CpuRegisters,
    pub(crate) interrupts: Interrupts,
}

impl Cpu {
    pub(crate) fn new(mmu: &mut Mmu) -> Cpu {
        Cpu {
            is_halted: false,
            registers: CpuRegisters::new(),
            interrupts: Interrupts::new(mmu),
        }
    }

    pub(crate) fn halt(&mut self) {
        self.is_halted = true;
    }

    pub(crate) fn check_interrupts(&mut self) -> i16 {
        if self.interrupts.ime {
            // TODO
            16
        } else {
            // TODO
            0
        }
    }

    pub(crate) fn step(&mut self, mmu: &mut Mmu) -> i16 {
        if self.is_halted {
            0
        } else {
            let start_pc = self.registers.get_word(CpuRegIndex::PC);

            let opcode = self.fetch_opcode(mmu);
            let mut instruction = Instruction::get_instruction(opcode);
            let (arg1, arg2) = self.fetch_args(&instruction, mmu);

            if DEBUG_PRINT {
                println!("{:#06X}\t{:#06X}\t{}\t{}\t{}", start_pc, opcode, instruction.mnemonic, arg1, arg2);
            }

            instruction.execute(self, mmu, &[arg1, arg2])
        }
    }

    fn read_byte_at_pc(&mut self, mmu: &mut Mmu) -> u8 {
        let pc = self.registers.get_word(CpuRegIndex::PC);
        let d8 = mmu.read_byte(pc);
        self.registers.increment(CpuRegIndex::PC, 1);
        d8
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

    fn fetch_args(&mut self, instruction: &Instruction, mmu: &mut Mmu) -> (u8, u8) {
        let mut arg1: u8 = 0;
        let mut arg2: u8 = 0;

        let num_args = if instruction.is_cbprefixed() {
            instruction.size - 2
        } else {
            instruction.size - 1
        };

        if num_args > 0 {
            arg1 = self.read_byte_at_pc(mmu);
        }
        if num_args > 1 {
            arg2 = self.read_byte_at_pc(mmu);
        }

        (arg1, arg2)
    }
}
