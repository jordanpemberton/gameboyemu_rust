#![allow(unused_assignments)]

use std::collections::HashMap;
use sdl2::event::Event::Window;
use sdl2::pixels::Color;

use crate::console::display::Display;
use crate::console::interrupts::Interrupts;
use crate::console::mmu::{Endianness, Mmu};

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

const TILE_PIXEL_WIDTH: u16 = 8;
const TILE_PIXEL_HEIGHT: u16 = 8;
const MAP_TILE_WIDTH: u16 = 32;
const MAP_TILE_HEIGHT: u16 = 32;
const MAP_PIXEL_WIDTH: u16 = 256;
const MAP_PIXEL_HEIGHT: u16 = 256;
pub(crate) const LCD_PIXEL_WIDTH: u32 = 160;
pub(crate) const LCD_PIXEL_HEIGHT: u32 = 144;

type Lcd = [[u8; LCD_PIXEL_WIDTH as usize]; LCD_PIXEL_HEIGHT as usize];

#[derive(Clone,Copy)]
pub(crate) struct Tile {
    pub(crate) data: [[u8; TILE_PIXEL_WIDTH as usize]; TILE_PIXEL_HEIGHT as usize],
}

impl Tile {
    fn new() -> Tile {
        Tile {
            data: [[0; TILE_PIXEL_WIDTH as usize]; TILE_PIXEL_HEIGHT as usize],
        }
    }
}

pub(crate) struct TileMap {
    enabled: bool,
    position: (u8, u8),
    tile_map: [[Tile; MAP_TILE_WIDTH as usize]; MAP_TILE_HEIGHT as usize],
}

impl TileMap {
    fn new() -> TileMap {
        TileMap {
            enabled: true,
            position: (0, 0),
            tile_map: [[Tile::new(); MAP_TILE_WIDTH as usize]; MAP_TILE_HEIGHT as usize],
        }
    }

    pub(crate) fn to_lcd(&self, palette: [u8; 4]) -> [[u8; LCD_PIXEL_WIDTH as usize]; LCD_PIXEL_HEIGHT as usize] {
        let mut lcd = [[0; LCD_PIXEL_WIDTH as usize]; LCD_PIXEL_HEIGHT as usize];

        // TODO map position offset
        for row in 0..LCD_PIXEL_HEIGHT as usize {
            for col in 0..LCD_PIXEL_WIDTH as usize {
                let tile_row = row / TILE_PIXEL_HEIGHT as usize;
                let tile_col = col / TILE_PIXEL_WIDTH as usize;
                let tile_y = row % TILE_PIXEL_HEIGHT as usize;
                let tile_x = col % TILE_PIXEL_WIDTH as usize;
                let tile = self.tile_map[tile_row][tile_col];
                let pixel = palette[tile.data[tile_y][tile_x] as usize];
                lcd[row][col] = pixel;
            }
        }

        lcd
    }
}

#[derive(Clone, Copy)]
enum PpuMode {
    HBLANK,
    OAM,
    VBLANK,
    VRAM,
}

pub(crate) struct Ppu {
    tick: u16,

    // Registers -- in IO RAM
    bgp: u8,   // palette
    // Bit 7-6 - Color for index 3
    // Bit 5-4 - Color for index 2
    // Bit 3-2 - Color for index 1
    // Bit 1-0 - Color for index 0

    ly: u8,    // scanline
    // LY can hold any value from 0 to 153, with values from 144 to 153 indicating the VBlank period.
    lyc: u8,   // LY compare
    // The Game Boy constantly compares the value of the LYC and LY registers.
    // When both values are identical, the “LYC=LY” flag in the STAT register is set,
    // and (if enabled) a STAT interrupt is requested.

    // The SCY and SCX registers can be used to scroll the Background,
    // specifying the origin of the visible 160x144 pixel area within the total 256x256 pixel Background map.
    scx: u8,
    scy: u8,

    mode: PpuMode,
    pub(crate) palette: u8,
    pub(crate) palettes: [[u8; 4]; 4],
    interrupts: Interrupts,

    // Display output buffers
    pub(crate) background: TileMap,
    pub(crate) window: TileMap,
    pub(crate) sprites: TileMap,
}

