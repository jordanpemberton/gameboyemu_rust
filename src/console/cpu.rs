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

const DEBUG_PRINT: bool = true;

pub(crate) struct Cpu {
    is_halted: bool,
    pub(crate) registers: CpuRegisters,
    pub(crate) interrupts: Interrupts,
    visited: Vec<u16>,
}

impl Cpu {
    pub(crate) fn new(mmu: &mut Mmu) -> Cpu {
        Cpu {
            is_halted: false,
            registers: CpuRegisters::new(),
            interrupts: Interrupts::new(mmu),
            visited: vec![],
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
            let args = self.fetch_args(&instruction, mmu);

            if DEBUG_PRINT {
                if !self.visited.contains(&start_pc)
                {
                    print!("{:#06X}\t{:#06X}\t{:<14}", start_pc, opcode, instruction.mnemonic);
                    if args.len() > 0 {
                        print!("\t{:#04X}", args[0]);
                    };
                    if args.len() > 1 {
                        print!("\t{:#04X}", args[1]);
                    };
                    println!();
                }
                self.visited.push(start_pc);
            }

            instruction.execute(self, mmu, args.as_ref())
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

    fn fetch_args(&mut self, instruction: &Instruction, mmu: &mut Mmu) -> Vec<u8> {
        let mut args: Vec<u8> = vec![];

        let num_args = if instruction.is_cbprefixed() {
            instruction.size - 2
        } else {
            instruction.size - 1
        };

        if num_args > 0 {
            args.push(self.read_byte_at_pc(mmu));
        }
        if num_args > 1 {
            args.push(self.read_byte_at_pc(mmu));
        }

        args
    }
}
