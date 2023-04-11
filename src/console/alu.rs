use crate::console::cpu_registers::{Flags};

// Two's compliment, converted to i16
pub(crate) fn signed_byte(value: u8) -> i16 {
    if value & 0x80 == 0 {
        value as i16
    } else {
        -(value.wrapping_neg() as i16)
    }
}

pub(crate) fn signed_word(value: u16) -> i16 {
    if value & 0x8000 == 0 {
        value as i16
    } else {
        -(value.wrapping_neg() as i16)
    }
}

/// ROTATE

/// RL (rotate left)
/// Rotates register one bit to the left.
/// Previous carry flag becomes the least significant bit,
/// and previous most significant bit becomes the new carry flag.
pub(crate) fn rotate_left(value: u8, carry: bool) -> (u8, Flags) {
    let most_signif_bit = value >> 7;
    let result = (value << 1) | if carry { 1 } else { 0 };
    (result, Flags {
        zero: value == 0,
        subtract: false,
        half_carry: false,
        carry: most_signif_bit == 1,
    })
}

/// RLC (rotate left circular)
/// Rotates register one bit to the left.
/// Previous most significant bit becomes the new least significant bit,
/// as well as the new carry flag.
pub(crate) fn rotate_left_circular(value: u8) -> (u8, Flags) {
    let most_signif_bit = value >> 7;
    let result = (value << 1) | most_signif_bit;
    (result, Flags {
        zero: value == 0,
        subtract: false,
        half_carry: false,
        carry: most_signif_bit == 1,
    })
}

/// ADD

pub(crate) fn add_byte(a: u8, b: u8) -> (u8, Flags) {
    let half_carry = (a & 0x0F) + (b & 0x0F) > 0x0F;
    let (result, carry) = a.overflowing_add(b);

    (result, Flags {
        zero: result == 0,
        subtract: false,
        half_carry,
        carry,
    })
}

pub(crate) fn add_word(a: u16, b: u16) -> (u16, Flags) {
    let half_carry = (a & 0x00FF) + (b & 0x00FF) > 0x00FF;
    let (result, carry) = a.overflowing_add(b);

    (result, Flags {
        zero: result == 0,
        subtract: false,
        half_carry,
        carry,
    })
}

/// SUB

pub(crate) fn subtract_byte(a: u8, b: u8) -> (u8, Flags) {
    let half_carry = (a & 0x0F) < (b & 0x0F);
    let (result, carry) = a.overflowing_sub(b);

    (result, Flags {
        zero: result == 0,
        subtract: true,
        half_carry,
        carry,
    })
}

pub(crate) fn subtract_word(a: u16, b: u16) -> (u16, Flags) {
    let half_carry = (a & 0x00FF) < (b & 0x00FF);
    let (result, carry) = a.overflowing_sub(b);
    (result, Flags {
        zero: result == 0,
        subtract: true,
        half_carry,
        carry,
    })
}

/// INC

/// Original carry flag is preserved
pub(crate) fn increment_byte(a: u8, original_carry: bool) -> (u8, Flags) {
    let (result, add_flags) = add_byte(a, 1);
    (result, Flags {
        zero: add_flags.zero,
        subtract: add_flags.subtract,
        half_carry: add_flags.half_carry,
        carry: original_carry,
    })
}

/// All original flags are preserved
pub(crate) fn increment_word(a: u16) -> u16 {
    let (result, _) = add_word(a, 1);
    result
}

/// DEC

/// Original carry flag is preserved
pub(crate) fn decrement_byte(a: u8, original_carry: bool) -> (u8, Flags) {
    let (result, sub_flags) = subtract_byte(a, 1);

    (result, Flags {
        zero: sub_flags.zero,
        subtract: sub_flags.subtract,
        half_carry: sub_flags.half_carry,
        carry: original_carry,
    })
}

/// All original flags are preserved
pub(crate) fn decrement_word(a: u16) -> u16 {
    let (result, _) = subtract_word(a, 1);
    result
}

/// OR
pub(crate) fn or_byte(a: u8, b: u8) -> (u8, Flags) {
    let result = a | b;
    (result, Flags {
        zero: result == 0,
        subtract: false,
        half_carry: false,
        carry: false,
    })
}

/// XOR
pub(crate) fn xor_byte(a: u8, b: u8) -> (u8, Flags) {
    let result = a ^ b;
    (result, Flags {
        zero: result == 0,
        subtract: false,
        half_carry: false,
        carry: false,
    })
}

/// AND
pub(crate) fn and_byte(a: u8, b: u8) -> (u8, Flags) {
    let result = a & b;
    (result, Flags {
        zero: result == 0,
        subtract: false,
        half_carry: true,
        carry: false,
    })
}
