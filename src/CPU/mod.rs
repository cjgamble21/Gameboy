pub mod memory;
pub mod arithmetic;
pub mod bitwise;
pub mod control;
pub mod instructions;
pub mod registers;
mod utils;

use std::ops::RangeInclusive;

use registers::Registers;
use crate::Memory;
use instructions::Instruction;

const BUS_SIZE: usize = 65536; // 64 KB

const INSTRUCTIONS: [Instruction; 17] = [
    Instruction::new("NOP", CPU::nop, 1),
    Instruction::new("LD_BC_NN", CPU::ld_imm_bc, 3),
    Instruction::new("LD_MEM_BC_A", CPU::str_bc_a, 2),
    Instruction::new("INC_BC", CPU::inc_bc, 2),
    Instruction::new("INC_B", CPU::inc_b, 1),
    Instruction::new("DEC_B", CPU::dec_b, 1),
    Instruction::new("LD_IMM_B", CPU::ld_imm_b, 2),
    Instruction::new("ROTATE_LEFT_A", CPU::rotate_left_a, 1),
    Instruction::new("LD_MEM_SP", CPU::str_sp_mem, 5),
    Instruction::new("ADD_HL_BC", CPU::add_hl_bc, 2),
    Instruction::new("LD_MEM_A_BC", CPU::ld_bc_a, 2),
    Instruction::new("DEC_BC", CPU::dec_bc, 2),
    Instruction::new("INC_C", CPU::inc_c, 1),
    Instruction::new("DEC_C", CPU::dec_c, 1),
    Instruction::new("LD_IMM_C", CPU::ld_imm_c, 2),
    Instruction::new("ROTATE_RIGHT_A", CPU::rotate_right_a, 1),
    Instruction::new("STOP", CPU::stop, 1),
    // Instruction::new("LD_DE_A", CPU::ld_, 3)
];

pub struct CPU {
    memory: [u8; BUS_SIZE],
    registers: Registers,
    cycles: i32,
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
        CPU {
            memory: [0; BUS_SIZE],
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

        (instruction.function)(self);

        self.cycles += instruction.cycles;
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
