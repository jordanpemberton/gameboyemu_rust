#[allow(dead_code)]
const ROM_BANK_SIZE: u16 = 0x4000;
#[allow(dead_code)]
const RAM_BANK_SIZE: u16 = 0x2000;

#[allow(dead_code)]
struct Mbc {
    rom_bank: usize,
    ram_bank: usize,
    rom: Vec<u8>,
    ram: Vec<u8>,
}


