use super::CPU;
use crate::Memory;
use paste::paste;

macro_rules! register_and {
    ($reg:ident) => {
        paste! {
            pub(super) fn [<and_a_ $reg>](&mut self) {
                self.registers.a &= self.registers.$reg;

                self.registers.f.zero = self.registers.a == 0;
                self.registers.f.sub = false;
                self.registers.f.half_carry = true;
                self.registers.f.carry = false;
            }
        }
    };
}

macro_rules! register_xor {
    ($reg:ident) => {
        paste! {
            pub(super) fn [<xor_a_ $reg>](&mut self) {
                self.registers.a ^= self.registers.$reg;

                self.registers.f.zero = self.registers.a == 0;
                self.registers.f.sub = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = false;
            }
        }
    };
}

macro_rules! register_or {
    ($reg:ident) => {
        paste! {
            pub(super) fn [<or_a_ $reg>](&mut self) {
                self.registers.a |= self.registers.$reg;

                self.registers.f.zero = self.registers.a == 0;
                self.registers.f.sub = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = false;
            }
        }
    };
}

impl CPU {
    pub(super) fn flip_register_a(&mut self) {
        self.registers.a = !self.registers.a;

        self.registers.f.half_carry = true;
        self.registers.f.sub = true;
    }

    pub(super) fn flip_carry_flag(&mut self) {
        self.registers.f.carry = !self.registers.f.carry;

        self.registers.f.half_carry = false;
        self.registers.f.sub = false;
    }

    pub(super) fn set_carry_flag(&mut self) {
        self.registers.f.carry = true;

        self.registers.f.half_carry = false;
        self.registers.f.sub = false;
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

    // Register AND operations
    register_and!(b);
    register_and!(c);
    register_and!(d);
    register_and!(e);
    register_and!(h);
    register_and!(l);

    pub(super) fn and_ind_hl_a(&mut self) {
        let addr = self.registers.hl();

        let value = self.read(addr);

        self.registers.a &= value;

        self.registers.f.zero = self.registers.a == 0;
        self.registers.f.sub = false;
        self.registers.f.half_carry = true;
        self.registers.f.carry = false;
    }

    pub(super) fn and_imm_a(&mut self) {
        let value = self.read_from_pc();

        self.registers.a &= value;
        self.registers.f.zero = self.registers.a == 0;
        self.registers.f.sub = false;
        self.registers.f.half_carry = true;
        self.registers.f.carry = false;
    }

    // Register XOR operations
    register_xor!(b);
    register_xor!(c);
    register_xor!(d);
    register_xor!(e);
    register_xor!(h);
    register_xor!(l);

    pub(super) fn xor_ind_hl_a(&mut self) {
        let addr = self.registers.hl();

        let value = self.read(addr);

        self.registers.a ^= value;

        self.registers.f.zero = self.registers.a == 0;
        self.registers.f.sub = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;
    }

    pub(super) fn xor_imm_a(&mut self) {
        let value = self.read_from_pc();

        self.registers.a ^= value;
        self.registers.f.zero = self.registers.a == 0;
        self.registers.f.sub = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;
    }

    // Register OR operations
    register_or!(b);
    register_or!(c);
    register_or!(d);
    register_or!(e);
    register_or!(h);
    register_or!(l);

    pub(super) fn or_ind_hl_a(&mut self) {
        let addr = self.registers.hl();

        let value = self.read(addr);

        self.registers.a ^= value;

        self.registers.f.zero = self.registers.a == 0;
        self.registers.f.sub = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;
    }

    pub(super) fn or_imm_a(&mut self) {
        let value = self.read_from_pc();

        self.registers.a |= value;
        self.registers.f.zero = self.registers.a == 0;
        self.registers.f.sub = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;
    }
}
