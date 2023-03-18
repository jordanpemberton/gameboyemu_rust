use std::collections::HashMap;
use sdl2::pixels::Color;

/*
    COLORS / PALETTES

    Memory location 0xFF47 is mapped to a special register of the LCD Display device.
    The register does the mapping between the four possible values of the Game Boy’s colors (0, 1, 2, 3)
    to actual colors (white, light gray, dark gray and black); ie, it initializes the color palette.

    The register at 0xFF47 is divided as follows:
        – Bits 7-6 – defines color number 3
        – Bits 5-4 – defines color number 2
        – Bits 3-2 – defines color number 1
        – Bits 1-0 – defines color number 0

    Each pair of bits can hold a value from 0 to 3. These values are interpreted as follows:
        - 0 is white
        - 1 is light gray
        - 2 is dark fray
        - 3 is black

    Example:
        In bootrom, the register was written with value 0xF3, which is binary 11110011.
        This means that color number 0 is assigned black, as well as colors number 2 and 3;
        color number 1 is assigned white.
 */

/*
    TILE DATA

    Tile data is stored in VRAM in the memory area at $8000-$97FF; with each tile taking 16 bytes,
    this area defines data for 384 tiles.

    There are three “blocks” of 128 tiles each:
        $8000–$87FF
        $8800–$8FFF
        $9000–$97FF 	(Can't use)

    Each tile occupies 16 bytes, where each line is represented by 2 bytes:
        Byte 0-1    Topmost line  (top 8 pixels)
        Byte 2-3    Second line   (next 8 pixels)
        ...
        Byte 14-15  Bottom line

    Gameboy tile example:
        0x3C 0x7E 0x42 0x42 0x42 0x42 0x42 0x42
        0x7E 0x5E 0x7E 0x0A 0x7C 0x56 0x38 0x7C
*/

/*
    TILE MAPS

    The Game Boy contains two 32x32 tile maps in VRAM at the memory areas $9800-$9BFF and $9C00-$9FFF.
    Any of these maps can be used to display the Background or the Window.

    Each tile map contains the 1-byte indexes of the tiles to be displayed.
    Tiles are obtained from the Tile Data Table using either of the two addressing modes,
    selected via the LCDC register.

    Since one tile has 8x8 pixels, each map holds a 256x256 pixels picture.
    Only 160x144 of those pixels are displayed on the LCD at any given time.
*/

type Tile = [[u8; 8]; 8];

pub(crate) struct Ppu {
    palettes: [[u8; 4]; 4],
}

impl Ppu {
    pub(crate) fn new(palettes: [[u8; 4]; 4]) -> Ppu {
        Ppu {
            palettes: palettes,
        }
    }

    pub(crate) fn read_tile(&self, tile_bytes: [u8; 16]) -> Tile {
        let mut tile: Tile = [[0; 8]; 8];

        for row in 0..8 {
            for col in 0..8 {
                // Possible values = 0,1,2,3 (0b00,0b01,0b10,0b11)
                let low = ((tile_bytes[row * 2] << col) >> 7) << 1;
                let high = (tile_bytes[row * 2 + 1] << col) >> 7;
                tile[row][col] = high + low;
            }
        }
        tile
    }

    pub(crate) fn read_tiles(&self, tile_data_buffer: &[u8]) -> Vec<Tile> {
        let mut tiles: Vec<Tile> = Vec::from([]);
        let n = tile_data_buffer.len();
        let mut tile_data: [u8; 16];
        let mut i = 0;

        while i + 16 <= n {
            tile_data = [0; 16];
            tile_data.clone_from_slice(&tile_data_buffer[i..i+16]);
            let tile = self.read_tile(tile_data);
            tiles.push(tile);
            i += 16;
        }

        tiles
    }
}
