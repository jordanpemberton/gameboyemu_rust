use crate::console::mmu::{Caller, Mmu};

pub(crate) struct Register {
    pub(crate) address: u16,
}

impl Register {
    pub(crate) fn new(address: u16) -> Register {
        Register {
            address
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
