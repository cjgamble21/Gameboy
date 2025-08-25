#[path = "CPU/mod.rs"]
pub mod cpu;

trait Memory {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, data: u8);
}
