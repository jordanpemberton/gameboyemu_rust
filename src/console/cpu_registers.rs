use std::fmt::{Display, Formatter};

const FLAG_ZERO_BYTE: u8 = 7;
const FLAG_SUBTRACT_BYTE: u8 = 6;
const FLAG_HALF_CARRY_BYTE: u8 = 5;
const FLAG_CARRY_BYTE: u8 = 4;

pub(crate) struct Flags {
    pub(crate) zero: bool,          // Z
    pub(crate) subtract: bool,      // N
    pub(crate) half_carry: bool,    // H
    pub(crate) carry: bool,         // C
}

#[derive(Copy, Clone)]
pub(crate) enum CpuRegIndex {
    A, B, C, D, E, F, H, L,
    AF, BC, DE, HL,
    PC, SP
}

pub(crate) struct CpuRegisters {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    pc: u16,
    sp: u16,
}

impl CpuRegisters {
    pub(crate) fn new() -> CpuRegisters {
        CpuRegisters {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            pc: 0,
            sp: 0,
        }
    }

    pub(crate) fn get_byte(&self, register: CpuRegIndex) -> u8 {
        match register {
            CpuRegIndex::A => self.a,
            CpuRegIndex::B => self.b,
            CpuRegIndex::C => self.c,
            CpuRegIndex::D => self.d,
            CpuRegIndex::E => self.e,
            CpuRegIndex::F => self.f,
            CpuRegIndex::H => self.h,
            CpuRegIndex::L => self.l,
            _ => panic!("Invalid RegIndex"),
        }
    }

    pub(crate) fn get_word(&self, register: CpuRegIndex) -> u16 {
        match register {
            CpuRegIndex::AF => self.bytes_to_word(self.a, self.f),
            CpuRegIndex::BC => self.bytes_to_word(self.b, self.c),
            CpuRegIndex::DE => self.bytes_to_word(self.d, self.e),
            CpuRegIndex::HL => self.bytes_to_word(self.h, self.l),
            CpuRegIndex::PC => self.pc,
            CpuRegIndex::SP => self.sp,
            _ => panic!("Invalid RegIndex"),
        }
    }

    pub(crate) fn get_flags(&mut self) -> Flags {
        Flags {
            zero: ((self.f >> FLAG_ZERO_BYTE) & 0b1) != 0,
            subtract: ((self.f >> FLAG_SUBTRACT_BYTE) & 0b1) != 0,
            half_carry: ((self.f >> FLAG_HALF_CARRY_BYTE) & 0b1) != 0,
            carry: ((self.f >> FLAG_CARRY_BYTE) & 0b1) != 0,
        }
    }

    pub(crate) fn set_byte(&mut self, register: CpuRegIndex, value: u8) {
        match register {
            CpuRegIndex::A => { self.a = value; }
            CpuRegIndex::B => { self.b = value; }
            CpuRegIndex::C => { self.c = value; }
            CpuRegIndex::D => { self.d = value; }
            CpuRegIndex::E => { self.e = value; }
            CpuRegIndex::F => { self.f = value; }
            CpuRegIndex::H => { self.h = value; }
            CpuRegIndex::L => { self.l = value; }
            _ => panic!("Invalid RegIndex"),
        }
    }

    pub(crate) fn set_flags(&mut self, flags: Flags) {
        self.f =
            (if flags.zero { 1 } else { 0 }) << FLAG_ZERO_BYTE |
                (if flags.subtract { 1 } else { 0 }) << FLAG_SUBTRACT_BYTE |
                (if flags.half_carry { 1 } else { 0 }) << FLAG_HALF_CARRY_BYTE |
                (if flags.carry { 1 } else { 0 }) << FLAG_CARRY_BYTE;
    }

    pub(crate) fn set_word(&mut self, register: CpuRegIndex, value: u16) {
        match register {
            CpuRegIndex::AF => {
                self.a = ((value & 0xFF00) >> 8) as u8;
                self.f = (value & 0xFF) as u8;
            }
            CpuRegIndex::BC => {
                self.b = ((value & 0xFF00) >> 8) as u8;
                self.c = (value & 0xFF) as u8;
            }
            CpuRegIndex::DE => {
                self.d = ((value & 0xFF00) >> 8) as u8;
                self.e = (value & 0xFF) as u8;
            }
            CpuRegIndex::HL => {
                self.h = ((value & 0xFF00) >> 8) as u8;
                self.l = (value & 0xFF) as u8;
            }
            CpuRegIndex::PC => {
                self.pc =value;
            }
            CpuRegIndex::SP => {
                self.sp = value;
            }
            _ => panic!("Invalid RegIndex"),
        }
    }

