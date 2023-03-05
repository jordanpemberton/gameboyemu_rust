use std::collections::HashMap;
use sdl2::pixels::Color;

pub(crate) struct Ppu {
    palettes: [[u8; 4]; 4],
    colors: HashMap<u8, Color>,
}

impl Ppu {
    pub(crate) fn new(palettes: [[u8; 4]; 4]) -> Ppu {
        Ppu {
            palettes: palettes,
            colors: HashMap::from([
                ( 0x00, Color::RGB(0, 0, 0) ),
                ( 0x01, Color::RGB(0, 0, 0) ),
                ( 0x10, Color::RGB(0, 0, 0) ),
                ( 0x11, Color::RGB(0, 0, 0) ),
            ]),
        }
    }

    pub(crate) fn get_pixel_buffer(&self, memory_buffer: &[u8], palette_index: u8) -> Vec<Color> {
        memory_buffer.iter()
            .map(|byte| self.colors[
                    (&self.palettes[palette_index as usize][
                        match byte {
                            0x00 => 0,
                            0x01 => 1,
                            0x10 => 2,
                            0x11 => 3,
                            _ => 0,
                        }
                    ])
                ])
            .collect()
    }
}
