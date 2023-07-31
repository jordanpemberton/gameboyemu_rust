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
const OAM_ADDRESS: u16 = 0xFE00;

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

    pub(crate) fn check_interrupts(&mut self, mmu: &mut Mmu) -> i16 {
        let mut cycles = 0;
        if !self.interrupts.ime {
            if self.is_halted && self.interrupts.peek_if_has_requests() {
                self.is_halted = false;
            }
        } else {
            let value = self.interrupts.poll(mmu);
            if value > 0 {
                Instruction::call(self, mmu, value as u16);
                self.interrupts.ime = false;
            }
            cycles = 16;
        }
        cycles
    }

    pub(crate) fn step(&mut self, mmu: &mut Mmu) -> i16 {
        if self.is_halted {
            4
        } else {
            self.oam_dma(mmu);
            self.execute_instruction(mmu)
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

    fn oam_dma(&mut self, mmu: &mut Mmu) {
        if let Some(mut src_address) = mmu.oam_dma_source_address {
            let value = mmu.read_8(src_address);
            mmu.write_8(OAM_ADDRESS | (src_address & 0x00FF), value);
            src_address += 1;
            mmu.oam_dma_source_address = if (src_address & 0xFF) > 0x9F {
                None
            } else {
                Option::from(src_address)
            };
        }
    }

    fn execute_instruction(&mut self, mmu: &mut Mmu) -> i16 {
        let start_pc = self.registers.get_word(CpuRegIndex::PC);

        let opcode = self.fetch_opcode(mmu);
        let mut instruction = Instruction::get_instruction(opcode);
        let args = self.fetch_args(&instruction, mmu);

        if self.debug_print_on
        {
            // Option 1, Example: "0xC375	0x00CA	JP Z,a16      	0xB9	0xC1"
            if !self.visited.contains(&start_pc) { // 0x234E == DrMario PollJoypad
                Debugger::print_cpu_exec(self, mmu, start_pc, opcode, instruction.mnemonic, args.as_slice());
                self.visited.insert(start_pc);
            }

            // Option 2, Example: "A: 0C F: 40 B: 06 C: FF D: C8 E: 46 H: 8A L: 74 SP: DFF7 PC: 00:C762 (13 A9 22 22)"
            // Debugger::print_cpu_exec_log(self, mmu, start_pc);
        }

        let args = args.as_ref();
        instruction.execute(self, mmu, args)
    }
}
