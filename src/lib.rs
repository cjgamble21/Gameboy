mod registers;
mod cpu;

pub const RAM_SIZE: usize = 4096; // 32 KB

static MEMORY: [u8; RAM_SIZE] = [0; RAM_SIZE];

trait Memory {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, data: u8);
}
