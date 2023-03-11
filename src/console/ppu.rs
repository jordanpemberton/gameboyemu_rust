use std::collections::HashMap;
use sdl2::pixels::Color;

const COLORS: [Color; 4] = [
    Color::RGB(255, 255, 255),
    Color::RGB(255, 255, 0),
    Color::RGB(255, 0, 255),
    Color::RGB(0, 255, 255),
];

/*
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

Example: In bootrom, the register was written with value 0xF3, which is binary 11110011.
This means that color number 0 is assigned black, as well as colors number 2 and 3;
color number 1 is assigned white.
 */

pub(crate) struct Ppu {
    palettes: [[u8; 4]; 4],
    colors: [Color; 4],
}

impl Ppu {
    pub(crate) fn new(palettes: [[u8; 4]; 4]) -> Ppu {
        Ppu {
            palettes: palettes,
            colors: COLORS,
        }
    }

    pub(crate) fn get_pixel_buffer(&self, memory_buffer: &[u8], palette_index: u8) -> Vec<Color> {
        memory_buffer.iter()
            .map(|byte|
                Color::RGB(
                    byte & 0b1110_0000,
                    (byte & 0b0001_1000) << 3,
                    (byte & 0b0000_0111) << 5,
                )
                // self.colors[self.palettes[palette_index as usize][
                //     match byte {
                //         0x00 => 0,
                //         0x10 => 1,
                //         0x01 => 2,
                //         _ => 3,
                //     }] as usize]
            )
            .collect()
    }
}
