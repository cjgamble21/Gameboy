use super::CPU;

impl CPU {
    pub(crate) fn nop(&mut self) {}

    // TODO: Research this more and implement
    pub(crate) fn stop(&mut self) {
        self.registers.pc += 1; // Instruction is 2 bytes long
    }
}
