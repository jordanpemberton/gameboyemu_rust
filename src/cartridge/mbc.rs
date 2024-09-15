use crate::cartridge::cartridge_header::CartridgeType;
use crate::cartridge::mbc1::Mbc1;

pub(crate) const ROM_BANK_SIZE: usize = 0x4000;
pub(crate) const RAM_BANK_SIZE: usize = 0x2000;

const CARTRIDGE_TYPE_ADDRESS: usize = 0x0147;
const ROM_SIZE_ADDRESS: usize = 0x0148;
const RAM_SIZE_ADDRESS: usize = 0x0149;

#[allow(dead_code)]
pub(crate) enum Mbc {
    None,
    Mbc1 { mbc: Mbc1 },
    Mbc2, // { mbc: Mbc2 },
    Mbc3, // { mbc: Mbc3 },
    Mbc5, // { mbc: Mbc5 },
    Huc1,
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
            CartridgeType::Mbc2 { .. } => {
                panic!("UNIMPLEMENTED CartridgeType MBC2");
            },
            CartridgeType::Mbc3 { .. } => {
                panic!("UNIMPLEMENTED CartridgeType MBC3");
            },
            CartridgeType::Mbc5 { .. } => {
                panic!("UNIMPLEMENTED CartridgeType MBC5");
            },
            CartridgeType::Huc1 { .. } => {
                panic!("UNIMPLEMENTED CartridgeType HUC1");
            }
            _ => {
                panic!("UNIMPLEMENTED CartridgeType ???");
            }
        };

        mbc
    }
}
