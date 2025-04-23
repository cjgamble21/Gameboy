struct FlagRegister {
    zero: bool,
    sub: bool,
    half_carry: bool,
    carry: bool,
}

impl std::convert::From<FlagRegister> for u8 {
    fn from(value: FlagRegister) -> Self {
        let a = if value.zero { 1 } else { 0 };
        let b = if value.sub { 1 } else { 0 };
        let c = if value.half_carry { 1 } else { 0 };
        let d = if value.carry { 1 } else { 0 };

        let register_value = ((a << 7) | (b << 6) | (c << 5) | (d << 4)) as u8;

        register_value
    }
}

impl std::convert::From<u8> for FlagRegister {
    fn from(value: u8) -> Self {
        let zero = (value & 0x80) != 0;
        let sub = (value & 0x40) != 0;
        let half_carry = (value & 0x20) != 0;
        let carry = (value & 0x10) != 0;

        Self {
            zero,
            sub,
            half_carry,
            carry,
        }
    }
}

pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    f: FlagRegister, // flags
    sp: u16, // stack pointer
    pc: u16, // program counter
}
