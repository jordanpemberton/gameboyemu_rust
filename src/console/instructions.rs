#![allow(unused_variables)]

use crate::console::cpu::Cpu;
use crate::console::mmu::Mmu;
use crate::console::registers::{Flags, RegIndex};

pub(crate) struct Instruction {
    opcode: u16,
    mnemonic: &'static str,
    size: usize,
    time: usize,
    _fn: fn(&Instruction, cpu: &mut Cpu, mmu: &mut Mmu),
}

impl Instruction {
    pub(crate) fn get_instruction(opcode: u16) -> Instruction {
        match opcode {
            0x0000 => Instruction{ opcode, mnemonic: "NOP", size: 1, time: 4, _fn: Instruction::op_0000 },
            0x0002 => Instruction{ opcode, mnemonic: "LD (BC),A", size: 1, time: 8, _fn: Instruction::op_0002 },
            0x0006 => Instruction{ opcode, mnemonic: "LD B,d8", size: 2, time: 8, _fn: Instruction::op_0006 },
            0x0031 => Instruction{ opcode, mnemonic: "LD SP,d16", size: 3, time: 12, _fn: Instruction::op_0031 },
            0x00AF => Instruction{ opcode, mnemonic: "XOR A", size: 1, time: 4, _fn: Instruction::op_00af },
            _ => Instruction{ opcode, mnemonic: "Unimplemented", size: 1, time: 4, _fn: Instruction::unimplemented },
        }
    }

    pub(crate) fn execute(&self, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
        println!("{:#x}: {}", self.opcode, self.mnemonic);
        (self._fn)(&self, cpu, mmu);
        (self.size, self.time)
    }

    /// unimplemented
    fn unimplemented(&self, cpu: &mut Cpu, mmu: &mut Mmu) {
        panic!();
    }

    /// NOP
    fn op_0000(&self, cpu: &mut Cpu, mmu: &mut Mmu) {
    }

    /// LD (BC),A
    fn op_0002(&self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let value = cpu.registers.get_byte(RegIndex::A) as u16;
        cpu.registers.set_word(RegIndex::BC, value);
    }

    /// LD B,d8
    fn op_0006(&self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let address = cpu.pc.wrapping_add(self.size as u16) as usize;
        let value = mmu.read_byte(address);
        cpu.registers.set_byte(RegIndex::B, value);
    }

    /// LD SP,d16
    fn op_0031(&self, cpu: &mut Cpu, mmu: &mut Mmu) {
        let address = cpu.pc.wrapping_add(self.size as u16) as usize;
        let value = mmu.read_word(address);
        cpu.sp = value;
    }

    /// XOR A
    fn op_00af(&self, cpu: &mut Cpu, mmu: &mut Mmu) {
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
}
