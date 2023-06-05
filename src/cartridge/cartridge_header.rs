use crate::cartridge::mbc::MbcType;

#[allow(dead_code)]
pub(crate) struct CartridgeHeader {
    pub(crate) rom_size: usize,
    pub(crate) ram_size: usize,
    pub(crate) cartridge_type: MbcType,
}

impl CartridgeHeader {
    pub(crate) fn new(data: &[u8]) -> CartridgeHeader {
        let rom_size = data[0x0148] as usize;
        let ram_size = data[0x0148] as usize;
        let cartridge_type = match data[0x014] {
            01 => MbcType::Mbc1,
            _ => MbcType::None,
        };

        CartridgeHeader {
            rom_size,
            ram_size,
            cartridge_type,
        }
    }
}
