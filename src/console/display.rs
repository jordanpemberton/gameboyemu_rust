use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;
use crate::console::ppu::{Lcd, Ppu};

pub(crate) const WINDOW_SCALE: u32 = 5;

const WHITE: Color = Color::RGB(255, 255, 255);
const GRAY_LT: Color = Color::RGB(170, 170, 170);
const GRAY_DK: Color = Color::RGB(85, 85, 85);
const BLACK: Color = Color::RGB(0, 0, 0);

const PINK_LT: Color = Color::RGB(255, 128, 128);
const BLUE_LT: Color = Color::RGB(64, 191, 255);
const RED: Color = Color::RGB(255, 0, 0);

const YELLOW: Color = Color::RGB(255, 255, 0);

const COLORS: [Color; 15] = [
    // Background
    WHITE,
    GRAY_LT,
    GRAY_DK,
    BLACK,

    // Window
    WHITE,
    BLUE_LT,
    PINK_LT,
    BLACK,

    // Sprites
    WHITE,
    Color::RGB(BLUE_LT.r / 4 * 3, BLUE_LT.g / 4 * 3, BLUE_LT.b),
    Color::RGB(PINK_LT.r, PINK_LT.g / 4 * 3, PINK_LT.b / 4 * 3),
    BLACK,

    // Debugging
    RED,
    YELLOW,
    WHITE,
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
    canvas: WindowCanvas,
    pixels: Vec<Vec<Rect>>,
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
        self.canvas.set_draw_color(COLORS[0]);
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
            let colors = row.map(|pixel| COLORS[*pixel as usize]).collect();
            self.draw_scanline(y, colors);
        }
    }
}
