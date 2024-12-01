use crate::console::interrupts::{InterruptRegBit, Interrupts};
use crate::console::mmu;
use crate::console::mmu::{Caller, Mmu};
use crate::console::register::{Register};
use crate::console::sprite_attribute::SpriteAttribute;

pub(crate) const STAT_MODES: [StatMode; 4] = [
    StatMode::HBlank,
    StatMode::VBlank,
    StatMode::OamSearch,
    StatMode::PixelTransfer,
];

// From Michael Steil, Ultimate Game Boy Talk
// (Gekkio (Mooneye) has HBlank=50 and OAM=21)
const MODE_DURATION: [usize; 4] = [
    51 * 4,             // = 204    HBlank (0)
    (20 + 43 + 51) * 4, // = 456    VBlank (1)
    20 * 4,             // = 80     OAM (2)
    43 * 4,             // = 172    PixelTransfer (3)
];

#[allow(dead_code)]
const MODE_LINE_RANGE: [(u8, u8); 4] = [
    (0, 144),   // HBlank
    (144, 154), // VBlank
    (0, 144),   // OamSearch
    (0, 144),   // PixelTransfer
];

#[derive(Copy, Clone, PartialEq)]
pub(crate) enum StatMode {
    HBlank = 0,
    VBlank = 1,
    OamSearch = 2,
    PixelTransfer = 3,
}

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
enum DrawMode {
    Background,
    Sprites,
    Window,
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

#[allow(dead_code)]
pub(crate) struct Ppu {
    mode_cycle_count: usize,
    scy: Register,
    scx: Register,
    ly: Register,
    lyc: Register,
    dma: Register,
    bgp: Register,
    obp0: Register,
    obp1: Register,
    wy: Register,
    wx: Register,
    lcd_control: Register,
    lcd_status: Register,
    sprite_attributes: [SpriteAttribute; 40],
    pub(crate) lcd: Lcd,
}

impl Ppu {
    pub(crate) fn new() -> Ppu {
        Ppu {
            mode_cycle_count: 0,
            scy: Register::new(mmu::SCY_REG),
            scx: Register::new(mmu::SCX_REG),
            ly: Register::new(mmu::LY_REG),
            lyc: Register::new(mmu::LYC_REG),
            dma: Register::new(mmu::DMA_REG),
            bgp: Register::new(mmu::BGP_REG),
            obp0: Register::new(mmu::OBP0_REG),
            obp1: Register::new(mmu::OBP1_REG),
            wy: Register::new(mmu::WY_REG),
            wx: Register::new(mmu::WX_REG),
            lcd_control: Register::new(mmu::LCD_CONTROL_REG),
            lcd_status: Register::new(mmu::LCD_STATUS_REG),
            sprite_attributes: [SpriteAttribute::new(&[0; 4]); 40],
            lcd: Lcd::new(),
        }
    }

    // TODO Fix OAM DMA timing (mooneye acceptance/add_sp_e_timing, jp_timing)
    pub(crate) fn oam_dma(&mut self, mmu: &mut Mmu) {
        if let Some(mut src_address) = mmu.oam_dma_src_addr {
            let value = mmu.read_8(src_address, Caller::PPU);
            let oam_address = mmu::OAM_START | (src_address & 0xFF);

            // Writes to OAM not allowed in certain PPU modes
            let lcd_status_value = self.lcd_status.read(mmu, Caller::PPU);
            let stat_mode = STAT_MODES[(lcd_status_value & 0x03) as usize];
            if stat_mode != StatMode::PixelTransfer && stat_mode != StatMode::OamSearch {
                mmu.write_8(oam_address, value, Caller::PPU);
            }

            // Increment source address
            src_address += 1;
            mmu.oam_dma_src_addr = if (src_address & 0xFF) > 0x9F {
                None // Stop
            } else {
                Option::from(src_address)
            };
        }
    }

