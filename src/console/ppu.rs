#![allow(unused_assignments)]

use std::collections::HashMap;
use std::num;
use sdl2::event::Event::Window;
use sdl2::pixels::Color;
use sdl2::render::BlendMode::Add;
use sdl2::render::TextureAccess::Static;

use crate::console::display::Display;
use crate::console::interrupts::{InterruptRegBit, Interrupts};
use crate::console::mmu::{Endianness, Mmu};

pub(crate) const LCD_PIXEL_WIDTH: u32 = 160;
pub(crate) const LCD_PIXEL_HEIGHT: u32 = 144;

const TILE_PIXEL_WIDTH: u16 = 8;
const TILE_PIXEL_HEIGHT: u16 = 8;
const MAP_TILE_WIDTH: u16 = 32;
const MAP_TILE_HEIGHT: u16 = 32;
const MAP_PIXEL_WIDTH: u16 = 256;
const MAP_PIXEL_HEIGHT: u16 = 256;

const LY_REG_ADDRESS: u16 = 0xFF44;
const LYC_REG_ADDRESS: u16 = 0xFF45;

const MODE_CLOCKS: [u16; 4] = [
    204,        // HBlank[0]
    456,        // VBlank [1]
    80,         // OAM [2]
    172,        // PixelTransfer [3]
];

const MODE_LINE_RANGE: [(u8, u8); 4] = [
    (0, 144),   // HBlank
    (144, 154), // VBlank
    (0, 144),   // OamSearch
    (0, 144),   // PixelTransfer
];

pub(crate) struct Lcd {
    pub(crate) data: [[u8; LCD_PIXEL_WIDTH as usize]; LCD_PIXEL_HEIGHT as usize],
}

impl Lcd {
    fn new() -> Lcd {
        Lcd {
            data: [[0; LCD_PIXEL_WIDTH as usize]; LCD_PIXEL_HEIGHT as usize]
        }
    }
}

#[derive(Clone,Copy)]
struct Tile {
    /// OAM VRAM = $FE00-FE9F (40 sprites)
    /// OAM Entry:
    tile_index: u8,     // byte 2
    x: u8,              // byte 1
    y: u8,              // byte 0
    flip_x: u8,
    flip_y: u8,
    priority: u8,
    palette: u8,

    data: [[u8; TILE_PIXEL_WIDTH as usize]; TILE_PIXEL_HEIGHT as usize],
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

struct TileMap {}

impl TileMap {
    fn to_lcd(tiles: &[[Tile; 32]; 32], palettes: &[[u8; 4]; 4]) -> Lcd {
        let mut lcd = Lcd::new();

        // TODO sort by priority
        for row in 0..32 {
            for col in 0..32 {
                let tile = tiles[row][col];
                let palette = palettes[tile.palette as usize];
                for i in 0..TILE_PIXEL_HEIGHT as usize {
                    let lcd_row = row * TILE_PIXEL_HEIGHT as usize + i;
                    if lcd_row >= LCD_PIXEL_HEIGHT as usize { break }
                    for j in 0..TILE_PIXEL_WIDTH as usize {
                        let lcd_col = col * TILE_PIXEL_WIDTH as usize + j;
                        if lcd_col >= LCD_PIXEL_WIDTH as usize { break }
                        let pixel = tile.data[i][j];
                        lcd.data[row * TILE_PIXEL_HEIGHT as usize + i][col * TILE_PIXEL_WIDTH as usize + j] = palette[pixel as usize];
                        // TODO use tile attr (x,y): lcd[tile.y as usize][tile.x as usize] = palette[pixel as usize];
                    }
                }
            }
        }

        lcd
    }

