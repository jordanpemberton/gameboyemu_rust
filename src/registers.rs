const FLAG_ZERO_BYTE: u8 = 7;
const FLAG_SUBTRACT_BYTE: u8 = 6;
const FLAG_HALF_CARRY_BYTE: u8 = 5;
const FLAG_CARRY_BYTE: u8 = 4;

pub(crate) struct Flags {
    pub(crate) zero: bool,
    pub(crate) subtract: bool,
    pub(crate) half_carry: bool,
    pub(crate) carry: bool,
    pub(crate) always: bool,
    
}

pub(crate) enum RegIndex {
    A, B, C, D, E, F, H, L,
    AF, BC, DE, HL,
    D8,
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
    pub(crate) fn get_byte(&self, index: RegIndex) -> u8 {
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

    pub(crate) fn get_word(&self, index: RegIndex) -> u16 {
        match index {
            RegIndex::AF => self.to_word(self.a, self.f),
            RegIndex::BC => self.to_word(self.b, self.c),
            RegIndex::DE => self.to_word(self.d, self.e),
            RegIndex::HL => self.to_word(self.h, self.l),
            _ => 0
        }
    }

    pub(crate) fn to_word(&self, reg1: u8, reg2: u8) -> u16 {
        (reg1 as u16) << 8 | (reg2 as u16)
    }

    pub(crate) fn get_flags(&mut self) -> Flags {
        Flags {
            zero: ((self.f >> FLAG_ZERO_BYTE) & 0b1) != 0,
            subtract: ((self.f >> FLAG_SUBTRACT_BYTE) & 0b1) != 0,
            half_carry: ((self.f >> FLAG_HALF_CARRY_BYTE) & 0b1) != 0,
            carry: ((self.f >> FLAG_CARRY_BYTE) & 0b1) != 0,
            always: false,
        }
    }

    pub(crate) fn set_f(&mut self, flags: Flags) {
        self.f =
            (if flags.zero { 1 } else { 0 }) << FLAG_ZERO_BYTE |
                (if flags.subtract { 1 } else { 0 }) << FLAG_SUBTRACT_BYTE |
                (if flags.half_carry { 1 } else { 0 }) << FLAG_HALF_CARRY_BYTE |
                (if flags.carry { 1 } else { 0 }) << FLAG_CARRY_BYTE;
    }

    pub(crate) fn set_byte(&mut self, index: RegIndex, value: u8) {
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

    pub(crate) fn set_word(&mut self, index: RegIndex, value: u16) {
        match index {
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
            _ => ()
        }
    }
}
