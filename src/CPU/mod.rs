mod arithmetic;
mod bitwise;
mod branching;
mod control;
mod instructions;
mod memory;
mod registers;
mod utils;

use crate::Memory;
use instructions::*;
use registers::Registers;

const BUS_SIZE: usize = std::u16::MAX as usize;
const STACK_SIZE: usize = 1000;

/*
    Future consideration for the CPU struct:
    When we get to interrupts / CPU state , can we make use of the type state pattern?
    Might not be applicable, but I can see it being useful when dealing with halted states,
    Interrupts being enabled / disabled, etc.
*/
pub struct CPU {
    memory: Vec<u8>,
    registers: Registers,
    cycles: u32,
}

impl Memory for CPU {
    fn read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }
}

impl CPU {
    pub fn new() -> Self {
        Self {
            memory: Vec::with_capacity(BUS_SIZE),
            registers: Registers::new(),
            cycles: 0,
        }
    }

    pub fn tick(&mut self) {
        let opcode = self.read_from_pc();

        self.execute(opcode);
    }

    // TODO: Verify we always need to increment PC after reading from it
    fn read_from_pc(&mut self) -> u8 {
        let value = self.read(self.registers.pc);
        self.registers.pc += 1;
        value
    }

    fn execute(&mut self, opcode: u8) {
        let instruction = &INSTRUCTIONS[opcode as usize];

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
}
