#[allow(dead_code)]
pub(crate) const ROM_BANK_SIZE: usize = 0x4000;
#[allow(dead_code)]
pub(crate) const RAM_BANK_SIZE: usize = 0x2000;

#[allow(dead_code)]
pub(crate) enum Mbc {
    None,
    Mbc1 { state: Mbc1State },
    Mbc2,
    Mbc3,
    Mbc5,
}

#[allow(dead_code)]
pub(crate) struct Mbc1State {
    ram_enabled: bool,
    advram_banking_mode: bool,
    // 00 = Simple Banking Mode (default)
    //    0000–3FFF and A000–BFFF locked to bank 0 of ROM/RAM
    // 01 = RAM Banking Mode / Advanced ROM Banking Mode
    //    0000–3FFF and A000–BFFF can be bank-switched via the 4000–5FFF bank register
    // If the cart is not large enough to use the 2-bit register (<= 8 KiB RAM and <= 512 KiB ROM) this mode select has no observable effect.

    is_multicart: bool,

    bank1: u8,  // 5-bit register: ROM bank (0x01..=0x1E)
    // If this register is set to $00, it behaves as if it is set to $01.

    bank2: u8,  // 2-bit register: RAM bank (0x01..=0x03), or ROM bank upper bits (0x20..=0x60, or 0x10..=0x30 in 1MB MBC1 multicart) (?)
    // Can be used to select a RAM Bank in range from $00–$03 (32 KiB ram carts only),
    // OR to specify the upper two bits (bits 5-6) of the ROM Bank number (1 MiB ROM or larger carts only).
    // In 1MB MBC1 multi-carts, this 2-bit register is instead applied to bits 4-5 of the ROM bank number
    // and the top bit of the main 5-bit main ROM banking register is ignored.
    // If neither ROM nor RAM is large enough, setting this register does nothing.
}

impl Mbc1State {
    #[allow(dead_code)]
    fn rom_offsets(&self) -> (usize, usize) {
        let (lower_bits, upper_bits) = if self.is_multicart {
            (self.bank1 & 0x0F, self.bank2 << 4)
        } else {
            (self.bank1, self.bank2 << 5)
        };

        let upper_bank = upper_bits | lower_bits;

        let lower_bank = if self.advram_banking_mode {
            upper_bits
        } else {
            0
        };

        (ROM_BANK_SIZE * lower_bank as usize, ROM_BANK_SIZE * upper_bank as usize)
    }
}

#[allow(unused_variables)]
impl Mbc {
    pub(crate) fn new(data: &[u8]) -> Mbc {
        let rom_size = data[0x0148] as usize;
        let ram_size = data[0x0148] as usize;

        let mbc = match data[0x014] {
            01 => Mbc::Mbc1 { // TODO
                state: Mbc1State{
                    ram_enabled: false,
                    advram_banking_mode: false,
                    is_multicart: false,
                    bank1: 0,
                    bank2: 0,
                }
            },
            _ => Mbc::None,
        };

        mbc
    }
}
