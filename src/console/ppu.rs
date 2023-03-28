#![allow(unused_assignments)]

use std::collections::HashMap;
use sdl2::event::Event::Window;
use sdl2::pixels::Color;
use sdl2::render::TextureAccess::Static;

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

/*
FF40 — LCDC: LCD control

LCDC is the main LCD Control register. Its bits toggle what elements are displayed on the screen, and how.
*/

/*
SCROLLING

FF42–FF43 — SCY, SCX: Viewport Y position, X position

Those specify the top-left coordinates of the visible 160×144 pixel area within the 256×256 pixels BG map. Values in the range 0–255 may be used.
*/

const TILE_PIXEL_WIDTH: u16 = 8;
const TILE_PIXEL_HEIGHT: u16 = 8;
const MAP_TILE_WIDTH: u16 = 32;
const MAP_TILE_HEIGHT: u16 = 32;
const MAP_PIXEL_WIDTH: u16 = 256;
const MAP_PIXEL_HEIGHT: u16 = 256;
pub(crate) const LCD_PIXEL_WIDTH: u32 = 160;
pub(crate) const LCD_PIXEL_HEIGHT: u32 = 144;

const STAT_REG_ADDRESS: u16 = 0xFF41;
const LY_REG_ADDRESS: u16 = 0xFF44;
const LYC_REG_ADDRESS: u16 = 0xFF45;

type Lcd = [[u8; LCD_PIXEL_WIDTH as usize]; LCD_PIXEL_HEIGHT as usize];

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

    pub(crate) fn read_tile(tile_bytes: [u8; 16]) -> Tile {
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

            let mut tile = Tile::new();
            for row in 0..8 {
                for col in 0..8 {
                    // Possible values = 0,1,2,3 (0b00,0b01,0b10,0b11)
                    let low = ((tile_bytes[row * 2] << col) >> 7) << 1;
                    let high = (tile_bytes[row * 2 + 1] << col) >> 7;
                    tile.data[row][col] = high + low;
                }
            }
            tiles.push(tile);

            i += 16;
        }

        tiles
    }
}

pub(crate) struct TileMap {
    start_address: usize,
    end_address: usize,
    tiles: Vec<Tile>,
}

impl TileMap {
    fn new(start_address: usize, end_address: usize) -> TileMap {
        TileMap {
            start_address: start_address,
            end_address: end_address,
            tiles: vec![],
        }
    }

    pub(crate) fn fetch_tiles(&mut self, mmu: &mut Mmu) {
        let ram = mmu.read_buffer(self.start_address, self.end_address, Endianness::BIG);
        self.tiles = Tile::read_tiles(&ram);
    }

    pub(crate) fn to_lcd(&self, palettes: &[[u8; 4]; 4]) -> [[u8; LCD_PIXEL_WIDTH as usize]; LCD_PIXEL_HEIGHT as usize] {
        let mut lcd = [[0; LCD_PIXEL_WIDTH as usize]; LCD_PIXEL_HEIGHT as usize];

        let mut x: usize = 0;
        let mut y: usize = 0;

        // TODO sort by priority
        for tile in self.tiles.clone().into_iter() {
            for i in 0..TILE_PIXEL_WIDTH as usize {
                for j in 0..TILE_PIXEL_HEIGHT as usize {
                    let pixel = tile.data[i][j];
                    let palette = palettes[tile.palette as usize];

                    if y + i >= LCD_PIXEL_HEIGHT as usize || x + j >= LCD_PIXEL_WIDTH as usize {
                        break;
                    }
                    lcd[y + i][x + j] = palette[pixel as usize];

                    // TODO use tile attr (x,y): lcd[tile.y as usize][tile.x as usize] = palette[pixel as usize];
                }
            }
            x += TILE_PIXEL_WIDTH as usize;
            if x >= LCD_PIXEL_WIDTH as usize {
                x = 0;
                y += TILE_PIXEL_HEIGHT as usize;
            }
        }

        lcd
    }
}

/*
FF41 — STAT: LCD status

Bit 6 - LYC=LY STAT Interrupt source         (1=Enable) (Read/Write)
Bit 5 - Mode 2 OAM STAT Interrupt source     (1=Enable) (Read/Write)
Bit 4 - Mode 1 VBlank STAT Interrupt source  (1=Enable) (Read/Write)
Bit 3 - Mode 0 HBlank STAT Interrupt source  (1=Enable) (Read/Write)
Bit 2 - LYC=LY Flag                          (0=Different, 1=Equal) (Read Only)
Bit 1-0 - Mode Flag                          (Mode 0-3, see below) (Read Only)
          0: HBlank
          1: VBlank
          2: Searching OAM
          3: Transferring Data to LCD Controller
 */

#[derive(Clone, Copy)]
enum StatMode {
    /// 0	No action (HBlank)
    /// Duration: 85 to 208 dots, depending on previous mode 3 duration
    /// Accessible video memory: VRAM, OAM, CGB palettes
    HBLANK,

    /// 1	No action (VBlank)
    /// Duration: 4560 dots (10 scanlines)
    /// Accessible video memory: VRAM, OAM, CGB palettes
    VBLANK,

    /// 2	Searching OAM for OBJs whose Y coordinate overlap this line
    /// Duration: 80 dots
    /// Accessible video memory: VRAM, CGB palettes
    OAM,

