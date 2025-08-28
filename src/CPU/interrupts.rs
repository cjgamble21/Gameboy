use crate::bus::Bus;

use super::CPU;

const INTERRUPT_ADDRESS: u16 = 0xFFFF;

impl CPU {
    pub(super) fn enable_interrupts(&mut self) {
        self.bus.borrow_mut().enable_interrupts();
    }

    pub(super) fn disable_interrupts(&mut self) {
        self.bus.borrow_mut().disable_interrupts();
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::{interrupts::INTERRUPT_ADDRESS, test_helpers::make_cpu};

    #[test]
    fn test_enable_interrupts() {
        let mut cpu = make_cpu();

        cpu.enable_interrupts();

        assert_eq!(cpu.read(INTERRUPT_ADDRESS), 1)
    }
}
