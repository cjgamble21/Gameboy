use crate::Memory;

use super::CPU;
use super::utils::*;
use paste::paste;

macro_rules! inc_8_bit {
    ($reg:ident) => {
        paste! {
            pub(super) fn [<inc_$reg>](&mut self) {
                self.set_half_carry_add(self.registers.$reg, self.registers.$reg + 1);
                self.set_sub_flag(false);

                self.registers.$reg += 1;
                self.set_zero_flag(self.registers.$reg);
            }
        }
    };
}

macro_rules! dec_8_bit {
    ($reg:ident) => {
        paste! {
            pub(super) fn [<dec_ $reg>](&mut self) {
                self.set_half_carry_sub(self.registers.$reg, self.registers.$reg - 1);
                self.set_sub_flag(true);

                self.registers.$reg -= 1;
                self.set_zero_flag(self.registers.$reg);
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

macro_rules! add_register {
    ($reg:ident) => {
        paste! {
            pub(super) fn [<add_a_ $reg>](&mut self) {
                self.add_register(self.registers.$reg, false)
            }
        }
    };
}

macro_rules! add_register_with_carry {
    ($reg:ident) => {
        paste! {
            pub(super) fn [<add_a_ $reg _with_carry>](&mut self) {
                self.add_register(self.registers.$reg, true)
            }
        }
    };
}

macro_rules! subtract_register {
    ($reg:ident) => {
        paste! {
            pub(super) fn [<subtract_a_ $reg>](&mut self) {
                self.subtract_register(self.registers.$reg, false)
            }
        }
    };
}

macro_rules! subtract_register_with_carry {
    ($reg:ident) => {
        paste! {
            pub(super) fn [<subtract_a_ $reg _with_carry>](&mut self) {
                self.subtract_register(self.registers.$reg, true)
            }
        }
    };
}

impl CPU {
    pub(super) fn binary_coded_decimal(&mut self) {
        let sub_flag_set = self.registers.f.sub;
        let half_carry_flag_set = self.registers.f.half_carry;
        let carry_flag_set = self.registers.f.carry;

        let mut adjustment = 0;

        let should_add_carry = carry_flag_set || self.registers.a > 0x99;

        let should_add_six_to_adjustment = (sub_flag_set && half_carry_flag_set)
            || (!sub_flag_set && (half_carry_flag_set || self.registers.a & 0xf > 0x9));

        let should_add_sixty_to_adjustment =
            (sub_flag_set && carry_flag_set) || (!sub_flag_set && should_add_carry);

        if should_add_six_to_adjustment {
            adjustment += 0x6;
        }

        if should_add_sixty_to_adjustment {
            adjustment += 0x60;
        }

        if sub_flag_set {
            self.registers.a -= adjustment;
        } else {
            self.registers.a += adjustment;
        }

        self.registers.f.zero = self.registers.a == 0;
        self.registers.f.half_carry = false;
        self.registers.f.carry = should_add_carry;
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

    // Dual register increment of memory location
    pub(super) fn inc_ind_hl(&mut self) {
        let addr = self.registers.hl();

        let value = self.read(addr);

        let new_value = value + 1;

        self.set_half_carry_add(value, new_value);

        self.set_zero_flag(new_value);
        self.set_sub_flag(false);

        self.write(addr, new_value);
    }

    // Dual register decrements
    dec_16_bit!(bc);
    dec_16_bit!(de);
    dec_16_bit!(hl);
    #[inline]
    pub(super) fn dec_sp(&mut self) {
        self.registers.sp = self.registers.sp.wrapping_sub(1);
    }

    // Dual register decrement of memory location
    pub(super) fn dec_ind_hl(&mut self) {
        let addr = self.registers.hl();

        let value = self.read(addr);

        let new_value = value - 1;

        self.set_half_carry_add(value, new_value);

        self.set_zero_flag(new_value);
        self.set_sub_flag(true);

        self.write(addr, new_value);
    }

    // Addition operations
    pub(super) fn add_register(&mut self, register: u8, with_carry: bool) {
        let accumulator = self.registers.a;

        let to_add = register
            + (if self.registers.f.carry && with_carry {
                1
            } else {
                0
            });

        self.registers.f.half_carry = half_carry_occurred_8(accumulator, to_add);
        self.registers.f.carry = carry_occurred_8(accumulator, to_add);
        self.set_sub_flag(false);

        self.registers.a += to_add;
        self.set_zero_flag(accumulator);
    }

    pub(super) fn add_ind_hl_a(&mut self) {
        let addr = self.registers.hl();

        let value = self.read(addr);

        self.add_register(value, false);
    }

    pub(super) fn add_ind_hl_a_with_carry(&mut self) {
        let addr = self.registers.hl();

        let value = self.read(addr);

        self.add_register(value, true);
    }

    add_register!(b);
    add_register!(c);
    add_register!(d);
    add_register!(e);
    add_register!(h);
    add_register!(l);

    add_register_with_carry!(b);
    add_register_with_carry!(c);
    add_register_with_carry!(d);
    add_register_with_carry!(e);
    add_register_with_carry!(h);
    add_register_with_carry!(l);

    pub(super) fn add_hl_bc(&mut self) {
        let half_carry = half_carry_occurred_16(self.registers.bc(), self.registers.hl());
        let carry = carry_occurred_16(self.registers.bc(), self.registers.hl());

        self.registers
            .set_hl(self.registers.bc() + self.registers.hl());

        self.registers.f.carry = carry;
        self.registers.f.half_carry = half_carry;
        self.registers.f.sub = false;
    }

    fn _add_imm_a(&mut self, with_carry: bool) {
        let value = self.read_from_pc();

        let half_carry = half_carry_occurred_8(self.registers.a, value);
        let carry = carry_occurred_8(self.registers.a, value);

        if with_carry {
            self.registers.a += value + if carry { 1 } else { 0 };
        } else {
            self.registers.a += value;
        }

        self.registers.f.carry = carry;
        self.registers.f.half_carry = half_carry;
        self.registers.f.sub = false;
        self.registers.f.zero = self.registers.a == 0;
    }

    pub(super) fn add_imm_a(&mut self) {
        self._add_imm_a(false);
    }

    pub(super) fn add_imm_a_with_carry(&mut self) {
        self._add_imm_a(true);
    }

    // Subtraction operations
    pub(super) fn subtract_register(&mut self, register: u8, with_carry: bool) {
        let accumulator = self.registers.a;

        let to_sub = register
            - (if self.registers.f.carry && with_carry {
                1
            } else {
                0
            });

        self.registers.f.half_carry = half_carry_occurred_8_sub(accumulator, to_sub);
        self.registers.f.carry = carry_occurred_8_sub(accumulator, to_sub);
        self.set_sub_flag(true);

        self.registers.a -= to_sub;
        self.set_zero_flag(accumulator);
    }

    pub(super) fn subtract_ind_hl_a(&mut self) {
        let addr = self.registers.hl();

        let value = self.read(addr);

        self.subtract_register(value, false);
    }

    pub(super) fn subtract_ind_hl_a_with_carry(&mut self) {
        let addr = self.registers.hl();

        let value = self.read(addr);

        self.subtract_register(value, true);
    }

    subtract_register!(b);
    subtract_register!(c);
    subtract_register!(d);
    subtract_register!(e);
    subtract_register!(h);
    subtract_register!(l);

    subtract_register_with_carry!(b);
    subtract_register_with_carry!(c);
    subtract_register_with_carry!(d);
    subtract_register_with_carry!(e);
    subtract_register_with_carry!(h);
    subtract_register_with_carry!(l);
}
