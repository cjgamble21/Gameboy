use super::CPU;
use super::utils::*;
use paste::paste;

macro_rules! inc_8_bit {
    ($reg:ident) => {
        paste! {
            #[inline]
            pub(crate) fn [<inc_$reg>](&mut self) {
                self.set_half_carry_add(self.registers.$reg, self.registers.$reg + 1);

                self.registers.$reg += 1;

                self.set_zero_flag(self.registers.$reg);
                self.set_sub_flag(false);
            }
        }
    };
}

macro_rules! dec_8_bit {
    ($reg:ident) => {
        paste! {
            #[inline]
            pub(crate) fn [<dec_$reg>](&mut self) {
                self.set_half_carry_sub(self.registers.$reg, self.registers.$reg - 1);

                self.registers.$reg -= 1;

                self.set_zero_flag(self.registers.$reg);
                self.set_sub_flag(true);
            }
        }
    };
}

impl CPU {
    pub(crate) fn inc_bc(&mut self) {
        let mut data = self.registers.bc();
        data += 1;
        self.registers.set_bc(data);
    }

    // Single register increments
    inc_8_bit!(a);
    inc_8_bit!(b);
    inc_8_bit!(c);
    inc_8_bit!(d);
    inc_8_bit!(e);
    inc_8_bit!(h);
    inc_8_bit!(l);

    // Single register decrements
    dec_8_bit!(a);
    dec_8_bit!(b);
    dec_8_bit!(c);
    dec_8_bit!(d);
    dec_8_bit!(e);
    dec_8_bit!(h);
    dec_8_bit!(l);

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
}
