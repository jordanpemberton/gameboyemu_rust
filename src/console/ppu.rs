#![allow(unused_assignments)]

use std::collections::HashMap;
use sdl2::event::Event::Window;
use sdl2::pixels::Color;
use sdl2::render::TextureAccess::Static;

use crate::console::display::Display;
use crate::console::interrupts::Interrupts;
use crate::console::mmu::{Endianness, Mmu};

pub(crate) const LCD_PIXEL_WIDTH: u32 = 160;
pub(crate) const LCD_PIXEL_HEIGHT: u32 = 144;

const TILE_PIXEL_WIDTH: u16 = 8;
const TILE_PIXEL_HEIGHT: u16 = 8;
const MAP_TILE_WIDTH: u16 = 32;
const MAP_TILE_HEIGHT: u16 = 32;
const MAP_PIXEL_WIDTH: u16 = 256;
const MAP_PIXEL_HEIGHT: u16 = 256;

const STAT_REG_ADDRESS: u16 = 0xFF41;
const LY_REG_ADDRESS: u16 = 0xFF44;
const LYC_REG_ADDRESS: u16 = 0xFF45;

const MODE_CLOCK_RANGE: [(u16, u16); 4] = [
    (80 + 172, 80 + 172 + 204), // HBlank [0]
    (0, 456),                   // VBlank [1]
    (0, 80),                    // OAM [2]
    (80, 80 + 172),             // PixelTransfer [3]
];

const MODE_LINE_RANGE: [(u8, u8); 4] = [
    (0, 144),
    (144, 154),
    (0, 144),
    (0, 144),
];

pub(crate) struct Lcd {
    pub(crate) data: [[u8; LCD_PIXEL_WIDTH as usize]; LCD_PIXEL_HEIGHT as usize],
}

impl Lcd {
    pub(crate) fn new() -> Lcd {
        Lcd {
            data: [[0; LCD_PIXEL_WIDTH as usize]; LCD_PIXEL_HEIGHT as usize]
        }
    }
}


#[derive(Clone,Copy)]
pub(crate) struct Tile {
    /// OAM VRAM = $FE00-FE9F (40 sprites)
    /// OAM Entry:
    tile_index: u8,     // byte 2
    x: u8,              // byte 1
    y: u8,              // byte 0
    flip_x: u8,
    flip_y: u8,
    priority: u8,
    palette: u8,

    pub(crate) data: [[u8; TILE_PIXEL_WIDTH as usize]; TILE_PIXEL_HEIGHT as usize],
}

impl Tile {
    fn new() -> Tile {
        Tile {
            tile_index: 0,
            x: 0,
            y: 0,
            flip_x: 0,
            flip_y: 0,
            priority: 0,
            palette: 0,

            data: [[0; TILE_PIXEL_WIDTH as usize]; TILE_PIXEL_HEIGHT as usize],
        }
    }

    fn read_tile(tile_bytes: [u8; 16]) -> Tile {
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

    fn read_tiles(tile_data_buffer: &Vec<u8>) -> Vec<Tile> {
        let mut tiles: Vec<Tile> = Vec::from([]);
        let n = tile_data_buffer.len();
        let mut tile_bytes: [u8; 16];
        let mut i = 0;

        while i + 16 <= n {
            tile_bytes = [0; 16];
            tile_bytes.clone_from_slice(&tile_data_buffer[i..i+16]);

            let tile = Tile::read_tile(tile_bytes);
            tiles.push(tile);

            i += 16;
        }

        tiles
    }
}

pub(crate) struct TileMap {
    tile_indexes: [u8; 32 * 32],
    tiles: [[Tile; 32]; 32],
}

impl TileMap {
    fn new(tilemap_address: usize, mmu: &mut Mmu) -> TileMap {
        let mut tile_map = TileMap {
            tile_indexes: [0; 32 * 32],
            tiles: [[Tile::new(); 32]; 32],
        };
        tile_map.fetch_tiles(tilemap_address, mmu);
        tile_map
    }

