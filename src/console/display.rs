use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;
use crate::console::ppu::{Lcd, Ppu};

const COLORS: [Color; 13] = [
    Color::RGB(0, 0, 0),
    Color::RGB(255, 255, 0),
    Color::RGB(255, 0, 255),
    Color::RGB(0, 255, 255),

    Color::RGB(0, 0, 0),
    Color::RGB(128, 255, 0),
    Color::RGB(128, 0, 255),
    Color::RGB(0, 128, 255),

    Color::RGB(0, 0, 0),
    Color::RGB(128, 128, 0),
    Color::RGB(128, 0, 128),
    Color::RGB(0, 128, 128),

    Color::RGB(255, 0, 0),
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
