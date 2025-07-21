use super::CPU;

type CPUCycles = u32;

type InstructionFn = fn(&mut CPU) -> CPUCycles;

pub struct Instruction {
    name: &'static str,
    pub function: InstructionFn,
}

impl Instruction {
    pub const fn new(name: &'static str, function: InstructionFn) -> Self {
        Instruction {
            name,
            function,
        }
    }
}

macro_rules! instr {
    // Used with opcodes that have static cycle counts
    ($name:expr, $func:expr, $cycles:literal) => {
        Instruction::new($name, |cpu| {
            $func(cpu);
            $cycles
        })
    };
    // Allows instruction to return number of cycles - only used for opcodes with dynamic cycle counts
    ($name:expr, $func:expr) => {
        Instruction::new($name, $func)
    };
}

/*
    This can just be a normal const array in the future - 
    I declared it static to avoid the language server yelling at me as I add instructions
*/
// Index of each instruction corresponds to its relevant opcode
pub const INSTRUCTIONS: &'static [Instruction] = &[
    instr!("NOP", CPU::nop, 1),
    instr!("LD_IMM_BC", CPU::ld_imm_bc, 3),
    instr!("STR_BC_A", CPU::str_ind_bc_a, 2),
    instr!("INC_BC", CPU::inc_bc, 2),
    instr!("INC_B", CPU::inc_b, 1),
    instr!("DEC_B", CPU::dec_b, 1),
    instr!("LD_IMM_B", CPU::ld_imm_b, 2),
    instr!("ROTATE_LEFT_A", CPU::rotate_left_a, 1),
    instr!("STR_SP_MEM", CPU::str_sp_mem, 5),
    instr!("ADD_HL_BC", CPU::add_hl_bc, 2),
    instr!("LD_IND_BC_A", CPU::ld_ind_bc_a, 2),
    instr!("DEC_BC", CPU::dec_bc, 2),
    instr!("INC_C", CPU::inc_c, 1),
    instr!("DEC_C", CPU::dec_c, 1),
    instr!("LD_IMM_C", CPU::ld_imm_c, 2),
    instr!("ROTATE_RIGHT_A", CPU::rotate_right_a, 1),
    instr!("STOP", CPU::stop, 1),
    instr!("LD_IMM_DE", CPU::ld_imm_de, 3),
    instr!("LD_IND_DE_A", CPU::ld_ind_de_a, 2),
    instr!("INC_DE", CPU::inc_de, 2),
    instr!("INC_D", CPU::inc_d, 1),
    instr!("DEC_D", CPU::dec_d, 1),
    instr!("LD_IMM_D", CPU::ld_imm_d, 2),
    instr!("ROTATE_LEFT_A_WITH_CARRY", CPU::rotate_left_a_with_carry, 1),
    instr!("JUMP_SIGNED_DEFAULT", CPU::jump_signed_default, 3),
    instr!("ADD_HL_DE", CPU::nop, 2), // TODO: Implement
    instr!("LD_IND_DE_A", CPU::ld_ind_de_a, 2),
    instr!("DEC_DE", CPU::dec_de, 2),
    instr!("INC_E", CPU::inc_e, 1),
    instr!("DEC_E", CPU::dec_e, 1),
    instr!("LD_IMM_E", CPU::ld_imm_e, 2),
    instr!("ROTATE_RIGHT_A_WITH_CARRY", CPU::rotate_right_a_with_carry, 1),
    instr!("JUMP_SIGNED_ZERO_FLAG_OFF", CPU::jump_signed_zero_flag_off), // Dynamic cycle count
    instr!("LD_IMM_HL", CPU::ld_imm_hl, 3),
    instr!("STR_IND_HL_A_ADD", CPU::str_ind_hl_a_add, 2),
    instr!("INC_HL", CPU::inc_hl, 2),
    instr!("INC_H", CPU::inc_h, 1),
    instr!("DEC_H", CPU::dec_h, 1),
    instr!("LD_IMM_H", CPU::ld_imm_h, 2),
    instr!("DAA", CPU::binary_coded_decimal, 1),
    instr!("JUMP_SIGNED_ZERO_FLAG_ON", CPU::jump_signed_zero_flag_on), // Dynamic cycle count
    instr!("ADD_HL_HL", CPU::nop, 2), // TODO: Implement
    instr!("LD_IND_HL_A_ADD", CPU::ld_ind_hl_a_add, 2),
    instr!("DEC_HL", CPU::dec_hl, 2),
    instr!("INC_L", CPU::inc_l, 1),
    instr!("DEC_L", CPU::dec_l, 1),
    instr!("LD_IMM_E", CPU::ld_imm_l, 2),
    instr!("CPL", CPU::flip_register_a, 1),
    instr!("JUMP_SIGNED_CARRY_FLAG_OFF", CPU::jump_signed_carry_flag_off), // Dynamic cycle count
    instr!("LD_IMM_SP", CPU::ld_imm_sp, 3),
    instr!("STR_IND_HL_A_SUB", CPU::str_ind_hl_a_sub, 2),
    instr!("INC_SP", CPU::inc_sp, 2),
];
