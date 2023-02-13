pub(crate) fn signed(value: u8) -> u16 {
    if value & 0x80 == 0 {
        value as u16
    } else {
        0xFF00 | value as u16
    }
}
