use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;
use crate::console::ppu::{Lcd, Ppu};

pub(crate) const WINDOW_SCALE: u32 = 5;

const GRAY_LT: Color = Color::RGB(170, 170, 170);
const GRAY_DK: Color = Color::RGB(85, 85, 85);
const PINK_LT: Color = Color::RGB(255, 128, 128);
const PINK_LT_MUTED: Color = Color::RGB(BLUE_LT.r / 4 * 3, BLUE_LT.g / 4 * 3, BLUE_LT.b);
const BLUE_LT: Color = Color::RGB(64, 191, 255);
const BLUE_LT_MUTED: Color = Color::RGB(PINK_LT.r, PINK_LT.g / 4 * 3, PINK_LT.b / 4 * 3);

const PALETTE_BW: [Color; 4] = [
    Color::WHITE,
    GRAY_LT,
    GRAY_DK,
    Color::BLACK,
];

const PALETTE_CMYK: [Color; 4] = [
    Color::CYAN,
    Color::MAGENTA,
    Color::YELLOW,
    Color::BLACK,
];

const PALETTE_TRANS: [Color; 4] = [
    Color::WHITE,
    BLUE_LT,
    PINK_LT,
    Color::BLACK,
];

const PALETTE_TRANS_MUTED: [Color; 4] = [
    Color::WHITE,
    BLUE_LT_MUTED,
    PINK_LT_MUTED,
    Color::BLACK,
];

const PALETTE_DEBUG: [Color; 4] = [
    Color::RED,
    Color::GREEN,
    Color::BLUE,
    Color::WHITE,
];

fn create_sdl_canvas(sdl_context: &Sdl, window_width: u32, window_height: u32, window_title: &str) -> WindowCanvas {
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(window_title, window_width, window_height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    window.into_canvas()
        .build()
        .unwrap()
}

pub(crate) struct Display {
    selected_palette: usize,
    palettes: [[Color; 16]; 4],
    pixels: Vec<Vec<Rect>>,
    canvas: WindowCanvas,
}

impl Display {
    pub(crate) fn new(
            window_scale: u32,
            window_title: &str,
            sdl_context: &Sdl,
            lcd_pixel_width: usize,
            lcd_pixel_height: usize) -> Display {
        let mut pixels = vec![vec![Rect::new(0,0,0,0); lcd_pixel_width]; lcd_pixel_height];
        for y in 0..lcd_pixel_height as u32 {
            for x in 0..lcd_pixel_width as u32 {
                pixels[y as usize][x as usize] = Rect::new(
                    (x * window_scale) as i32,
                    (y * window_scale) as i32,
                    window_scale,
                    window_scale);
            }
        }

        Display {
            selected_palette: 1,
            palettes: [
                Display::make_palette(PALETTE_BW, PALETTE_BW, PALETTE_BW, PALETTE_DEBUG),
                Display::make_palette(PALETTE_BW, PALETTE_TRANS, PALETTE_TRANS_MUTED, PALETTE_DEBUG),
                Display::make_palette(PALETTE_CMYK, PALETTE_CMYK, PALETTE_CMYK, PALETTE_DEBUG),
                Display::generate_random_palette()
            ],
            canvas:
                create_sdl_canvas(
                    sdl_context,
                    lcd_pixel_width as u32 * window_scale,
                    lcd_pixel_height as u32 * window_scale,
                    window_title
                ),
            pixels,
        }
    }

    pub(crate) fn draw(&mut self, ppu: &mut Ppu) {
        self.canvas.clear();

        self.draw_screen(&ppu.lcd);

        self.canvas.present();
        self.canvas.set_draw_color(self.palettes[self.selected_palette][0]);
    }

    pub(crate) fn cycle_color_palette(&mut self) {
        self.selected_palette = (self.selected_palette + 1) % 4;
        if self.selected_palette == 3 {
            self.palettes[3] = Display::generate_random_palette();
        }
    }

    fn draw_scanline(&mut self, y: usize, scanline: Vec<Color>) {
        for x in 0..scanline.len() {
            let color = scanline[x];
            self.canvas.set_draw_color(color);
            self.canvas.fill_rect(self.pixels[y][x]).unwrap();
        }
    }

    fn draw_screen(&mut self, lcd: &Lcd) {
        let height = self.pixels.len();
        for y in 0..height {
            let row = lcd.data[y].as_slice().iter();
            let colors = row.map(|pixel| self.palettes[self.selected_palette][*pixel as usize]).collect();
            self.draw_scanline(y, colors);
        }
    }

    fn make_palette(background_palette: [Color; 4], window_palette: [Color; 4], sprite_palette: [Color; 4], debug_palette: [Color; 4]) -> [Color; 16] {
        let mut palette: [Color; 16] = [Color::CYAN; 16];
        palette[..4].copy_from_slice(&background_palette);
        palette[4..8].copy_from_slice(&window_palette);
        palette[8..12].copy_from_slice(&sprite_palette);
        palette[12..16].copy_from_slice(&debug_palette);
        palette
    }

    fn generate_random_palette() -> [Color; 16] {
        let mut palette: [Color; 16] = [Color::CYAN; 16];
        for i in 0..12 {
            palette[i] = Self::random_color();
        }
        palette[12..16].copy_from_slice(&PALETTE_DEBUG);
        palette
    }

    fn random_color() -> Color {
        let r: u8 = rand::random_range(0..=255);
        let g: u8 = rand::random_range(0..=255);
        let b: u8 = rand::random_range(0..=255);
        Color::RGB(r, g, b)
    }
}
