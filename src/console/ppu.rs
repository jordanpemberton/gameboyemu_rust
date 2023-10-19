use crate::console::interrupts::{InterruptRegBit, Interrupts};
use crate::console::mmu;
use crate::console::mmu::{Memory, Mmu};
use crate::console::register::Register;
use crate::console::sprite_attribute::SpriteAttribute;

pub(crate) const LCD_CONTROL_REG: u16 = 0xFF40;
pub(crate) const LCD_STATUS_REG: u16 = 0xFF41;
pub(crate) const SCY_REG: u16 = 0xFF42;
pub(crate) const SCX_REG: u16 = 0xFF43;
pub(crate) const LY_REG: u16 = 0xFF44;
pub(crate) const LYC_REG: u16 = 0xFF45;
pub(crate) const DMA_REG: u16 = 0xFF46;
pub(crate) const BGP_REG: u16 = 0xFF47;
pub(crate) const OBP0_REG: u16 = 0xFF48;
pub(crate) const OBP1_REG: u16 = 0xFF49;
pub(crate) const WY_REG: u16 = 0xFF4A;
pub(crate) const WX_REG: u16 = 0xFF4B;

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
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) data: Vec<Vec<u8>>,
}

impl Lcd {
    pub(crate) fn new() -> Lcd {
        let width: usize = 160;
        let height: usize = 144;

        Lcd {
            width,
            height,
            data: vec![vec![0; width]; height],
        }
    }
}

pub(crate) struct Ppu {
    clocks: usize,

    // Registers in IO RAM, 0xFF40..=0xFF4B
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
    // VRAM, 0x8000..=0x9FFF
    vram: [u8; 0x1000],

    sprite_attributes: [SpriteAttribute; 40],
    pub(crate) lcd: Lcd,
}

impl Memory for Ppu {
    fn read_8(&mut self, address: u16) -> u8 {
        match address {
            LCD_CONTROL_REG => self.lcd_control.value,
            LCD_STATUS_REG => self.lcd_status.value,
            SCY_REG => self.scy,
            SCX_REG => self.scx,
            LY_REG => self.ly,
            LYC_REG => self.lyc,
            DMA_REG => self.dma,
            BGP_REG => self.bgp,
            OBP0_REG => self.obp0,
            OBP1_REG => self.obp1,
            WY_REG => self.wy,
            WX_REG => self.wx,
            0x8000..=0x9FFF => self.vram[address as usize - 0x8000],
            _ => panic!("Address {:} is not available to PPU", address)
        }
    }

    #[allow(unused_variables)]
    fn write_8(&mut self, address: u16, value: u8) {
        todo!()
    }
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
            lcd_control: Register::new(mmu, mmu::LCD_CONTROL_REG),
            lcd_status: Register::new(mmu, mmu::LCD_STATUS_REG),
            vram: [0; 0x1000],
            sprite_attributes: [SpriteAttribute::new(&[0; 4]); 40],
            lcd: Lcd::new(),
        }
    }

    pub(crate) fn step(&mut self, cycles: u16, interrupts: &mut Interrupts, mmu: &mut Mmu) {
        self.refresh_from_mem(mmu);

        if self.lcd_control.check_bit(mmu, LcdControlRegBit::LcdAndPpuEnabled as u8) {
            self.clocks += cycles as usize;

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
                    if self.clocks < mode_duration {
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
                        if self.ly < MODE_LINE_RANGE[StatMode::OamSearch as usize].1 {
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
                        if self.ly < MODE_LINE_RANGE[StatMode::OamSearch as usize].1 {
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
        self.draw_background_line(mmu);
        self.draw_window_line(mmu);
        self.draw_sprites_line(mmu);
    }

    fn draw_background_line(&mut self, mmu: &mut Mmu) {
        if self.lcd_control.check_bit(mmu, LcdControlRegBit::BackgroundAndWindowEnabled as u8) {
            self.draw_background_or_window_line(mmu, DrawMode::Background);
        }
    }

    #[allow(dead_code)]
    fn draw_window_line(&mut self, mmu: &mut Mmu) {
        let is_dmg = true;  // TODO
        if (is_dmg && self.lcd_control.check_bit(mmu, LcdControlRegBit::BackgroundAndWindowEnabled as u8))
                || (!is_dmg && self.lcd_control.check_bit(mmu, LcdControlRegBit::WindowEnabled as u8)) {
            self.draw_background_or_window_line(mmu, DrawMode::Window);
        }
    }

    fn fetch_tile(mmu: &mut Mmu, tile_address: u16) -> [[u8; 8]; 8] {
        let tile_bytes: [u8; 16] = mmu.read_buffer(tile_address, tile_address + 16)
            .try_into().unwrap();
        let mut tile = [[0; 8]; 8];
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

    fn draw_sprites_line(&mut self, mmu: &mut Mmu) {
        if self.lcd_control.check_bit(mmu, LcdControlRegBit::ObjEnabled as u8) {
            let tiledata_address: usize = 0x8000;
            let y = self.ly;

            for attr in self.sprite_attributes {
                let is_16_sprite = self.lcd_control.check_bit(mmu, LcdControlRegBit::SpriteSizeIs16 as u8);
                let sprite_top_y = attr.y.wrapping_sub(16);
                let sprite_height: u8 = if is_16_sprite { 16 } else { 8 };

                if sprite_top_y <= y && y < sprite_top_y + sprite_height {
                    let palette = Ppu::read_palette(self.bgp); // TODO Check attr.palette_is_obp1 to get palette

                    let mut tile_row_n = (y - sprite_top_y) as usize;
                    if attr.flip_y {
                        tile_row_n = sprite_height as usize - tile_row_n - 1;
                    }

                    let tile_row = if tile_row_n < 8 {
                        let tile_index_1 = if is_16_sprite { attr.tile_index & 0xFE } else { attr.tile_index } as usize;
                        let tile_address_1 = (tiledata_address + tile_index_1 * 16) as u16;
                        let tile1= Ppu::fetch_tile(mmu, tile_address_1);
                        tile1[tile_row_n]
                    } else {
                        let tile_index_2 = if is_16_sprite { attr.tile_index | 0x01 } else { attr.tile_index } as usize;
                        let tile_address_2 = (tiledata_address + tile_index_2 * 16) as u16;
                        let tile2 = Ppu::fetch_tile(mmu, tile_address_2);
                        tile2[tile_row_n - 8]
                    };

                    for tile_col in 0..8 {
                        let color_i = tile_row[tile_col as usize];

                        if color_i > 0 {
                            let color = palette[color_i as usize];

                            // TODO selection priority and drawing priority
                            let has_priority = true;
                            let mut x = attr.x.wrapping_sub(8);
                            let x_offset = if attr.flip_x { 7 - tile_col } else { tile_col };
                            x = x.wrapping_add(x_offset);
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

    fn draw_background_or_window_line(&mut self, mmu: &mut Mmu, mode: DrawMode) { // TOO SLOW
        let palette = Ppu::read_palette(self.bgp);

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

        let y = self.ly as usize;
        let y_offset = self.scy as usize;
        let x_offset = self.scx as usize;
        let tilemap_row = (y + y_offset) / 8;
        let tile_row = (y + y_offset) % 8;

        for x in 0..self.lcd.width {
            let tilemap_col = (x + x_offset) / 8;
            let tile_col = (x + x_offset) % 8;

            let tile_index_address = background_tilemap_address + tilemap_row * 32 + tilemap_col;
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
                self.lcd.data[y][x] = palette[color as usize] + match mode {
                    DrawMode::Window => 4,
                    DrawMode::Background | _ => 0,
                };
            }
        }
    }
}
