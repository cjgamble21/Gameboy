use crate::cpu::CPU;

type InstructionFn = fn(&mut CPU);

pub struct Instruction {
    name: &'static str,
    pub function: InstructionFn,
    cycles: i32,
}

impl Instruction {
    pub const fn new(name: &'static str, function: InstructionFn, cycles: i32) -> Self {
        Instruction {
            name,
            function,
            cycles,
        }
    }
}
