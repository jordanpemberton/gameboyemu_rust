use std::collections::HashMap;
use std::fs::read;
use std::hash::Hash;
use std::path::Path;

#[derive(PartialEq)]
pub(crate) enum Endianness {
    BIG,
    LITTLE,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub(crate) enum MemoryType {
    BOOT_ROM,
    CARTRIDGE_ROM,
    VRAM,
    HRAM,
    DEBUG,
}

struct MemoryLocation {
    begin: u16,
    end: u16,
    size: u16,
}

impl MemoryLocation {
    fn new(begin: u16, end: u16) -> MemoryLocation {
        MemoryLocation {
            begin,
            end,
            size: end - begin,
        }
    }
}

pub(crate) struct Mmu {
    memory: [u8; 0xFFFF as usize],
    memory_locations: HashMap<MemoryType, MemoryLocation>,
}

impl Mmu {
    pub(crate) fn new() -> Mmu {
        let mut mmu = Mmu {
            memory: [0; 0xFFFF],
            memory_locations: Default::default(),
        };
        mmu.init();
        mmu
    }

    pub(crate) fn init(&mut self) {
        let bootrom_filepath = "/home/jordan/RustProjs/GameBoyEmu/src/console/DMG_ROM.bin";
        self.load_rom_from_file(bootrom_filepath.as_ref());

        self.memory_locations = HashMap::from([
            ( MemoryType::BOOT_ROM, MemoryLocation::new(0x0000, 0x0100) ),
            ( MemoryType::CARTRIDGE_ROM, MemoryLocation::new(0x0000, 0x7FFF) ),
            ( MemoryType::VRAM, MemoryLocation::new(0x8000, 0x9FFF) ),
            ( MemoryType::HRAM, MemoryLocation::new(0xFF80, 0xFFFF) ),
            ( MemoryType::DEBUG, MemoryLocation::new(0x0000, 0xFFFF) ),
        ]);
    }

    pub(crate) fn load_rom_from_file(&mut self, filepath: &Path) {
        let rom = &*(read(filepath).unwrap());
        // TODO read cartridges correctly
        let s: usize = 0;
        let n: usize = if rom.len() <= self.memory.len() { rom.len() } else { self.memory.len() };
        self.memory[s..n].clone_from_slice(&rom[..n]);
    }

    fn validate_address_in_bounds(&self, address: u16, begin: u16, end: u16) {
        if address < begin || end < address {
            panic!("Out of bounds error: Address {} is not in range {}..{}.", address, begin, end);
        }
    }

    fn get_memory_type_from_address(&self, address: u16) -> MemoryType {
        for (memory_type, location) in self.memory_locations.iter() {
            if location.begin <= address && address <= location.end {
                return *memory_type;
            }
        }
        panic!("Out of bounds error: No memory type found for address {}", address);
    }

    pub(crate) fn read_byte(&self, address: u16) -> u8 {
        let memory_type = self.get_memory_type_from_address(address);
        self.read_byte_from_internal(address, memory_type)
    }

    pub(crate) fn read_byte_from(&self, address: u16, memory_type: MemoryType) -> u8 {
        let location = self.memory_locations.get(&memory_type).unwrap();
        self.validate_address_in_bounds(address, location.begin, location.end);
        self.read_byte_from_internal(address, memory_type)
    }

    fn read_byte_from_internal(&self, address: u16, memory_type: MemoryType) -> u8 {
        match memory_type {
            _ => {
                self.memory[address as usize]
            }
        }
    }

    pub(crate) fn read_word(&self, address: u16, endian: Endianness) -> u16 {
        let memory_type = self.get_memory_type_from_address(address);
        self.read_word_from_internal(address, endian, memory_type)
    }

    pub(crate) fn read_word_from(&self, address: u16, endian: Endianness, memory_type: MemoryType) -> u16 {
        let location = self.memory_locations.get(&memory_type).unwrap();
        self.validate_address_in_bounds(address, location.begin, location.end);
        self.read_word_from_internal(address, endian, memory_type)
    }

    fn read_word_from_internal(&self, address: u16, endian: Endianness, memory_type: MemoryType) -> u16 {
        let lsb: u8;
        let msb: u8;

        match memory_type {
            _ => {
                lsb = self.memory[address as usize];
                msb = self.memory[(address + 1) as usize];
            }
        }

        match endian {
            Endianness::BIG => (msb as u16) << 8 | lsb as u16,
            Endianness::LITTLE | _ => (lsb as u16) << 8 | msb as u16,
        }
    }

    pub(crate) fn load_byte(&mut self, address: u16, value: u8) {
        let memory_type = self.get_memory_type_from_address(address);
        self.load_byte_to_internal(address, value, memory_type);
    }

    pub(crate) fn load_byte_to(&mut self, address: u16, value: u8, memory_type: MemoryType) {
        let location = self.memory_locations.get(&memory_type).unwrap();
        self.validate_address_in_bounds(address, location.begin, location.end);
        self.load_byte_to_internal(address, value, memory_type);
    }

    fn load_byte_to_internal(&mut self, address: u16, value: u8, memory_type: MemoryType) {
        match memory_type {
            _ => {
                self.memory[address as usize] = value;
            }
        }
    }

    pub(crate) fn load_word(&mut self, address: u16, value: u16, endian: Endianness) {
        let memory_type = self.get_memory_type_from_address(address);
        self.load_word_to_internal(address, value, endian, memory_type);
    }

    pub(crate) fn load_word_to(&mut self, address: u16, value: u16, endian: Endianness, memory_type: MemoryType) {
        let location = self.memory_locations.get(&memory_type).unwrap();
        self.validate_address_in_bounds(address, location.begin, location.end);
        self.load_word_to_internal(address, value, endian, memory_type);
    }

    fn load_word_to_internal(&mut self, address: u16, value: u16, endian: Endianness, memory_type: MemoryType) {
        let mut lsb = ((value & 0xFF00) >> 8) as u8;
        let mut msb = (value & 0x00FF) as u8;
        if endian == Endianness::BIG {
            let temp = lsb;
            lsb = msb;
            msb = temp;
        }

        match memory_type {
            _ => {
                self.memory[address as usize] = lsb;
                self.memory[(address + 1) as usize] = msb;
            }
        }
    }
}
