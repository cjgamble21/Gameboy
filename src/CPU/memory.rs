use super::CPU;
use super::registers::Registers;
use super::utils::*;
use paste::paste;

/*
    ld -> Load memory
    str -> Store memory
    imm -> Immediate (Unsigned literal)
    ind -> Indirect (Value in RAM stored at an address in a 16 bit register pair)

    All functions follow this template:
    (ld | str)_(source)_(destination)

    So "ld_imm_bc" stands for "Load immediate value into register BC"
*/

macro_rules! ld_imm_16_bit {
    ($reg:ident, $setter:expr) => {
        paste! {
            #[inline]
            pub(super) fn [<ld_imm_$reg>](&mut self) {
                self.ld_imm_16_bit($setter)
            }
        }
    };
}

macro_rules! ld_imm_8_bit {
    ($reg:ident) => {
        paste! {
            pub(super) fn [<ld_imm_$reg>](&mut self) {
                let immediate = self.read_from_pc();

                self.registers.$reg = immediate
            }
        }
    };
}

macro_rules! ld_ind {
    ($src:ident, $dest:ident, $label:expr, $transformation:expr) => {
        paste! {
            pub(super) fn [<ld_ind_ $src _ $dest _ $label>](&mut self) {
                let value = self.read(self.registers.$src());

                self.registers.$dest = value;

                $transformation(self)
            }
        }
    };

    ($src:ident, $dest:ident) => {
        paste! {
            pub(super) fn [<ld_ind_ $src _ $dest>](&mut self) {
                let value = self.read(self.registers.$src());

                self.registers.$dest = value;
            }
        }
    };
}

macro_rules! str_ind {
    // Added this version of the macro for the (HL-) and (HL+) variants that mutate the address
    ($src:ident, $dest:ident, $label:expr, $transformation:expr) => {
        paste! {
            pub(super) fn [<str_ind_ $src _ $dest _ $label>](&mut self) {
                let addr = self.registers.$src();

                self.write(addr, self.registers.$dest);

                $transformation(self)
            }
        }
    };

    ($src:ident, $dest:ident) => {
        paste! {
            pub(super) fn [<str_ind_ $src _ $dest>](&mut self) {
                let addr = self.registers.$src();

                self.write(addr, self.registers.$dest);
            }
        }
    };
}

macro_rules! __generate_register_contents {
    ($left:ident, $right:ident) => {
        paste! {
            pub(super) fn [<ld_ $left _ $right>](&mut self) {
                self.registers.$left = self.registers.$right;
            }
            pub(super) fn [<ld_ $right _ $left>](&mut self) {
                self.registers.$right = self.registers.$left;
            }
        }
    };
}
// -------- outer recursion: pick a HEAD and pass it + REST -----
macro_rules! _ld_pairs_outer {
    // ≥2 idents left
    ($head:ident, $($tail:ident),+ $(,)?) => {
        _ld_pairs_inner! { $head; $($tail),+ }   // pair HEAD with everyone in TAIL
        _ld_pairs_outer! { $($tail),+ }          // recurse on the TAIL
    };
    // 1‑ident base case – nothing left to pair
    ($single:ident $(,)?) => {};
}

// -------- inner recursion: pair one HEAD with each in REST ----
macro_rules! _ld_pairs_inner {
    // still more partners to pair with HEAD
    ($head:ident; $next:ident, $($rest:ident),* $(,)?) => {
        __generate_register_contents!($head, $next);
        _ld_pairs_inner! { $head; $($rest),* }
    };
    // last partner for this HEAD
    ($head:ident; $last:ident $(,)?) => {
        __generate_register_contents!($head, $last);
    };
}

// ─────────────────────────────────────────────────────────────
//  Public macro you call: expands on *all* registers given.
//    ld_pairs!(a, b, c)  →  AB + AC + BC (each direction)
// ─────────────────────────────────────────────────────────────
macro_rules! ld_register_contents {
    ($($regs:ident),+ $(,)?) => {
        _ld_pairs_outer! { $($regs),+ }
    };
}

macro_rules! ld_indirect_contents {
    ($reg:ident) => {
        paste! {
            pub(super) fn [<ld_ind_hl_ $reg>](&mut self) {
                let addr = self.registers.hl();

                let value = self.read(addr);

                self.registers.$reg = value
            }
        }
    };
}

macro_rules! str_indirect_contents {
    ($reg:ident) => {
        paste! {
            pub(super) fn [<str_ind_hl_ $reg>](&mut self) {
                let addr = self.registers.hl();

                let value = self.registers.$reg;

                self.write(addr, value)
            }
        }
    };
}

impl CPU {
    // Load immediate value to 16 bit register pairs
    fn ld_imm_16_bit(&mut self, set_fn: fn(&mut Registers, u16)) {
        let low_byte = self.read_from_pc();

        let high_byte = self.read_from_pc();

        let data = ((high_byte as u16) << 8) | (low_byte as u16);

        set_fn(&mut self.registers, data)
    }

