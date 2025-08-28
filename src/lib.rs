use std::cell::RefCell;
use std::rc::Rc;

use crate::bus::SystemBus;
use crate::cpu::CPU;

mod bus;
#[path = "Cartridge/mod.rs"]
mod cartridge;
#[path = "CPU/mod.rs"]
mod cpu;

#[allow(dead_code)]
pub struct Emulator {
    bus: Rc<RefCell<SystemBus>>,
    cpu: CPU,
}

#[allow(dead_code)]
impl Emulator {
    pub fn new(file_name: &str) -> Self {
        let bus = Rc::new(RefCell::new(SystemBus::new(file_name)));
        let cpu = CPU::new(bus.clone());

        Self { bus, cpu }
    }

    pub fn execute(&mut self) {
        loop {
            self.cpu.tick();
        }
    }
}
