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

use crate::console::mmu;
use crate::console::mmu::Mmu;
use crate::console::register::Register;

pub(crate) enum InterruptRegBit {
    VBlank = 0,
    LcdStat = 1,
    Timer = 2,
    Serial = 3,
    Joypad = 4
}

pub(crate) struct Interrupts {
    pub(crate) enabled: Register,
    pub(crate) requested: Register,
    pub(crate) ime: bool,
}

impl Interrupts {
    pub(crate) fn new() -> Interrupts {
        Interrupts {
            ime: false,
            enabled: Register::new(mmu::IE_REG),
            requested: Register::new(mmu::IF_REG),
        }
    }

    pub(crate) fn request(&mut self, requesting: InterruptRegBit, mmu: &mut Mmu) {
        self.requested.set_bit(mmu, requesting as u8, true);
    }

    pub(crate) fn peek_requested(&mut self, mmu: &mut Mmu) -> bool {
        self.requested.read(mmu) != 0
    }

    pub(crate) fn peek_interrupts(&mut self, mmu: &mut Mmu) -> u8 {
        if self.ime {
            self.enabled.read(mmu) & self.requested.read(mmu)
        } else {
            0
        }
    }

    pub(crate) fn poll(&mut self, mmu: &mut Mmu) -> u8 {
        let interrupt = self.peek_interrupts(mmu);
        if interrupt & (1 << InterruptRegBit::VBlank as u8) == (1 << InterruptRegBit::VBlank as u8) {
            self.requested.set_bit(mmu, InterruptRegBit::VBlank as u8, false);
            0x40
        } else if interrupt & (1 << InterruptRegBit::LcdStat as u8) == (1 << InterruptRegBit::LcdStat as u8) {
            self.requested.set_bit(mmu, InterruptRegBit::LcdStat as u8, false);
           0x48
        } else if interrupt & (1 << InterruptRegBit::Timer as u8) == (1 << InterruptRegBit::Timer as u8) {
            self.requested.set_bit(mmu, InterruptRegBit::Timer as u8, false);
            0x50
        } else if interrupt & (1 << InterruptRegBit::Serial as u8) == (1 << InterruptRegBit::Serial as u8) {
            self.requested.set_bit(mmu, InterruptRegBit::Serial as u8, false);
            0x58
        } else if interrupt & (1 << InterruptRegBit::Joypad as u8) == (1 << InterruptRegBit::Joypad as u8) {
            self.requested.set_bit(mmu, InterruptRegBit::Joypad as u8, false);
            0x60
        } else {
            0
        }
    }
}

impl Display for Interrupts {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "TODO"
            // \tRequested: {} {} {} {} {}\tEnabled: {} {} {} {} {}",
            // if self.requested.read(mmu) & (1 << InterruptRegBit::Joypad as u8) == (1 << InterruptRegBit::Joypad as u8) {"J"} else {"_"},
            // if self.requested.read(mmu) & (1 << InterruptRegBit::Serial as u8) == (1 << InterruptRegBit::Serial as u8) {"S"} else {"_"},
            // if self.requested.read(mmu) & (1 << InterruptRegBit::Timer as u8) == (1 << InterruptRegBit::Timer as u8) {"T"} else {"_"},
            // if self.requested.read(mmu) & (1 << InterruptRegBit::LcdStat as u8) == (1 << InterruptRegBit::LcdStat as u8) {"L"} else {"_"},
            // if self.requested.read(mmu) & (1 << InterruptRegBit::VBlank as u8) == (1 << InterruptRegBit::VBlank as u8) {"V"} else {"_"},
            //
            // if self.enabled.read(mmu) & (1 << InterruptRegBit::Joypad as u8) == (1 << InterruptRegBit::Joypad as u8) {"J"} else {"_"},
            // if self.enabled.read(mmu) & (1 << InterruptRegBit::Serial as u8) == (1 << InterruptRegBit::Serial as u8) {"S"} else {"_"},
            // if self.enabled.read(mmu) & (1 << InterruptRegBit::Timer as u8) == (1 << InterruptRegBit::Timer as u8) {"T"} else {"_"},
            // if self.enabled.read(mmu) & (1 << InterruptRegBit::LcdStat as u8) == (1 << InterruptRegBit::LcdStat as u8) {"L"} else {"_"},
            // if self.enabled.read(mmu) & (1 << InterruptRegBit::VBlank as u8) == (1 << InterruptRegBit::VBlank as u8) {"V"} else {"_"},
        )
    }
}
