pub(super) fn carry_occurred_8(a: u8, b: u8) -> bool {
    (a as u16) + (b as u16) > 0xff
}

pub(super) fn half_carry_occurred_8(a: u8, b: u8) -> bool {
    (a & 0xf) + (b & 0xf) > 0xf
}

pub(super) fn half_carry_occurred_16(a: u16, b: u16) -> bool {
    (a & 0x0fff) + (b & 0x0fff) > 0x0fff
}

pub(super) fn carry_occurred_16(a: u16, b: u16) -> bool {
    (a as u32) + (b as u32) > 0xffff
}
