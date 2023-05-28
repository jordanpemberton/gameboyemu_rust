use crate::console::interrupts::{InterruptRegBit, Interrupts};
use crate::console::mmu::Mmu;
use crate::console::register::Register;
use crate::console::sprite_attribute::SpriteAttribute;
use crate::console::tilemap;
use crate::console::tilemap::TileMap;

const DEBUG_DISPLAY: bool = true;

pub(crate) const LCD_PIXEL_WIDTH: usize = if DEBUG_DISPLAY { 32 * 8 } else { 160 };
pub(crate) const LCD_PIXEL_HEIGHT: usize = if DEBUG_DISPLAY { 32 * 8 } else { 144 };

const LCD_CONTROL_REG: u16 = 0xFF40;
const LCD_STATUS_REG: u16 = 0xFF41;
const SCY_REG: u16 = 0xFF42;
const SCX_REG: u16 = 0xFF43;
const LY_REG: u16 = 0xFF44;
const LYC_REG: u16 = 0xFF45;
const DMA_REG: u16 = 0xFF46;
const BGP_REG: u16 = 0xFF47;
const OBP0_REG: u16 = 0xFF48;
const OBP1_REG: u16 = 0xFF49;
const WY_REG: u16 = 0xFF4A;
const WX_REG: u16 = 0xFF4B;

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

const STAT_MODES: [&StatMode; 4] = [
    &StatMode::HBlank,
    &StatMode::VBlank,
    &StatMode::OamSearch,
    &StatMode::PixelTransfer,
];

enum StatMode {
    HBlank = 0,
    VBlank = 1,
    OamSearch = 2,
    PixelTransfer = 3,
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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

pub(crate) struct Lcd {
    pub(crate) data: [[u8; LCD_PIXEL_WIDTH]; LCD_PIXEL_HEIGHT],
}

impl Lcd {
    pub(crate) fn new() -> Lcd {
        Lcd {
            data: [[0; LCD_PIXEL_WIDTH]; LCD_PIXEL_HEIGHT],
        }
    }

    fn fill_from_tilemap(&mut self, tilemap: TileMap, scy: u8, scx: u8, palette: &[u8; 4]) {
        for tilemap_row in 0..32 {
            for tilemap_col in 0..32 {
                let tile = tilemap[tilemap_row][tilemap_col];

                for i in 0..8 {
                    let lcd_row = if DEBUG_DISPLAY {
                        tilemap_row * 8 + i
                    } else {
                        (tilemap_row * 8 + i + scy as usize) as u8 as usize
                    };

                    if lcd_row < LCD_PIXEL_HEIGHT {
                        for j in 0..8 {
                            let lcd_col = if DEBUG_DISPLAY {
                                tilemap_col * 8 + j
                            } else {
                                (tilemap_col * 8 + j + scx as usize) as u8 as usize
                            };

                            if lcd_col < LCD_PIXEL_WIDTH {
                                let pixel = if DEBUG_DISPLAY && Lcd::is_on_border(lcd_row, lcd_col, scy, scx) {
                                    3
                                } else {
                                    tile[i][j]
                                };

                                self.data[lcd_row as usize][lcd_col as usize] = palette[pixel as usize];
                            }
                        }
                    }
                }
            }
        }
    }

    // For debuggin
    # [allow(dead_code)]
    fn is_on_border(lcd_row: usize, lcd_col: usize, scy: u8, scx: u8) -> bool {
        let bottom_edge = scy.wrapping_add(144 as u8) as usize - 1;
        let right_edge = scx.wrapping_add(160 as u8) as usize - 1;

        let mut is_y_border = lcd_row == scy as usize || lcd_row == bottom_edge;
        let mut is_x_border = lcd_col == scx as usize || lcd_col == right_edge;

        is_y_border &= {
            if right_edge < scx as usize {
                lcd_col >= scx as usize || lcd_col <= right_edge
            } else {
                lcd_col >= scx as usize && lcd_col <= right_edge
            }
        };

        is_x_border &= {
            if bottom_edge < scy as usize {
                lcd_row >= scy as usize || lcd_row <= bottom_edge
            } else {
                lcd_row >= scy as usize && lcd_row <= bottom_edge
            }
        };

        is_y_border || is_x_border
    }
}

pub(crate) struct Ppu {
    clocks: usize,

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

    lcd_control: Register,
    lcd_status: Register,

    sprite_attributes: [SpriteAttribute; 40],
    pub(crate) lcd: Lcd,
}

impl Ppu {
    pub(crate) fn new(mmu: &mut Mmu) -> Ppu {
        Ppu {
            clocks: 0,
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
            lcd_control: Register::new(mmu, LCD_CONTROL_REG),
            lcd_status: Register::new(mmu, LCD_STATUS_REG),
            sprite_attributes: [SpriteAttribute::new(&[0; 4]); 40],
            lcd: Lcd::new(),
        }
    }

