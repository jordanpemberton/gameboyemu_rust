use crate::console::mmu::Mmu;

pub(crate) struct Register {
    pub(crate) address: u16,
    pub(crate) value: u8,
}

impl Register {
    pub(crate) fn new(mmu: &mut Mmu, address: u16) -> Register {
        let mut register = Register {
            address,
            value: 0,
        };
        register.read_from_mem(mmu);
        register
    }

    pub(crate) fn read_from_mem(&mut self, mmu: &mut Mmu) {
        self.value = mmu.read_8(self.address);
    }

    pub(crate) fn write_to_mem(&mut self, mmu: &mut Mmu) {
        mmu.write_8(self.address, self.value);
    }

    pub(crate) fn check_bit(&mut self, mmu: &mut Mmu, bit: u8) -> bool {
        self.read_from_mem(mmu);
        (self.value & (1 << bit)) >> bit == 1
    }

    pub(crate) fn set_bit(&mut self, mmu: &mut Mmu, bit: u8, set: bool) {
        self.value ^= self.value & (1 << bit);
        self.value |= if set { 1 << bit } else { 0 };
        self.write_to_mem(mmu);
    }
}