    /// 3	Reading OAM and VRAM to generate the picture
    /// Duration: 168 to 291 dots, depending on sprite count
    /// Accessible video memory: None
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

    ly: u8,    // scanline,     0xFF44
    // LY can hold any value from 0 to 153, with values from 144 to 153 indicating the VBlank period.

    lyc: u8,   // LY compare,   0xFF45
    // The Game Boy constantly compares the value of the LYC and LY registers.
    // When both values are identical, the “LYC=LY” flag in the STAT register is set,
    // and (if enabled) a STAT interrupt is requested.

    // The SCY and SCX registers can be used to scroll the Background,
    // specifying the origin of the visible 160x144 pixel area within the total 256x256 pixel Background map.
    scx: u8,
    scy: u8,

    mode: StatMode,     // 0xFF41
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

            mode: StatMode::HBLANK,
            palette: 0,
            palettes: palettes,
            interrupts: Interrupts{
                enabled: true,
                hblank: false,
                lcd: false,
                oam: false,
                vblank: false,
            },

            background: TileMap::new(0x9800, 0x9C00),
            window: TileMap::new(0x8000, 0x10000), // TODO
            sprites: TileMap::new(0xFE00, 0xFE9F),
        }
    }

    pub(crate) fn step(&mut self, cycles: u16, interrupt_request: &mut Interrupts, mmu: &mut Mmu) {
        self.tick += cycles;

        /*
            The LCD controller operates on a 2^22 Hz = 4.194 MHz dot clock.
            An entire frame is 154 scanlines = 70224 dots = 16.74 ms. (456/line)
            On scanlines 0 through 143, the PPU cycles through modes 2, 3, and 0 once every 456 dots.
            Scanlines 144 through 153 are mode 1.
        */
        let (new_tick, new_mode): (u16, StatMode) =
            match self.mode {
                StatMode::HBLANK => {
                    if self.tick >= 204 {
                        self.hblank_mode_step(interrupt_request, mmu)
                    } else {
                        (self.tick, StatMode::HBLANK)
                    }
                }
                StatMode::VBLANK => {
                    if self.tick >= 456 {
                        self.vblank_mode_step(interrupt_request, mmu)
                    } else {
                        (self.tick, StatMode::VBLANK)
                    }
                }
                StatMode::OAM => {
                    if self.tick >= 80 {
                        (0, StatMode::VRAM)
                    } else {
                        (self.tick, StatMode::OAM)
                    }
                }
                StatMode::VRAM => {
                    if self.tick >= 172 {
                        self.vram_mode_step(interrupt_request, mmu)
                    } else {
                        (self.tick, StatMode::VRAM)
                    }
                }
                _ => {
                    (self.tick, StatMode::HBLANK)
                }
        };

        // Checl LYC
        // interrupt ?
        // if self.interrupts.lcd && self.ly == self.lyc {
        //     interrupt_request.lcd = true;
        // }
        // or ?
        if self.ly == self.lyc {
            let stat_reg = mmu.read_byte(STAT_REG_ADDRESS) | 0b0000_0100;
            mmu.load_byte(STAT_REG_ADDRESS, stat_reg);
        }

        self.tick = new_tick;
        self.mode = new_mode;
    }

    fn hblank_mode_step(&mut self, interrupt_request: &mut Interrupts, mmu: &mut Mmu) -> (u16, StatMode) {
        let tick = 0;
        let mode: StatMode;

        // Increment line count
        self.ly += 1;
        mmu.load_byte(LY_REG_ADDRESS, self.ly);

        // On scanlines 0 through 143, the PPU cycles through modes 2, 3, and 0 once every 456 dots (line).
        if self.ly < 0x90 {     // 0d144
            // TODO interrupts
            // if self.interrupts.oam {
            //     interrupt_request.lcd = true;
            // }
            mode = StatMode::OAM;
        }
        // Scanlines 144 through 153 are mode 1.
        else {
            // TODO interrupts
            // if interrupt_request.enabled {
            //     interrupt_request.vblank = true; // ?
            // }
            // if self.interrupts.vblank {
            //     interrupt_request.lcd = true; // ?
            // }
            mode = StatMode::VBLANK;
        }

        (tick, mode)
    }

    fn vblank_mode_step(&mut self, interrupt_request: &mut Interrupts, mmu: &mut Mmu) -> (u16, StatMode) {
        self.ly += 1;

        if self.ly > 153 {
            self.ly = 0;
            // TODO interrupts
            // if self.interrupts.oam {
            //     interrupt_request.lcd = true;
            // }
            (0, StatMode::OAM)
        } else {
            (0, StatMode::VBLANK)
        }
    }

    fn vram_mode_step(&mut self, interrupt_request: &mut Interrupts, mmu: &mut Mmu) -> (u16, StatMode) {
        let tick = 0;
        let mode: StatMode;

        if self.ly < LCD_PIXEL_HEIGHT as u8 {
            // TODO pick one
            // self.background.fetch_tiles(mmu);
            self.window.fetch_tiles(mmu);
            // self.sprites.fetch_tiles(mmu);
        }

        // (draw)

        // TODO Interrupts
        // if self.interrupts.hblank {
        //     interrupt_request.lcd = true;
        // }

        mode = StatMode::HBLANK;

        (tick, mode)
    }
}
