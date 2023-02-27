use crate::console::registers::{Flags};

// Two's compliment, converted to i16
pub(crate) fn signed(value: u8) -> i16 {
    if value & 0x80 == 0 {
        value as i16
    } else {
        -(value.wrapping_neg() as i16)
    }
}

/// RL (rotate left)
/// Rotates register one bit to the left.
/// Previous carry flag becomes the least significant bit,
/// and previous most significant bit becomes the new carry flag.
pub(crate) fn rl(mut value: u8, carry: bool) -> Flags {
    let bit_7 = value >> 7;
    value = (value << 1) | if carry { 1 } else { 0 };
    // OR value = value.wrapping_shl(1);

    Flags {
        zero: value == 0,
        subtract: false,
        half_carry: false,
        carry: bit_7 == 1,
    }
}

/// RLC (rotate left circular)
/// Rotates register one bit to the left.
/// Previous most significant bit becomes the new least significant bit,
/// as well as the new carry flag.
pub(crate) fn rlc(mut value: u8) -> Flags {
    let bit_7 = value >> 7;
    value = (value << 1) | bit_7;
    // OR value = value.rotate_left(1);

    Flags {
        zero: value == 0,
        subtract: false,
        half_carry: false,
        carry: bit_7 == 1,
    }
}

pub(crate) fn add_byte(a: u8, b: u8) -> (u8, Flags) {
    let (_, half_carry) = (a & 0x0F).overflowing_add(b & 0x0F);
    let (result, carry) = a.overflowing_add(b);

    (result, Flags {
        zero: result == 0,
        subtract: false,
        half_carry,
        carry,
    })
}

pub(crate) fn subtract_byte(a: u8, b: u8) -> (u8, Flags) {
    let (_, half_carry) = (a & 0x0F).overflowing_sub(b & 0x0F);
    let (result, carry) = a.overflowing_sub(b);

    (result, Flags {
        zero: result == 0,
        subtract: true,
        half_carry,
        carry,
    })
}
