mod CPU;
trait Memory {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, data: u8);
}

fn main() {
    // let mut cpu = CPU::CPU::new();
    // cpu.tick();
}

// main();
