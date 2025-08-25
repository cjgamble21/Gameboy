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

pub(super) fn carry_occurred_8_sub(a: u8, b: u8) -> bool {
    a < b
}

pub(super) fn half_carry_occurred_8_sub(a: u8, b: u8) -> bool {
    a & 0xf < b & 0xf
}

pub(super) fn carry_occurred_16_sub(a: u16, b: u16) -> bool {
    a < b
}

pub(super) fn half_carry_occurred_16_sub(a: u16, b: u16) -> bool {
    a & 0xfff < b & 0xfff
}

pub(super) fn get_high_byte(value: u16) -> u8 {
    (value >> 8) as u8
}

pub(super) fn get_low_byte(value: u16) -> u8 {
    (value & 0xff) as u8
}

pub(super) fn set_high_byte(to_set: u16, value: u8) -> u16 {
    (to_set & 0x00FF) | ((value as u16) << 8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_high_byte() {
        assert_eq!(get_high_byte(0xABCD), 0xAB);
        assert_eq!(get_high_byte(0x1234), 0x12);
        assert_ne!(get_high_byte(0x3482), 0x82);
    }

    #[test]
    fn test_get_low_byte() {
        assert_eq!(get_low_byte(0xABCD), 0xCD);
        assert_eq!(get_low_byte(0x1234), 0x34);
        assert_ne!(get_low_byte(0x1234), 0x12);
    }

    #[test]
    fn test_set_high_byte() {
        assert_eq!(set_high_byte(0xABCD, 0x67), 0x67CD);
        assert_eq!(set_high_byte(0x7934, 0x9D), 0x9D34);
        assert_ne!(set_high_byte(0x8723, 0xFF), 0x8723);
    }
}
