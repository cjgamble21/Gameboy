use super::CPU;
use super::utils::*;
use crate::Memory;
use paste::paste;

macro_rules! register_cmp {
    ($reg:ident) => {
        paste! {
            pub(super) fn [<cmp_a_ $reg>](&mut self) {
                let a = self.registers.a;
                let to_cmp = self.registers.$reg;

                self.registers.f.zero = a == to_cmp;
                self.registers.f.sub = true;
                self.registers.f.half_carry = half_carry_occurred_8_sub(a, to_cmp);
                self.registers.f.carry = carry_occurred_8_sub(a, to_cmp);
            }
        }
    };
}

impl CPU {
    // Jump instructions
    fn jump(&mut self, offset: i8) {
        self.registers.pc = ((self.registers.pc as i16) + (offset as i16)) as u16;
    }

    pub(super) fn jump_to_address(&mut self, addr: u16) {
        self.registers.pc = addr;
    }

    pub(super) fn jump_signed_default(&mut self) {
        let addr = self.read_from_pc() as i8;
        self.jump(addr);
    }

    fn jump_signed_zero_flag(&mut self, on: bool) -> u32 {
        let addr = self.read_from_pc() as i8;

        let zero_flag = self.registers.f.zero;

        let mut num_cycles = 2;

        if on {
            if zero_flag {
                self.jump(addr);
                num_cycles = 3;
            }
        } else {
            if !zero_flag {
                self.jump(addr);
                num_cycles = 3;
            }
        }

        num_cycles
    }

    pub(super) fn jump_signed_zero_flag_on(&mut self) -> u32 {
        self.jump_signed_zero_flag(true)
    }

    pub(super) fn jump_signed_zero_flag_off(&mut self) -> u32 {
        self.jump_signed_zero_flag(false)
    }

    fn jump_signed_carry_flag(&mut self, on: bool) -> u32 {
        let addr = self.read_from_pc() as i8;

        let carry_flag = self.registers.f.carry;

        let mut num_cycles = 2;

        if on {
            if carry_flag {
                self.jump(addr);
                num_cycles = 3;
            }
        } else {
            if !carry_flag {
                self.jump(addr);
                num_cycles = 3;
            }
        }

        num_cycles
    }

    pub(super) fn jump_signed_carry_flag_on(&mut self) -> u32 {
        self.jump_signed_carry_flag(true)
    }

    pub(super) fn jump_signed_carry_flag_off(&mut self) -> u32 {
        self.jump_signed_carry_flag(false)
    }

    pub(super) fn jump_16_bit(&mut self) {
        let low_byte = self.read_from_pc();
        let high_byte = self.read_from_pc();

        self.registers.pc = build_16_bit(high_byte, low_byte);
    }

    pub(super) fn jump_nz(&mut self) -> u32 {
        let mut num_cycles = 3;
        if self.registers.f.zero {
            self.registers.pc += 2;
            num_cycles
        } else {
            self.jump_16_bit();

            num_cycles = 4;
            num_cycles
        }
    }

    pub(super) fn jump_z(&mut self) -> u32 {
        let mut num_cycles = 3;
        if !self.registers.f.zero {
            self.registers.pc += 2;
            num_cycles
        } else {
            self.jump_16_bit();

            num_cycles = 4;
            num_cycles
        }
    }

    // Comparison instructions
    register_cmp!(b);
    register_cmp!(c);
    register_cmp!(d);
    register_cmp!(e);
    register_cmp!(h);
    register_cmp!(l);

    pub(super) fn cmp_ind_hl_a(&mut self) {
        let a = self.registers.a;
        let addr = self.registers.hl();

        let to_cmp = self.read(addr);

        self.registers.f.zero = a == to_cmp;
        self.registers.f.sub = true;
        self.registers.f.half_carry = half_carry_occurred_8_sub(a, to_cmp);
        self.registers.f.carry = carry_occurred_8_sub(a, to_cmp);
    }
}