    fn to_lcd(&self, palettes: &[[u8; 4]; 4]) -> Lcd {
        let mut lcd = Lcd::new();

        // TODO sort by priority
        for row in 0..32 {
            for col in 0..32 {
                let tile = self.tiles[row][col];
                for i in 0..TILE_PIXEL_WIDTH as usize {
                    for j in 0..TILE_PIXEL_HEIGHT as usize {
                        let pixel = tile.data[i][j];
                        let palette = palettes[tile.palette as usize];

                        if row + i >= LCD_PIXEL_HEIGHT as usize || col + j >= LCD_PIXEL_WIDTH as usize {
                            break;
                        }
                        lcd.data[row + i][col + j] = palette[pixel as usize];
                        // TODO use tile attr (x,y): lcd[tile.y as usize][tile.x as usize] = palette[pixel as usize];
                    }
                }
            }
        }

        lcd
    }

    fn fetch_tiles(&mut self, tilemap_address: usize, mmu: &mut Mmu) {
        let tile_indexes = mmu.read_buffer(
            tilemap_address,
            tilemap_address + 32 * 32 + 1,
            Endianness::BIG);
        self.tile_indexes = tile_indexes
            .try_into().expect("Casting vec<u8> to [u8; 1024] failed.");

        // TODO
        let tiledata_location: usize = 0x8000; // 0x8800;

        for row in 0..32 {
            for col in 0..32 {
                let tile_index = self.tile_indexes[row * 32 + col] as usize;
                let tile_address = tiledata_location + tile_index;
                let tile_bytes = mmu.read_buffer(tile_index, tile_index + 16, Endianness::BIG)
                    .try_into().expect("Casting vec<u8> to [u8; 16] failed.");
                self.tiles[row][col] = Tile::read_tile(tile_bytes);
            }
        }
    }
}

/*
<-20/80c-> <------43/172c------> <--51/204c->
OAM [2]   |  PixelTransfer [3]  |  HBlank [0]    (144 lines)
--------------------------------------------
                VBlank [1]                      (10 lines)
<--------------114/456c-------------------->
 */
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum StatMode {
    HBlank = 0,
    VBlank = 1,
    OamSearch = 2,
    PixelTransfer = 3,
}

pub(crate) struct Ppu {
    clocks: u16,

    // Registers -- in IO RAM
    bgp: u8,   // palette
    // Bit 7-6 - Color for index 3
    // Bit 5-4 - Color for index 2
    // Bit 3-2 - Color for index 1
    // Bit 1-0 - Color for index 0

    ly: u8,    // scanline,     0xFF44
    lyc: u8,   // LY compare,   0xFF45
    scx: u8,
    scy: u8,

    mode: StatMode,     // 0xFF41
    pub(crate) palette: u8,
    pub(crate) palettes: [[u8; 4]; 4],

    // Tilemaps
    tilemap_9800: TileMap,
    tilemap_9C00: TileMap,

    // Display output buffers
    pub(crate) background: Lcd,
    pub(crate) window: Lcd,
    pub(crate) sprites: Lcd,
}

impl Ppu {
    pub(crate) fn new(mmu: &mut Mmu) -> Ppu {
        Ppu {
            clocks: 0,
            bgp: 0,
            ly: 0,
            lyc: 0,
            scy: 0,
            scx: 0,
            mode: StatMode::HBlank,
            palette: 0,
            palettes: // TODO
                [
                    [0,1,2,3],
                    [0,1,2,3],
                    [0,1,2,3],
                    [0,1,2,3],
                ],
            tilemap_9800: TileMap::new(0x9800, mmu),
            tilemap_9C00: TileMap::new(0x9C00, mmu),

            background: Lcd::new(),
            window: Lcd::new(),
            sprites: Lcd::new(),
        }
    }

