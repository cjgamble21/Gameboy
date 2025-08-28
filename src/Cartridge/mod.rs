mod header;

pub(super) struct Cartridge {
    size: u64,
    pub rom: Vec<u8>,
    ram: [u8; 8 * 1024],
}

impl Cartridge {
    pub fn new(size: u64, rom: Vec<u8>) -> Self {
        Self {
            size,
            rom,
            ram: [0; 8 * 1024],
        }
    }
}