    pub(crate) fn step(&mut self, cycles: u16, interrupts: &mut Interrupts, mmu: &mut Mmu) {
        self.clocks += cycles as usize;

        self.refresh_from_mem(mmu);

        // TODO check incoming interrupt requests /enables

        let x = (self.lcd_status.value & 0x03) as usize;
        let mode = STAT_MODES[x];
        let t = MODE_CLOCKS[x];
        let (y0, y1) = MODE_LINE_RANGE[x];
        if self.ly < y0 || y1 <= self.ly {
            self.ly = y0;
        }

        match *mode {
            StatMode::OamSearch => {
                if self.clocks < t {
                    self.oam_search(mmu);
                } else {
                    self.clocks = 0;
                    self.set_stat_mode(mmu, StatMode::PixelTransfer);
                }
            }
            StatMode::PixelTransfer => {
                if self.clocks < t {
                    // TODO FIFO
                } else {
                    self.pixel_transfer(mmu);
                    self.clocks = 0;
                    self.set_stat_mode(mmu, StatMode::HBlank);
                    if self.lcd_status.check_bit(mmu, LcdStatRegBit::HBlankInterruptEnabled as u8) {
                        interrupts.requested.set_bit(mmu, InterruptRegBit::LcdStat as u8, true);
                    }
                }
            }
            StatMode::HBlank => {
                if self.clocks < t {
                    // wait
                } else {
                    self.increment_ly(mmu);
                    if self.ly < MODE_LINE_RANGE[StatMode::OamSearch as usize].1 as u8 {
                        self.set_stat_mode(mmu, StatMode::OamSearch);
                        if self.lcd_status.check_bit(mmu, LcdStatRegBit::OamInterruptEnabled as u8) {
                            interrupts.requested.set_bit(mmu, InterruptRegBit::LcdStat as u8, true);
                        }
                    } else {
                        self.set_stat_mode(mmu, StatMode::VBlank);
                        interrupts.requested.set_bit(mmu, InterruptRegBit::VBlank as u8, true);
                        if self.lcd_status.check_bit(mmu, LcdStatRegBit::VBlankInterruptEnabled as u8) {
                            interrupts.requested.set_bit(mmu, InterruptRegBit::LcdStat as u8, true);
                        }
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
                        self.set_stat_mode(mmu, StatMode::OamSearch);
                        if self.lcd_status.check_bit(mmu, LcdStatRegBit::OamInterruptEnabled as u8) {
                            interrupts.requested.set_bit(mmu, InterruptRegBit::LcdStat as u8, true);
                        }
                    }
                }
            }
        }

        if self.ly == self.lyc {
            self.lcd_status.set_bit(mmu, LcdStatRegBit::LycEqLy as u8, true);
        }
    }

    fn refresh_from_mem(&mut self, mmu: &mut Mmu) {
        self.scy = mmu.read_8(SCY_REG);
        self.scx = mmu.read_8(SCX_REG);
        self.ly = mmu.read_8(LY_REG);
        self.lyc = mmu.read_8(LYC_REG);
        self.dma = mmu.read_8(DMA_REG);
        self.bgp = mmu.read_8(BGP_REG);
        self.obp0 = mmu.read_8(OBP0_REG);
        self.obp1 = mmu.read_8(OBP1_REG);
        self.wy = mmu.read_8(WY_REG);
        self.wx = mmu.read_8(WX_REG);
        self.lcd_control.read_from_mem(mmu);
        self.lcd_status.read_from_mem(mmu);
    }

    fn increment_ly(&mut self, mmu: &mut Mmu) {
        self.ly += 1;
        self.clocks = 0;

        if self.ly >= MODE_LINE_RANGE[StatMode::VBlank as usize].1 {
            self.ly = 0;
        }
        mmu.write_8(LY_REG, self.ly);
    }

    fn set_stat_mode(&mut self, mmu: &mut Mmu, mode: StatMode) {
        let x = match mode {
            StatMode::HBlank => 0,
            StatMode::VBlank => 1,
            StatMode::OamSearch => 2,
            StatMode::PixelTransfer => 3,
        };
        let bit0 = (x & 0x01) == 0x01;
        let bit1 = (x & 0x02) == 0x02;
        self.lcd_status.set_bit(mmu, LcdStatRegBit::ModeBit0 as u8, bit0);
        self.lcd_status.set_bit(mmu, LcdStatRegBit::ModeBit1 as u8, bit1);
    }

    fn oam_search(&mut self, mmu: &mut Mmu) {
        // TODO
        let is_last_line = self.ly >= MODE_LINE_RANGE[StatMode::OamSearch as usize].1 - 1;
        if is_last_line
        {
            self.lcd_control.read_from_mem(mmu);
            let obj_enabled = self.lcd_control.check_bit(mmu, LcdControlRegBit::ObjEnabled as u8);
            if obj_enabled {
                let attribute_data = mmu.read(0xFE00, 0xFEA0);
                for i in 0..40 {
                    let data = &attribute_data[(i * 4)..(i * 4) + 4];
                    let attr = SpriteAttribute::new(data.try_into().unwrap());
                    self.sprite_attributes[i] = attr;
                }
            }
        }
    }

