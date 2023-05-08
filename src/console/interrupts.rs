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
use crate::console::mmu::Mmu;

pub(crate) enum InterruptRegBit {
    VBlank = 0,
    LcdStat = 1,
    Timer = 2,
    Serial = 3,
    Joypad = 4
}

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

    pub(crate) fn set_all(&mut self, enable: bool, mmu: &mut Mmu) {
        self.value = if enable { 0xFF } else { 0x00 };
        self.write_to_mem(mmu);
    }

    pub(crate) fn set_bit(&mut self, bit: InterruptRegBit, enable: bool, mmu: &mut Mmu) {
        let p = bit as usize;
        let b = self.value & (1 << p);
        self.value &= !b;
        self.value |= (enable as u8) << p;
        self.write_to_mem(mmu);
    }

    fn read_from_mem(&mut self, mmu: &mut Mmu) {
        self.value = mmu.read_byte(self.address);
    }

    fn write_to_mem(&self, mmu: &mut Mmu) {
        mmu.load_byte(self.address, self.value);
    }
}

pub(crate) struct Interrupts {
    pub(crate) ime: bool,
    pub(crate) enabled: InterruptReg,
    pub(crate) requested: InterruptReg,
}

impl Interrupts {
    pub(crate) fn new(mmu: &mut Mmu) -> Interrupts {
        Interrupts {
            ime: false,
            enabled: InterruptReg::new(0xFFFF, mmu),
            requested: InterruptReg::new(0xFF0F, mmu),
        }
    }

    pub(crate) fn set_ime(&mut self, enable: bool, mmu: &mut Mmu) {
        self.ime = enable;
        self.enabled.set_all(enable, mmu);
    }
}

