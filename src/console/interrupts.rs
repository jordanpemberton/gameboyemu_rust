#![allow(dead_code)]

/*
FFFF — IE: Interrupt enable
Bit 0: VBlank   Interrupt Enable  (INT $40)  (1=Enable)
Bit 1: LCD STAT Interrupt Enable  (INT $48)  (1=Enable)
Bit 2: Timer    Interrupt Enable  (INT $50)  (1=Enable)
Bit 3: Serial   Interrupt Enable  (INT $58)  (1=Enable)
Bit 4: Joypad   Interrupt Enable  (INT $60)  (1=Enable)

FF0F — IF: Interrupt flag
Bit 0: VBlank   Interrupt Request (INT $40)  (1=Request)
Bit 1: LCD STAT Interrupt Request (INT $48)  (1=Request)
Bit 2: Timer    Interrupt Request (INT $50)  (1=Request)
Bit 3: Serial   Interrupt Request (INT $58)  (1=Request)
Bit 4: Joypad   Interrupt Request (INT $60)  (1=Request)
*/

use std::fmt::{Display, Formatter};

use crate::console::mmu::Mmu;

pub(crate) enum InterruptRegBit {
    VBlank = 0,
    LcdStat = 1,
    Timer = 2,
    Serial = 3,
    Joypad = 4
}

// TODO replace with Register
pub(crate) struct InterruptReg {
    address: u16,
    value: u8,
}

impl InterruptReg {
    fn new(address: u16, mmu: &mut Mmu) -> InterruptReg {
        let mut reg = InterruptReg {
            address: address,
            value: 0,
        };
        reg.read_from_mem(mmu);
        reg
    }

    pub(crate) fn get_bit(&self, bit: InterruptRegBit) -> bool {
        let b = bit as usize;
        self.value & (1 << b) == 1 << b
    }

    pub(crate) fn set_bit(&mut self, bit: InterruptRegBit, enable: bool, mmu: &mut Mmu) {
        let p = bit as usize;
        let b = self.value & (1 << p);
        self.value ^= b;
        self.value |= (enable as u8) << p;
        self.write_to_mem(mmu);
    }

    pub(crate) fn set_all(&mut self, enable: bool, mmu: &mut Mmu) {
        self.value = if enable { 0xFF } else { 0x00 };
        self.write_to_mem(mmu);
    }

    fn read_from_mem(&mut self, mmu: &mut Mmu) {
        self.value = mmu.read_8(self.address);
    }

    fn write_to_mem(&self, mmu: &mut Mmu) {
        mmu.write_8(self.address, self.value);
    }
}

impl Display for InterruptReg {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\t{} {} {} {} {}",
            if self.get_bit(InterruptRegBit::VBlank) {"V"} else {"_"},
            if self.get_bit(InterruptRegBit::LcdStat) {"L"} else {"_"},
            if self.get_bit(InterruptRegBit::Timer) {"T"} else {"_"},
            if self.get_bit(InterruptRegBit::Serial) {"S"} else {"_"},
            if self.get_bit(InterruptRegBit::Joypad) {"J"} else {"_"}
        )
    }
}

pub(crate) struct Interrupts {
    pub(crate) enabled: InterruptReg,
    pub(crate) requested: InterruptReg,
    ime: bool,
}

impl Interrupts {
    pub(crate) fn new(mmu: &mut Mmu) -> Interrupts {
        Interrupts {
            ime: false,
            enabled: InterruptReg::new(0xFFFF, mmu),
            requested: InterruptReg::new(0xFF0F, mmu),
        }
    }

    pub(crate) fn set_ime(&mut self, enable: bool) {
        self.ime = enable;
    }

    pub(crate) fn request(&mut self, requesting: InterruptRegBit, mmu: &mut Mmu) {
        self.requested.set_bit(requesting, true, mmu);
    }

    pub(crate) fn poll(&mut self, mmu: &mut Mmu) -> u8 {
        let mut value = 0x00;

        self.refresh_from_mem(mmu);

        if self.ime {
            if self.enabled.get_bit(InterruptRegBit::VBlank) && self.requested.get_bit(InterruptRegBit::VBlank) {
                self.requested.set_bit(InterruptRegBit::VBlank, false, mmu);
                value = 0x40;
            } else if self.enabled.get_bit(InterruptRegBit::LcdStat) && self.requested.get_bit(InterruptRegBit::LcdStat) {
                self.requested.set_bit(InterruptRegBit::LcdStat, false, mmu);
                value = 0x48;
            } else if self.enabled.get_bit(InterruptRegBit::Timer) && self.requested.get_bit(InterruptRegBit::Timer) {
                self.requested.set_bit(InterruptRegBit::Timer, false, mmu);
                value = 0x50;
            } else if self.enabled.get_bit(InterruptRegBit::Serial) && self.requested.get_bit(InterruptRegBit::Serial) {
                self.requested.set_bit(InterruptRegBit::Serial, false, mmu);
                value = 0x58;
            } else if self.enabled.get_bit(InterruptRegBit::Joypad) && self.requested.get_bit(InterruptRegBit::Joypad) {
                self.requested.set_bit(InterruptRegBit::Joypad, false, mmu);
                value = 0x60;
            }
        }

        value
    }

    fn refresh_from_mem(&mut self, mmu: &mut Mmu) {
        self.enabled.read_from_mem(mmu);
        self.requested.read_from_mem(mmu);
    }
}