    fn fetch_tiles(mmu: &mut Mmu, tiledata_address: usize, index_mode_8000: bool) -> [[Tile; 32]; 32] {
        let indices: [u8; 32 * 32] = mmu.read_buffer(tiledata_address, tiledata_address + 0x400, Endianness::LITTLE)
            .try_into().unwrap();

        let mut tiles = [[Tile::new(); 32]; 32];

        for row in 0..32 {
            for col in 0..32 {
                let tile_index = indices[row * 32 + col] as i32;

                let address = if index_mode_8000 {
                    // The “$8000 method” uses $8000 as its base pointer and uses an unsigned addressing,
                    // meaning that tiles 0-127 are in block 0, and tiles 128-255 are in block 1.
                    0x8000 + tile_index * 16
                } else {
                    // The “$8800 method” uses $9000 as its base pointer and uses a signed addressing,
                    // meaning that tiles 0-127 are in block 2, and tiles -128 to -1 are in block 1.
                    // Or, tiles 0-127 from block 2 and tiles 128-255 from block 1. (?)
                    0x9000 + (if tile_index < 128 { tile_index } else { -(tile_index - 255) }) * 16
                } as usize;

                let tile_bytes: [u8; 16] = mmu.read_buffer(address, address + 16, Endianness::LITTLE)
                    .try_into().unwrap();
                tiles[row][col] = Tile::read_tile(tile_bytes);
            }
        }

        tiles
    }
}

enum LcdControlRegBit {
    // 0	BG and Window enable/priority	0=Off, 1=On
    BackgroundAndWindowEnabled = 0,
    // 1	OBJ enable	0=Off, 1=On
    ObjEnabled = 1,
    // 2	OBJ size	0=8x8, 1=8x16
    SpriteSizeIs16 = 2,
    // 3	BG tile map area	0=9800-9BFF, 1=9C00-9FFF
    BackgroundTilemapIsAt9C00 = 3,
    // 4	BG and Window tile data area	0=8800-97FF, 1=8000-8FFF
    TiledataIsAt8000 = 4,
    // 5	Window enable	0=Off, 1=On
    WindowEnabled = 5,
    // 6	Window tile map area	0=9800-9BFF, 1=9C00-9FFF
    WindowTilemapIsAt9C00 = 6,
    // 7	LCD and PPU enable	0=Off, 1=On
    LcdAndPpuEnabled = 7,
}

struct LcdControlRegister {
    address: u16,
    value: u8
}

impl LcdControlRegister {
    fn new() -> LcdControlRegister {
        LcdControlRegister {
            address: 0xFF40,
            value: 0,
        }
    }

    fn read_from_mem(&mut self, mmu: &mut Mmu) {
        self.value = mmu.read_byte(self.address);
    }

    fn write_to_mem(&self, mmu: &mut Mmu) {
        mmu.load_byte(self.address, self.value);
    }

    fn check_bit(&self, bit: LcdControlRegBit) -> bool {
        let b = bit as u8;
        (self.value & (1 << b)) >> b == 1
    }

