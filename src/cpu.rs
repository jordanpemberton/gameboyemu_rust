/* Following: https://rylev.github.io/DMG-01/public/book/ */

#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unreachable_patterns)]

use crate::instructions::{Instruction, LoadType};
use crate::registers::{Flags, RegIndex, Registers};

pub(crate) struct MemoryBus {
    pub(crate) memory: [u8; 0xFFFF]
}

impl MemoryBus {
    pub(crate) fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub(crate) fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
}

pub(crate) struct CPU {
    pub(crate) pc: u16,
    pub(crate) sp: u16,
    pub(crate) registers: Registers,
    pub(crate) bus: MemoryBus,
}

impl CPU {
    pub(crate) fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);
        let is_prefixed = instruction_byte == 0xCB;
        if is_prefixed {
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }

        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte, is_prefixed) {
            self.execute(instruction)
        } else {
            let description = format!("0x{}{:x}", if is_prefixed { "cb" } else { "" }, instruction_byte);
            panic!("Unknown instruction found for: {}", description)
        };

        self.pc = next_pc;
    }

    pub(crate) fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::ADD(target) => self.add(target),

            Instruction::JP(is_zero, is_carry, is_always) => self.jump(is_zero, is_carry, is_always),

            Instruction::LD(load_type, target, source) => self.load(load_type, target, source),

            Instruction::POP() => self.pop(),

            Instruction::PUSH(target) => self.push(target),

            _ => { self.pc }
        }
    }

    fn read_next_byte(&mut self) -> u8 {
        // TODO
        0
    }

    fn add(&mut self, target: RegIndex) -> u16 {
        let value = self.registers.get_8b(target);
        let (result, did_overflow) = self.registers.a.overflowing_add(value); // or hl? bc?
        self.registers.set_f(
            Flags {
                zero: result == 0,
                subtract: false,
                half_carry: did_overflow,
                carry: ( self.registers.a & 0xF) + (value & 0xF) > 0xF,
            }
        );
        self.registers.a = result;
        self.pc.wrapping_add(1)
    }

    fn jump(&mut self, is_zero: bool, is_carry: bool, is_always: bool) -> u16 {
        let flags = self.registers.get_flags();
        let should_jump = is_always || is_zero == flags.zero || is_carry == flags.carry;
        if should_jump {
            // Gameboy is little endian. Read pc + 2 as most signif, pc + 1 as least signif.
            let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
            let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
            (most_significant_byte << 8) | least_significant_byte
        } else {
            // If we don't jump we still need to increment pc by 3 (width of jmp instruction).
            self.pc.wrapping_add(3)
        }
    }

    fn load(&mut self, load_type: LoadType, target: RegIndex, source: RegIndex) -> u16 {
        match load_type {
            LoadType::Byte => {
                let source_value = match source {
                    RegIndex::A => self.registers.a,
                    RegIndex::D8 => self.read_next_byte(),
                    // RegIndex::HLI => self.bus.read_byte(self.registers.get_16b(RegIndex::H)),
                    _ => { panic!("TODO: implement other sources") }
                };
                match target {
                    RegIndex::A => self.registers.a = source_value,
                    // RegIndex::HLI => self.bus.write_byte(self.registers.get_16b(RegIndex::H), source_value),
                    _ => { panic!("TODO: implement other targets") }
                };
                match source {
                    RegIndex::D8 => self.pc.wrapping_add(2),
                    _ => self.pc.wrapping_add(1),
                }
            }
            _ => { panic!("TODO: implement other load types") }
        }
    }

    fn nop() { }

    fn pop(&mut self) -> u16 {
        // Read least signif byte from memory
        let lsb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
        // Read most signif byte from memory
        let msb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
        // Combine
        (msb << 8) | lsb
    }

    fn push(&mut self, target: RegIndex) -> u16 {
        let value = self.registers.get_16b(target);

        // Write most signif byte into memory
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, ((value & 0xFF00) >> 8) as u8);
        // Write least signif byte into memory
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, (value & 0xFF) as u8);
        self.pc.wrapping_add(1)
    }

    fn rlc(&mut self, target: RegIndex) -> u16 {
        // TODO
        0
    }
}
