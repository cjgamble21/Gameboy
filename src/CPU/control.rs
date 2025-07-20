use super::CPU;

impl CPU {
    pub(super) fn nop(&mut self) {}

    // TODO: Research this more and implement
    pub(super) fn stop(&mut self) {
        self.registers.pc += 1; // Instruction is 2 bytes long
    }
}
