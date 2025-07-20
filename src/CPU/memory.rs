use super::CPU;
use crate::Memory;
use paste::paste;
use super::registers::Registers;

/*
    ld -> Load
    str -> Store
    imm -> Immediate
    ind -> Indirect

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

impl CPU {
    // Load immediate value to 16 bit register pairs
    fn ld_imm_16_bit(&mut self, set_fn: fn(&mut Registers, u16)) {
        let low_byte = self.read_from_pc();

        let high_byte = self.read_from_pc();

        let data = ((high_byte as u16) << 8) | (low_byte as u16);

        set_fn(&mut self.registers, data)
    }

    ld_imm_16_bit!(bc, Registers::set_bc);
    ld_imm_16_bit!(de, Registers::set_de);
    ld_imm_16_bit!(hl, Registers::set_hl);
    ld_imm_16_bit!(sp, |reg, value| {
        reg.sp = value;
    });

    // Load immediate value to 8 bit register
    fn ld_imm_8_bit(&mut self) {
        let immediate = self.read_from_pc();
    }

    // Load indirect value to 8 bit register

    pub(super) fn str_bc_a(&mut self) {
        let addr = self.registers.bc();

        self.write(addr, self.registers.a);
    }

    pub(super) fn ld_imm_b(&mut self) {
        let data = self.read_from_pc();
        self.registers.b = data;
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

    pub(super) fn ld_bc_a(&mut self) {
        let value = self.read(self.registers.bc());

        self.registers.a = value;
    }

    pub(super) fn ld_imm_c(&mut self) {
        let value = self.read_from_pc();

        self.registers.c = value;
    }
}
