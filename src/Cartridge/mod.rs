mod header;

pub(super) struct Cartridge {
    rom: [u8; 32 * 1024],
    ram: [u8; 8 * 1024],
}
