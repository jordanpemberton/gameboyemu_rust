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

pub(crate) const LCD_PIXEL_WIDTH: usize = 160;
pub(crate) const LCD_PIXEL_HEIGHT: usize = 144;

const SCY_REG_ADDRESS: u16 = 0xFF42;
const SCX_REG_ADDRESS: u16 = 0xFF43;
const LY_REG_ADDRESS: u16 = 0xFF44;
const LYC_REG_ADDRESS: u16 = 0xFF45;
const DMA_REG_ADDRESS: u16 = 0xFF46;
const BGP_REG_ADDRESS: u16 = 0xFF47;
const OBP0_REG_ADDRESS: u16 = 0xFF48;
const OBP1_REG_ADDRESS: u16 = 0xFF49;
const WY_REG_ADDRESS: u16 = 0xFF4A;
const WX_REG_ADDRESS: u16 = 0xFF4B;

const MODE_CLOCKS: [usize; 4] = [
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
    pub(crate) data: [[u8; LCD_PIXEL_WIDTH]; LCD_PIXEL_HEIGHT],
}

impl Lcd {
    fn new() -> Lcd {
        Lcd {
            data: [[0; LCD_PIXEL_WIDTH]; LCD_PIXEL_HEIGHT]
        }
    }
}

#[derive(Clone,Copy)]
struct SpriteAttribute {
    y: u8,                      // byte 0
    x: u8,                      // byte 1
    tile_index: u8,             // byte 2
    bg_window_over_obj: bool,   // byte 3, bit 7
    flip_y: bool,               // byte 3, bit 6
    flip_x: bool,               // byte 3, bit 5
    palette_is_obp1: bool,      // byte 3, bit 4
    tile_vram_bank_cgb: bool,   // byte 3, bit 3 (CGB ONLY)
    palette_cgb: u8,            // byte 3, bit 2-0 (CGB ONLY)
}

impl SpriteAttribute {
    fn new(data: &[u8; 4]) -> SpriteAttribute {
        SpriteAttribute {
            y: data[0],
            x: data[1],
            tile_index: data[2],
            bg_window_over_obj: data[3] & 0b1000_0000 == 0b1000_0000,
            flip_y: data[3] & 0b0100_0000 == 0b0100_0000,
            flip_x: data[3] & 0b0010_0000 == 0b0010_0000,
            palette_is_obp1: data[3] & 0b0001_0000 == 0b0001_0000,
            tile_vram_bank_cgb: data[3] & 0b0000_1000 == 0b0000_1000,
            palette_cgb: data[3] & 0b0000_0111,
        }
    }
}

type Tile = [[u8; 8]; 8];

struct TileMap {
    tiles: [[Tile; 32]; 32],
}

impl TileMap {
    fn new() -> TileMap {
        TileMap {
            tiles: [[[[0; 8]; 8]; 32]; 32],
        }
    }

    fn create(mmu: &mut Mmu, tilemap_address: usize, index_mode_8000: bool) -> TileMap {
        let mut tilemap = TileMap::new();
        let tile_indices = tilemap.fetch_tile_indices(mmu, tilemap_address);
        let tile_addresses = tilemap.fetch_tile_addresses(mmu, &tile_indices, index_mode_8000);
        tilemap.fetch_tiles(mmu, &tile_addresses);
        tilemap
    }

