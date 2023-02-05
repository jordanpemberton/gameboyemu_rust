/* Following: https://rylev.github.io/DMG-01/public/book/ */

#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unreachable_patterns)]

const FLAG_ZERO_BYTE: u8 = 7;
const FLAG_SUBTRACT_BYTE: u8 = 6;
const FLAG_HALF_CARRY_BYTE: u8 = 5;
const FLAG_CARRY_BYTE: u8 = 4;

pub(crate) enum RegIndex {
    A, B, C, D, E, F, H, L, D8,
}

pub(crate) struct Flags {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool,
}

pub(crate) struct Registers {
    pub(crate) a: u8,
    pub(crate) b: u8,
    pub(crate) c: u8,
    pub(crate) d: u8,
    pub(crate) e: u8,
    pub(crate) f: u8,
    pub(crate) h: u8,
    pub(crate) l: u8,
}

impl Registers {
    fn get_8b(&self, index: RegIndex) -> u8 {
        match index {
            RegIndex::A => self.a,
            RegIndex::B => self.b,
            RegIndex::C => self.c,
            RegIndex::D => self.d,
            RegIndex::E => self.e,
            RegIndex::F => self.f,
            RegIndex::H => self.h,
            RegIndex::L => self.l,
            _ => 0
        }
    }

    fn get_16b(&self, index: RegIndex) -> u16 {
        match index {
            RegIndex::A => self.to_16b(self.a, self.f),
            RegIndex::B => self.to_16b(self.b, self.c),
            RegIndex::D => self.to_16b(self.d, self.e),
            RegIndex::H => self.to_16b(self.h, self.l),
            _ => 0
        }
    }

    fn to_16b(&self, reg1: u8, reg2: u8) -> u16 {
        (reg1 as u16) << 8 | (reg2 as u16)
    }

    fn get_flags(&mut self) -> Flags {
        Flags {
            zero: ((self.f >> FLAG_ZERO_BYTE) & 0b1) != 0,
            subtract: ((self.f >> FLAG_SUBTRACT_BYTE) & 0b1) != 0,
            half_carry: ((self.f >> FLAG_HALF_CARRY_BYTE) & 0b1) != 0,
            carry: ((self.f >> FLAG_CARRY_BYTE) & 0b1) != 0
        }
    }

    fn set_f(&mut self, flags: Flags) {
        self.f =
            (if flags.zero { 1 } else { 0 }) << FLAG_ZERO_BYTE |
            (if flags.subtract { 1 } else { 0 }) << FLAG_SUBTRACT_BYTE |
            (if flags.half_carry { 1 } else { 0 }) << FLAG_HALF_CARRY_BYTE |
            (if flags.carry { 1 } else { 0 }) << FLAG_CARRY_BYTE;
    }

    fn set_8b(&mut self, index: RegIndex, value: u8) {
        match index {
            RegIndex::A => { self.a = value; }
            RegIndex::B => { self.b = value; }
            RegIndex::C => { self.c = value; }
            RegIndex::D => { self.d = value; }
            RegIndex::E => { self.e = value; }
            RegIndex::F => { self.f = value; }
            RegIndex::H => { self.h = value; }
            RegIndex::L => { self.l = value; }
            _ => { }
        }
    }

    fn set_16b(&mut self, index: RegIndex, value: u16) {
        match index {
            RegIndex::A => {
                self.a = ((value & 0xFF00) >> 8) as u8;
                self.f = (value & 0xFF) as u8;
            }
            RegIndex::B => {
                self.b = ((value & 0xFF00) >> 8) as u8;
                self.c = (value & 0xFF) as u8;
            }
            RegIndex::D => {
                self.d = ((value & 0xFF00) >> 8) as u8;
                self.e = (value & 0xFF) as u8;
            }
            RegIndex::H => {
                self.h = ((value & 0xFF00) >> 8) as u8;
                self.l = (value & 0xFF) as u8;
            }
            _ => ()
        }
    }
}

pub(crate) enum LoadType {
    Byte
}

pub(crate) struct MemoryBus {
    pub(crate) memory: [u8; 0xFFFF]
}

impl MemoryBus {
    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}

pub(crate) enum Instruction {
    ADD(RegIndex),
    JP(bool, bool, bool),
    LD(LoadType, RegIndex, RegIndex)
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
    fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        if prefixed {
            Instruction::from_byte_prefixed(byte)
        } else {
            Instruction::from_byte_not_prefixed(byte)
        }
    }

    fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            // 0x00 => Some(Instruction::RLC(PrefixTarget::B)),
            _ => /* TODO: Add mapping for rest of instructions */ None
        }
    }

    fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            // 0x02 => Some(Instruction::INC(IncDecTarget::BC)),
            _ => /* TODO: Add mapping for rest of instructions */ None
        }
    }
}

pub(crate) struct CPU {
    pub(crate) pc: u16,
    pub(crate) sc: u16,
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

            /* TODO: support more instructions */
            _ => { self.pc }
        }
    }

    fn nop() { }

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
                    // RegIndex::D8 => self.read_next_byte(),
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
}
