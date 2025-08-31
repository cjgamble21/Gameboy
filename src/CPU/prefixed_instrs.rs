// All instructions prefixed with "0xCB"

use crate::cpu::CPU;
use paste::paste;

macro_rules! implement_all_registers {
    ($macro:ident) => {
        $macro!(a);
        $macro!(b);
        $macro!(c);
        $macro!(d);
        $macro!(e);
        $macro!(h);
        $macro!(l);
        $macro!(ind_hl);
    };
}

macro_rules! rotate_left_carry {
    (ind_hl) => {
        paste! {
            pub(super) fn rotate_left_carry_ind_hl(&mut self) {
                let addr = self.registers.hl();

                let value = self.read(addr);

                let most_significant_bit = (value & 0x8) >> 7;

                let new_value = (value << 1) | most_significant_bit;

                self.registers.f.carry = most_significant_bit == 1;
                self.registers.f.half_carry = false;
                self.registers.f.sub = false;
                self.registers.f.zero = new_value == 1;

                self.write(addr, new_value);
            }
        }
    };
    ($reg:expr) => {
        paste! {
            pub(super) fn [<rotate_left_carry_ $reg>](&mut self) {
                let most_significant_bit = (self.registers.$reg & 0x8) >> 7;

                self.registers.$reg = (self.registers.$reg << 1) | most_significant_bit;

                self.registers.f.carry = most_significant_bit == 1;
                self.registers.f.half_carry = false;
                self.registers.f.sub = false;
                self.registers.f.zero = self.registers.$reg == 1;
            }
        }
    };
}

impl CPU {
    pub(super) fn execute_prefixed_instr() -> u32 {
        let num_cycles = 2;

        num_cycles
    }

    implement_all_registers!(rotate_left_carry);
}
