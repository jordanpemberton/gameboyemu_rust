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
pub(crate) enum RegIndex {
    A, B, C, D, E, F, H, L,
    AF, BC, DE, HL,
    PC, SP
}

pub(crate) struct Registers {
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

impl Registers {
    pub(crate) fn new() -> Registers {
        Registers {
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

    pub(crate) fn get_byte(&self, register: RegIndex) -> u8 {
        match register {
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

    pub(crate) fn get_word(&self, register: RegIndex) -> u16 {
        match register {
            RegIndex::AF => self.bytes_to_word(self.a, self.f),
            RegIndex::BC => self.bytes_to_word(self.b, self.c),
            RegIndex::DE => self.bytes_to_word(self.d, self.e),
            RegIndex::HL => self.bytes_to_word(self.h, self.l),
            RegIndex::PC => self.pc,
            RegIndex::SP => self.sp,
            _ => 0
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

    pub(crate) fn set_byte(&mut self, register: RegIndex, value: u8) {
        match register {
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

    pub(crate) fn set_f(&mut self, flags: Flags) {
        self.f =
            (if flags.zero { 1 } else { 0 }) << FLAG_ZERO_BYTE |
                (if flags.subtract { 1 } else { 0 }) << FLAG_SUBTRACT_BYTE |
                (if flags.half_carry { 1 } else { 0 }) << FLAG_HALF_CARRY_BYTE |
                (if flags.carry { 1 } else { 0 }) << FLAG_CARRY_BYTE;
    }

    pub(crate) fn set_word(&mut self, register: RegIndex, value: u16) {
        match register {
            RegIndex::AF => {
                self.a = ((value & 0xFF00) >> 8) as u8;
                self.f = (value & 0xFF) as u8;
            }
            RegIndex::BC => {
                self.b = ((value & 0xFF00) >> 8) as u8;
                self.c = (value & 0xFF) as u8;
            }
            RegIndex::DE => {
                self.d = ((value & 0xFF00) >> 8) as u8;
                self.e = (value & 0xFF) as u8;
            }
            RegIndex::HL => {
                self.h = ((value & 0xFF00) >> 8) as u8;
                self.l = (value & 0xFF) as u8;
            }
            RegIndex::PC => {
                self.pc =value;
            }
            RegIndex::SP => {
                self.sp = value;
            }
            _ => { }
        }
    }

    pub(crate) fn increment(&mut self, register: RegIndex, increment_by: u16) {
        match register {
            RegIndex::PC => {
                self.pc = self.pc.wrapping_add(increment_by);
            }
            RegIndex::SP => {
                self.sp = self.sp.wrapping_add(increment_by);
            }
            _ => { }
        }
    }

    pub(crate) fn decrement(&mut self, register: RegIndex, decrement_by: u16) {
        match register {
            RegIndex::PC => {
                self.pc = self.pc.wrapping_sub(decrement_by);
            }
            RegIndex::SP => {
                self.sp = self.sp.wrapping_sub(decrement_by);
            }
            _ => { }
        }
    }

    fn bytes_to_word(&self, reg1: u8, reg2: u8) -> u16 {
        (reg1 as u16) << 8 | (reg2 as u16)
    }
}
