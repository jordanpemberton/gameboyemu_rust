#![allow(unused_variables)]

// Tile data is stored between 0x8000 and 0x97FF
pub(crate) const TILE_SET_SIZE: usize = 384;
pub(crate) const VRAM_BEGIN: usize = 0x8000;
pub(crate) const VRAM_END: usize = 0x9FFF;
pub(crate) const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;


#[derive(Clone, Copy)]
enum PixelColor { _0, _1, _2, _3 }

type Tile = [[PixelColor; 8]; 8];

fn empty_tile() -> Tile {
    [[PixelColor::_0; 8]; 8]
}

fn empty_tile_set() -> [Tile; TILE_SET_SIZE] {
    [empty_tile(); TILE_SET_SIZE]
}

pub(crate) struct Gpu {
    vram: [u8; VRAM_SIZE],
    tile_set: [Tile; TILE_SET_SIZE],
}

impl Gpu {
     pub(crate) fn default() -> Gpu {
        Gpu {
            vram: [0; VRAM_SIZE],
            tile_set: empty_tile_set(),
        }
    }

    pub(crate) fn read_vram(&self,addr: usize) -> u8 { 0 }

    pub(crate) fn write_vram(&mut self, index: usize, value: u8) {
        self.vram[index] = value;

        if index >= 0x1800 { return }

        // Tiles rows are two bytes starting on an even address.
        // Bitwise ANDing the address with 0xffe gives us the address of the first byte.
        let normalized_index = index & 0xFFFE;

        let byte1 = self.vram[normalized_index];
        let byte2 = self.vram[normalized_index + 1];

        // 8 rows of 2 byte tiles = 16 bytes
        let tile_index = index / 16;
        let row_index = (index % 16) / 2;

        // Get the 8 pixels that make up a given row.
        for pixel_index in 0..8 {
            // Mask pixel
            let mask = 1 << (7 - pixel_index);
            let lsb = byte1 & mask;
            let msb = byte2 & mask;

            let color = match (lsb != 0, msb != 0) {
                (false, false) => PixelColor::_0,
                (true, false) => PixelColor::_1,
                (false, true) => PixelColor::_2,
                (true, true) => PixelColor::_3,
            };

            self.tile_set[tile_index][row_index][pixel_index] = color;
        }
    }
}
