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

pub(super) fn set_low_byte(to_set: u16, value: u8) -> u16 {
    (to_set & 0xff00) | (value as u16)
}

pub(super) fn build_16_bit(high: u8, low: u8) -> u16 {
    ((high as u16) << 8) | (low as u16)
}

#[cfg(test)]
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

    #[test]
    fn test_set_low_byte() {
        assert_eq!(set_low_byte(0x5432, 0x98), 0x5498);
        assert_eq!(set_low_byte(0xFF83, 0xAB), 0xFFAB);
        assert_ne!(set_low_byte(0xFF87, 0x54), 0xFF87);
    }

    #[test]
    fn test_build_16_bit() {
        assert_eq!(build_16_bit(0x12, 0x34), 0x1234);
        assert_eq!(build_16_bit(0x00, 0x00), 0x0000);
        assert_eq!(build_16_bit(0xFF, 0xAB), 0xFFAB);
    }

    #[test]
    fn test_carry_occurred_8() {
        assert!(!carry_occurred_8(0x00, 0x00));
        assert!(carry_occurred_8(0xFF, 0x01));
        assert!(carry_occurred_8(0x80, 0x80));
        assert!(!carry_occurred_8(0x7F, 0x00));
        assert!(!carry_occurred_8(0x10, 0x0F));
    }

    #[test]
    fn test_half_carry_occurred_8() {
        assert!(half_carry_occurred_8(0x0F, 0x01));
        assert!(!half_carry_occurred_8(0x08, 0x07));
        assert!(!half_carry_occurred_8(0x0F, 0x00));
        assert!(half_carry_occurred_8(0x0A, 0x06));
    }

    #[test]
    fn test_half_carry_occurred_16() {
        assert!(half_carry_occurred_16(0x0FFF, 0x0001));
        assert!(!half_carry_occurred_16(0x0F00, 0x00FF));
        assert!(!half_carry_occurred_16(0x0ABC, 0x0543));
        assert!(half_carry_occurred_16(0x08FF, 0x0701));
    }

    #[test]
    fn test_carry_occurred_16() {
        assert!(carry_occurred_16(0xFFFF, 0x0001));
        assert!(carry_occurred_16(0x8000, 0x8000));
        assert!(!carry_occurred_16(0x7FFF, 0x0000));
        assert!(!carry_occurred_16(0x1234, 0xEDCB));
    }

    #[test]
    fn test_carry_occurred_8_sub() {
        assert!(carry_occurred_8_sub(0x00, 0x01));
        assert!(!carry_occurred_8_sub(0x10, 0x0F));
        assert!(!carry_occurred_8_sub(0x01, 0x01));
        assert!(carry_occurred_8_sub(0x80, 0x81));
    }

    #[test]
    fn test_half_carry_occurred_8_sub() {
        assert!(half_carry_occurred_8_sub(0x10, 0x01));
        assert!(!half_carry_occurred_8_sub(0x1F, 0x0F));
        assert!(!half_carry_occurred_8_sub(0x00, 0x00));
        assert!(half_carry_occurred_8_sub(0x20, 0x11));
    }

    #[test]
    fn test_carry_occurred_16_sub() {
        assert!(carry_occurred_16_sub(0x0000, 0x0001));
        assert!(!carry_occurred_16_sub(0x1234, 0x1234));
        assert!(!carry_occurred_16_sub(0x8000, 0x7FFF));
        assert!(carry_occurred_16_sub(0x0001, 0x0002));
    }

    #[test]
    fn test_half_carry_occurred_16_sub() {
        assert!(half_carry_occurred_16_sub(0x1000, 0x0001));
        assert!(!half_carry_occurred_16_sub(0x0FFF, 0x00FF));
        assert!(!half_carry_occurred_16_sub(0x0000, 0x0000));
        assert!(half_carry_occurred_16_sub(0x2000, 0x1001));
    }
}