    fn pixel_transfer(&mut self, mmu: &mut Mmu) {
        // TODO implement pixel FIFO correctly
        let is_last_line = self.ly >= MODE_LINE_RANGE[StatMode::PixelTransfer as usize].1 - 1;
        if is_last_line {
            self.draw_background(mmu);
            // self.draw_window(mmu);
            // self.draw_sprites(mmu);
        }
    }

    fn draw_background(&mut self, mmu: &mut Mmu) {
        self.lcd_control.read_from_mem(mmu);
        if self.lcd_control.check_bit(mmu, LcdControlRegBit::BackgroundAndWindowEnabled as u8) {
            let index_mode_8000 = self.lcd_control.check_bit(mmu, LcdControlRegBit::AddressingMode8000 as u8);

            let background_tilemap_address = if self.lcd_control.check_bit(mmu, LcdControlRegBit::BackgroundTilemapIsAt9C00 as u8) {
                0x9C00
            } else {
                0x9800
            };

            let background_tilemap = tilemap::get_tilemap(mmu, background_tilemap_address, index_mode_8000);

            self.lcd.fill_from_tilemap(background_tilemap, self.scy, self.scx, &[0, 1, 2, 3]); // TODO palettes

            // self.lcd = background_tilemap.to_lcd(0, 0, &[0, 1, 2, 3]); // TODO fix scy, scx
        }
    }

    #[allow(dead_code)]
    fn draw_window(&mut self, mmu: &mut Mmu) {
        self.lcd_control.read_from_mem(mmu);

        let is_dmg = true;
        let enabled_bit = if is_dmg {
            LcdControlRegBit::BackgroundAndWindowEnabled
        } else {
            LcdControlRegBit::WindowEnabled
        } as u8;

        if self.lcd_control.check_bit(mmu, enabled_bit) {
            let index_mode_8000 = self.lcd_control.check_bit(mmu, LcdControlRegBit::AddressingMode8000 as u8);

            let window_tilemap_address = if self.lcd_control.check_bit(mmu, LcdControlRegBit::WindowTilemapIsAt9C00 as u8) {
                0x9C00
            } else {
                0x9800
            };

            let window_tilemap = tilemap::get_tilemap(mmu, window_tilemap_address, index_mode_8000);

            let mut window_lcd = Lcd::new();
            window_lcd.fill_from_tilemap(window_tilemap, self.wy, self.wx, &[0, 1, 2, 3]); // TODO palettes

            for row in 0..LCD_PIXEL_HEIGHT {
                for col in 0..LCD_PIXEL_WIDTH {
                    if window_lcd.data[row][col] > 0 { // TODO priority, transparency
                        self.lcd.data[row][col] = window_lcd.data[row][col] + 8; // TEMP colors to distinguish window
                    }
                }
            }
        }
    }

    #[allow(dead_code)]
    fn draw_sprites(&mut self, mmu: &mut Mmu) {
        self.lcd_control.read_from_mem(mmu);
        if self.lcd_control.check_bit(mmu, LcdControlRegBit::ObjEnabled as u8) {
            let tiledata_address: usize = 0x8000; // TODO read this?

            for attr in self.sprite_attributes {
                let tile_index = attr.tile_index as usize;
                let tile_address = tiledata_address + tile_index * 16;
                let tile_data = mmu.read(tile_address, tile_address + 16);
                let tile = tilemap::read_tile(tile_data.try_into().unwrap());

                let sprite_height = if self.lcd_control.check_bit(mmu, LcdControlRegBit::SpriteSizeIs16 as u8) { 16 } else { 8 };
                let palette = if attr.palette_is_obp1 { 1 } else { 0 };
                // TODO palettes
                let palettes = &[
                    [0, 1, 2, 3],
                    [0, 1, 2, 3],
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

    // For debugging
    #[allow(dead_code)]
    fn speed_check_pixel_transfer(&mut self) {
        self.lcd.data[self.ly as usize % LCD_PIXEL_HEIGHT][self.clocks as usize % LCD_PIXEL_WIDTH] += 1;
        self.lcd.data[self.ly as usize % LCD_PIXEL_HEIGHT][self.clocks as usize % LCD_PIXEL_WIDTH] %= 8;
    }

    // For debugging
    #[allow(dead_code)]
    pub(crate) fn display_tiles_at(&mut self, mmu: &mut Mmu, tilemap_address: usize, scy: u8, scx: u8) {
        let mut tile_addresses = [0; 32 * 32];
        for i in 0..32 * 32 {
            tile_addresses[i] = tilemap_address + 16 * i;
        }
        let background_tilemap = tilemap::fetch_tilemap(mmu, tile_addresses);
        self.lcd.fill_from_tilemap(background_tilemap, scy, scx, &[0, 1, 2, 3]);
    }

    // For debugging
    #[allow(dead_code)]
    fn display_tiles_at_pixel_transfer(&mut self, mmu: &mut Mmu) {
        self.display_tiles_at(mmu, 0x2000, 0, 0);
    }
}
