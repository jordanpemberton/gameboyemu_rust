#[allow(dead_code)]
#[derive(Clone,Copy)]
pub(crate) struct SpriteAttribute {
    pub(crate) y: u8,                      // byte 0
    pub(crate) x: u8,                      // byte 1
    pub(crate) tile_index: u8,             // byte 2
    pub(crate) bg_window_over_obj: bool,   // byte 3, bit 7
    pub(crate) flip_y: bool,               // byte 3, bit 6
    pub(crate) flip_x: bool,               // byte 3, bit 5
    pub(crate) palette_is_obp1: bool,      // byte 3, bit 4
    pub(crate) tile_vram_bank_cgb: bool,   // byte 3, bit 3    (CGB ONLY)
    pub(crate) palette_cgb: u8,            // byte 3, bit 2-0  (CGB ONLY)
}

impl SpriteAttribute {
    pub(crate) fn new(data: &[u8; 4]) -> SpriteAttribute {
        SpriteAttribute {
            y: data[0],
            x: data[1],
            tile_index: data[2],
            bg_window_over_obj: data[3] & 0x80 == 0x80,
            flip_y: data[3] & 0x40 == 0x40,
            flip_x: data[3] & 0x20 == 0x20,
            palette_is_obp1: data[3] & 0x10 == 0x10,
            tile_vram_bank_cgb: data[3] & 0x08 == 0x80,
            palette_cgb: data[3] & 0x07,
        }
    }
}
