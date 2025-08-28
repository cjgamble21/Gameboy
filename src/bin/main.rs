use std::env;

use gb_core::Emulator;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut emu = Emulator::new(&args[1]);

    emu.execute();
}
