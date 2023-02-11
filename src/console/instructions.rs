#![allow(unused_variables)]

use crate::console::cpu::Cpu;
use crate::console::mmu::Mmu;
use crate::console::registers::RegIndex;

pub(crate) const PREFIX_BYTE: u8 = 0xCB;

#[derive(Copy, Clone)]
pub(crate) struct InstrArgs {
    pub(crate) source: RegIndex,
    pub(crate) target: RegIndex,
}

impl InstrArgs {
    pub(crate) fn new() -> Self {
        Self {
            source: RegIndex::A,
            target: RegIndex::A,
        }
    }
}

pub(crate) struct Instruction {
    opcode: u16,
    size: usize,
    time: usize,
    _fn: fn(&Instruction, args: InstrArgs, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize),
    args: InstrArgs,
}

impl Instruction {
    pub(crate) fn get_instruction(opcode: u16) -> Instruction {
        match opcode {
            0x0000 => Instruction{ opcode, size: 1, time: 4, _fn: Instruction::nop, args: InstrArgs::new() },
            0x0002 => Instruction{ opcode, size: 1, time: 8, _fn: Instruction::ld, args: InstrArgs { source: RegIndex::A, target: RegIndex::BC } },
            _ => Instruction{ opcode, size: 1, time: 4, _fn: Instruction::nop, args: InstrArgs::new() },
        }
    }

    pub(crate) fn execute(&self, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
        (self._fn)(&self, self.args, cpu, mmu)
    }

    fn nop(&self, args: InstrArgs, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
        (self.size, self.time)
    }

    fn ld(&self, args: InstrArgs, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
        let value = cpu.registers.get_byte(args.source) as u16;
        cpu.registers.set_word(args.target, value);
        (self.size, self.time)
    }
}
