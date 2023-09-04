use std::cmp::min;
use crate::console::interrupts::{InterruptRegBit, Interrupts};
use crate::console::mmu;
use crate::console::mmu::Mmu;
use crate::console::register::Register;
use crate::console::sprite_attribute::SpriteAttribute;
use crate::console::tilemap;
use crate::console::tilemap::TileMap;

const DEBUG_COLOR_I: u8 = 12;

// From Michael Steil, Ultimate Game Boy Talk
// (Gekkio (Mooneye) has HBlank=50 and OAM=21)
const MODE_DURATION: [usize; 4] = [
    51 * 4,             // = 204    HBlank[0]
    (20 + 43 + 51) * 4, // = 456    VBlank [1]
    20 * 4,             // = 80     OAM [2]
    43 * 4,             // = 172    PixelTransfer [3]
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

#[allow(dead_code)]
#[derive(PartialEq)]
enum DrawMode {
    Background,
    Sprites,
    Window,
}

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
    in_debug_mode: bool,
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) data: Vec<Vec<u8>>,
}

impl Lcd {
    pub(crate) fn new(in_debug_mode: bool) -> Lcd {
        let width: usize = if in_debug_mode { 32 * 8 } else { 160 };
        let height: usize = if in_debug_mode { 32 * 8 } else { 144 };

        Lcd {
            in_debug_mode,
            width,
            height,
            data: vec![vec![0; width]; height],
        }
    }

    fn fill_from_tilemap(&mut self, tilemap: TileMap, scy: u8, scx: u8, palette: &[u8; 4]) {
        for tilemap_row in 0..32 {
            for tilemap_col in 0..32 {
                let tile = tilemap[tilemap_row][tilemap_col];

                for i in 0..8 {
                    let mut lcd_row = tilemap_row * 8 + i;
                    if !self.in_debug_mode {
                        lcd_row = (lcd_row as u8).wrapping_sub(scy) as usize;
                    }

                    if lcd_row < self.height {
                        for j in 0..8 {
                            let mut lcd_col = tilemap_col * 8 + j;
                            if !self.in_debug_mode {
                                lcd_col = (lcd_col as u8).wrapping_sub(scx) as usize;
                            }

                            if lcd_col < self.width {
                                let pixel = tile[i][j];
                                self.data[lcd_row as usize][lcd_col as usize] = palette[pixel as usize];
                            }
                        }
                    }
                }
            }
        }
    }
}

pub(crate) struct Ppu {
    in_debug_mode: bool,

    clocks: usize,

    // Registers -- in IO RAM
    scy: u8,
    scx: u8,
    ly: u8,
    lyc: u8,
    dma: u8,
    bgp: u8,
    obp0: u8,
    obp1: u8,
    wy: u8,
    wx: u8,

    lcd_control: Register,
    lcd_status: Register,

    sprite_attributes: [SpriteAttribute; 40],
    pub(crate) lcd: Lcd,
}

impl Ppu {
    pub(crate) fn new(mmu: &mut Mmu, in_debug_mode: bool) -> Ppu {
        Ppu {
            in_debug_mode,
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
            lcd_control: Register::new(mmu, mmu::LCD_CONTROL_REG),
            lcd_status: Register::new(mmu, mmu::LCD_STATUS_REG),
            sprite_attributes: [SpriteAttribute::new(&[0; 4]); 40],
            lcd: Lcd::new(in_debug_mode),
        }
    }

