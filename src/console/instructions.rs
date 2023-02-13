#![allow(unused_variables)]

use crate::console::alu;
use crate::console::cpu::Cpu;
use crate::console::mmu::Mmu;
use crate::console::registers::{Flags, RegIndex};

pub(crate) struct Instruction {
    opcode: u16,
    mnemonic: &'static str,
    size: usize,
    time: usize,
    _fn: fn(&mut Instruction, cpu: &mut Cpu, mmu: &mut Mmu),
}

impl Instruction {
    pub(crate) fn get_instruction(opcode: u16) -> Instruction {
        match opcode {
            0x0000 => Instruction{ opcode, mnemonic: "NOP", size: 1, time: 4, _fn: Instruction::op_0000 },
            0x0002 => Instruction{ opcode, mnemonic: "LD (BC),A", size: 1, time: 8, _fn: Instruction::op_0002 },
            0x0006 => Instruction{ opcode, mnemonic: "LD B,d8", size: 2, time: 8, _fn: Instruction::op_0006 },
            0x0020 => Instruction{ opcode, mnemonic: "JR NZ,r8", size: 2, time: 8, _fn: Instruction::op_0020 },
            0x0021 => Instruction{ opcode, mnemonic: "LD HL,d16", size: 3, time: 12, _fn: Instruction::op_0021 },
            0x0031 => Instruction{ opcode, mnemonic: "LD SP,d16", size: 3, time: 12, _fn: Instruction::op_0031 },
            0x0032 => Instruction{ opcode, mnemonic: "LD (HL-),A", size: 1, time: 8, _fn: Instruction::op_0032 },
            0x00AF => Instruction{ opcode, mnemonic: "XOR A", size: 1, time: 4, _fn: Instruction::op_00af },
            0xCB7C => Instruction{ opcode, mnemonic: "BIT 7,H", size: 2, time: 8, _fn: Instruction::op_cb7c },
            _ => Instruction{ opcode, mnemonic: "Unimplemented", size: 1, time: 4, _fn: Instruction::unimplemented },
        }
    }

    pub(crate) fn execute(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
        println!("{:#x}:\t{}", self.opcode, self.mnemonic);
        (self._fn)(self, cpu, mmu);
        (self.size, self.time)
    }

    fn unimplemented(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        panic!();
    }

    /// NOP
    /// 1  4
    /// - - - -
    fn op_0000(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
    }

    /// LD (BC),A
    /// 1  8
    /// - - - -
    fn op_0002(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let value = cpu.registers.get_byte(RegIndex::A) as u16;
        cpu.registers.set_word(RegIndex::BC, value);
    }

    /// LD B,d8
    /// 2  8
    /// - - - -
    fn op_0006(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let address = cpu.pc.wrapping_add(self.size as u16) as usize;
        let value = mmu.read_byte(address);
        cpu.registers.set_byte(RegIndex::B, value);
    }

    /// JR NZ,r8
    /// 2  12/8
    /// - - - -
    fn op_0020(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let flag = cpu.registers.get_flags();
        if !flag.zero {
            let value = cpu.pc.wrapping_add(self.size as u16) as u8;
            cpu.pc = cpu.pc.wrapping_add(alu::signed(value));
            self.time += 4;
        }
    }

    /// LD HL,d16
    /// 3  12
    /// - - - -
    fn op_0021(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let address = cpu.pc.wrapping_add(self.size as u16) as usize;
        let value = mmu.read_word(address);
        cpu.registers.set_word(RegIndex::HL, value);
    }

    /// LD SP,d16
    /// 3  12
    /// - - - -
    fn op_0031(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let address = cpu.pc.wrapping_add(self.size as u16) as usize;
        let value = mmu.read_word(address);
        cpu.sp = value;
    }

    /// LD (HL-),A
    /// 1  8
    /// - - - -
    fn op_0032(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let mut value = cpu.registers.get_byte(RegIndex::A);
        cpu.registers.set_word(RegIndex::HL, value as u16);
        value = (cpu.registers.get_word(RegIndex::HL) as u8).wrapping_sub(1); // ?
        cpu.registers.set_word(RegIndex::HL, value as u16);
    }

    /// XOR A
    /// 1  4
    /// Z 0 0 0
    fn op_00af(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let mut value = cpu.registers.get_byte(RegIndex::A);
        value ^= value;
        cpu.registers.set_byte(RegIndex::A, value);
        let z = cpu.registers.get_byte(RegIndex::A) == 0;
        let flags = Flags {
            zero: z,
            subtract: false,
            half_carry: false,
            carry: false,
            always: false,
        };
    }

    /// BIT 7,H
    /// 2  8
    /// Z 0 1 -
    fn op_cb7c(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let shift = 7;
        let value = cpu.registers.get_byte(RegIndex::H);
        let mut flags = cpu.registers.get_flags();
        flags.zero = value & (1 << shift) == 0;
        flags.subtract = false;
        flags.half_carry = true;
        cpu.registers.set_f(flags);
    }
}
