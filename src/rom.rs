pub const ROM_BANK_00_START: usize = 0x0000;
pub const ROM_BANK_00_END: usize = 0x3fff;
pub const ROM_BANK_01_START: usize = 0x4000;
pub const ROM_BANK_01_END: usize = 0x7fff;

#[derive(Copy, Clone)]
pub struct Rom {
    rom: [u8; ROM_BANK_00_END + 1],
}

impl Rom {
    pub fn new() -> Self {
        Self {
            rom: [0; ROM_BANK_00_END + 1],
        }
    }

    pub fn read_memory(&self, index: usize) -> u8 {
        self.rom[index]
    }

    pub fn write_memory(&mut self, index: usize, data: u8) {
        self.rom[index] = data;
    }
}
