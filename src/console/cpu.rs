// #![allow(dead_code)]
// #![allow(non_snake_case)]
// #![allow(unreachable_patterns)]
// #![allow(unused_variables)]

use std::collections::HashSet;
use crate::console::instruction::{Instruction};
use crate::console::mmu::Mmu;
use crate::console::cpu_registers::{CpuRegIndex, CpuRegisters};
use crate::console::debugger::Debugger;
use crate::console::interrupts::Interrupts;

pub(crate) const PREFIX_BYTE: u8 = 0xCB;

#[allow(dead_code)]
pub(crate) struct Cpu {
    pub(crate) is_halted: bool,
    pub(crate) registers: CpuRegisters,
    pub(crate) interrupts: Interrupts,
    pub(crate) visited: HashSet<u16>,
    debug_print_on: bool,
}

impl Cpu {
    pub(crate) fn new(mmu: &mut Mmu, debug_print: bool) -> Cpu {
        Cpu {
            is_halted: false,
            registers: CpuRegisters::new(),
            interrupts: Interrupts::new(mmu),
            visited: HashSet::from([]),
            debug_print_on: debug_print,
        }
    }

    pub(crate) fn halt(&mut self) {
        self.is_halted = true;
    }

    pub(crate) fn check_interrupts(&mut self, mmu: &mut Mmu) -> i16 {
        self.is_halted = false;

        let value = self.interrupts.poll(mmu);
        if value > 0 {
            self.interrupts.set_ime(false);
            Instruction::call(self, mmu, value as u16);
            16
        } else {
            0
        }
    }

    pub(crate) fn step(&mut self, mmu: &mut Mmu) -> i16 {
        if self.is_halted {
            // TODO -- un-halt when interrupts
            0
        } else {
            let start_pc = self.registers.get_word(CpuRegIndex::PC);

            let opcode = self.fetch_opcode(mmu);
            let mut instruction = Instruction::get_instruction(opcode);
            let args = self.fetch_args(&instruction, mmu);

            if self.debug_print_on && !self.visited.contains(&start_pc)
            {
                Debugger::print_cpu_exec(self, mmu, start_pc, opcode, instruction.mnemonic, args.as_slice());
                self.visited.insert(start_pc);
            }

            let args = args.as_ref();
            instruction.execute(self, mmu, args)
        }
    }

    fn read_byte_at_pc(&mut self, mmu: &mut Mmu) -> u8 {
        let pc = self.registers.get_word(CpuRegIndex::PC);
        let d8 = mmu.read_8(pc);
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

        if instruction.cycles < 0 {
            return args;
        }

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
