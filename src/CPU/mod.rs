mod arithmetic;
mod bitwise;
mod branching;
mod control;
mod instructions;
mod interrupts;
mod memory;
mod registers;
mod stack;
mod test_helpers;
mod utils;

use std::{cell::RefCell, rc::Rc};

use crate::bus::{Bus, SystemBus};
use instructions::*;
use registers::Registers;

/*
    Future consideration for the CPU struct:
    When we get to interrupts / CPU state , can we make use of the type state pattern?
    Might not be applicable, but I can see it being useful when dealing with halted states,
    Interrupts being enabled / disabled, etc.
*/
pub struct CPU {
    registers: Registers,
    cycles: u32,
    bus: Rc<RefCell<dyn Bus>>,
}

impl CPU {
    pub fn new(bus: Rc<RefCell<dyn Bus>>) -> Self {
        Self {
            registers: Registers::new(),
            cycles: 0,
            bus: bus,
        }
    }

    pub fn tick(&mut self) {
        let opcode = self.read_from_pc();

        self.execute(opcode);
    }

    fn read(&self, addr: u16) -> u8 {
        self.bus.borrow().read(addr)
    }

    fn write(&mut self, addr: u16, value: u8) {
        self.bus.borrow_mut().write(addr, value)
    }

    // TODO: Verify we always need to increment PC after reading from it
    fn read_from_pc(&mut self) -> u8 {
        let value = self.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        value
    }

    fn execute(&mut self, opcode: u8) {
        let instruction = &INSTRUCTIONS[opcode as usize];

        println!("Executing instruction: {} {}", instruction.name, opcode);

        let cycles = (instruction.function)(self);

        self.cycles += cycles;
    }

    fn set_zero_flag(&mut self, result: u8) {
        self.registers.f.zero = result == 0;
    }

    fn set_sub_flag(&mut self, value: bool) {
        self.registers.f.sub = value;
    }

    fn set_half_carry_add(&mut self, a: u8, b: u8) {
        self.registers.f.half_carry = (a & 0xf) + (b & 0xf) > 0xf;
    }

    fn set_half_carry_sub(&mut self, a: u8, b: u8) {
        self.registers.f.half_carry = a & 0xf < b & 0xf;
    }

    fn no_impl(&self) {
        println!(
            "Attempted to invoke unimplemented opcode at PC={}",
            self.registers.pc
        );
    }
}
