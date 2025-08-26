use std::io::Write;

pub trait Bus {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, data: u8);
    fn request_interrupt(&mut self);
}

const BUS_SIZE: usize = std::u16::MAX as usize + 1;

pub struct SystemBus {
    memory: Vec<u8>,
}

impl SystemBus {
    pub fn new() -> Self {
        Self {
            memory: vec![0; BUS_SIZE],
        }
    }
}

impl Bus for SystemBus {
    fn read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    fn request_interrupt(&mut self) {
        unimplemented!("TODO: implement interrupt requests")
    }
}
