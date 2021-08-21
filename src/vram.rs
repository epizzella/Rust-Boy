pub const VRAM_START: usize = 0x8000;
pub const VRAM_END: usize = 0x9fff;

//Blocks contain the pixel artwork (tiles)
const BLOCK_0_START: usize = 0x8000;
const BLOCK_0_END: usize = 0x87ff;

const BLOCK_1_START: usize = 0x8800;
const BLOCK_1_END: usize = 0x8fff;

const BLOCK_2_START: usize = 0x9000;
const BLOCK_2_END: usize = 0x97ff;

//Tile maps using bytes as indexes for the tiles in blocks
const TILE_MAP_1_START: usize = 0x9800;
const TILE_MAP_1_END: usize = 0x9BFF;

const TILE_MAP_2_START: usize = 0x9C00;
const TILE_MAP2_END: usize = 0x9FFF;

const BLOCKS_PER_VRAM: usize = 3;
const BYTES_PER_TILE: usize = 16;
const TILES_PER_BANK: usize = (BLOCK_0_END - BLOCK_0_START + 1) / BYTES_PER_TILE;

pub struct Vram {
    //There are 3 different memory block: https://gbdev.io/pandocs/Tile_Data.html
    ram: [Block; BLOCKS_PER_VRAM],
}

impl Vram {
    pub fn new() -> Self {
        Self {
            ram: [Block::new(); 3],
        }
    }

    //TODO: Add functions for ppu to read from vram banks

    /*
    pub fn cache_line(&mut self, tile_index: usize, high_byte: u8, low_byte: u8) {
        let mut color_index: u8;

        for i in 0..8 as u8 {
            let mut high_bit = high_byte & (1 << i);
            let mut low_bit = low_byte & (1 << i);

            high_bit >>= i;
            low_bit >>= i;

            high_bit <<= 1;

            color_index = high_bit | low_bit;

            self.ram[1].block[tile_index].tile[i as usize] = color_index;
        }
    }
    */

    //Reads a specified byte from vram.  Used by the cpu.
    pub fn read_vram(&self, mut address: usize) -> u8 {
        let index: usize;
        match address {
            BLOCK_0_START..=BLOCK_0_END => {
                address -= BLOCK_0_START;
                index = 0;
            }
            BLOCK_1_START..=BLOCK_1_END => {
                address -= BLOCK_1_START;
                index = 1;
            }
            BLOCK_2_START..=BLOCK_2_END => {
                address -= BLOCK_2_START;
                index = 2;
            }
            _ => index = 0, //TODO: add some kind of error logging here
        }

        let tile_index = address / BYTES_PER_TILE;
        let byte_index = address % BYTES_PER_TILE;

        //return
        self.ram[index].block[tile_index].tile[byte_index]
    }

    //Writes a specific byte of a tile to vram.  Used by the cpu.
    pub fn write_vram(&mut self, mut address: usize, data: u8) {
        let index: usize;
        match address {
            BLOCK_0_START..=BLOCK_0_END => {
                address -= BLOCK_0_START;
                index = 0;
            }
            BLOCK_1_START..=BLOCK_1_END => {
                address -= BLOCK_1_START;
                index = 1;
            }
            BLOCK_2_START..=BLOCK_2_END => {
                address -= BLOCK_2_START;
                index = 2;
            }
            _ => index = 0, //TODO: add some kind of error logging here
        }

        let tile_index = address / BYTES_PER_TILE;
        let byte_index = address % BYTES_PER_TILE;

        self.ram[index].block[tile_index].tile[byte_index] = data;
    }
}

#[derive(Copy, Clone)]
struct Block {
    //Each tile in the gb's vram is 16 bytes in total
    //Each bank holds 128 tiles total
    block: [Tile; TILES_PER_BANK],
}

impl Block {
    pub fn new() -> Self {
        Self {
            block: [Tile::new(); TILES_PER_BANK],
        }
    }
}

#[derive(Copy, Clone)]
struct Tile {
    tile: [u8; BYTES_PER_TILE],
}

impl Tile {
    pub fn new() -> Self {
        Self {
            tile: [0; BYTES_PER_TILE],
        }
    }
}