    fn read_tile(tile_bytes: [u8; 16]) -> Tile {
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

    fn read_tiles(tile_data_buffer: &Vec<u8>) -> Vec<Tile> {
        let mut tiles: Vec<Tile> = Vec::from([]);
        let n = tile_data_buffer.len();
        let mut tile_bytes: [u8; 16];
        let mut i = 0;

        while i + 16 <= n {
            tile_bytes = [0; 16];
            tile_bytes.clone_from_slice(&tile_data_buffer[i..i+16]);

            let tile = TileMap::read_tile(tile_bytes);
            tiles.push(tile);

            i += 16;
        }

        tiles
    }

    fn fetch_tile_indices(&mut self, mmu: &mut Mmu, tilemap_address: usize) -> [usize; 32 * 32]{
        let indices: [usize; 32 * 32] = mmu.read_buffer(
            tilemap_address,
            tilemap_address + 32 * 32,
            Endianness::LITTLE)
            .iter().map(|x| *x as usize)
            .collect::<Vec<usize>>()
            .try_into().unwrap();
        indices
    }

    fn fetch_tile_addresses(&mut self, mmu: &mut Mmu, tile_indices: &[usize; 32 * 32], index_mode_8000: bool) -> [usize; 32 * 32] {
        // The “$8000 method” uses $8000 as its base pointer and uses an unsigned addressing,
        // meaning that tiles 0-127 are in block 0, and tiles 128-255 are in block 1.
        // The “$8800 method” uses $9000 as its base pointer and uses a signed addressing,
        // meaning that tiles 0-127 are in block 2, and tiles 128-255 are in block 1.

        let mut addresses = [0; 32 * 32];

        for i in 0..32 {
            let tile_index = tile_indices[i] as i32;
            addresses[i] = if !index_mode_8000 && tile_index < 128 {
                0x9000 + tile_index * 16
            } else {
                0x8000 + tile_index * 16
            } as usize;
        }

        addresses
    }

    fn fetch_tiles(&mut self, mmu: &mut Mmu, tile_addresses: &[usize; 32 * 32]) {
        for row in 0..32 {
            for col in 0..32 {
                let address = tile_addresses[row * 32 + col];

                let tile_bytes: [u8; 16] = mmu.read_buffer(
                        address, 
                        address + 16, 
                        Endianness::LITTLE)
                    .try_into().unwrap();

                self.tiles[row][col] = TileMap::read_tile(tile_bytes);
            }
        }
    }

    fn to_lcd(&self, scy: usize, scx: usize, palette: &[u8; 4]) -> Lcd {
        let mut lcd = Lcd::new();

        for row in 0..32 {
            for col in 0..32 {
                let tile = self.tiles[row][col];
                for i in 0..8 {
                    let lcd_row = row * 8 + scy + i;
                    if lcd_row >= LCD_PIXEL_HEIGHT { break }
                    for j in 0..8 {
                        let lcd_col = col * 8 + scx + j;
                        if lcd_col >= LCD_PIXEL_WIDTH { break }
                        let pixel = tile[i][j];
                        lcd.data[lcd_row][lcd_col] = palette[pixel as usize];
                    }
                }
            }
        }

        lcd
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
    AddressingMode8000 = 4,
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

    fn set_lyc_eq_lc(&mut self, lyc_equals_ly: bool, mmu: &mut Mmu) {
        self.set_bit(LcdStatRegBit::LycEqLy, true, mmu);
    }
}

pub(crate) struct Ppu {
    clocks: usize,
    pub(crate) lcd_updated: bool,

    // Registers -- in IO RAM
    scy: u8,    // 0xFF42
    scx: u8,    // 0xFF43
    ly: u8,     // 0xFF44
    lyc: u8,    // 0xFF45
    dma: u8,    // 0xFF46 — DMA: OAM DMA source address & start
    bgp: u8,    // 0xFF47
    obp0: u8,   // 0xFF48 (Non-CGB Mode only)
    obp1: u8,   // 0xFF49 (Non-CGB Mode only)
    wy: u8,     // 0xFF4A
    wx: u8,     // 0xFF4B

    lcd_control: LcdControlRegister,
    lcd_status: LcdStatusRegister,

    sprite_attributes: [SpriteAttribute; 40],
    pub(crate) lcd: Lcd,
}

impl Ppu {
    pub(crate) fn new(mmu: &mut Mmu) -> Ppu {
        Ppu {
            clocks: 0,
            lcd_updated: false,
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            dma: 0,
            bgp: 0,
            obp0: 0,
            obp1: 0,
            wy: 0,
            wx: 0,
            lcd_control: LcdControlRegister::new(),
            lcd_status: LcdStatusRegister::new(mmu),
            sprite_attributes: [SpriteAttribute::new(&[0; 4]); 40],
            lcd: Lcd::new(),
        }
    }

    pub(crate) fn step(&mut self, cycles: u16, interrupts: &mut Interrupts, mmu: &mut Mmu) {
        self.clocks += cycles as usize;

        self.refresh_from_mem(mmu);

        // TODO check incoming interrupt requests /enables

        let (y0, y1) = MODE_LINE_RANGE[self.lcd_status.mode as usize];

        if self.ly < y0 || y1 <= self.ly {
            // panic!("Line {} is out of range {}..{} for Mode {:?}.", self.ly, y0, y1, self.lcd_status.mode);
            self.ly = y0;
        }

        let t = MODE_CLOCKS[self.lcd_status.mode as usize];

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

        if self.ly == self.lyc {
            self.lcd_status.set_lyc_eq_lc(true, mmu);
        }
    }

    pub(crate) fn display_tiles_at(&mut self, mmu: &mut Mmu, tilemap_address: usize, scy: usize, scx: usize) {
        let mut background_tilemap = TileMap::new();
        let mut addresses = [0; 32 * 32];
        for i in 0..32 * 32 {
            addresses[i] = tilemap_address + 16 * i;
        }
        background_tilemap.fetch_tiles(mmu, &addresses);
        self.lcd = background_tilemap.to_lcd(scy, scx, &[0,1,2,3]);
    }

    fn refresh_from_mem(&mut self, mmu: &mut Mmu) {
        self.scy = mmu.read_byte(SCY_REG_ADDRESS);
        self.scx = mmu.read_byte(SCX_REG_ADDRESS);
        self.ly = mmu.read_byte(LY_REG_ADDRESS);
        self.lyc = mmu.read_byte(LYC_REG_ADDRESS);
        self.dma = mmu.read_byte(DMA_REG_ADDRESS);
        self.bgp = mmu.read_byte(BGP_REG_ADDRESS);
        self.obp0 = mmu.read_byte(OBP0_REG_ADDRESS);
        self.obp1 = mmu.read_byte(OBP1_REG_ADDRESS);
        self.wy = mmu.read_byte(WY_REG_ADDRESS);
        self.wx = mmu.read_byte(WX_REG_ADDRESS);

        self.lcd_control.read_from_mem(mmu);
        self.lcd_status.read_from_mem(mmu);
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
        let is_last_line = self.ly >= MODE_LINE_RANGE[StatMode::OamSearch as usize].1 - 1;
        if is_last_line
        {
            self.lcd_control.read_from_mem(mmu);
            let obj_enabled = self.lcd_control.check_bit(LcdControlRegBit::ObjEnabled);
            if obj_enabled {
                let attribute_data = mmu.read_buffer(0xFE00, 0xFEA0, Endianness::LITTLE);
                for i in 0..40 {
                    let data = &attribute_data[(i * 4)..(i * 4) + 4];
                    let attr = SpriteAttribute::new(data.try_into().unwrap());
                    self.sprite_attributes[i] = attr;
                }
            }
        }
    }

    fn pixel_transfer(&mut self, mmu: &mut Mmu) {
        // TEMP debugging perf problems
        // self.lcd.data[self.ly as usize % LCD_PIXEL_HEIGHT][self.clocks as usize % LCD_PIXEL_WIDTH] += 1;
        // self.lcd.data[self.ly as usize % LCD_PIXEL_HEIGHT][self.clocks as usize % LCD_PIXEL_WIDTH] %= 8;
        // self.lcd_updated = true;
        // return;

        // TODO implement pixel FIFO correctly
        let is_first_line = self.ly <= MODE_LINE_RANGE[StatMode::PixelTransfer as usize].0;
        let is_last_line = self.ly >= MODE_LINE_RANGE[StatMode::PixelTransfer as usize].1 - 1;
        if is_first_line {
            self.lcd_updated = false;
        }
        if is_last_line && !self.lcd_updated {
            // TEMP DEBUG
            // self.display_tiles_at(mmu, 0x2000, 0, 0);
            // self.lcd_updated = true;
            // return;

            self.lcd_control.read_from_mem(mmu);

            if self.lcd_control.check_bit(LcdControlRegBit::BackgroundAndWindowEnabled) {
                let index_mode_8000 = self.lcd_control.check_bit(LcdControlRegBit::AddressingMode8000);

                let background_tilemap_address = if self.lcd_control.check_bit(LcdControlRegBit::BackgroundTilemapIsAt9C00) { 0x9C00 } else { 0x9800 };
                let background_tilemap = TileMap::create(mmu, background_tilemap_address, index_mode_8000);
                self.lcd = background_tilemap.to_lcd(self.scy as usize, self.scx as usize, &[0,1,2,3]); // TODO palettes

                // TODO how does window get read?
                let window_tilemap_address = if self.lcd_control.check_bit(LcdControlRegBit::WindowTilemapIsAt9C00) { 0x9C00 } else { 0x9800 };
                let window_tilemap = TileMap::create(mmu, window_tilemap_address, index_mode_8000);
                let window_lcd = window_tilemap.to_lcd(self.wy as usize, self.wx as usize, &[0,1,2,3]); // TODO palettes
                for row in 0..LCD_PIXEL_HEIGHT {
                    for col in 0..LCD_PIXEL_WIDTH {
                        if window_lcd.data[row][col] > 0 { // TODO priority, transparency
                            self.lcd.data[row][col] = window_lcd.data[row][col] + 8; // TEMP colors to distinguish window
                        }
                    }
                }

                self.lcd_updated = true;
            }

            if self.lcd_control.check_bit(LcdControlRegBit::ObjEnabled) {
                self.draw_sprites(mmu);
                self.lcd_updated = true;
            }
        }
    }

    fn draw_sprites(&mut self, mmu: &mut Mmu) {
        let tiledata_address: usize = 0x8000;

        for attr in self.sprite_attributes {
            let tile_index = attr.tile_index as usize;
            let tile_address = tiledata_address + tile_index * 16;
            let tile_data = mmu.read_buffer(tile_address, tile_address + 16, Endianness::LITTLE);
            let tile = TileMap::read_tile(tile_data.try_into().unwrap());

            let sprite_height = if self.lcd_control.check_bit(LcdControlRegBit::SpriteSizeIs16) { 16 } else { 8 };
            let palette = if attr.palette_is_obp1 { 1 } else { 0 };
            let palettes = &[
                [0,1,2,3],
                [0,1,2,3],
            ];

            for row in 0..8 {
                let y = attr.y as i16 - sprite_height + row as i16;
                if y < 0 { continue };
                let tile_row = if attr.flip_x { 7 - row } else { row };
                for col in 0..8 {
                    let x = attr.x as i16 - 8 + col;
                    if x < 0 { continue };
                    let tile_col = if attr.flip_x { 7 - col } else { col };
                    let color_i = tile[tile_row as usize][tile_col as usize];
                    if color_i > 0 {
                        let color = palettes[palette][color_i as usize];
                        // TODO selection priority and drawing priority
                        let has_priority = true;
                        let bg_color = self.lcd.data[y as usize][x as usize];
                        if has_priority || bg_color == 0 {
                            self.lcd.data[y as usize][x as usize] = color + 4; // TEMP colors to distinguish sprites
                        }
                    }
                }
            }
        }
    }
}