    pub(crate) fn step(&mut self, cycles: u16, interrupts: &mut Interrupts, mmu: &mut Mmu) {
        self.clocks += cycles as usize;

        self.refresh_from_mem(mmu);

        // TODO check incoming interrupt requests /enables

        let mode_flag = (self.lcd_status.value & 0x03) as usize;
        let mode = STAT_MODES[mode_flag];
        let mode_duration = MODE_DURATION[mode_flag];
        let (mode_y_start, mode_y_end) = MODE_LINE_RANGE[mode_flag];
        // Force current line ly to align with current mode (hack)
        if self.ly < mode_y_start || mode_y_end <= self.ly {
            self.set_ly(mmu, mode_y_start);
        }

        match *mode {
            StatMode::OamSearch => {
                if self.clocks < mode_duration {
                    self.oam_search(mmu);
                } else {
                    self.clocks = 0;
                    self.set_stat_mode(mmu, StatMode::PixelTransfer);
                }
            }
            StatMode::PixelTransfer => {
                if self.clocks < mode_duration { // where is correct to check if enabled?
                    self.pixel_transfer(mmu);
                } else {
                    self.clocks = 0;
                    self.set_stat_mode(mmu, StatMode::HBlank);
                    if self.lcd_status.check_bit(mmu, LcdStatRegBit::HBlankInterruptEnabled as u8) {
                        interrupts.requested.set_bit(mmu, InterruptRegBit::LcdStat as u8, true);
                    }
                }
            }
            StatMode::HBlank => {
                if self.clocks < mode_duration {
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
                if self.clocks < mode_duration {
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
        self.scy = mmu.read_8(mmu::SCY_REG);
        self.scx = mmu.read_8(mmu::SCX_REG);
        self.ly = mmu.read_8(mmu::LY_REG);
        self.lyc = mmu.read_8(mmu::LYC_REG);
        self.dma = mmu.read_8(mmu::DMA_REG);
        self.bgp = mmu.read_8(mmu::BGP_REG);
        self.obp0 = mmu.read_8(mmu::OBP0_REG);
        self.obp1 = mmu.read_8(mmu::OBP1_REG);
        self.wy = mmu.read_8(mmu::WY_REG);
        self.wx = mmu.read_8(mmu::WX_REG);
        self.lcd_control.read_from_mem(mmu);
        self.lcd_status.read_from_mem(mmu);
    }

    fn read_palette(byte: u8) -> [u8; 4] {
        // Bit 1-0 - Color for index 0
        // Bit 3-2 - Color for index 1
        // Bit 5-4 - Color for index 2
        // Bit 7-6 - Color for index 3
        [
            byte & 0x03,
            (byte >> 2) & 0x03,
            (byte >> 4) & 0x03,
            byte >> 6,
        ]
    }

    fn set_ly(&mut self, mmu: &mut Mmu, value: u8) {
        self.ly = value;
        if self.ly >= MODE_LINE_RANGE[StatMode::VBlank as usize].1 {
            self.ly = 0;
        }

        mmu.write_8(mmu::LY_REG, self.ly);
    }

    fn increment_ly(&mut self, mmu: &mut Mmu) {
        self.ly += 1;
        self.clocks = 0;
        if self.ly >= MODE_LINE_RANGE[StatMode::VBlank as usize].1 {
            self.ly = 0;
        }

        mmu.write_8(mmu::LY_REG, self.ly);
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
        if self.ly == 0 {
            self.lcd_control.read_from_mem(mmu);
            let obj_enabled = self.lcd_control.check_bit(mmu, LcdControlRegBit::ObjEnabled as u8);
            if obj_enabled {
                let attribute_data = mmu.read_buffer(mmu::OAM_START, mmu::OAM_END + 1);
                for i in 0..40 {
                    let data = &attribute_data[(i * 4)..(i * 4) + 4];
                    let attr = SpriteAttribute::new(data.try_into().unwrap());
                    self.sprite_attributes[i] = attr;
                }
            }
        }
    }

    fn pixel_transfer(&mut self, mmu: &mut Mmu) {
        self.refresh_from_mem(mmu);

        // TODO Pixel FIFO
        self.draw_background_line(mmu);
        // self.draw_window_line(mmu);
        self.draw_sprites_line(mmu);

        // for debugging
        if self.ly == 0 {
            // self.display_tiles_at(mmu, 0x8000, 0, 0); // for debugging
            // self.display_tiles_from_indices_at(mmu, false, true, 0, 0); // what tetris should draw (mostly)
            // self.display_tiles_from_indices_at(mmu, false, false, 0, 0); // what tetris is drawing (punctuation/noise)
            // self.draw_sprites(mmu);
        }

        if self.in_debug_mode {
            self.draw_lcd_border();
        }
    }

    #[allow(dead_code)]
    fn draw_background_line(&mut self, mmu: &mut Mmu) {
        if self.lcd_control.check_bit(mmu, LcdControlRegBit::BackgroundAndWindowEnabled as u8) {
                // && self.lcd_control.check_bit(mmu, LcdControlRegBit::LcdAndPpuEnabled as u8) { // why is this never enabled...
            self.fill_lcd_row(mmu, DrawMode::Background);
        }
    }

    #[allow(dead_code)]
    fn draw_window_line(&mut self, mmu: &mut Mmu) {
        let is_dmg = true;  // TODO
        if (is_dmg && self.lcd_control.check_bit(mmu, LcdControlRegBit::BackgroundAndWindowEnabled as u8))
            || (!is_dmg && self.lcd_control.check_bit(mmu, LcdControlRegBit::WindowEnabled as u8)) {
            self.fill_lcd_row(mmu, DrawMode::Window);
        }
    }

    #[allow(dead_code)]
    fn draw_sprites_line(&mut self, mmu: &mut Mmu) {
        if self.lcd_control.check_bit(mmu, LcdControlRegBit::ObjEnabled as u8) {
            let tiledata_address: usize = 0x8000;
            let scanline = self.ly;

            for attr in self.sprite_attributes {
                let sprite_height: u8 = if self.lcd_control.check_bit(mmu, LcdControlRegBit::SpriteSizeIs16 as u8) { 16 } else { 8 };
                let sprite_top_y = attr.y.wrapping_sub(16);

                let distance_sprite_top_y_to_scanline = scanline.wrapping_sub(sprite_top_y);

                if sprite_top_y <= scanline && distance_sprite_top_y_to_scanline < sprite_height {
                    let tile_index = attr.tile_index as usize;
                    let tile_address = tiledata_address + tile_index * 16;
                    let tile_bytes = mmu.read_buffer(tile_address as u16, tile_address as u16 + 16)
                        .try_into().unwrap();
                    let tile = tilemap::read_tile(tile_bytes);   // TODO support 16 height tiles...
                    let palette = Ppu::read_palette(self.bgp); // TODO Check attr.palette_is_obp1 to get palette

                    let mut tile_row = distance_sprite_top_y_to_scanline;
                    if attr.flip_y { tile_row = sprite_height - tile_row - 1 };
                    if tile_row >= 8 { break; }  // TODO fix 16 height tiles...
                    for tile_col in 0..8 {
                        let color_i = tile[tile_row as usize][tile_col as usize];

                        if color_i > 0 {
                            let color = palette[color_i as usize];

                            // TODO selection priority and drawing priority
                            let has_priority = true;

                            let x_offset = if attr.flip_x { 7 - tile_col } else { tile_col };
                            let x = attr.x.wrapping_sub(8).wrapping_add(x_offset);
                            if x as usize >= self.lcd.data[0].len() { continue };

                            let bg_color = self.lcd.data[scanline as usize][x as usize];

                            if has_priority || bg_color == 0 {
                                self.lcd.data[scanline as usize][x as usize] = color + 8; // TEMP colors to distinguish sprites
                            }
                        }
                    }
                }
            }
        }
    }

    #[allow(dead_code)]
    fn fill_lcd_row(&mut self, mmu: &mut Mmu, mode: DrawMode) {
        let index_mode_8000 = self.lcd_control.check_bit(mmu, LcdControlRegBit::AddressingMode8000 as u8);
        let tilemap_at_9c00_bit = match mode {
            DrawMode::Window => LcdControlRegBit::WindowTilemapIsAt9C00,
            DrawMode::Background | _ => LcdControlRegBit::BackgroundTilemapIsAt9C00,
        } as u8;
        let background_tilemap_address = if self.lcd_control.check_bit(mmu, tilemap_at_9c00_bit) {
            0x9C00
        } else {
            0x9800
        };
        let (y_offset, x_offset, x_start) = if self.in_debug_mode {
            match mode {
                DrawMode::Window => (self.wy, 0, self.wx.wrapping_sub(7)), // TODO fix
                DrawMode::Background | _ => (0, 0, 0),
            }
        } else {
            match mode {
                DrawMode::Window => (self.wy, 0, self.wx.wrapping_sub(7)), // TODO fix
                DrawMode::Background | _ => (self.scy, self.scx, 0),
            }
        };
        let lcd_row = self.ly.wrapping_sub(y_offset) as usize;
        let tilemap_row = (self.ly / 8) as usize;
        let tile_row = (self.ly % 8) as usize;
        let palette = Ppu::read_palette(self.bgp);

        if lcd_row < self.lcd.height {
            for x in x_start as usize..self.lcd.width {
                let lcd_col = (x as u8).wrapping_sub(x_offset as u8) as usize; // TODO fix
                if lcd_col >= self.lcd.width { break; }
                let tilemap_col = (lcd_col / 8) as usize;
                let tile_col = (lcd_col % 8) as usize;

                let tile_index_address = (tilemap_row * 32 + tilemap_col) + background_tilemap_address;
                let tile_index = mmu.read_8(tile_index_address as u16) as i32;

                let mut tile_address = if index_mode_8000 {
                    0x8000 + tile_index * 16
                } else {
                    let tile_index = (tile_index as i8) as i32; // signed
                    0x9000 + tile_index * 16
                } as usize;
                tile_address += tile_row * 2;

                let tile_byte1 = mmu.read_8(tile_address as u16);
                let tile_byte2 = mmu.read_8((tile_address + 1) as u16);

                let low = ((tile_byte1 << tile_col) >> 7) << 1;
                let high = (tile_byte2 << tile_col) >> 7;
                let color = high + low;

                if mode == DrawMode::Background || color > 0 { // TODO priority, transparency
                    self.lcd.data[lcd_row][lcd_col] = palette[color as usize] + match mode {
                        DrawMode::Background => 0,
                        DrawMode::Window => 4,
                        DrawMode::Sprites => 8,
                    }
                }
            }
        }
    }

    #[allow(dead_code)]
    fn draw_sprites(&mut self, mmu: &mut Mmu) {
        if self.lcd_control.check_bit(mmu, LcdControlRegBit::ObjEnabled as u8) {
            let tiledata_address: usize = 0x8000;

            for attr in self.sprite_attributes {
                let tile_index = attr.tile_index as usize;
                let tile_address = tiledata_address + tile_index * 16;
                let tile_bytes = mmu.read_buffer(tile_address as u16, tile_address as u16 + 16)
                    .try_into().unwrap();
                let tile = tilemap::read_tile(tile_bytes);  // TODO support 16 height tiles...
                let sprite_height: u8 = if self.lcd_control.check_bit(mmu, LcdControlRegBit::SpriteSizeIs16 as u8) { 16 } else { 8 };
                let palette = Ppu::read_palette(self.bgp); // TODO Check attr.palette_is_obp1 to get palette

                for tile_row in 0..sprite_height {
                    let y_offset = if attr.flip_y { sprite_height - tile_row - 1 } else { tile_row };
                    let y = attr.y.wrapping_sub(16).wrapping_add(y_offset);
                    if y as usize >= self.lcd.data.len() { continue };

                    for tile_col in 0..8 {
                        let color_i = tile[tile_row as usize][tile_col as usize];

                        if color_i > 0 {
                            let color = palette[color_i as usize];

                            // TODO selection priority and drawing priority
                            let has_priority = true;

                            let x_offset = if attr.flip_x { 7 - tile_col } else { tile_col };
                            let x = attr.x.wrapping_sub(8).wrapping_add(x_offset);
                            if x as usize >= self.lcd.data[0].len() { continue };

                            let bg_color = self.lcd.data[y as usize][x as usize];

                            if has_priority || bg_color == 0 {
                                self.lcd.data[y as usize][x as usize] = color + 8; // TEMP colors to distinguish sprites
                            }
                        }
                    }
                }
            }
        }
    }

    // For debugging
    #[allow(dead_code)]
    fn draw_lcd_border(&mut self) {
        let y0 = self.scy as usize;
        let y1 = self.scy.wrapping_add(144 as u8) as usize;
        let x0 = self.scx as usize;
        let x1 = self.scx.wrapping_add(160 as u8) as usize;

        if x0 <= x1 {
            for x in x0..min(x1, self.lcd.data[0].len()) {
                if y0 < self.lcd.data.len() {
                    self.lcd.data[y0][x] = DEBUG_COLOR_I;
                }
                if y1.wrapping_sub(1) < self.lcd.data.len() {
                    self.lcd.data[y1.wrapping_sub(1)][x] = DEBUG_COLOR_I;
                }
            }
        } else {
            for x in 0..min(x1, self.lcd.data.last().unwrap().len()) {
                if y0 < self.lcd.data.len() {
                    self.lcd.data[y0][x] = DEBUG_COLOR_I;
                }
                if y1.wrapping_sub(1) < self.lcd.data.len() {
                    self.lcd.data[y1.wrapping_sub(1)][x] = DEBUG_COLOR_I;
                }
            }
            for x in x0..self.lcd.data[0].len() {
                if y0 < self.lcd.data.len() {
                    self.lcd.data[y0][x] = DEBUG_COLOR_I;
                }
                if y1.wrapping_sub(1) < self.lcd.data.len() {
                    self.lcd.data[y1.wrapping_sub(1)][x] = DEBUG_COLOR_I;
                }
            }
        }

        if y0 <= y1 {
            for y in y0..min(y1, self.lcd.data.len()) {
                if x0 < self.lcd.data[0].len() {
                    self.lcd.data[y][x0] = DEBUG_COLOR_I;
                }
                if x1.wrapping_sub(1) < self.lcd.data[0].len() {
                    self.lcd.data[y][x1.wrapping_sub(1)] = DEBUG_COLOR_I;
                }
            }
        } else {
            for y in 0..min(y1, self.lcd.data.len()) {
                if x0 < self.lcd.data[0].len() {
                    self.lcd.data[y][x0] = DEBUG_COLOR_I;
                }
                if x1.wrapping_sub(1) < self.lcd.data[0].len() {
                    self.lcd.data[y][x1.wrapping_sub(1)] = DEBUG_COLOR_I;
                }
            }
            for y in y0..self.lcd.data.len() {
                if x0 < self.lcd.data[0].len() {
                    self.lcd.data[y][x0] = DEBUG_COLOR_I;
                }
                if x1.wrapping_sub(1) < self.lcd.data[0].len() {
                    self.lcd.data[y][x1.wrapping_sub(1)] = DEBUG_COLOR_I;
                }
            }
        }
    }

    // For debugging
    #[allow(dead_code)]
    fn speed_check_pixel_transfer(&mut self) {
        self.lcd.data[self.ly as usize % self.lcd.height][self.clocks as usize % self.lcd.width] += 1;
        self.lcd.data[self.ly as usize % self.lcd.height][self.clocks as usize % self.lcd.width] %= 8;
    }

    // For debugging
    #[allow(dead_code)]
    pub(crate) fn display_tiles_at(&mut self, mmu: &mut Mmu, tiledata_address: usize, scy: u8, scx: u8) {
        let mut tile_addresses = [0; 32 * 32];
        for i in 0..32 * 32 {
            tile_addresses[i] = tiledata_address + 16 * i;
        }
        let background_tilemap = tilemap::fetch_tilemap(mmu, tile_addresses);
        self.lcd.fill_from_tilemap(background_tilemap, scy, scx, &[0, 1, 2, 3]);
    }

    // For debugging
    #[allow(dead_code)]
    pub(crate) fn display_tiles_from_indices_at(&mut self, mmu: &mut Mmu, tilemap_at_9c00: bool, index_mode_8000: bool, scy: u8, scx: u8) {
        let tilemap_address = if tilemap_at_9c00 {
            0x9C00
        } else {
            0x9800
        };

        let mut tilemap: TileMap = [[[[0; 8]; 8]; 32]; 32];

        for i in 0..32 * 32 {
            let tile_index = mmu.read_8((tilemap_address + i) as u16) as i32;
            let tile_address = if index_mode_8000 {
                0x8000 + tile_index * 16
            } else {
                let tile_index = (tile_index as i8) as i32; // signed
                0x9000 + tile_index * 16
            } as usize;

            let tile_bytes: [u8; 16] = mmu.read_buffer(tile_address as u16, tile_address as u16 + 16)
                .try_into().unwrap();

            let tile = tilemap::read_tile(tile_bytes);
            tilemap[i / 32][i % 32] = tile;
        }

        self.lcd.fill_from_tilemap(tilemap, scy, scx, &[0, 1, 2, 3]);
    }
}
