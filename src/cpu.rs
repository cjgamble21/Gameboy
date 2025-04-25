use crate::registers::Registers;
use crate::Memory;
use crate::instructions::Instruction;

const BUS_SIZE: usize = 65536; // 64 KB

const INSTRUCTIONS: [Instruction; 8] = [
    Instruction::new("NOP", CPU::nop, 1),
    Instruction::new("LD_BC_NN", CPU::ld_bc_nn, 3),
    Instruction::new("LD_MEM_BC_A", CPU::ld_mem_bc_a, 2),
    Instruction::new("INC_BC", CPU::inc_bc, 2),
    Instruction::new("INC_B", CPU::inc_b, 1),
    Instruction::new("DEC_B", CPU::dec_b, 1),
    Instruction::new("LD_IMM_B", CPU::ld_imm_b, 2),
    Instruction::new("RLC_A", CPU::rlc_a, 1),
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
    fn set_zero_flag(&mut self, result: u8) {
        if result == 0 {
            self.registers.f.zero = true;
        }
    }

    fn set_sub_flag(&mut self, value: bool) {
        self.registers.f.sub = value;
    }

    fn set_half_carry_add(&mut self, old_value: u8) {
        let half_carry = (old_value & 0x0f) == 0xf;

        self.registers.f.half_carry = half_carry;
    }

    fn set_half_carry_sub(&mut self, old_value: u8) {
        let half_carry = (old_value & 0x10) == 0x10;

        self.registers.f.half_carry = half_carry;
    }

    fn execute(&mut self, opcode: u8) {
        // increment program counter before hand
        match opcode {
            0x00 => self.nop(),
            0x01 => self.ld_bc_nn(),
            0x02 => self.ld_mem_bc_a(),
            0x03 => self.inc_bc(),
            0x04 => self.inc_b(),
            0x05 => self.dec_b(),
            0x06 => self.ld_imm_b(),
            0x07 => self.rlc_a(),
            _ => unimplemented!("Unimplemented opcode..."),
        }
    }
    fn nop(&mut self) {}

    fn ld_bc_nn(&mut self) {
        let low_byte = self.read(self.registers.pc);
        self.registers.pc += 1;

        let high_byte = self.read(self.registers.pc);
        self.registers.pc += 1;

        let data = ((high_byte << 8) | low_byte) as u16;

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
        self.set_half_carry_add(self.registers.b);

        self.registers.b += 1;

        self.set_zero_flag(self.registers.b);
        self.set_sub_flag(false);
    }

    fn dec_b(&mut self) {
        self.set_half_carry_sub(self.registers.b);
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

        self.registers.pc += 1;
    }
}
