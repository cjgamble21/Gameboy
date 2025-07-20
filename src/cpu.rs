use std::ops::RangeInclusive;

use crate::registers::Registers;
use crate::Memory;
use crate::instructions::Instruction;

fn carry_occurred_8(a: u8, b: u8) -> bool {
    (a as u16) + (b as u16) > 0xff
}

fn half_carry_occurred_8(a: u8, b: u8) -> bool {
    (a & 0xf) + (b & 0xf) > 0xf
}

fn half_carry_occurred_16(a: u16, b: u16) -> bool {
    (a & 0x0fff) + (b & 0x0fff) > 0x0fff
}

fn carry_occurred_16(a: u16, b: u16) -> bool {
    (a as u32) + (b as u32) > 0xffff
}

const BUS_SIZE: usize = 65536; // 64 KB

const INSTRUCTIONS: [Instruction; 11] = [
    Instruction::new("NOP", CPU::nop, 1),
    Instruction::new("LD_BC_NN", CPU::ld_bc_nn, 3),
    Instruction::new("LD_MEM_BC_A", CPU::ld_mem_bc_a, 2),
    Instruction::new("INC_BC", CPU::inc_bc, 2),
    Instruction::new("INC_B", CPU::inc_b, 1),
    Instruction::new("DEC_B", CPU::dec_b, 1),
    Instruction::new("LD_IMM_B", CPU::ld_imm_b, 2),
    Instruction::new("RLC_A", CPU::rlc_a, 1),
    Instruction::new("LD_MEM_SP", CPU::ld_mem_sp, 5),
    Instruction::new("ADD_HL_BC", CPU::add_hl_bc, 2),
    Instruction::new("LD_MEM_A_BC", CPU::ld_mem_a_bc, 2),
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
    pub fn tick(&mut self) {
        let opcode = self.read(self.registers.pc);
        self.registers.pc += 1;

        self.execute(opcode);
    }

    fn execute(&mut self, opcode: u8) {
        let instruction = &INSTRUCTIONS[opcode as usize];

        (instruction.function)(self);

        self.cycles += instruction.cycles;
    }

    fn set_zero_flag(&mut self, result: u8) {
        if result == 0 {
            self.registers.f.zero = true;
        }
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

    fn nop(&mut self) {}

    fn ld_bc_nn(&mut self) {
        let low_byte = self.read(self.registers.pc);
        self.registers.pc += 1;

        let high_byte = self.read(self.registers.pc);
        self.registers.pc += 1;

        let data = ((high_byte as u16) << 8) | (low_byte as u16);

        self.registers.set_bc(data);
    }

    fn ld_mem_bc_a(&mut self) {
        let addr = self.registers.bc();

        self.write(addr, self.registers.a);
    }

    fn inc_bc(&mut self) {
        let mut data = self.registers.bc();
        data += 1;
        self.registers.set_bc(data);
    }

    fn inc_b(&mut self) {
        self.set_half_carry_add(self.registers.b, self.registers.b + 1);

        self.registers.b += 1;

        self.set_zero_flag(self.registers.b);
        self.set_sub_flag(false);
    }

    fn dec_b(&mut self) {
        self.set_half_carry_sub(self.registers.b, self.registers.b - 1);
        self.registers.b -= 1;

        self.set_zero_flag(self.registers.b);
        self.set_sub_flag(true);
    }

    fn ld_imm_b(&mut self) {
        let data = self.read(self.registers.pc);
        self.registers.pc += 1;
        self.registers.b = data;
    }

    fn rlc_a(&mut self) {
        let most_significant_bit = (self.registers.a & 0x80) >> 7;

        self.registers.f.carry = most_significant_bit == 1;
        self.registers.a = (self.registers.a << 1) | most_significant_bit;
    }

    fn ld_mem_sp(&mut self) {
        let low_byte = self.read(self.registers.pc);
        self.registers.pc += 1;

        let high_byte = self.read(self.registers.pc);
        self.registers.pc += 1;

        let addr = ((high_byte as u16) << 8) | (low_byte as u16);

        let low_byte_sp = (self.registers.sp & 0x00ff) as u8;
        let high_byte_sp = (self.registers.sp >> 8) as u8;

        self.write(addr, low_byte_sp);
        self.write(addr + 1, high_byte_sp);
    }

    fn add_hl_bc(&mut self) {
        let half_carry = half_carry_occurred_16(self.registers.bc(), self.registers.hl());
        let carry = carry_occurred_16(self.registers.bc(), self.registers.hl());

        self.registers.set_hl(self.registers.bc() + self.registers.hl());

        self.registers.f.carry = carry;
        self.registers.f.half_carry = half_carry;
        self.registers.f.sub = false;
    }

    fn ld_mem_a_bc(&mut self) {
        let value = self.read(self.registers.bc());

        self.registers.a = value;
    }

    fn dec_bc(&mut self) {
        self.registers.set_bc(self.registers.bc() - 1);
    }

    fn inc_c(&mut self) {
        self.set_half_carry_add(self.registers.c, self.registers.c + 1);
        self.registers.c += 1;

        self.set_zero_flag(self.registers.c);
        self.set_sub_flag(false);
    }
}
