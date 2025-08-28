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
    fn new() -> Self {
        let bus = Rc::new(RefCell::new(SystemBus::new()));
        let cpu = CPU::new(bus.clone());

        Self { bus, cpu }
    }
}
