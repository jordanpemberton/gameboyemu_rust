use crate::console::cpu_registers::{Flags};

// Two's compliment, converted to i16
pub(crate) fn signed_8(value: u8) -> i16 {
    if value & 0x80 == 0 {
        value as i16
    } else {
        -(value.wrapping_neg() as i16)
    }
}

/// ROTATE

/// RL
/// Rotates register one bit to the left.
/// Previous carry flag becomes the least significant bit,
/// and previous most significant bit becomes the new carry flag.
pub(crate) fn rotate_left(value: u8, original_carry: bool) -> (u8, Flags) {
    let result = (value << 1) | if original_carry { 1 } else { 0 };
    (result, Flags {
        zero: result == 0,
        subtract: false,
        half_carry: false,
        carry: (value >> 7) == 1,
    })
}

/// RLC
/// Rotates register one bit to the left.
/// Previous most significant bit becomes the new least significant bit,
/// as well as the new carry flag.
pub(crate) fn rotate_left_circular(value: u8) -> (u8, Flags) {
    let result = (value << 1) | (value >> 7);
    (result, Flags {
        zero: result == 0,
        subtract: false,
        half_carry: false,
        carry: (value >> 7) == 1,
    })
}

/// RR
pub(crate) fn rotate_right(value: u8, original_carry: bool) -> (u8, Flags) {
    let result = (value >> 1) | ((original_carry as u8) << 7);
    (result, Flags {
        zero: result == 0,
        subtract: false,
        half_carry: false,
        carry: (value & 1) == 1,
    })
}

/// RRC
pub(crate) fn rotate_right_circular(value: u8) -> (u8, Flags) {
    let result = (value >> 1) | ((value << 7) as u8);
    (result, Flags {
        zero: result == 0,
        subtract: false,
        half_carry: false,
        carry: (value & 1) == 1,
    })
}

/// ADD

pub(crate) fn add_8(a: u8, b: u8) -> (u8, Flags) {
    let half_carry = (a & 0x0F) + (b & 0x0F) > 0x0F;
    let (result, carry) = a.overflowing_add(b);
    (result, Flags {
        zero: result == 0,
        subtract: false,
        half_carry,
        carry,
    })
}

pub(crate) fn adc_8(a: u8, b: u8, original_flags: Flags) -> (u8, Flags) {
    let a = a as i16;
    let b = b as i16;
    let c = original_flags.carry as i16;
    let result = a + b + c;
    let half_carry = ((a & 0x0F) + (b & 0x0F) + (c & 0x0F)) >= 0x10;
    let carry = result >= 0x100;
    let result = result as u8;
    (result, Flags {
        zero: result == 0,
        subtract: false,
        half_carry,
        carry,
    })
}

pub(crate) fn add_16(a: u16, b: u16) -> (u16, Flags) {
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

pub(crate) fn subtract_8(a: u8, b: u8) -> (u8, Flags) {
    let half_carry = (a & 0x0F) < (b & 0x0F);
    let (result, carry) = a.overflowing_sub(b);
    (result, Flags {
        zero: result == 0,
        subtract: true,
        half_carry,
        carry,
    })
}

pub(crate) fn subtract_16(a: u16, b: u16) -> (u16, Flags) {
    let half_carry = (a & 0x00FF) < (b & 0x00FF);
    let (result, carry) = a.overflowing_sub(b);
    (result, Flags {
        zero: false,
        subtract: true,
        half_carry,
        carry,
    })
}

pub(crate) fn sbc_8(a: u8, b: u8, original_flags: Flags) -> (u8, Flags) {
    let a = a as i16;
    let b = b as i16;
    let c = original_flags.carry as i16;
    let result = a - b - c;
    let half_carry = ((a & 0x0F) - (b & 0x0F) - (c & 0x0F)) < 0;
    let carry = result < 0;
    let result = result as u8;
    (result, Flags {
        zero: result == 0,
        subtract: true,
        half_carry,
        carry,
    })
}

/// INC

/// Original carry flag is preserved
pub(crate) fn increment_8(a: u8, original_carry: bool) -> (u8, Flags) {
    let (result, add_flags) = add_8(a, 1);
    (result, Flags {
        zero: add_flags.zero,
        subtract: add_flags.subtract,
        half_carry: add_flags.half_carry,
        carry: original_carry,
    })
}

/// All original flags are preserved
pub(crate) fn increment_16(a: u16) -> u16 {
    let (result, _) = a.overflowing_add(1);
    result
}

/// DEC

/// Original carry flag is preserved
pub(crate) fn decrement_8(a: u8, original_carry: bool) -> (u8, Flags) {
    let (result, sub_flags) = subtract_8(a, 1);

    (result, Flags {
        zero: sub_flags.zero,
        subtract: sub_flags.subtract,
        half_carry: sub_flags.half_carry,
        carry: original_carry,
    })
}

/// All original flags are preserved
pub(crate) fn decrement_16(a: u16) -> u16 {
    let (result, _) = subtract_16(a, 1);
    result
}

/// OR
pub(crate) fn or_8(a: u8, b: u8) -> (u8, Flags) {
    let result = a | b;
    (result, Flags {
        zero: result == 0,
        subtract: false,
        half_carry: false,
        carry: false,
    })
}

/// XOR
pub(crate) fn xor_8(a: u8, b: u8) -> (u8, Flags) {
    let result = a ^ b;
    (result, Flags {
        zero: result == 0,
        subtract: false,
        half_carry: false,
        carry: false,
    })
}

/// AND
pub(crate) fn and_8(a: u8, b: u8) -> (u8, Flags) {
    let result = a & b;
    (result, Flags {
        zero: result == 0,
        subtract: false,
        half_carry: true,
        carry: false,
    })
}

/// SLA
pub(crate) fn shift_left(a: u8) -> (u8, Flags) {
    let result = a << 1;
    (result, Flags {
        zero: result == 0,
        subtract: false,
        half_carry: false,
        carry: (a >> 7) == 1,
    })
}

/// SRA, SRL
pub(crate) fn shift_right(a: u8, is_arithmetic: bool) -> (u8, Flags) {
    let mut result = a >> 1;
    if is_arithmetic {
        result |= a & 0x80;
    }
    (result, Flags {
        zero: result == 0,
        subtract: false,
        half_carry: false,
        carry: (a & 1) == 1,
    })
}

/// DAA -- decimal adjust accumulator, BCD adjust
///
/// if the least significant four bits of A contain a non-BCD digit
/// (i. e. it is greater than 9) or the H flag is set,
/// then $06 is added to the register.
///
/// Then the four most significant bits are checked.
/// If this more significant digit also happens to be greater than 9
/// or the C flag is set, then $60 is added.
///
/// If this second addition was needed, the C flag is set after execution,
/// otherwise it is reset.
pub(crate) fn daa(a: u8, original_flags: Flags) -> (u8, Flags) {
    let mut result = a;
    let mut carry = false;

    if ((result & 0x0F) > 9) || original_flags.half_carry {
        if original_flags.subtract {
            result = result.wrapping_sub(0x06);
        } else {
            result = result.wrapping_add(0x06);
        }
    }
    if (((result & 0xF0) >> 4) > 9) || original_flags.carry {
        if original_flags.subtract {
            result = result.wrapping_sub(0x60);
        } else {
            result = result.wrapping_add(0x60);
        }
        carry = true;
    }

    (result, Flags {
        zero: result == 0,
        subtract: original_flags.subtract,
        half_carry: false,
        carry: carry,
    })
}
