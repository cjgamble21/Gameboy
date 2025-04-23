use crate::registers::Registers;

const RAM_SIZE: usize = 4096; // 32 KB

struct CPU {
    memory: [u8; RAM_SIZE],
    registers: Registers,
}

impl CPU {
    fn execute(&mut self, opcode: u8) {
        match opcode {
            0x00 => self.no_op(),
            _ => unimplemented!("Unimplemented opcode..."),
        }
    }
    fn no_op(&mut self) {
        self.registers.pc += 1;
    }
}
