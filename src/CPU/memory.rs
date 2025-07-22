use super::CPU;
use crate::Memory;
use paste::paste;
use super::registers::Registers;

/*
    ld -> Load memory
    str -> Store memory
    imm -> Immediate (Unsigned literal)
    ind -> Indirect (Value in RAM stored at an address in a 16 bit register pair)

    All functions follow this template:
    (ld | str)_(source)_(destination)

    So "ld_imm_bc" stands for "Load immediate value into register BC"
*/

macro_rules! ld_imm_16_bit {
    ($reg:ident, $setter:expr) => {
        paste! {
            #[inline]
            pub(super) fn [<ld_imm_$reg>](&mut self) {
                self.ld_imm_16_bit($setter)
            }
        }
    };
}

macro_rules! ld_imm_8_bit {
    ($reg:ident) => {
        paste! {
            pub(super) fn [<ld_imm_$reg>](&mut self) {
                let immediate = self.read_from_pc();

                self.registers.$reg = immediate
            }
        }
    };
}

macro_rules! ld_ind {
    ($src:ident, $dest:ident, $label:expr, $transformation:expr) => {
        paste! {
            pub(super) fn [<ld_ind_ $src _ $dest _ $label>](&mut self) {
                let value = self.read(self.registers.$src());

                self.registers.$dest = value;

                $transformation(self)
            }
        }
    };

    ($src:ident, $dest:ident) => {
        paste! {
            pub(super) fn [<ld_ind_ $src _ $dest>](&mut self) {
                let value = self.read(self.registers.$src());

                self.registers.$dest = value;
            }
        }
    };
}

macro_rules! str_ind {
    // Added this version of the macro for the (HL-) and (HL+) variants that mutate the address
    ($src:ident, $dest:ident, $label:expr, $transformation:expr) => {
        paste! {
            pub(super) fn [<str_ind_ $src _ $dest _ $label>](&mut self) {
                let addr = self.registers.$src();

                self.write(addr, self.registers.$dest);

                $transformation(self)
            }
        }
    };

    ($src:ident, $dest:ident) => {
        paste! {
            pub(super) fn [<str_ind_ $src _ $dest>](&mut self) {
                let addr = self.registers.$src();

                self.write(addr, self.registers.$dest);
            }
        }
    };
}

impl CPU {
    // Load immediate value to 16 bit register pairs
    fn ld_imm_16_bit(&mut self, set_fn: fn(&mut Registers, u16)) {
        let low_byte = self.read_from_pc();

        let high_byte = self.read_from_pc();

        let data = ((high_byte as u16) << 8) | (low_byte as u16);

        set_fn(&mut self.registers, data)
    }

    fn decrement_hl(&mut self) {
        self.registers.set_hl(self.registers.hl() - 1)
    }

    fn increment_hl(&mut self) {
        self.registers.set_hl(self.registers.hl() + 1)
    }

    ld_imm_16_bit!(bc, Registers::set_bc);
    ld_imm_16_bit!(de, Registers::set_de);
    ld_imm_16_bit!(hl, Registers::set_hl);
    ld_imm_16_bit!(sp, |reg, value| {
        reg.sp = value;
    });

    // Load immediate value to 8 bit register
    ld_imm_8_bit!(a);
    ld_imm_8_bit!(b);
    ld_imm_8_bit!(c);
    ld_imm_8_bit!(d);
    ld_imm_8_bit!(e);
    ld_imm_8_bit!(h);
    ld_imm_8_bit!(l);

    // Load indirect value provided by register pair to 8 bit register
    ld_ind!(bc, a);
    ld_ind!(de, a);
    ld_ind!(hl, a, "add", |cpu: &mut CPU| {
        cpu.increment_hl();
    });
    ld_ind!(hl, a, "sub", |cpu: &mut CPU| {
        cpu.decrement_hl();
    });

    // Store register in address provided by indirect register memory
    str_ind!(bc, a);
    str_ind!(de, a);
    str_ind!(hl, a, "add", |cpu: &mut CPU| {
        cpu.increment_hl();
    });
    str_ind!(hl, a, "sub", |cpu: &mut CPU| {
        cpu.decrement_hl();
    });

    // Store immediate value in address provided by indirect register memory
    pub(super) fn str_imm_ind_hl(&mut self) {
        let addr = self.registers.hl();

        let value = self.read_from_pc();

        self.write(addr, value);
    }

    pub(super) fn str_sp_mem(&mut self) {
        let low_byte = self.read_from_pc();

        let high_byte = self.read_from_pc();

        let addr = ((high_byte as u16) << 8) | (low_byte as u16);

        let low_byte_sp = (self.registers.sp & 0x00ff) as u8;
        let high_byte_sp = (self.registers.sp >> 8) as u8;

        self.write(addr, low_byte_sp);
        self.write(addr + 1, high_byte_sp);
    }
}
