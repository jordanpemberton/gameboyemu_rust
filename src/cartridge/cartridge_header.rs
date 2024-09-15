#[allow(dead_code)]
pub(crate) enum CartridgeType {
    NoMbc {
        ram: bool,
        battery: bool,
    },
    Mbc1 {
        ram: bool,
        battery: bool,
        is_multicart: bool,
    },
    Mbc2 {
        battery: bool,
    },
    Mbc3 {
        ram: bool,
        battery: bool,
        rtc: bool,
    },
    Mbc5 {
        ram: bool,
        battery: bool,
        rumble: bool,
    },
    Mbc6,
    Mbc7,
    Huc1,
    Huc3,
}

impl CartridgeType {
    pub(crate) fn from_u8(value: u8) -> CartridgeType {
        match value {
            0x00 => CartridgeType::NoMbc {
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
            0x05 => CartridgeType::Mbc2 {
                battery: false
            },
            0x06 => CartridgeType::Mbc2 {
                battery: true
            },
            0x08 => CartridgeType::NoMbc {
                ram: true,
                battery: false,
            },
            0x09 => CartridgeType::NoMbc {
                ram: true,
                battery: true,
            },
            0x0F => CartridgeType::Mbc3 {
                ram: false,
                battery: true,
                rtc: true,
            },
            0x10 => CartridgeType::Mbc3 {
                ram: true,
                battery: true,
                rtc: true,
            },
            0x11 => CartridgeType::Mbc3 {
                ram: false,
                battery: false,
                rtc: false,
            },
            0x12 => CartridgeType::Mbc3 {
                ram: true,
                battery: false,
                rtc: false,
            },
            0x13 => CartridgeType::Mbc3 {
                ram: true,
                battery: true,
                rtc: false,
            },
            0x19 => CartridgeType::Mbc5 {
                ram: false,
                battery: false,
                rumble: false,
            },
            0x1A => CartridgeType::Mbc5 {
                ram: true,
                battery: false,
                rumble: false,
            },
            0x1B => CartridgeType::Mbc5 {
                ram: true,
                battery: true,
                rumble: false,
            },
            0x1C => CartridgeType::Mbc5 {
                ram: false,
                battery: false,
                rumble: true,
            },
            0x1D => CartridgeType::Mbc5 {
                ram: true,
                battery: false,
                rumble: true,
            },
            0x1E => CartridgeType::Mbc5 {
                ram: true,
                battery: true,
                rumble: true,
            },
            0x20 => CartridgeType::Mbc6,
            0x22 => CartridgeType::Mbc7,
            0xFF => CartridgeType::Huc1,
            0xFE => CartridgeType::Huc3,
            _ => {
                panic!("UNIMPLEMENTED CartridgeType ??? {:6X}", value)
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
