pub struct FlagRegister {
    pub zero: bool,
    pub sub: bool,
    pub half_carry: bool,
    pub carry: bool,
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
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub f: FlagRegister, // flags
    pub sp: u16, // stack pointer
    pub pc: u16, // program counter
}

impl Registers {
    pub fn bc(&self) -> u16 {
        ((self.b << 8) as u16) | (self.c as u16)
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xff00) >> 8) as u8;
        self.c = (value & 0x00ff) as u8;
    }
}
