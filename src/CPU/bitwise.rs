use super::CPU;

impl CPU {
    pub(super) fn flip_register_a(&mut self) {
        self.registers.a = !self.registers.a;
    }

    pub(super) fn rotate_left_a(&mut self) {
        let most_significant_bit = (self.registers.a & 0x80) >> 7;

        self.registers.f.carry = most_significant_bit == 1;
        self.registers.f.half_carry = false;
        self.registers.f.sub = false;
        self.registers.f.zero = false;
        self.registers.a = (self.registers.a << 1) | most_significant_bit;
    }

    pub(super) fn rotate_left_a_with_carry(&mut self) {
        let most_significant_bit = (self.registers.a & 0x80) >> 7;

        let old_carry = self.registers.f.carry as u8;

        self.registers.f.carry = most_significant_bit == 1;
        self.registers.f.half_carry = false;
        self.registers.f.sub = false;
        self.registers.f.zero = false;
        self.registers.a = (self.registers.a << 1) | old_carry;
    }

    pub(super) fn rotate_right_a(&mut self) {
        let first_bit = self.registers.a & 1;

        self.registers.f.carry = first_bit == 1;
        self.registers.f.half_carry = false;
        self.registers.f.sub = false;
        self.registers.f.zero = false;

        self.registers.a = (first_bit << 7) | (self.registers.a >> 1);
    }

    pub(super) fn rotate_right_a_with_carry(&mut self) {
        let first_bit = self.registers.a & 1;

        let old_carry = self.registers.f.carry as u8;

        self.registers.f.carry = first_bit == 1;
        self.registers.f.half_carry = false;
        self.registers.f.sub = false;
        self.registers.f.zero = false;

        self.registers.a = (old_carry << 7) | (self.registers.a >> 1);
    }
}
