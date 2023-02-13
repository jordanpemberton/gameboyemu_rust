#![allow(unused_variables)]

// Tile data is stored between 0x8000 and 0x97FF
pub(crate) const TILE_SET_SIZE: usize = 384;

// VRAM Sprite Attribute Table = OAM (Object Attribute Memory)
pub(crate) const VRAM_BEGIN: usize = 0x8000;
pub(crate) const VRAM_END: usize = 0x9FFF;
pub(crate) const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;

#[derive(Clone, Copy)]
enum PixelColor { _00, _01, _10, _11 }

type Tile = [[PixelColor; 8]; 8];

fn empty_tile() -> Tile {
    [[PixelColor::_00; 8]; 8]
}

fn empty_tile_set() -> [Tile; TILE_SET_SIZE] {
    [empty_tile(); TILE_SET_SIZE]
}

pub(crate) struct Gpu {
    vram: [u8; VRAM_SIZE],
    tile_set: [Tile; TILE_SET_SIZE],
}

impl Gpu {
    pub(crate) fn new() -> Self {
        Self {
            tile_set: empty_tile_set(),
            vram: [0; VRAM_SIZE],
        }
    }

    fn read_vram(&self, address: usize) -> u8 {
        self.vram[address]
    }

    fn write_vram(&mut self, address: usize, value: u8) {
        self.vram[address] = value;
    }

    fn update_tileset(&mut self, address: usize) {
        if address >= 0x1800 {
            return
        }

        // Tiles rows are two bytes starting on an even address.
        // Bitwise ANDing the address with 0xFFFE gives us the address of the first byte.
        let normalized_index = address & 0xFFFE;
        let byte1 = self.vram[normalized_index];
        let byte2 = self.vram[normalized_index + 1];

        // 8 rows of 2 byte tiles = 16 bytes
        let tile_index = address / 16;
        let row_index = (address % 16) / 2;

        // Get the 8 pixels that make up a given row.
        for pixel_index in 0..8 {
            // Mask pixel
            let mask = 1 << (7 - pixel_index);
            let lsb = byte1 & mask;
            let msb = byte2 & mask;

            let color = match (lsb != 0, msb != 0) {
                (false, false) => PixelColor::_00,
                (true, false) => PixelColor::_01,
                (false, true) => PixelColor::_10,
                (true, true) => PixelColor::_11,
            };

            self.tile_set[tile_index][row_index][pixel_index] = color;
        }
    }
}
