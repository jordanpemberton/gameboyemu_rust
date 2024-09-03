use std::collections::HashSet;
use crate::console::instruction::{Instruction};
use crate::console::mmu::{Caller, Mmu};
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
    pub(crate) fn new(debug_print: bool) -> Cpu {
        Cpu {
            is_halted: false,
            registers: CpuRegisters::new(),
            interrupts: Interrupts::new(),
            visited: HashSet::from([]),
            debug_print_on: debug_print,
        }
    }

    pub(crate) fn handle_interrupts(&mut self, mmu: &mut Mmu) -> i16 {
        let mut cycles = 0;

        if !self.interrupts.ime {
            if self.is_halted && self.interrupts.peek_requested(mmu) {
                self.is_halted = false;
            }
        } else {
            let value = self.interrupts.poll(mmu);
            if value > 0 {
                if self.debug_print_on {
                    println!("{:#06X}\t{} INTERRUPT! -- CALL {:#06X}",
                        self.registers.get_word(CpuRegIndex::PC),
                        match value {
                            0x40 => "VBLANK",
                            0x48 => "LCDSTAT",
                            0x50 => "TIMER",
                            0x58 => "SERIAL",
                            0x60 => "JOYPAD",
                            _ => "?",
                        },
                        value);
                }
                self.interrupts.ime = false;
                Instruction::call(self, mmu, value as u16);
                cycles = 20; // 2M wait + 2M push + 1M jump
            }
        }
        cycles
    }

    pub(crate) fn step(&mut self, mmu: &mut Mmu) -> i16 {
        if self.is_halted {
            4
        } else {
            self.execute_instruction(mmu)
        }
    }

    fn read_byte_at_pc(&mut self, mmu: &mut Mmu) -> u8 {
        let pc = self.registers.get_word(CpuRegIndex::PC);
        let d8 = mmu.read_8(pc, Caller::CPU);
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

    fn execute_instruction(&mut self, mmu: &mut Mmu) -> i16 {
        let start_pc = self.registers.get_word(CpuRegIndex::PC);

        let opcode = self.fetch_opcode(mmu);
        let mut instruction = Instruction::get_instruction(opcode);
        let args = self.fetch_args(&instruction, mmu);

        if self.debug_print_on {
            Debugger::print_cpu_exec(self, mmu, start_pc, opcode, instruction.mnemonic, args.as_slice());
        }

        let args = args.as_ref();
        instruction.execute(self, mmu, args)
    }
}