    pub(crate) fn increment(&mut self, register: CpuRegIndex, increment_by: u16) {
        match register {
            CpuRegIndex::A => {
                self.a = self.a.wrapping_add(increment_by as u8);
            }
            CpuRegIndex::B => {
                self.b = self.b.wrapping_add(increment_by as u8);
            }
            CpuRegIndex::C => {
                self.c = self.c.wrapping_add(increment_by as u8);
            }
            CpuRegIndex::D => {
                self.d = self.d.wrapping_add(increment_by as u8);
            }
            CpuRegIndex::E => {
                self.e = self.e.wrapping_add(increment_by as u8);
            }
            CpuRegIndex::F => {
                self.f = self.f.wrapping_add(increment_by as u8);
            }
            CpuRegIndex::H => {
                self.h = self.h.wrapping_add(increment_by as u8);
            }
            CpuRegIndex::L => {
                self.l = self.l.wrapping_add(increment_by as u8);
            }
            CpuRegIndex::AF => {
                let af = self.get_word(CpuRegIndex::AF).wrapping_add(increment_by);
                self.set_word(CpuRegIndex::AF, af);
            }
            CpuRegIndex::BC => {
                let bc = self.get_word(CpuRegIndex::BC).wrapping_add(increment_by);
                self.set_word(CpuRegIndex::BC, bc);
            }
            CpuRegIndex::DE => {
                let de = self.get_word(CpuRegIndex::DE).wrapping_add(increment_by);
                self.set_word(CpuRegIndex::DE, de);
            }
            CpuRegIndex::HL => {
                let hl = self.get_word(CpuRegIndex::HL).wrapping_add(increment_by);
                self.set_word(CpuRegIndex::HL, hl);
            }
            CpuRegIndex::PC => {
                self.pc = self.pc.wrapping_add(increment_by);
            }
            CpuRegIndex::SP => {
                self.sp = self.sp.wrapping_add(increment_by);
            }
            _ => panic!("Invalid RegIndex"),
        }
    }

    pub(crate) fn decrement(&mut self, register: CpuRegIndex, decrement_by: u16) {
        match register {
            CpuRegIndex::A => {
                self.a = self.a.wrapping_sub(decrement_by as u8);
            }
            CpuRegIndex::B => {
                self.b = self.b.wrapping_sub(decrement_by as u8);
            }
            CpuRegIndex::C => {
                self.c = self.c.wrapping_sub(decrement_by as u8);
            }
            CpuRegIndex::D => {
                self.d = self.d.wrapping_sub(decrement_by as u8);
            }
            CpuRegIndex::E => {
                self.e = self.e.wrapping_sub(decrement_by as u8);
            }
            CpuRegIndex::F => {
                self.f = self.f.wrapping_sub(decrement_by as u8);
            }
            CpuRegIndex::H => {
                self.h = self.h.wrapping_sub(decrement_by as u8);
            }
            CpuRegIndex::L => {
                self.l = self.l.wrapping_sub(decrement_by as u8);
            }
            CpuRegIndex::AF => {
                let af = self.get_word(CpuRegIndex::AF).wrapping_sub(decrement_by);
                self.set_word(CpuRegIndex::AF, af);
            }
            CpuRegIndex::BC => {
                let bc = self.get_word(CpuRegIndex::BC).wrapping_sub(decrement_by);
                self.set_word(CpuRegIndex::BC, bc);
            }
            CpuRegIndex::DE => {
                let de = self.get_word(CpuRegIndex::DE).wrapping_sub(decrement_by);
                self.set_word(CpuRegIndex::DE, de);
            }
            CpuRegIndex::HL => {
                let hl = self.get_word(CpuRegIndex::HL).wrapping_sub(decrement_by);
                self.set_word(CpuRegIndex::HL, hl);
            }
            CpuRegIndex::PC => {
                self.pc = self.pc.wrapping_sub(decrement_by);
            }
            CpuRegIndex::SP => {
                self.sp = self.sp.wrapping_sub(decrement_by);
            }
            _ => panic!("Invalid RegIndex"),
        }
    }

    fn bytes_to_word(&self, reg1: u8, reg2: u8) -> u16 {
        (reg1 as u16) << 8 | (reg2 as u16)
    }
}

impl Display for CpuRegisters {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f,
            "\ta: {:#04X}\n\
            \tb: {:#04X}\n\
            \tc: {:#04X}\n\
            \td: {:#04X}\n\
            \te: {:#04X}\n\
            \tf: {:#04X}\n\
            \th: {:#04X}\n\
            \tl: {:#04X}\n\
            \tpc: {:#06X}\n\
            \tsp: {:#06X}",
            self.a,
            self.b,
            self.c,
            self.d,
            self.e,
            self.f,
            self.h,
            self.l,
            self.pc,
            self.sp
        )
    }
}
