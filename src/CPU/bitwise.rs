use super::CPU;

impl CPU {
    pub(crate) fn rotate_left_a(&mut self) {
        let most_significant_bit = (self.registers.a & 0x80) >> 7;

        self.registers.f.carry = most_significant_bit == 1;
        self.registers.a = (self.registers.a << 1) | most_significant_bit;
    }

    pub(crate) fn rotate_right_a(&mut self) {
        let first_bit = self.registers.a & 1;

        self.registers.f.carry = first_bit == 1;

        self.registers.a = (first_bit << 7) | (self.registers.a >> 1);
    }
}
