mod memory;
mod arithmetic;
mod bitwise;
mod control;
mod branching;
mod instructions;
mod registers;
mod utils;

use registers::Registers;
use crate::Memory;
use instructions::Instruction;

const BUS_SIZE: usize = 65536; // 64 KB

/*
    This can just be a normal const array in the future - 
    I declared it static to avoid the language server yelling at me as I add instructions
*/
// Index of each instruction corresponds to its relevant opcode
const INSTRUCTIONS: &'static [Instruction] = &[
    Instruction::new("NOP", CPU::nop, 1),
    Instruction::new("LD_IMM_BC", CPU::ld_imm_bc, 3),
    Instruction::new("STR_BC_A", CPU::str_ind_bc_a, 2),
    Instruction::new("INC_BC", CPU::inc_bc, 2),
    Instruction::new("INC_B", CPU::inc_b, 1),
    Instruction::new("DEC_B", CPU::dec_b, 1),
    Instruction::new("LD_IMM_B", CPU::ld_imm_b, 2),
    Instruction::new("ROTATE_LEFT_A", CPU::rotate_left_a, 1),
    Instruction::new("STR_SP_MEM", CPU::str_sp_mem, 5),
    Instruction::new("ADD_HL_BC", CPU::add_hl_bc, 2),
    Instruction::new("LD_IND_BC_A", CPU::ld_ind_bc_a, 2),
    Instruction::new("DEC_BC", CPU::dec_bc, 2),
    Instruction::new("INC_C", CPU::inc_c, 1),
    Instruction::new("DEC_C", CPU::dec_c, 1),
    Instruction::new("LD_IMM_C", CPU::ld_imm_c, 2),
    Instruction::new("ROTATE_RIGHT_A", CPU::rotate_right_a, 1),
    Instruction::new("STOP", CPU::stop, 1),
    Instruction::new("LD_IMM_DE", CPU::ld_imm_de, 3),
    Instruction::new("LD_IND_DE_A", CPU::ld_ind_de_a, 2),
    Instruction::new("INC_DE", CPU::inc_de, 2),
    Instruction::new("INC_D", CPU::inc_d, 1),
    Instruction::new("DEC_D", CPU::dec_d, 1),
    Instruction::new("LD_IMM_D", CPU::ld_imm_d, 2),
    Instruction::new("ROTATE_LEFT_A_WITH_CARRY", CPU::rotate_left_a_with_carry, 1),
    Instruction::new("JUMP_SIGNED_DEFAULT", CPU::jump_signed_default, 3),
    Instruction::new("ADD_HL_DE", CPU::nop, 2),
    Instruction::new("LD_IND_DE_A", CPU::ld_ind_de_a, 2),
    Instruction::new("DEC_DE", CPU::dec_de, 2),
    Instruction::new("INC_E", CPU::inc_e, 1),
    Instruction::new("DEC_E", CPU::dec_e, 1),
    Instruction::new("LD_IMM_E", CPU::ld_imm_e, 2),
    Instruction::new("ROTATE_RIGHT_A_WITH_CARRY", CPU::rotate_right_a_with_carry, 1),
];

/* 
    Future consideration for the CPU struct:
    When we get to interrupts / CPU state , can we make use of the type state pattern?
    Might not be applicable, but I can see it being useful when dealing with halted states,
    Interrupts being enabled / disabled, etc.
*/
pub struct CPU {
    memory: Vec<u8>,
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
