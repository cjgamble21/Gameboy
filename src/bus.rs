use crate::cartridge::Cartridge;
use std::{fs::File, io::Read, path::Path};

pub trait Bus {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, data: u8);
    fn request_interrupt(&mut self);
    fn enable_interrupts(&mut self);
    fn disable_interrupts(&mut self);
}

const VRAM_SIZE: usize = 8 * 1024;
const VRAM_OFFSET: u16 = 0x8000;

const WRAM_SIZE: usize = 8 * 1024;
const WRAM_OFFSET: u16 = 0xC000;

const OAM_SIZE: usize = 160;
const OAM_OFFSET: u16 = 0xFE00;

const HRAM_SIZE: usize = 127;
const HRAM_OFFSET: u16 = 0xFF80;

pub struct SystemBus {
    cartridge: Cartridge,
    vram: [u8; 8 * 1024],     // 0x8000 -> 0x9FFF
    wram: [u8; 8 * 1024],     // 0xC000 -> 0xDFFF
    oam: [u8; 160],           // 0xFE00 -> FE9F
    hram: [u8; 127],          // 0xFF80 -> 0xFFFE
    interrupts_enabled: bool, // 0xFFFF
}

pub fn load_cartridge(name: &str) -> Cartridge {
    let path = Path::new(name);

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't read path {}", why),
        Ok(file) => file,
    };

    let size = file.metadata().unwrap().len();
    let mut buffer = Vec::<u8>::with_capacity(size as usize);

    match file.read_to_end(&mut buffer) {
        Err(why) => panic!("Error reading cartridge data {}", why),
        Ok(_) => Cartridge::new(size, buffer),
    }
}

impl SystemBus {
    pub fn new(cartridge_file: &str) -> Self {
        Self {
            cartridge: load_cartridge(cartridge_file),
            vram: [0; VRAM_SIZE],
            wram: [0; WRAM_SIZE],
            oam: [0; OAM_SIZE],
            hram: [0; HRAM_SIZE],
            interrupts_enabled: false,
        }
    }
}

impl Bus for SystemBus {
    fn read(&self, addr: u16) -> u8 {
        // TODO: Implement memory mapping
        match addr {
            0x0000..=0x7FFF => self.cartridge.rom[addr as usize],
            0x8000..=0x9FFF => self.vram[(addr - VRAM_OFFSET) as usize],
            0xC000..=0xDFFF => self.wram[(addr - WRAM_OFFSET) as usize],
            0xFE00..=0xFE9F => self.oam[(addr - OAM_OFFSET) as usize],
            0xFF80..=0xFFFE => self.hram[(addr - HRAM_OFFSET) as usize],
            0xFFFF => self.interrupts_enabled as u8,
            _ => 0,
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        // TODO: Implement memory mapping
        match addr {
            0x0000..=0x7FFF => self.cartridge.rom[addr as usize] = data,
            0x8000..=0x9FFF => self.vram[(addr - VRAM_OFFSET) as usize] = data,
            0xC000..=0xDFFF => self.wram[(addr - WRAM_OFFSET) as usize] = data,
            0xFE00..=0xFE9F => self.oam[(addr - OAM_OFFSET) as usize] = data,
            0xFF80..=0xFFFE => self.hram[(addr - HRAM_OFFSET) as usize] = data,
            0xFFFF => {
                if data == 1 {
                    self.enable_interrupts();
                } else {
                    self.disable_interrupts();
                }
            }
            _ => {}
        }
    }

    fn request_interrupt(&mut self) {
        unimplemented!("TODO: implement interrupt requests")
    }

    fn enable_interrupts(&mut self) {
        self.interrupts_enabled = true;
    }

    fn disable_interrupts(&mut self) {
        self.interrupts_enabled = false;
    }
}
