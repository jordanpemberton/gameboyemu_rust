#![allow(unused_variables)]

use crate::console::cpu::Cpu;
use crate::console::mmu::Mmu;
use crate::console::registers::RegIndex;

pub(crate) struct Instruction {
    opcode: u16,
    size: usize,
    time: usize,
    _fn: fn(&Instruction, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize),
}

impl Instruction {
    pub(crate) fn get_instruction(opcode: u16) -> Instruction {
        match opcode {
            0x0000 => Instruction{ opcode, size: 1, time: 4, _fn: Instruction::op_0000 },
            0x0002 => Instruction{ opcode, size: 1, time: 8, _fn: Instruction::op_0002 },
            0x0006 => Instruction{ opcode, size: 2, time: 8, _fn: Instruction::op_0006 },
            _ => Instruction{ opcode, size: 1, time: 4, _fn: Instruction::op_0000 },
        }
    }

    pub(crate) fn execute(&self, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
        (self._fn)(&self, cpu, mmu)
    }

    /// nop
    fn op_0000(&self, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
        (self.size, self.time)
    }

    /// ld (bc),a
    fn op_0002(&self, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
        let value = cpu.registers.get_byte(RegIndex::A) as u16;
        cpu.registers.set_word(RegIndex::BC, value);
        (self.size, self.time)
    }

    /// ld b,d8
    fn op_0006(&self, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
        let address = cpu.pc.wrapping_add(self.size as u16) as usize;
        let value = mmu.read_byte(address);
        cpu.registers.set_byte(RegIndex::B, value);
        (self.size, self.time)
    }
}
