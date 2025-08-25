use super::CPU;
use crate::Memory;

impl CPU {
    pub(super) fn enable_interrupts(&mut self) {
        self.write(0xFFFF, 1)
    }

    pub(super) fn disable_interrupts(&mut self) {
        self.write(0xFFFF, 0)
    }
}
