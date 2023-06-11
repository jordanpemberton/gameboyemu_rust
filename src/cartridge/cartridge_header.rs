#[allow(dead_code)]
pub(crate) enum CartridgeType {
    None,
    Mbc1,
    Mbc2,
    Mbc3,
    Mbc5,
}

#[allow(dead_code)]
pub(crate) struct CartridgeHeader {
    pub(crate) rom_size: usize,
    pub(crate) ram_size: usize,
    pub(crate) cartridge_type: CartridgeType,
}

impl CartridgeHeader {
    pub(crate) fn new(data: &[u8]) -> CartridgeHeader {
        let rom_size = data[0x0148] as usize;
        let ram_size = data[0x0148] as usize;
        let cartridge_type = match data[0x014] {
            01 => CartridgeType::Mbc1,
            _ => CartridgeType::None,
        };

        CartridgeHeader {
            rom_size,
            ram_size,
            cartridge_type,
        }
    }
}
