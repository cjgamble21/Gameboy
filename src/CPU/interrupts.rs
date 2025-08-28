use crate::bus::Bus;

use super::CPU;

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
    use crate::cpu::test_helpers::make_cpu;

    #[test]
    fn test_enable_interrupts() {
        let mut cpu = make_cpu();

        cpu.enable_interrupts();

        assert_eq!(cpu.bus.borrow().interrupts_enabled(), true)
    }
}