    pub(crate) fn step(&mut self, cycles: u16, interrupts: &mut Interrupts, mmu: &mut Mmu) {
        if self.lcd_control.check_bit(mmu, mmu::LcdControlRegBit::LcdAndPpuEnabled as u8, Caller::PPU) {
            // let mode = mmu.ppu_mode;
            let mode_flag = (self.lcd_status.read(mmu, Caller::PPU) & 0x03) as usize;
            let mode = STAT_MODES[mode_flag];
            let mode_duration = MODE_DURATION[mode as usize];

            // HACK Force current line ly to align with current mode
            // let (mode_y_start, mode_y_end) = MODE_LINE_RANGE[mode as usize];
            // if self.ly.read(mmu, Caller::PPU) < mode_y_start || mode_y_end <= self.ly.read(mmu, Caller::PPU) {
            //     self.set_ly(mmu, mode_y_start);
            // }

            // If time to do so, move to next stat mode
            if self.mode_cycle_count >= mode_duration {
                self.move_to_next_stat_mode(mmu, interrupts, mode);
            }

            // Do mode-specific action
            match mode {
                StatMode::OamSearch => {
                    self.oam_search(mmu);
                }
                StatMode::PixelTransfer => {
                    self.pixel_transfer(mmu);
                }
                StatMode::HBlank => {}
                StatMode::VBlank => {}
            }

            // If LY == LYC
            let ly = mmu.read_8_ram(mmu::LY_REG, Caller::PPU);
            let lyc = self.lyc.read(mmu, Caller::PPU);

            if ly == lyc {
                self.lcd_status.set_bit(mmu, mmu::LcdStatRegBit::LycEqLy as u8, true, Caller::PPU);
                if self.lcd_status.check_bit(mmu, mmu::LcdStatRegBit::LycInterruptEnabled as u8, Caller::PPU) {
                    interrupts.requested.set_bit(mmu, InterruptRegBit::LcdStat as u8, true, Caller::PPU);
                }
            } else {
                self.lcd_status.set_bit(mmu, mmu::LcdStatRegBit::LycEqLy as u8, false, Caller::PPU);
            }
        }

        self.mode_cycle_count += cycles as usize;
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

    fn increment_ly(&mut self, mmu: &mut Mmu) -> u8 {
        let mut new_ly = mmu.read_8_ram(mmu::LY_REG, Caller::PPU).wrapping_add(1);
        if new_ly >= 154 {
            new_ly = 0;
        }

        mmu.write_8_ram(mmu::LY_REG, new_ly, Caller::PPU)
    }

    fn set_stat_mode(&mut self, mmu: &mut Mmu, new_mode: StatMode, interrupts: &mut Interrupts) {
        let curr_value = self.lcd_status.read(mmu, Caller::PPU);
        let new_value = (curr_value & 0xFC) | (new_mode as u8);
        self.lcd_status.write(mmu, new_value, Caller::PPU);

        // Request interrupts
        match new_mode {
            StatMode::HBlank => {
                if self.lcd_status.check_bit(mmu, mmu::LcdStatRegBit::HBlankInterruptEnabled as u8, Caller::PPU) {
                    interrupts.requested.set_bit(mmu, InterruptRegBit::LcdStat as u8, true, Caller::PPU);
                }
            }
            StatMode::VBlank => {
                // Always request VBLank interrupt
                interrupts.requested.set_bit(mmu, InterruptRegBit::VBlank as u8, true, Caller::PPU);

                if self.lcd_status.check_bit(mmu, mmu::LcdStatRegBit::VBlankInterruptEnabled as u8, Caller::PPU) {
                    interrupts.requested.set_bit(mmu, InterruptRegBit::LcdStat as u8, true, Caller::PPU);
                }
                // Mooneye includes a check if OAM INT enabled when swithcing to VBLank also, but I'm not sure why
                if self.lcd_status.check_bit(mmu, mmu::LcdStatRegBit::OamInterruptEnabled as u8, Caller::PPU) {
                    interrupts.requested.set_bit(mmu, InterruptRegBit::LcdStat as u8, true, Caller::PPU);
                }
            }
            StatMode::OamSearch => {
                if self.lcd_status.check_bit(mmu, mmu::LcdStatRegBit::OamInterruptEnabled as u8, Caller::PPU) {
                    interrupts.requested.set_bit(mmu, InterruptRegBit::LcdStat as u8, true, Caller::PPU);
                }
            }
            StatMode::PixelTransfer => {
                // Do nothing
            }
        }
    }

    fn move_to_next_stat_mode(&mut self, mmu: &mut Mmu, interrupts: &mut Interrupts, curr_mode: StatMode) {
        self.mode_cycle_count = 0;

        match curr_mode {
            StatMode::OamSearch => {
                self.set_stat_mode(mmu, StatMode::PixelTransfer, interrupts);
            }
            StatMode::PixelTransfer => {
                self.set_stat_mode(mmu, StatMode::HBlank, interrupts);
            }
            StatMode::HBlank => {
                let ly = self.increment_ly(mmu);
                if ly < 144 {
                    self.set_stat_mode(mmu, StatMode::OamSearch, interrupts);
                } else {
                    self.set_stat_mode(mmu, StatMode::VBlank, interrupts);
                }
            }
            StatMode::VBlank => {
                let ly = self.increment_ly(mmu);
                if ly == 0 { // ly < MODE_LINE_RANGE[StatMode::OamSearch as usize].1 {
                    self.set_stat_mode(mmu, StatMode::OamSearch, interrupts);
                }
                //else still in VBlank mode, just on a new line.
            }
        }
    }

    fn oam_search(&mut self, mmu: &mut Mmu) {
        let ly = mmu.read_8_ram(mmu::LY_REG, Caller::PPU);
        if ly == 0 {
            self.lcd_control.read(mmu, Caller::PPU);
            let obj_enabled = self.lcd_control.check_bit(mmu, mmu::LcdControlRegBit::ObjEnabled as u8, Caller::PPU);
            if obj_enabled {
                let attribute_data = mmu.read_buffer(mmu::OAM_START, mmu::OAM_END + 1, Caller::PPU);
                for i in 0..40 {
                    let data = &attribute_data[(i * 4)..(i * 4) + 4];
                    let attr = SpriteAttribute::new(data.try_into().unwrap());
                    self.sprite_attributes[i] = attr;
                }
            }
        }
    }

    // TODO: Fix graphics misaligned on screen bug
    fn pixel_transfer(&mut self, mmu: &mut Mmu) {
        self.draw_background_line(mmu);
        self.draw_window_line(mmu);
        self.draw_sprites_line(mmu);
    }

    fn draw_background_line(&mut self, mmu: &mut Mmu) {
        if self.lcd_control.check_bit(mmu, mmu::LcdControlRegBit::BackgroundAndWindowEnabled as u8, Caller::PPU) {
            self.draw_background_or_window_line(mmu, DrawMode::Background);
        }
    }

    fn draw_window_line(&mut self, mmu: &mut Mmu) {
        if self.lcd_control.check_bit(mmu, mmu::LcdControlRegBit::BackgroundAndWindowEnabled as u8, Caller::PPU)
                && self.lcd_control.check_bit(mmu, mmu::LcdControlRegBit::WindowEnabled as u8, Caller::PPU) {
            self.draw_background_or_window_line(mmu, DrawMode::Window);
        }
    }

    fn fetch_tile(mmu: &mut Mmu, tile_address: u16) -> [[u8; 8]; 8] {
        let tile_bytes: [u8; 16] = mmu.read_buffer(tile_address, tile_address + 16, Caller::PPU)
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
        if self.lcd_control.check_bit(mmu, mmu::LcdControlRegBit::ObjEnabled as u8, Caller::PPU) {
            let tiledata_address: usize = 0x8000;
            let ly = mmu.read_8_ram(mmu::LY_REG, Caller::PPU);

            for attr in self.sprite_attributes {
                let is_16_sprite = self.lcd_control.check_bit(mmu, mmu::LcdControlRegBit::SpriteSizeIs16 as u8, Caller::PPU);
                let sprite_top_y = attr.y.wrapping_sub(16);
                let sprite_height: u8 = if is_16_sprite { 16 } else { 8 };

                if sprite_top_y <= ly && ly < (sprite_top_y + sprite_height) {
                    let palette = Ppu::read_palette(self.bgp.read(mmu, Caller::PPU)); // TODO Check attr.palette_is_obp1 to get palette

                    let mut tile_row_n = (ly - sprite_top_y) as usize;
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

                            let bg_color = self.lcd.data[ly as usize][x as usize];

                            if has_priority || bg_color == 0 {
                                self.lcd.data[ly as usize][x as usize] = color + 8; // TEMP colors to distinguish sprites
                            }
                        }
                    }
                }
            }
        }
    }

