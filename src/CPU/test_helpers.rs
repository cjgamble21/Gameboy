use std::{cell::RefCell, rc::Rc};

use crate::{
    bus::{Bus, SystemBus},
    cpu::{CPU, registers::Registers},
};

struct FakeBus {
    memory: Vec<u8>,
    interrupts_enabled: bool,
}

impl FakeBus {
    pub fn new() -> Self {
        Self {
            memory: vec![0; (std::u16::MAX as usize) + 1],
            interrupts_enabled: false,
        }
    }
}

impl Bus for FakeBus {
    fn read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    fn request_interrupt(&mut self) {}

    fn enable_interrupts(&mut self) {
        self.interrupts_enabled = true;
    }

    fn disable_interrupts(&mut self) {
        self.interrupts_enabled = false;
    }

    fn interrupts_enabled(&self) -> bool {
        self.interrupts_enabled
    }
}

pub(super) fn make_cpu() -> CPU {
    let new_cpu = CPU::new(Rc::new(RefCell::new(FakeBus::new())));

    new_cpu
}
