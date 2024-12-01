use crate::console::mmu::{Caller, Mmu};

/*
pub(crate) trait ReadWrite {
    fn read_8(&mut self) -> u8;
    fn write_8(&mut self, value: u8) -> u8;
}
*/

#[allow(dead_code)]
pub(crate) struct Register {
    pub(crate) address: u16,
    value: u8,
}

impl Register {
    pub(crate) fn new(address: u16) -> Register {
        Register::_new(address, 0)
    }
    pub(crate) fn _new(address: u16, value: u8) -> Register {
        Register {
            address,
            value,
        }
    }

    pub(crate) fn read(&mut self, mmu: &mut Mmu, caller: Caller) -> u8 {
        mmu.read_8(self.address, caller)
    }

    pub(crate) fn write(&mut self, mmu: &mut Mmu, value: u8, caller: Caller) {
        mmu.write_8(self.address, value, caller);
    }

    pub(crate) fn check_bit(&mut self, mmu: &mut Mmu, bit: u8, caller: Caller) -> bool {
        let value = self.read(mmu, caller);
        self.read(mmu, caller);
        (value & (1 << bit)) >> bit == 1
    }

    pub(crate) fn set_bit(&mut self, mmu: &mut Mmu, bit: u8, set: bool, caller: Caller) {
        let mut value = self.read(mmu, caller);
        value ^= value & (1 << bit);
        value |= if set { 1 << bit } else { 0 };
        self.write(mmu, value, caller);
    }
}

/*
impl ReadWrite for Register {
    fn read_8(&mut self) -> u8 {
        self.value
    }

    fn write_8(&mut self, value: u8) -> u8 {
        self.value = value;
        self.value
    }
}
*/

/*
impl PartialEq<Register> for Register {
    fn eq(&self, other: &Register) -> bool {
        if let Some(other_register) = &other.value {
            *other_register.value == self.value
        } else {
            false
        }
    }
}

impl PartialEq<u8> for Register {
    fn eq(&self, other: &u8) -> bool {
        self.value == *other
    }
}

impl PartialOrd<u8> for Register {
    fn partial_cmp(&self, other: &u8) -> Option<Ordering> {
        self.value.partial_cmp(other)
    }
    fn lt(&self, other: &u8) -> bool {
        self.value.lt(other)
    }
    fn le(&self, other: &u8) -> bool {
        self.value.le(other)
    }
    fn gt(&self, other: &u8) -> bool {
        self.value.gt(other)
    }
    fn ge(&self, other: &u8) -> bool {
        self.value.gt(other)
    }
}
 */
