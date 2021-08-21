pub const LCD_ADDR_START: usize = 0xff40;
pub const LCD_ADDR_END: usize = 0xff4b;
pub const LCD_WIDTH: usize = 160;
pub const LCD_HEIGHT: usize = 144;
pub const SCREEN_RESOLUTION: usize = LCD_WIDTH * LCD_HEIGHT;

pub struct Lcd {
    memory_registers: [u8; LCD_ADDR_END - LCD_ADDR_START + 1],
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
    screen_buffer_a: [u8; SCREEN_RESOLUTION],
    screen_buffer_b: [u8; SCREEN_RESOLUTION],
    bg_color_pallet: [u8; 4],
}

impl Lcd {
    pub fn read_register(&self,index: usize ) ->u8 {
        self.memory_registers[index - LCD_ADDR_START] 
    }
}
