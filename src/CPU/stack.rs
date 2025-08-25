use super::CPU;
use super::Memory;
use super::utils::*;

impl CPU {
    fn read_from_sp(&mut self) -> u8 {
        let addr = self.registers.sp;
        let value = self.read(addr);
        self.registers.sp += 1;

        value
    }

    // Reset instructions
    fn reset(&mut self, target: u16) {
        self.push(self.registers.pc);

        self.jump_to_address(target);
    }

    pub(super) fn reset_00(&mut self) {
        self.reset(0x00);
    }

    pub(super) fn reset_08(&mut self) {
        self.reset(0x08);
    }

    // Push instructions
    fn push(&mut self, value: u16) {
        self.registers.sp -= 1;
        let high_byte = get_high_byte(value);
        self.write(self.registers.sp, high_byte);

        self.registers.sp -= 1;
        let low_byte = get_low_byte(value);
        self.write(self.registers.sp, low_byte);
    }

    pub(super) fn push_bc(&mut self) {
        self.push(self.registers.bc())
    }

    pub(super) fn push_de(&mut self) {
        self.push(self.registers.de())
    }

    pub(super) fn push_hl(&mut self) {
        self.push(self.registers.hl())
    }

    pub(super) fn push_af(&mut self) {
        self.push(self.registers.af())
    }

    // Pop instructions
    fn pop(&mut self) -> u16 {
        let low_byte = self.read_from_sp();
        let high_byte = self.read_from_sp();

        build_16_bit(high_byte, low_byte)
    }
    pub(super) fn pop_bc(&mut self) {
        let new_bc = self.pop();

        self.registers.set_bc(new_bc);
    }

    pub(super) fn pop_de(&mut self) {
        let new_de = self.pop();

        self.registers.set_de(new_de);
    }

    pub(super) fn pop_hl(&mut self) {
        let new_hl = self.pop();

        self.registers.set_hl(new_hl);
    }

    pub(super) fn pop_af(&mut self) {
        let new_af = self.pop();

        self.registers.set_af(new_af);
    }

    pub(super) fn call(&mut self) {
        self.jump_16_bit();

        self.push(self.registers.pc);
    }

    fn conditional_call(&mut self, condition: bool) -> u32 {
        let mut num_cycles = 3;
        if condition {
            self.registers.pc += 2;
            num_cycles
        } else {
            self.call();

            num_cycles = 6;
            num_cycles
        }
    }

    pub(super) fn call_nz(&mut self) -> u32 {
        self.conditional_call(!self.registers.f.zero)
    }

    pub(super) fn call_z(&mut self) -> u32 {
        self.conditional_call(self.registers.f.zero)
    }

    pub(super) fn call_nc(&mut self) -> u32 {
        self.conditional_call(!self.registers.f.carry)
    }

    pub(super) fn call_c(&mut self) -> u32 {
        self.conditional_call(self.registers.f.carry)
    }

    // Return instructions
    pub(super) fn instr_return(&mut self) {
        let low_byte = self.read_from_sp();

        let mut new_pc = set_low_byte(self.registers.pc, low_byte);

        let high_byte = self.read_from_sp();

        new_pc = set_high_byte(new_pc, high_byte);

        self.registers.pc = new_pc;
    }

    pub(super) fn return_with_condition(&mut self, condition: bool) -> u32 {
        let mut num_cycles: u32 = 2;
        if !condition {
            num_cycles
        } else {
            self.instr_return();

            num_cycles = 5;

            num_cycles
        }
    }

    pub(super) fn ret_nz(&mut self) -> u32 {
        self.return_with_condition(!self.registers.f.zero)
    }

    pub(super) fn ret_z(&mut self) -> u32 {
        self.return_with_condition(self.registers.f.zero)
    }

    pub(super) fn ret_nc(&mut self) -> u32 {
        self.return_with_condition(!self.registers.f.carry)
    }

    pub(super) fn ret_c(&mut self) -> u32 {
        self.return_with_condition(self.registers.f.carry)
    }
}
