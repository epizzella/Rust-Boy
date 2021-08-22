pub const VRAM_START: usize = 0x8000;
pub const VRAM_END: usize = 0x9fff;

//Blocks contain the pixel artwork (tiles)
const BLOCK_0_START: usize = 0x8000;
const BLOCK_0_END: usize = 0x87ff;

const BLOCK_1_START: usize = 0x8800;
const BLOCK_1_END: usize = 0x8fff;

const BLOCK_2_START: usize = 0x9000;
pub const BLOCK_2_END: usize = 0x97ff;

//Tile maps using bytes as indexes for the tiles in blocks
const TILE_MAP_1_START: usize = 0x9800;
const TILE_MAP_1_END: usize = 0x9BFF;

const TILE_MAP_2_START: usize = 0x9C00;
const TILE_MAP2_END: usize = 0x9FFF;

const TILE_RAM_SIZE: usize = TILE_MAP2_END - TILE_MAP_1_START + 1;

const BLOCKS_PER_VRAM: usize = 3;
const TILES_PER_BANK: usize = (BLOCK_0_END - BLOCK_0_START + 1) / BYTES_PER_TILE;
const BYTES_PER_TILE: usize = 16;

pub struct Vram {
    //There are 3 different memory block: https://gbdev.io/pandocs/Tile_Data.html
    tile_ram: [Block; BLOCKS_PER_VRAM],
    map_ram: [u8; TILE_RAM_SIZE],
}

impl Vram {
    pub fn new() -> Self {
        Self {
            tile_ram: [Block::new(); 3],
            map_ram: [0; TILE_RAM_SIZE],
        }
    }

    pub fn cache_tile_line(&self, block_index: usize, title_index: usize, line_index: usize) -> TilePixelLine {
        let mut pixel_data = TilePixelLine::new();

        if block_index >= BLOCKS_PER_VRAM || title_index >= TILES_PER_BANK || line_index >= 8 {
            //TODO:log an error here
        } else {
            pixel_data.low_byte = self.tile_ram[block_index].block[title_index].tile[line_index];
            pixel_data.hight_byte = self.tile_ram[block_index].block[title_index].tile[line_index + 1];
        }

        //return
        pixel_data
    }

    //Reads a specified byte from vram.  Used by the cpu.
    pub fn read_vram_tile(&self, mut address: usize) -> u8 {
        let block_index: usize;
        match address {
            BLOCK_0_START..=BLOCK_0_END => {
                address -= BLOCK_0_START;
                block_index = 0;
            }
            BLOCK_1_START..=BLOCK_1_END => {
                address -= BLOCK_1_START;
                block_index = 1;
            }
            BLOCK_2_START..=BLOCK_2_END => {
                address -= BLOCK_2_START;
                block_index = 2;
            }
            _ => block_index = 0, //TODO: add some kind of error logging here
        }

        let tile_index = address / BYTES_PER_TILE;
        let byte_index = address % BYTES_PER_TILE;

        //return
        self.tile_ram[block_index].block[tile_index].tile[byte_index]
    }

    //Writes a specific byte of a tile to vram.  Used by the cpu.
    pub fn write_vram_tile(&mut self, mut address: usize, data: u8) {
        let block_index: usize;
        match address {
            BLOCK_0_START..=BLOCK_0_END => {
                address -= BLOCK_0_START;
                block_index = 0;
            }
            BLOCK_1_START..=BLOCK_1_END => {
                address -= BLOCK_1_START;
                block_index = 1;
            }
            BLOCK_2_START..=BLOCK_2_END => {
                address -= BLOCK_2_START;
                block_index = 2;
            }
            _ => block_index = 0, //TODO: add some kind of error logging here
        }

        let tile_index = address / BYTES_PER_TILE;
        let byte_index = address % BYTES_PER_TILE;

        self.tile_ram[block_index].block[tile_index].tile[byte_index] = data;
    }

    pub fn read_vram_map(&self, mut address: usize) -> u8 {
        self.map_ram[address - TILE_MAP_1_START]
    }
    pub fn write_vram_map(&mut self, mut address: usize, data: u8) {
        self.map_ram[address - TILE_MAP_1_START] = data;
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

pub struct TilePixelLine {
    hight_byte: u8,
    low_byte: u8,
}

impl TilePixelLine {
    pub fn new() -> Self {
        Self {
            hight_byte: 0,
            low_byte: 0,
        }
    }
}