    fn set_bit(&mut self, bit: LcdControlRegBit, value: bool, mmu: &mut Mmu) {
        let b = bit as u8;
        let x = self.value & (1 << b);
        self.value &= !x;
        self.value |= if value { 1 << b } else { 0 };
        self.write_to_mem(mmu);
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

enum LcdStatRegBit {
    // Bit 0-1 - Mode Flag
    ModeBit0 = 0,
    ModeBit1 = 1,

    // Bit 2 - LYC=LY Flag
    LycEqLy = 2,

    // STAT Interrupt bits
    // Bit 3 - Mode 0 HBlank STAT Interrupt source  (1=Enable)
    HBlankInterruptEnabled = 3,
    // Bit 4 - Mode 1 VBlank STAT Interrupt source  (1=Enable)
    VBlankInterruptEnabled = 4,
    // Bit 5 - Mode 2 OAM STAT Interrupt source     (1=Enable)
    OamInterruptEnabled = 5,
    // Bit 6 - LYC=LY STAT Interrupt source         (1=Enable)
    LcyInterruptEnabled = 6,
}

struct LcdStatusRegister {
    address: u16,
    value: u8,
    mode: StatMode,
}

impl LcdStatusRegister {
    fn new(mmu: &mut Mmu) -> LcdStatusRegister {
        let mut lcd_stat_reg = LcdStatusRegister {
            address: 0xFF41,
            value: 0,
            mode: StatMode::HBlank,
        };
        lcd_stat_reg.read_from_mem(mmu);
        lcd_stat_reg
    }

    fn read_from_mem(&mut self, mmu: &mut Mmu) {
        self.value = mmu.read_byte(self.address);
        let x = self.value & 0b0000_0011;
        self.mode = match x {
            0 => StatMode::HBlank,
            1 => StatMode::VBlank,
            2 => StatMode::OamSearch,
            3 => StatMode::PixelTransfer,
            _ => panic!("How did you get value {} from 2 bits.", x)
        }
    }

    fn write_to_mem(&self, mmu: &mut Mmu) {
        mmu.load_byte(self.address, self.value);
    }

    fn check_bit(&self, bit: LcdStatRegBit) -> bool {
        let b = bit as u8;
        (self.value & (1 << b)) >> b == 1
    }

    fn set_bit(&mut self, bit: LcdStatRegBit, value: bool, mmu: &mut Mmu) {
        let b = bit as u8;
        let x = self.value & (1 << b);
        self.value &= !x;
        self.value |= if value { 1 << b } else { 0 };
        self.write_to_mem(mmu);
    }

    fn set_mode(&mut self, mode: StatMode, mmu: &mut Mmu) {
        self.mode = mode;
        self.value >>= 2;
        self.value <<= 2;
        self.value |= mode as u8;
        self.write_to_mem(mmu);
    }

    fn set_lyc(&mut self, lyc_equals_ly: bool, mmu: &mut Mmu) {
        self.set_bit(LcdStatRegBit::LycEqLy, true, mmu);
    }
}

pub(crate) struct Ppu {
    clocks: usize,
    pub(crate) lcd_updated: bool,

    // Registers -- in IO RAM
    bgp: u8,        // palette
    // Bit 7-6 - Color for index 3
    // Bit 5-4 - Color for index 2
    // Bit 3-2 - Color for index 1
    // Bit 1-0 - Color for index 0

    ly: u8,    // scanline,     0xFF44
    lyc: u8,   // LY compare,   0xFF45
    scx: u8,
    scy: u8,

    lcd_control: LcdControlRegister,
    lcd_status: LcdStatusRegister,

    pub(crate) palette: u8,
    palettes: [[u8; 4]; 4],

    // Display output buffers
    pub(crate) background: Lcd,
    pub(crate) window: Lcd,
    pub(crate) sprites: Lcd,
}

impl Ppu {
    pub(crate) fn new(mmu: &mut Mmu) -> Ppu {
        // TODO
        let palettes = [
            [0,1,2,3],
            [0,1,2,3],
            [0,1,2,3],
            [0,1,2,3],
        ];

        let background = Lcd::new();
        let window = Lcd::new();
        let sprites = Lcd::new();

        Ppu {
            clocks: 0,
            lcd_updated: false,
            bgp: 0,
            ly: 0,
            lyc: 0,
            scy: 0,
            scx: 0,
            lcd_control: LcdControlRegister::new(),
            lcd_status: LcdStatusRegister::new(mmu),
            palette: 0,
            palettes,
            background,
            window,
            sprites,
        }
    }

    pub(crate) fn step(&mut self, cycles: u16, interrupts: &mut Interrupts, mmu: &mut Mmu) {
        self.clocks += cycles as usize;

        // Update
        self.lcd_control.read_from_mem(mmu);
        self.lcd_status.read_from_mem(mmu);

        // TODO check incoming interrupt requests /enables

        let (y0, y1) = MODE_LINE_RANGE[self.lcd_status.mode as usize];

        if self.ly < y0 || y1 <= self.ly {
            panic!("Line {} is out of range {}..{} for Mode {:?}.", self.ly, y0, y1, self.lcd_status.mode);
        } else {
            let t = MODE_CLOCKS[self.lcd_status.mode as usize] as usize;

            match self.lcd_status.mode {
                StatMode::OamSearch => {
                    if self.clocks < t {
                        self.oam_search(mmu);
                    } else {
                        self.clocks = 0;
                        self.lcd_status.set_mode(StatMode::PixelTransfer, mmu);
                        interrupts.requested.set_bit(InterruptRegBit::LcdStat, true, mmu);
                        interrupts.requested.set_bit(InterruptRegBit::VBlank, false, mmu);
                    }
                }
                StatMode::PixelTransfer => {
                    if self.clocks < t {
                        // TODO FIFO
                    } else {
                        self.pixel_transfer(mmu);
                        self.clocks = 0;
                        self.lcd_status.set_mode(StatMode::HBlank, mmu);
                        interrupts.requested.set_bit(InterruptRegBit::LcdStat, false, mmu);
                        interrupts.requested.set_bit(InterruptRegBit::VBlank, false, mmu);
                    }
                }
                StatMode::HBlank => {
                    if self.clocks < t {
                        // wait
                    } else {
                        self.increment_ly(mmu);
                        if self.ly < MODE_LINE_RANGE[StatMode::OamSearch as usize].1 as u8 {
                            self.lcd_status.set_mode(StatMode::OamSearch, mmu);
                            interrupts.requested.set_bit(InterruptRegBit::LcdStat, true, mmu);
                            interrupts.requested.set_bit(InterruptRegBit::VBlank, false, mmu);
                        } else {
                            self.lcd_status.set_mode(StatMode::VBlank, mmu);
                            interrupts.requested.set_bit(InterruptRegBit::LcdStat, false, mmu);
                            interrupts.requested.set_bit(InterruptRegBit::VBlank, true, mmu);
                        }
                    }
                }
                StatMode::VBlank => {
                    if self.clocks < t {
                        // wait
                    } else {
                        self.increment_ly(mmu);
                        if self.ly < MODE_LINE_RANGE[StatMode::OamSearch as usize].1 as u8 {
                            self.clocks = 0;
                            self.lcd_status.set_mode(StatMode::OamSearch, mmu);
                            interrupts.requested.set_bit(InterruptRegBit::LcdStat, true, mmu);
                            interrupts.requested.set_bit(InterruptRegBit::VBlank, false, mmu);
                        }
                    }
                }
                _ => {
                    panic!("Unrecognized mode {:?}.", self.lcd_status.mode);
                }
            }
        }

        if self.ly == self.lyc {
            self.lcd_status.set_lyc(true, mmu);
        }
    }

    fn increment_ly(&mut self, mmu: &mut Mmu) {
        self.ly += 1;
        self.clocks = 0;

        if self.ly >= MODE_LINE_RANGE[StatMode::VBlank as usize].1 {
            self.ly = 0;
        }
        mmu.load_byte(LY_REG_ADDRESS, self.ly);
    }

    fn oam_search(&mut self, mmu: &mut Mmu) {
        // TODO
    }

    fn pixel_transfer(&mut self, mmu: &mut Mmu) {
        // TODO implement pixel FIFO correctly
        let is_first_line = self.ly <= MODE_LINE_RANGE[StatMode::PixelTransfer as usize].0;
        let is_last_line = self.ly >= MODE_LINE_RANGE[StatMode::PixelTransfer as usize].1 - 1;
        if is_first_line {
            self.lcd_updated = false;
        }
        if is_last_line && !self.lcd_updated
        {
            self.lcd_control.read_from_mem(mmu);

            let bg_enabled = self.lcd_control.check_bit(LcdControlRegBit::BackgroundAndWindowEnabled);
            if bg_enabled {
                let index_mode_8000 = self.lcd_control.check_bit(LcdControlRegBit::TiledataIsAt8000);
                let bg_tilemap_address = if self.lcd_control.check_bit(LcdControlRegBit::BackgroundTilemapIsAt9C00) { 0x9C00 } else { 0x9800 };
                let tilemap = TileMap::fetch_tiles(mmu, bg_tilemap_address, index_mode_8000);
                self.background = TileMap::to_lcd(&tilemap, &self.palettes);
                self.lcd_updated = true;
            }
        }
    }
}
