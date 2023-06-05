#[allow(dead_code)]
pub(crate) enum MbcType {
    None,
    Mbc1,
    Mbc2,
    Mbc3,
    Mbc4,
    Mbc5,
}

#[allow(dead_code)]
pub(crate) struct Mbc {
    mbc_type: MbcType,
    rom_bank: usize,
    ram_bank: usize,
    rom: Vec<u8>,
    ram: Vec<u8>,
}

impl Mbc {
    pub(crate) fn new(data: &[u8]) -> Mbc {
        let rom_size = data[0x0148] as usize;
        let ram_size = data[0x0148] as usize;
        let mbc_type = match data[0x014] {
            01 => MbcType::Mbc1,
            _ => MbcType::None,
        };

        Mbc {
            mbc_type,
            rom_bank: 0,
            ram_bank: 0,
            rom: data[..rom_size].to_vec(),
            ram: vec![0; ram_size],
        }
    }
}
