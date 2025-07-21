use super::CPU;

impl CPU {
    pub(super) fn jump_signed_default(&mut self) {
        let val = self.read_from_pc() as i8;
        self.registers.pc = ((self.registers.pc as i16) + (val as i16)) as u16;
    }
}
