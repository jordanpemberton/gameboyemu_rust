use crate::registers::{RegIndex};

pub(crate) enum LoadType {
    Byte
}

pub(crate) enum Instruction {
    ADD(RegIndex),
    JP(bool, bool, bool),
    LD(LoadType, RegIndex, RegIndex),
    POP(),
    PUSH(RegIndex),
    RLC(RegIndex)
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

impl Instruction {
    pub(crate) fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        if prefixed {
            Instruction::from_byte_prefixed(byte)
        } else {
            Instruction::from_byte_unprefixed(byte)
        }
    }

    fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::RLC(RegIndex::B)),
            _ => /* TODO: Add mapping for rest of instructions */ None
        }
    }

    fn from_byte_unprefixed(byte: u8) -> Option<Instruction> {
        match byte {
            // 0x02 => Some(Instruction::INC(IncDecTarget::BC)), // 0x03?
            // 0x09 => Some(Instruction::ADD(RegIndex::)) // ?
            _ => /* TODO: Add mapping for rest of instructions */ None
        }
    }
}
