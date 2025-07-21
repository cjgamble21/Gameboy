use super::CPU;

type InstructionFn = fn(&mut CPU) -> u32;

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

#[macro_export]
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
