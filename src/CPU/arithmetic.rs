use super::CPU;
use super::utils::*;
use paste::paste;

macro_rules! inc_8_bit {
    ($reg:ident) => {
        paste! {
            pub(super) fn [<inc_$reg>](&mut self) {
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
            pub(super) fn [<dec_ $reg>](&mut self) {
                self.set_half_carry_sub(self.registers.$reg, self.registers.$reg - 1);

                self.registers.$reg -= 1;

                self.set_zero_flag(self.registers.$reg);
                self.set_sub_flag(true);
            }
        }
    };
}

macro_rules! inc_16_bit {
    ($reg:ident) => {
        paste! {
            pub(super) fn [<inc_ $reg>](&mut self) {
                let data = self.registers.$reg();
                self.registers.[<set_ $reg>](data.wrapping_add(1));
            }
        }
    };
}

macro_rules! dec_16_bit {
    ($reg:ident) => {
        paste! {
            pub(super) fn [<dec_ $reg>](&mut self) {
                let data = self.registers.$reg();
                self.registers.[<set_ $reg>](data.wrapping_sub(1));
            }
        }
    };
}

impl CPU {
    pub(super) fn binary_coded_decimal(&mut self) {
        todo!("Adjusts register A to binary coded decimal number")
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

    // Dual register increments
    inc_16_bit!(bc);
    inc_16_bit!(de);
    inc_16_bit!(hl);
    #[inline]
    pub(super) fn inc_sp(&mut self) {
        self.registers.sp = self.registers.sp.wrapping_add(1);
    }

    // Dual register decrements
    dec_16_bit!(bc);
    dec_16_bit!(de);
    dec_16_bit!(hl);
    #[inline]
    pub(super) fn dec_sp(&mut self) {
        self.registers.sp = self.registers.sp.wrapping_sub(1);
    }

    pub(super) fn add_hl_bc(&mut self) {
        let half_carry = half_carry_occurred_16(self.registers.bc(), self.registers.hl());
        let carry = carry_occurred_16(self.registers.bc(), self.registers.hl());

        self.registers.set_hl(self.registers.bc() + self.registers.hl());

        self.registers.f.carry = carry;
        self.registers.f.half_carry = half_carry;
        self.registers.f.sub = false;
    }
}
