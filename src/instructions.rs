#![allow(unused_variables)]

use crate::cpu::Cpu;
use crate::mmu::Mmu;
use crate::registers::{RegIndex};

pub(crate) const PREFIX_BYTE: u8 = 0xCB;

pub(crate) enum LoadType {
    Byte
}

/*
pub(crate) enum Instruction {
    ADD(RegIndex),
    HALT,
    JP(Flags),
    LD(LoadType, RegIndex, RegIndex),
    NOP,
    POP,
    PUSH(RegIndex),
    RET(Flags),
    RLC(RegIndex)
}
*/

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
    size: usize,
    time: usize,
    _fn: fn(&Instruction, cpu: &mut Cpu, mmu: &mut Mmu, args: InstrArgs) -> (usize, usize),
    args: InstrArgs,
}

impl Instruction {
    pub(crate) fn get_instruction(opcode: u16) -> Instruction {
        match opcode {
            0x0000 => Instruction{ size: 1, time: 4, _fn: Instruction::nop, args: InstrArgs::new() },
            0x0002 => Instruction{ size: 1, time: 8, _fn: Instruction::ld, args: InstrArgs { source: RegIndex::A, target: RegIndex::BC } },
            _ => Instruction{ size: 1, time: 4, _fn: Instruction::nop, args: InstrArgs::new() },
        }
    }

    pub(crate) fn exec(&self, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
        (self._fn)(self, cpu, mmu, self.args)
    }


    fn nop(&self, cpu: &mut Cpu, mmu: &mut Mmu, args: InstrArgs) -> (usize, usize) {
        (self.size, self.time)
    }

    fn ld(&self, cpu: &mut Cpu, mmu: &mut Mmu, args: InstrArgs) -> (usize, usize) {
        let value = cpu.registers.get_byte(args.source) as u16;
        cpu.registers.set_word(args.target, value);

        (self.size, self.time)
    }
}

/*
ADDHL (add to HL) - just like ADD except that the target is added to the HL register
ADC (add with carry) - just like ADD except that the value of the carry flag is also added to the number
SUB (subtract) - subtract the value stored in a specific register with the value in the A register
SBC (subtract with carry) - just like ADD except that the value of the carry flag is also subtracted from the number
AND (logical and) - do a bitwise and on the value in a specific register and the value in the A register
OR (logical or) - do a bitwise or on the value in a specific register and the value in the A register
XOR (logical xor) - do a bitwise xor on the value in a specific register and the value in the A register
CP (compare) - just like SUB except the result of the subtraction is not stored back into A
INC (increment) - increment the value in a specific register by 1
DEC (decrement) - decrement the value in a specific register by 1
CCF (complement carry flag) - toggle the value of the carry flag
SCF (set carry flag) - set the carry flag to true
RRA (rotate right A register) - bit rotate A register right through the carry flag
RLA (rotate left A register) - bit rotate A register left through the carry flag
RRCA (rotate right A register) - bit rotate A register right (not through the carry flag)
RRLA (rotate left A register) - bit rotate A register left (not through the carry flag)
CPL (complement) - toggle every bit of the A register
BIT (bit test) - test to see if a specific bit of a specific register is set
RESET (bit reset) - set a specific bit of a specific register to 0
SET (bit set) - set a specific bit of a specific register to 1
SRL (shift right logical) - bit shift a specific register right by 1
RR (rotate right) - bit rotate a specific register right by 1 through the carry flag
RL (rotate left) - bit rotate a specific register left by 1 through the carry flag
RRC (rorate right) - bit rotate a specific register right by 1 (not through the carry flag)
RLC (rorate left) - bit rotate a specific register left by 1 (not through the carry flag)
SRA (shift right arithmetic) - arithmetic shift a specific register right by 1
SLA (shift left arithmetic) - arithmetic shift a specific register left by 1
SWAP (swap nibbles) - switch upper and lower nibble of a specific register
*/