    pub(crate) fn step(&mut self, incoming_interrupts: &Interrupts, mmu: &mut Mmu) -> Interrupts {
        // TODO check incoming interrupt requests /enables

        let mut outgoing_interrupts = Interrupts::new();
        let mut mode_misalignment_error: bool = false;

        match self.mode {
            StatMode::OamSearch => {
                if Ppu::mode_is_in_line_range(StatMode::OamSearch, self.ly) {
                    self.oam_search(mmu);
                    if !Ppu::mode_is_in_clock_range(StatMode::OamSearch, self.clocks) {
                        self.clocks = 0;
                        self.mode = StatMode::PixelTransfer;
                        outgoing_interrupts.lcd_stat_request = true;
                        outgoing_interrupts.vblank_request = true;
                    }
                } else {
                    mode_misalignment_error = true;
                }
            }
            StatMode::PixelTransfer => {
                if Ppu::mode_is_in_line_range(StatMode::PixelTransfer, self.ly) {
                    self.pixel_transfer(mmu);
                    if !Ppu::mode_is_in_clock_range(StatMode::PixelTransfer, self.clocks) {
                        self.clocks = 0;
                        self.mode = StatMode::HBlank;
                    }
                } else {
                    mode_misalignment_error = true;
                }
            }
            StatMode::HBlank => {
                if Ppu::mode_is_in_line_range(StatMode::HBlank, self.ly) {
                    self.hblank(mmu);
                    if !Ppu::mode_is_in_clock_range(StatMode::HBlank, self.clocks) {
                        self.clocks = 0;
                        if Ppu::mode_is_in_line_range(StatMode::OamSearch, self.ly) {
                            self.mode = StatMode::OamSearch;
                        } else {
                            self.mode = StatMode::VBlank;
                            outgoing_interrupts.vblank_request = true;
                        }
                    }
                } else {
                    mode_misalignment_error = true;
                }
            }
            StatMode::VBlank => {
                if Ppu::mode_is_in_line_range(StatMode::VBlank, self.ly) {
                    self.vblank(mmu);
                    if !Ppu::mode_is_in_clock_range(StatMode::VBlank, self.clocks) {
                        self.clocks = 0;
                        if Ppu::mode_is_in_line_range(StatMode::OamSearch, self.ly) {
                            self.mode = StatMode::OamSearch;
                        }
                    }
                } else {
                    mode_misalignment_error = true;
                }
            }
            _ => {
                mode_misalignment_error = true;
            }
        };

        if mode_misalignment_error {
            panic!("Line {} is out of range for Mode {:?}.", self.ly, self.mode);
        }

        self.check_set_lyc(mmu);

        outgoing_interrupts
    }

    fn check_set_lyc(&self, mmu: &mut Mmu) {
        if self.ly == self.lyc {
            let stat_reg = mmu.read_byte(STAT_REG_ADDRESS) | 0b0000_0100;
            mmu.load_byte(STAT_REG_ADDRESS, stat_reg);
        }
    }

    fn increment_ly(&mut self, mmu: &mut Mmu) {
        self.ly += 1;

        if self.ly >= MODE_LINE_RANGE[StatMode::VBlank as usize].1 {
            self.ly = 0;
        }
        mmu.load_byte(LY_REG_ADDRESS, self.ly);
    }

    fn mode_is_in_line_range(mode: StatMode, line: u8) -> bool {
        MODE_LINE_RANGE[mode as usize].0 <= line
            && line < MODE_LINE_RANGE[mode as usize].1
    }

    fn mode_is_in_clock_range(mode: StatMode, clocks: u16) -> bool {
        MODE_CLOCK_RANGE[mode as usize].0 <= clocks
            && clocks < MODE_CLOCK_RANGE[mode as usize].1
    }

    fn oam_search(&mut self, mmu: &mut Mmu) {
        // TODO

        self.clocks += 80;
    }

    fn pixel_transfer(&mut self, mmu: &mut Mmu) {
        // TODO Pixel FIFO / fetch /load
        self.tilemap_9800.fetch_tiles(0x9800, mmu);
        self.background = self.tilemap_9800.to_lcd(&self.palettes);

        // self.window = self.tilemap_9C00.to_lcd(&self.palettes);

        self.clocks += 172;
    }

    fn hblank(&mut self, mmu: &mut Mmu) {
        self.increment_ly(mmu);

        self.clocks += 204;
    }

    fn vblank(&mut self, mmu: &mut Mmu) {
        self.increment_ly(mmu);

        self.clocks += 456;
    }
}
