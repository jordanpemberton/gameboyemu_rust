// #![allow(dead_code)]
// #![allow(non_snake_case)]
// #![allow(unreachable_patterns)]
// #![allow(unused_variables)]

use crate::console::instructions::{Instruction};
use crate::console::mmu::Mmu;
use crate::console::cpu_registers::{CpuRegIndex, CpuRegisters};
use crate::console::interrupts::Interrupts;

pub(crate) const PREFIX_BYTE: u8 = 0xCB;

pub(crate) struct Cpu {
    is_halted: bool,
    pub(crate) registers: CpuRegisters,
    pub(crate) interrupts: Interrupts,
    visited: Vec<u16>,
    debug_print: bool,
}

impl Cpu {
    pub(crate) fn new(mmu: &mut Mmu, debug_print: bool) -> Cpu {
        Cpu {
            is_halted: false,
            registers: CpuRegisters::new(),
            interrupts: Interrupts::new(mmu),
            visited: vec![],
            debug_print,
        }
    }

    pub(crate) fn halt(&mut self) {
        self.is_halted = true;
    }

    pub(crate) fn check_interrupts(&mut self) -> i16 {
        // TODO
        self.is_halted = false;
        if self.interrupts.ime {
            16
        } else {
            let value = self.interrupts.get_interrupt_value();
            if value > 0 {
                self.interrupts.ime = false;
            }
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

            if self.debug_print {
                // to check against blargg test logging:
                // println!(
                //     "A: {:02X} F: {:02X} B: {:02X} C: {:02X} \
                //     D: {:02X} E: {:02X} H: {:02X} L: {:02X} \
                //     SP: {:04X} PC: {:02X}:{:04X} ({:02X} {:02X} {:02X} {:02X})",
                //     self.registers.get_byte(CpuRegIndex::A),
                //     self.registers.get_byte(CpuRegIndex::F),
                //     self.registers.get_byte(CpuRegIndex::B),
                //     self.registers.get_byte(CpuRegIndex::C),
                //     self.registers.get_byte(CpuRegIndex::D),
                //     self.registers.get_byte(CpuRegIndex::E),
                //     self.registers.get_byte(CpuRegIndex::H),
                //     self.registers.get_byte(CpuRegIndex::L),
                //     self.registers.get_word(CpuRegIndex::SP),
                //     0, // ?
                //     start_pc,
                //     mmu.read_byte(start_pc),
                //     mmu.read_byte(start_pc + 1),
                //     mmu.read_byte(start_pc + 2),
                //     mmu.read_byte(start_pc + 3),
                // );
                // example: "A: 0C F: 40 B: 06 C: FF D: C8 E: 46 H: 8A L: 74 SP: DFF7 PC: 00:C762 (13 A9 22 22)"

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
