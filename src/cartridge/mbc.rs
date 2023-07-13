use crate::cartridge::cartridge_header::CartridgeType;

pub(crate) const ROM_BANK_SIZE: usize = 0x4000;
pub(crate) const RAM_BANK_SIZE: usize = 0x2000;

const CARTRIDGE_TYPE_ADDRESS: usize = 0x0147;
const ROM_SIZE_ADDRESS: usize = 0x0148;
const RAM_SIZE_ADDRESS: usize = 0x0149;

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub(crate) struct Mbc1 {
    pub(crate) ram_enabled: bool,

    pub(crate) advram_banking_mode: bool,
    // 00 = Simple Banking Mode (default)
    //    0000–3FFF and A000–BFFF locked to bank 0 of ROM/RAM
    // 01 = RAM Banking Mode / Advanced ROM Banking Mode
    //    0000–3FFF and A000–BFFF can be bank-switched via the 4000–5FFF bank register
    // If the cart is not large enough to use the 2-bit register (<= 8 KiB RAM and <= 512 KiB ROM) this mode select has no observable effect.

    pub(crate) is_multicart: bool,

    pub(crate) bank1: u8,  // 5-bit register: ROM bank (0x01..=0x1E)
    // If this register is set to $00, it behaves as if it is set to $01. TODO Add code to do this

    pub(crate) bank2: u8,  // 2-bit register: RAM bank (0x01..=0x03), or ROM bank upper bits (0x20..=0x60, or 0x10..=0x30 in 1MB MBC1 multicart) (?)
    // Can be used to select a RAM Bank in range from $00–$03 (32 KiB ram carts only),
    // OR to specify the upper two bits (bits 5-6) of the ROM Bank number (1 MiB ROM or larger carts only).
    // If neither ROM nor RAM is large enough, setting this register does nothing.
    // In 1MB MBC1 multi-carts, this 2-bit register is instead applied to bits 4-5 of the ROM bank number
    // and the top bit of the main 5-bit main ROM banking register is ignored.
}

impl Mbc1 {
    pub(crate) fn rom_offsets(&self) -> (usize, usize) {
        let (upper_bits, lower_bits) = if self.is_multicart {
            (self.bank2 << 4, self.bank1 & 0x0F)
        } else {
            (self.bank2 << 5, self.bank1)
        };

        let lower_bank = if self.advram_banking_mode { upper_bits } else { 0 };
        let upper_bank = upper_bits | lower_bits;

        (ROM_BANK_SIZE * lower_bank as usize, ROM_BANK_SIZE * upper_bank as usize)
    }

    pub(crate) fn ram_offset(&self) -> usize {
        let bank = if self.advram_banking_mode {
            self.bank2 as usize
        } else {
            0
        };

        RAM_BANK_SIZE * bank
    }
}

#[allow(dead_code)]
pub(crate) enum Mbc {
    None,
    Mbc1 { mbc: Mbc1 },
    Mbc2,
    Mbc3,
    Mbc5,
}

#[allow(unused_variables)]
impl Mbc {
    pub(crate) fn new(data: &[u8]) -> Mbc {
        if data.len() < 0x0150 { return Mbc::None; }

        let rom_size = 32 * (1 << data[ROM_SIZE_ADDRESS]) as usize; // KiB

        let ram_size = match data[RAM_SIZE_ADDRESS] as usize { // KiB
            0 => 0,
            1 => 2,
            2 => 8,
            3 => 32,
            4 => 128,
            5 => 64,
            _ => 0,
        };

        let cartridge_type = CartridgeType::from_u8(data[CARTRIDGE_TYPE_ADDRESS]);

        let mbc = match cartridge_type {
            CartridgeType::NoMbc { .. } => Mbc::None,
            CartridgeType::Mbc1 { is_multicart, .. } => {
                Mbc::Mbc1 {
                    mbc: Mbc1 {
                        ram_enabled: false,
                        advram_banking_mode: false,
                        is_multicart,
                        bank1: 1,
                        bank2: 0,
                    }
                }
            },
            _ => Mbc::None,
        };

        mbc
    }
}
