use super::CPU;

const INTERRUPT_ADDRESS: u16 = 0xFFFF;

impl CPU {
    pub(super) fn enable_interrupts(&mut self) {
        self.write(INTERRUPT_ADDRESS, 1)
    }

    pub(super) fn disable_interrupts(&mut self) {
        self.write(INTERRUPT_ADDRESS, 0)
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
