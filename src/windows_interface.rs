use std::fs::File;
use std::io::Read;
use std::vec::Vec;

use crate::cpu::*;

//Loads the rom from a file to the cpu's memory array
pub fn load_rom(file_path: &str, cpu: &mut Cpu) {
    let mut rom = File::open(file_path).expect("Rom was not found");
    let mut buffer = Vec::new();
    let buffer_size = rom.read_to_end(&mut buffer).expect("Error when reading rom");

    if buffer_size < VRAM_END {
        //transfer rom into memory
        for i in 0..buffer_size {
            cpu.write_memory(i, buffer[i]);
        }
    } else {
        println!("Rom size ({} bytes) greater than end of vram.", buffer_size);
    }
}
