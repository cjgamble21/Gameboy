use super::CPU;

impl CPU {
    fn jump(&mut self, offset: i8) {
        self.registers.pc = ((self.registers.pc as i16) + (offset as i16)) as u16;
    }

    pub(super) fn jump_signed_default(&mut self) {
        let addr = self.read_from_pc() as i8;
        self.jump(addr);
    }

    fn jump_signed_zero_flag(&mut self, on: bool) -> u32 {
        let addr = self.read_from_pc() as i8;

        let zero_flag = self.registers.f.zero;

        let mut num_cycles = 2;

        if on {
            if zero_flag {
                self.jump(addr);
                num_cycles = 3;
            }
        } else {
            if !zero_flag {
                self.jump(addr);
                num_cycles = 3;
            }
        }

        num_cycles
    }

    pub(super) fn jump_signed_zero_flag_on(&mut self) -> u32 {
        self.jump_signed_zero_flag(true)
    }

    pub(super) fn jump_signed_zero_flag_off(&mut self) -> u32 {
        self.jump_signed_zero_flag(false)
    }

    fn jump_signed_carry_flag(&mut self, on: bool) -> u32 {
        let addr = self.read_from_pc() as i8;

        let carry_flag = self.registers.f.carry;

        let mut num_cycles = 2;

        if on {
            if carry_flag {
                self.jump(addr);
                num_cycles = 3;
            }
        } else {
            if !carry_flag {
                self.jump(addr);
                num_cycles = 3;
            }
        }

        num_cycles
    }

    pub(super) fn jump_signed_carry_flag_on(&mut self) -> u32 {
        self.jump_signed_carry_flag(true)
    }

    pub(super) fn jump_signed_carry_flag_off(&mut self) -> u32 {
        self.jump_signed_carry_flag(false)
    }
}