    fn decrement_hl(&mut self) {
        self.registers.set_hl(self.registers.hl() - 1)
    }

    fn increment_hl(&mut self) {
        self.registers.set_hl(self.registers.hl() + 1)
    }

    ld_imm_16_bit!(bc, Registers::set_bc);
    ld_imm_16_bit!(de, Registers::set_de);
    ld_imm_16_bit!(hl, Registers::set_hl);
    ld_imm_16_bit!(sp, |reg, value| {
        reg.sp = value;
    });

    // Load immediate value to 8 bit register
    ld_imm_8_bit!(a);
    ld_imm_8_bit!(b);
    ld_imm_8_bit!(c);
    ld_imm_8_bit!(d);
    ld_imm_8_bit!(e);
    ld_imm_8_bit!(h);
    ld_imm_8_bit!(l);

    // Load indirect value provided by register pair to 8 bit register
    ld_ind!(bc, a);
    ld_ind!(de, a);
    ld_ind!(hl, a, "add", |cpu: &mut CPU| {
        cpu.increment_hl();
    });
    ld_ind!(hl, a, "sub", |cpu: &mut CPU| {
        cpu.decrement_hl();
    });

    // Load from register
    ld_register_contents!(a, b, c, d, e, h, l);

    // Load from indirect address in HL to register
    ld_indirect_contents!(a);
    ld_indirect_contents!(b);
    ld_indirect_contents!(c);
    ld_indirect_contents!(d);
    ld_indirect_contents!(e);
    ld_indirect_contents!(h);
    ld_indirect_contents!(l);

    // Store from register to indirect address in HL
    str_indirect_contents!(a);
    str_indirect_contents!(b);
    str_indirect_contents!(c);
    str_indirect_contents!(d);
    str_indirect_contents!(e);
    str_indirect_contents!(h);
    str_indirect_contents!(l);

    // Store register in address provided by indirect register memory
    str_ind!(bc, a);
    str_ind!(de, a);
    str_ind!(hl, a, "add", |cpu: &mut CPU| {
        cpu.increment_hl();
    });
    str_ind!(hl, a, "sub", |cpu: &mut CPU| {
        cpu.decrement_hl();
    });

    // Store immediate value in address provided by indirect register memory
    pub(super) fn str_imm_ind_hl(&mut self) {
        let addr = self.registers.hl();

        let value = self.read_from_pc();

        self.write(addr, value);
    }

    pub(super) fn str_sp_mem(&mut self) {
        let low_byte = self.read_from_pc();

        let high_byte = self.read_from_pc();

        let addr = ((high_byte as u16) << 8) | (low_byte as u16);

        let low_byte_sp = (self.registers.sp & 0x00ff) as u8;
        let high_byte_sp = (self.registers.sp >> 8) as u8;

        self.write(addr, low_byte_sp);
        self.write(addr + 1, high_byte_sp);
    }

    // 8 bit memory instructions
    pub(super) fn str_ind_a_8_bit(&mut self) {
        let low_byte = self.read_from_pc();
        let addr = build_16_bit(0xff, low_byte);

        self.write(addr, self.registers.a);
    }

    pub(super) fn ld_ind_a_8_bit(&mut self) {
        let low_byte = self.read_from_pc();
        let addr = build_16_bit(0xff, low_byte);

        self.registers.a = self.read(addr);
    }

    pub(super) fn str_ind_a_c_8_bit(&mut self) {
        let addr = build_16_bit(0xff, self.registers.c);
        self.write(addr, self.registers.a)
    }

    pub(super) fn ld_ind_a_c_8_bit(&mut self) {
        let addr = build_16_bit(0xff, self.registers.c);
        self.registers.a = self.read(addr);
    }

    // 16 bit indirects on A register
    pub(super) fn str_ind_a(&mut self) {
        let low_byte = self.read_from_pc();
        let high_byte = self.read_from_pc();

        let addr = build_16_bit(high_byte, low_byte);

        self.write(addr, self.registers.a);
    }

    pub(super) fn ld_ind_a(&mut self) {
        let low_byte = self.read_from_pc();
        let high_byte = self.read_from_pc();

        let addr = build_16_bit(high_byte, low_byte);

        self.registers.a = self.read(addr);
    }

    pub(super) fn str_imm_sp_hl(&mut self) {
        let value = self.read_from_pc();

        let result = ((self.registers.pc as i32) + (value as i32)) as u16;

        self.registers.f.half_carry = half_carry_occurred_16(value as u16, self.registers.pc);
        self.registers.f.carry = carry_occurred_16(value as u16, self.registers.pc);

        self.registers.set_hl(result);
    }

    pub(super) fn ld_sp_hl(&mut self) {
        self.registers.sp = self.registers.hl();
    }
}
