use std::{cell::RefCell, rc::Rc};

use crate::{
    bus::SystemBus,
    cpu::{CPU, registers::Registers},
};

pub(super) fn make_cpu() -> CPU {
    let new_cpu = CPU::new(Rc::new(RefCell::new(SystemBus::new())));

    new_cpu
}
