use crate::console::registers::{Flags};

pub(crate) fn signed(value: u8) -> u16 {
    if value & 0x0080 == 0 {
        value as u16
    } else {
        0xFF00 | value as u16
    }
}

pub (crate) fn rl(value: u8) -> Flags {
    let bit_7 = (value & 0b1000_0000) >> 7;
    let value1 = (value << 1) | bit_7;
    let value2 = value.wrapping_shl(1);
    let value3 = value.rotate_left(1);
    assert!(value1 == value2 && value2 == value3, "Rotate lefts were different.");

    Flags {
        zero: value == 0,
        subtract: false,
        half_carry: false,
        carry: bit_7 == 1,
    }
}
