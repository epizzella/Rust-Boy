use crate::vram::*;

pub const LCD_ADDR_START: usize = 0xff40;
pub const LCD_ADDR_END: usize = 0xff4b;
pub const LCD_WIDTH: usize = 160;
pub const LCD_HEIGHT: usize = 144;
pub const SCREEN_RESOLUTION: usize = LCD_WIDTH * LCD_HEIGHT;

const MEM_SIZE: usize = LCD_ADDR_END - LCD_ADDR_START + 1;

pub const LCD_CRTL_REG: usize = 0xff40; //LCD Control Register

pub const LCD_Y_REG: usize = 0xff44; //LCD Y-Coordinate (R)

pub struct Lcd {
    memory_registers: [u8; MEM_SIZE],
    vram: Vram,
    //0xff40: LCD Control Register
    //0xff41: LCD Status Register (R/W)
    //0xff42: SCY - Scroll Y (R/W)
    //0xff43: SCX - Scroll X (R/W)
    //0xff44: LY  - LCD Y-Coordinate (R)
    //0xff45: LYC - LY Compare (R/W)
    //0xff46: DMA - DMA Transfer and Start Address (R/W)
    //0xff47: BGP  - Background Palette Data (R/W) - Non CGB Mode Only
    //0xff48: OBP0 - Object Palette 0 Data (R/W) - Non CGB Mode Only
    //0xff49: OBP1 - Object Palette 1 Data (R/W) - Non CGB Mode Only
    //0xff4a: WY - Window Y Position  (R/W)
    //0xff4b: WX - Window X Position minus 7  (R/W)
    dot_counter: u32, //Keeps track of the number of dot clock cyles. Max 70244
    current_row: u8,
    current_col: u8,
    screen_buffer_a: [u8; SCREEN_RESOLUTION],
    screen_buffer_b: [u8; SCREEN_RESOLUTION],
}

impl Lcd {
    pub fn new() -> Self {
        Self {
            memory_registers: [0; MEM_SIZE],
            vram: Vram::new(),
            dot_counter: 0,
            current_row: 0,
            current_col: 0,
            screen_buffer_a: [0; SCREEN_RESOLUTION],
            screen_buffer_b: [0; SCREEN_RESOLUTION],
        }
    }

    //Updates the current image buffer
    pub fn update_lcd(&mut self) {}

    pub fn read_vram(&self, address: usize) -> u8 {
        let data: u8;
        if address <= BLOCK_2_END {
            data = self.vram.read_vram_tile(address);
        } else {
            data = self.vram.read_vram_map(address);
        }

        //return
        data
    }

    pub fn write_vram(&mut self, address: usize, data: u8) {
        if address <= BLOCK_2_END {
            self.vram.write_vram_tile(address, data);
        } else {
            self.vram.write_vram_map(address, data);
        }
    }

    pub fn write_register(&mut self, index: usize, data: u8) {
        self.memory_registers[index - LCD_ADDR_START] = data;
    }

    pub fn read_register(&self, index: usize) -> u8 {
        self.memory_registers[index - LCD_ADDR_START]
    }
}
