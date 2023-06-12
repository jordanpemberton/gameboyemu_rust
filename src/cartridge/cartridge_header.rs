#[allow(dead_code)]
pub(crate) enum CartridgeType {
    None {
        ram: bool,
        battery: bool,
    },
    Mbc1 {
        ram: bool,
        battery: bool,
        is_multicart: bool,
    },
    Mbc2,
    Mbc3,
    Mbc5,
}


impl CartridgeType {
    pub(crate) fn from_u8(value: u8) -> CartridgeType {
        match value {
            0x00 => CartridgeType::None {
                ram: false,
                battery: false,
            },
            0x01 => CartridgeType::Mbc1 {
                ram: false,
                battery: false,
                is_multicart: false,
            },
            0x02 => CartridgeType::Mbc1 {
                ram: true,
                battery: false,
                is_multicart: false,
            },
            0x03 => CartridgeType::Mbc1 {
                ram: true,
                battery: true,
                is_multicart: false,
            },

            0x08 => CartridgeType::None {
                ram: true,
                battery: false,
            },
            0x09 => CartridgeType::None {
                ram: true,
                battery: true,
            },

            _ => CartridgeType::None {
                ram: false,
                battery: false,
            },
        }
    }
}

#[allow(dead_code)]
pub(crate) struct CartridgeHeader {
    pub(crate) rom_size: usize,
    pub(crate) ram_size: usize,
    pub(crate) cartridge_type: CartridgeType,
}

impl CartridgeHeader {
    pub(crate) fn new(data: &[u8]) -> Option<CartridgeHeader> {
        if data.len() < 0x0150 { return None; }

        let rom_size = 32 * (1 << data[0x0148]) as usize; // KiB

        let ram_size = match data[0x0149] as usize {
            0 => 0,
            1 => 2,
            2 => 8,
            3 => 32,
            4 => 128,
            5 => 64,
            _ => 0,
        };

        let cartridge_type = CartridgeType::from_u8(data[0x0147]);

       Some(CartridgeHeader {
            rom_size,
            ram_size,
            cartridge_type,
       })
    }
}
