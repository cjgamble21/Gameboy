use super::CPU;
use super::Memory;

impl CPU {
    fn read_from_sp(&mut self) -> u8 {
        let addr = self.registers.sp;
        let value = self.read(addr);
        self.registers.sp += 1;

        value
    }
    pub(super) fn pop_bc(&mut self) {
        let low_byte = self.read_from_sp();
        let high_byte = self.read_from_sp();

        let new_bc = (high_byte | low_byte) as u16;

        self.registers.set_bc(new_bc);
    }
}
