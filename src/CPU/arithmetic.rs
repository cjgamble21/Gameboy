use super::CPU;
use super::utils::*;

impl CPU {
    pub(crate) fn inc_bc(&mut self) {
        let mut data = self.registers.bc();
        data += 1;
        self.registers.set_bc(data);
    }

    pub(crate) fn inc_b(&mut self) {
        self.set_half_carry_add(self.registers.b, self.registers.b + 1);

        self.registers.b += 1;

        self.set_zero_flag(self.registers.b);
        self.set_sub_flag(false);
    }

    pub(crate) fn dec_b(&mut self) {
        self.set_half_carry_sub(self.registers.b, self.registers.b - 1);
        self.registers.b -= 1;

        self.set_zero_flag(self.registers.b);
        self.set_sub_flag(true);
    }

    pub(crate) fn add_hl_bc(&mut self) {
        let half_carry = half_carry_occurred_16(self.registers.bc(), self.registers.hl());
        let carry = carry_occurred_16(self.registers.bc(), self.registers.hl());

        self.registers.set_hl(self.registers.bc() + self.registers.hl());

        self.registers.f.carry = carry;
        self.registers.f.half_carry = half_carry;
        self.registers.f.sub = false;
    }

    pub(crate) fn dec_bc(&mut self) {
        self.registers.set_bc(self.registers.bc() - 1);
    }

    pub(crate) fn inc_c(&mut self) {
        self.set_half_carry_add(self.registers.c, self.registers.c + 1);
        self.registers.c += 1;

        self.set_zero_flag(self.registers.c);
        self.set_sub_flag(false);
    }

    pub(crate) fn dec_c(&mut self) {
        self.set_half_carry_sub(self.registers.b, self.registers.b - 1);

        self.registers.b -= 1;

        self.set_zero_flag(self.registers.b);
        self.set_sub_flag(true);
    }
}