    fn draw_background_or_window_line(&mut self, mmu: &mut Mmu, mode: DrawMode) { // TOO SLOW
        let ly = mmu.read_8_ram(mmu::LY_REG, Caller::PPU) as i32;

        let y_offset = match mode {
            DrawMode::Window => {
                let wy = self.wy.read(mmu, Caller::PPU) as i32;
                if wy >= ly  { return; } // offscreen
                -wy
            },
            DrawMode::Background => {
                self.scy.read(mmu, Caller::PPU) as i32
            },
            | _ => panic!("Mode {:?} is not allowed here", mode),
        };
        let x_offset = match mode {
            DrawMode::Window => {
                let wx = self.wx.read(mmu, Caller::PPU);
                if wx >= (self.lcd.width as u8 + 7) { return; } // offscreen
                -(wx as i32 - 7)
            },
            DrawMode::Background => {
                self.scx.read(mmu, Caller::PPU) as i32
            },
            | _ => panic!("Mode {:?} is not allowed here", mode),
        };

        let palette = Ppu::read_palette(self.bgp.read(mmu, Caller::PPU));
        let index_mode_8000 = self.lcd_control.check_bit(mmu, mmu::LcdControlRegBit::AddressingMode8000 as u8, Caller::PPU);
        let tilemap_base_address: i32 = if index_mode_8000 { 0x8000 } else { 0x9000 };
        let tilemap_at_9c00_bit = match mode {
            DrawMode::Window => mmu::LcdControlRegBit::WindowTilemapIsAt9C00,
            DrawMode::Background | _ => mmu::LcdControlRegBit::BackgroundTilemapIsAt9C00,
        } as u8;
        let is_tilemap_at_9c00 = self.lcd_control.check_bit(mmu, tilemap_at_9c00_bit, Caller::PPU);
        let tilemap_address: u16 = if is_tilemap_at_9c00 { 0x9C00 } else { 0x9800 };

        let tilemap_row = (ly + y_offset) as u8 / 8;
        let tile_row = (ly + y_offset) as u8 % 8;
        let tile_index_base_address = tilemap_address + tilemap_row as u16 * 32;

        for x in 0..self.lcd.width as i32 {
            let tilemap_col = (x + x_offset) as u8 / 8;
            let tile_col = (x + x_offset) as u8 % 8;

            let tile_index_address = tile_index_base_address + tilemap_col as u16;
            let tile_index = mmu.read_8(tile_index_address, Caller::PPU);
            let tile_address_offset: i32 = if index_mode_8000 {
                tile_index as i32 * 16
            } else {
                (tile_index as i8) as i32 * 16 // signed
            };
            let tile_address = (tilemap_base_address + tile_address_offset) + tile_row as i32 * 2;

            let tile_byte1 = mmu.read_8(tile_address as u16, Caller::PPU);
            let tile_byte2 = mmu.read_8(tile_address as u16 + 1, Caller::PPU);

            let low = ((tile_byte1 << tile_col) >> 7) << 1;
            let high = (tile_byte2 << tile_col) >> 7;
            let color = high + low;

            if mode == DrawMode::Background || color > 0 { // TODO priority, transparency
                let pixel_color = palette[color as usize]
                    // adjust colors of different layers
                    + match mode {
                        DrawMode::Window => 4,
                        _ => 0,
                    };
                self.lcd.data[ly as usize][x as usize] = pixel_color;
            }
        }
    }
}
