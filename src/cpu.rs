#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unreachable_patterns)]
#![allow(unused_variables)]

use crate::instructions::{Instruction, PREFIX_BYTE};
use crate::mmu::Mmu;
use crate::registers::{Registers};

pub(crate) struct Cpu {
    pub(crate) is_halted: bool,
    pub(crate) pc: u16,
    pub(crate) sp: u16,
    pub(crate) registers: Registers,
}

impl Cpu {
    pub(crate) fn new() -> Cpu {
        Cpu {
            is_halted: false,
            pc: 0,
            sp: 0,
            registers: Registers::new(),
        }
    }

    pub(crate) fn step(&mut self, mmu: &mut Mmu) -> usize {
        let mut time = 0;

        if !self.is_halted {
            let opcode = self.fetch_opcode(mmu);
            let instruction = Instruction::get_instruction(opcode);
            time = self.execute_instruction(instruction, mmu);
        }

        time
    }

    fn fetch_opcode(&mut self, mmu: &mut Mmu) -> u16 {
        let opcode: u16;
        let byte = mmu.read_byte(self.pc as usize);

        if byte == PREFIX_BYTE {
            opcode = mmu.read_word(self.pc as usize);
        }
        else {
            opcode = byte as u16;
        }

        opcode
    }

    fn execute_instruction(&mut self, instruction: Instruction, mmu: &mut Mmu) -> usize {
        let (size, time) = instruction.exec(self, mmu);
        self.pc = self.pc.wrapping_add(size as u16);
        time
    }


    /*
    pub(crate) fn execute(&mut self, instruction: Instruction) -> (usize, usize) {
       instruction.exec()
       match instruction {
           Instruction::ADD(target) => self.add(target),

           Instruction::HALT => self.halt(),

           Instruction::JP(conditions) => self.jump(conditions),

           Instruction::LD(load_type, target, source) => self.load(load_type, target, source),

           Instruction::NOP => self.nop(),

           Instruction::POP => self.pop(),

           Instruction::PUSH(target) => self.push(target),

           Instruction::RET(conditions) => self.ret(conditions),

           _ => self.pc
       }
    }
    */

    // TODO Move these
    /*
    fn add(&mut self, target: RegIndex) -> u16 {
        let value = self.registers.get_byte(target);
        let (result, did_overflow) = self.registers.a.overflowing_add(value); // or hl? bc?
        self.registers.set_f(
            Flags {
                zero: result == 0,
                subtract: false,
                half_carry: did_overflow,
                carry: ( self.registers.a & 0xF) + (value & 0xF) > 0xF,
                always: false,
            }
        );
        self.registers.a = result;
        self.pc.wrapping_add(1)
    }

    fn call(&mut self, conditions: Flags) -> u16 {
        let should_jump = self.should_jump(conditions);
        let next_pc = self.pc.wrapping_add(3);
        if should_jump {
            self.push_value(next_pc);
            self.read_next_word()
        } else {
            next_pc
        }
    }

    fn halt(&mut self) -> u16 {
        self.is_halted = true;
        self.pc
    }

    fn should_jump(&mut self, conditions: Flags) -> bool {
        let flags = self.registers.get_flags();
        conditions.always
            || conditions.zero == flags.zero
            || conditions.carry == flags.carry
    }

    fn jump(&mut self, conditions: Flags) -> u16 {
        if self.should_jump(conditions) {
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
                    // RegIndex::D8 => self.read_next_byte(),
                    // RegIndex::HLI => self.bus.read_byte(self.registers.get_word(RegIndex::H)),
                    _ => { panic!("TODO: implement other sources") }
                };
                match target {
                    RegIndex::A => self.registers.a = source_value,
                    // RegIndex::HLI => self.bus.write_byte(self.registers.get_word(RegIndex::H), source_value),
                    _ => { panic!("TODO: implement other targets") }
                };
                match source {
                    // RegIndex::D8 => self.pc.wrapping_add(2),
                    _ => self.pc.wrapping_add(1),
                }
            }
            _ => { panic!("TODO: implement other load types") }
        }
    }

    fn nop(&mut self) -> u16 {
        self.sp.wrapping_add(1)
    }

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
        let value = self.registers.get_word(target);
        self.push_value(value);
        self.pc.wrapping_add(1)
    }

    fn push_value(&mut self, value: u16) {
        // Write most signif byte into memory
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, ((value & 0xFF00) >> 8) as u8);
        // Write least signif byte into memory
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, (value & 0xFF) as u8);
    }

    fn ret(&mut self, conditions: Flags) -> u16 {
        if self.should_jump(conditions) {
            self.pop()
        } else {
            self.pc.wrapping_add(1)
        }
    }
    */
}