impl Ppu {
    pub(crate) fn new(palettes: [[u8; 4]; 4]) -> Ppu {
        Ppu {
            tick: 0,

            bgp: 0,
            ly: 0,
            lyc: 0,
            scy: 0,
            scx: 0,

            mode: PpuMode::HBLANK,
            palette: 0,
            palettes: palettes,
            interrupts: Interrupts{
                enabled: true,
                hblank: false,
                lcd: false,
                oam: false,
                vblank: false,
            },

            background: TileMap::new(),
            window: TileMap::new(),
            sprites: TileMap::new(),
        }
    }

    pub(crate) fn step(&mut self, cycles: u16, interrupt_request: &mut Interrupts, mmu: &mut Mmu) {
        let mut new_tick: u16 = 0;
        let mut new_mode: PpuMode = PpuMode::HBLANK;

        match self.mode {
            PpuMode::HBLANK => {
                if cycles >= 240 {
                    (new_tick, new_mode) = self.hblank_mode_step(interrupt_request);
                }
            }
            PpuMode::VBLANK => {
                panic!("VBLANK!");
            }
            PpuMode::VRAM => {
                if self.tick >= 172 {
                    (new_tick, new_mode) = self.vram_mode_step(interrupt_request, mmu);
                } else {
                    (new_tick, new_mode) = (cycles, PpuMode::VRAM);
                }
            }
            _ => { }
        }

        if self.interrupts.lcd && self.ly == self.lyc {
            interrupt_request.lcd = true;
        }

        self.tick = new_tick;
        self.mode = new_mode;
    }

    fn read_tile(&self, tile_bytes: [u8; 16]) -> Tile {
        let mut tile = Tile::new();

        for row in 0..8 {
            for col in 0..8 {
                // Possible values = 0,1,2,3 (0b00,0b01,0b10,0b11)
                let low = ((tile_bytes[row * 2] << col) >> 7) << 1;
                let high = (tile_bytes[row * 2 + 1] << col) >> 7;
                tile.data[row][col] = high + low;
            }
        }
        tile
    }

    fn read_tiles(&self, tile_data_buffer: &Vec<u8>) -> Vec<Tile> {
        let mut tiles: Vec<Tile> = Vec::from([]);
        let n = tile_data_buffer.len();
        let mut tile_bytes: [u8; 16];
        let mut i = 0;

        while i + 16 <= n {
            tile_bytes = [0; 16];
            tile_bytes.clone_from_slice(&tile_data_buffer[i..i+16]);
            let tile = self.read_tile(tile_bytes);
            tiles.push(tile);
            i += 16;
        }

        tiles
    }

    fn fetch_maps(&mut self, mmu: &mut Mmu) {
        if self.ly >= LCD_PIXEL_HEIGHT as u8 {
            return;
        }

        if self.background.enabled {
            self.fetch_background_map(mmu);
        }
    }

    fn fetch_background_map(&mut self, mmu: &mut Mmu) {
        // TODO read position
        self.background.position = (0, 0);

        // TODO read background map from vram correctly
        // Map tiles at $9800-$9BFF and $9C00-$9FFF.
        let begin = 0x9800;
        let end = 0x10000;
        let ram = mmu.read_buffer(begin, end, Endianness::BIG);

        let tiles = self.read_tiles(&ram);

        for row in 0..MAP_TILE_HEIGHT as usize {
            for col in 0..MAP_TILE_WIDTH as usize {
                let index = row * MAP_TILE_WIDTH as usize + col;
                if index >= tiles.len() {
                    break;
                }
                self.background.tile_map[row][col] = tiles[index];
            }
        }
    }

    // TODO interrupts
    fn hblank_mode_step(&mut self, interrupt_request: &mut Interrupts) -> (u16, PpuMode) {
        let tick = 0;
        let mode: PpuMode;

        self.ly += 1;

        if self.ly == 143 { // or 144 ?
            if interrupt_request.enabled {
                interrupt_request.vblank = true; // ?
            }
            if self.interrupts.vblank {
                interrupt_request.lcd = true; // ?
            }
            mode = PpuMode::VBLANK;
        } else {
            if self.interrupts.oam {
                interrupt_request.lcd = true;
            }
            mode = PpuMode::OAM;
        }

        (tick, mode)
    }

    fn vram_mode_step(&mut self, interrupt_request: &mut Interrupts, mmu: &mut Mmu) -> (u16, PpuMode) {
        let tick = 0;
        let mode: PpuMode;

        // Write data to buffers
        self.fetch_maps(mmu);
        // (draw)

        if self.interrupts.hblank {
            interrupt_request.lcd = true;
        }
        mode = PpuMode::HBLANK;

        (tick, mode)
    }
}
