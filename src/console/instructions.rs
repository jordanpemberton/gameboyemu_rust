#![allow(unused_variables)]

use crate::console::alu;
use crate::console::cpu::Cpu;
use crate::console::mmu::Mmu;
use crate::console::registers::{Flags, RegIndex};

pub(crate) struct Instruction {
    opcode: u16,
    mnemonic: &'static str,
    size: u16,
    pub(crate) cycles: u16,
    _fn: fn(&mut Instruction, cpu: &mut Cpu, mmu: &mut Mmu),
}

impl Instruction {
    pub(crate) fn get_instruction(opcode: u16) -> Instruction {
        match opcode {
            0x0000 => Instruction{ opcode, mnemonic: "NOP", size: 1, cycles: 4, _fn: Instruction::op_0000 },
            0x0002 => Instruction{ opcode, mnemonic: "LD (BC),A", size: 1, cycles: 8, _fn: Instruction::op_0002 },
            0x0006 => Instruction{ opcode, mnemonic: "LD B,d8", size: 2, cycles: 8, _fn: Instruction::op_0006 },
            0x000C => Instruction{ opcode, mnemonic: "INC C", size: 1, cycles: 4, _fn: Instruction::op_000c },
            0x000E => Instruction{ opcode, mnemonic: "LD C,d8", size: 2, cycles: 8, _fn: Instruction::op_000e },
            0x0020 => Instruction{ opcode, mnemonic: "JR NZ,r8", size: 2, cycles: 8, _fn: Instruction::op_0020 },
            0x0021 => Instruction{ opcode, mnemonic: "LD HL,d16", size: 3, cycles: 12, _fn: Instruction::op_0021 },
            0x0031 => Instruction{ opcode, mnemonic: "LD SP,d16", size: 3, cycles: 12, _fn: Instruction::op_0031 },
            0x0032 => Instruction{ opcode, mnemonic: "LD (HL-),A", size: 1, cycles: 8, _fn: Instruction::op_0032 },
            0x003E => Instruction{ opcode, mnemonic: "LD A,d8", size: 2, cycles: 8, _fn: Instruction::op_003e },
            0x0077 => Instruction{ opcode, mnemonic: "LD (HL),A", size: 1, cycles: 8, _fn: Instruction::op_0077 },
            0x00AF => Instruction{ opcode, mnemonic: "XOR A", size: 1, cycles: 4, _fn: Instruction::op_00af },
            0x00E0 => Instruction{ opcode, mnemonic: "LDH (a8), A", size: 2, cycles: 12, _fn: Instruction::op_00e0 },
            0x00E2 => Instruction{ opcode, mnemonic: "LD ($FF00+C),A", size: 1, cycles: 8, _fn: Instruction::op_00e2 },
            0xCB7C => Instruction{ opcode, mnemonic: "BIT 7,H", size: 2, cycles: 8, _fn: Instruction::op_cb7c },
            _ => Instruction{ opcode, mnemonic: "Unimplemented", size: 1, cycles: 4, _fn: Instruction::unimplemented },
        }
    }

    pub(crate) fn execute(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) -> u16 {
        println!("{:#06X}:\t{}", self.opcode, self.mnemonic);
        (self._fn)(self, cpu, mmu);
        self.cycles
    }

    fn unimplemented(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        panic!("Opcode {:#06X}(\"{}\") is unimplemented.", self.opcode, self.mnemonic);
    }

    /// UN-PREFIXED

    /// NOP
    /// 1  4
    /// - - - -
    fn op_0000(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
    }

    /// LD (BC),A
    /// 1  8
    /// - - - -
    fn op_0002(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let target_address = cpu.registers.get_word(RegIndex::BC);
        let value = cpu.registers.get_byte(RegIndex::A);
        mmu.load_byte(target_address, value);
    }

    /// LD B,d8
    /// 2  8
    /// - - - -
    fn op_0006(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let d8 = mmu.read_byte(cpu.pc);
        cpu.increment_pc(1);
        cpu.registers.set_byte(RegIndex::B, d8);
    }

    /// INC C
    /// 1  4
    /// Z 0 H -
    fn op_000c(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let value = cpu.registers.get_byte(RegIndex::C).wrapping_add(1);
        cpu.registers.set_byte(RegIndex::C, value);
        let mut flags = cpu.registers.get_flags();
        flags.zero = true;
        flags.subtract = false;
        flags.half_carry = true;
        cpu.registers.set_f(flags);
    }

    /// LD C,d8
    /// 2  8
    /// - - - -
    fn op_000e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let d8 = mmu.read_byte(cpu.pc);
        cpu.increment_pc(1);
        cpu.registers.set_byte(RegIndex::C, d8);
    }

    /// JR NZ,r8
    /// 2  12/8
    /// - - - -
    fn op_0020(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let jump = alu::signed(mmu.read_byte(cpu.pc) as u8);
        cpu.increment_pc(1);
        let should_jump = !(cpu.registers.get_flags().zero);
        if should_jump {
            cpu.increment_pc(jump);
            self.cycles += 4;
        }
    }

    /// LD HL,d16
    /// 3  12
    /// - - - -
    fn op_0021(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let d16 = mmu.read_word(cpu.pc);
        cpu.increment_pc(2);
        cpu.registers.set_word(RegIndex::HL, d16);
    }

    /// LD SP,d16
    /// 3  12
    /// - - - -
    fn op_0031(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let d16 = mmu.read_word(cpu.pc);
        cpu.increment_pc(2);
        cpu.sp = d16;
    }

    /// LD (HL-),A
    /// 1  8
    /// - - - -
    fn op_0032(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let target_address = cpu.registers.get_word(RegIndex::HL);
        let value = cpu.registers.get_byte(RegIndex::A);
        mmu.load_byte(target_address, value);
        cpu.registers.set_word(RegIndex::HL, target_address.wrapping_sub(1));
    }

    /// LD A,d8
    /// 2  8
    /// - - - -
    fn op_003e(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let value = mmu.read_byte(cpu.pc);
        cpu.increment_pc(1);
        cpu.registers.set_byte(RegIndex::A, value);
    }

    /// LD (HL),A
    /// 1  8
    /// - - - -
    fn op_0077(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let target_address = cpu.registers.get_word(RegIndex::HL);
        let value = cpu.registers.get_byte(RegIndex::A);
        mmu.load_byte(target_address, value);
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
        };
        cpu.registers.set_f(flags);
    }

    /// LDH (a8),A
    /// 2  12
    /// - - - -
    fn op_00e0(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let a8 = mmu.read_byte(cpu.pc);
        cpu.increment_pc(1);
        let target_address = 0xFF00 + (a8 as u16);
        let value = cpu.registers.get_byte(RegIndex::A);
        mmu.load_byte(target_address, value);
    }

    /// LD ($FF00+C),A
    /// 1  8
    /// - - - -
    fn op_00e2(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let target_address = 0xFF00 + (cpu.registers.get_byte(RegIndex::C) as u16);
        let value = cpu.registers.get_byte(RegIndex::A);
        mmu.load_byte(target_address, value);
    }

    /// CB PREFIXED

    /// BIT 7,H
    /// 2  8
    /// Z 0 1 -
    fn op_cb7c(&mut self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let value = cpu.registers.get_byte(RegIndex::H);
        let mut flags = cpu.registers.get_flags();
        flags.zero = value & (1 << 7) == 0;
        flags.subtract = false;
        flags.half_carry = true;
        cpu.registers.set_f(flags);
    }
}
