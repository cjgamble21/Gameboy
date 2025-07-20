use super::CPU;
use crate::Memory;

impl CPU {
    pub(crate) fn ld_bc_nn(&mut self) {
        let low_byte = self.read(self.registers.pc);
        self.registers.pc += 1;

        let high_byte = self.read(self.registers.pc);
        self.registers.pc += 1;

        let data = ((high_byte as u16) << 8) | (low_byte as u16);

        self.registers.set_bc(data);
    }

    pub(crate) fn ld_mem_bc_a(&mut self) {
        let addr = self.registers.bc();

        self.write(addr, self.registers.a);
    }

    pub(crate) fn ld_imm_b(&mut self) {
        let data = self.read(self.registers.pc);
        self.registers.pc += 1;
        self.registers.b = data;
    }

    pub(crate) fn ld_mem_sp(&mut self) {
        let low_byte = self.read(self.registers.pc);
        self.registers.pc += 1;

        let high_byte = self.read(self.registers.pc);
        self.registers.pc += 1;

        let addr = ((high_byte as u16) << 8) | (low_byte as u16);

        let low_byte_sp = (self.registers.sp & 0x00ff) as u8;
        let high_byte_sp = (self.registers.sp >> 8) as u8;

        self.write(addr, low_byte_sp);
        self.write(addr + 1, high_byte_sp);
    }

    pub(crate) fn ld_mem_a_bc(&mut self) {
        let value = self.read(self.registers.bc());

        self.registers.a = value;
    }

    pub(crate) fn ld_imm_c(&mut self) {
        let value = self.read(self.registers.pc);
        self.registers.pc += 1;

        self.registers.c = value;
    }
}
