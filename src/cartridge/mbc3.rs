#[derive(Copy, Clone, Debug)]
pub(crate) struct Mbc3
{
    pub(crate) rom_bank: u8,
    pub(crate) map_en: bool,
    pub(crate) map_select: u8,
    pub(crate) rom_offsets: (usize, usize),
    pub(crate) ram_offset: usize,
    pub(crate) mbc30: bool,
}

impl Mbc3 {
    pub(crate) fn new(ramsize: usize) -> Mbc3 {
        Mbc3 {
            rom_bank: 1,
            map_en: false,
            map_select: 0,
            rom_offsets: (0x0000, 0x4000),
            ram_offset: 0x0000,
            mbc30: ramsize > 65536
        }
    }
}
