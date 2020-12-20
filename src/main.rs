mod cpu;
mod instructions;
mod opcode_table;
mod user_interface;
mod windows_interface;

use crate::cpu::*;
use opcode_table::OpcodeTable;
use std::env;
use windows_interface::*;

fn main() {
    let mut gameboy_cpu: Cpu = Cpu::new();
    let unprifxed_instructions = OpcodeTable::init_unprefix_insturction_table();

    //get command line arguments
    let args: Vec<String> = env::args().collect();

    //load rom into cpu's memory array
    //load_rom(&args[1], &mut gameboy_cpu);

    //load tetris; hard coded to work with debug
    //load_rom("C:\\Repos\\GBCEmulator\\roms\\Tetris.gb", &mut gameboy_cpu);

    load_rom(
        "C:\\Repos\\GBCEmulator\\roms\\cpu_test\\01-special.gb",
        &mut gameboy_cpu,
    );

    loop {
        gameboy_cpu.execute_step(&unprifxed_instructions);

        if gameboy_cpu.read_memory(0xff02) > 0 {
            let mut buff = [0; 4];
            let output = (gameboy_cpu.read_memory(0xff01) as char).encode_utf8(&mut buff);

            println!("{}", output);
            gameboy_cpu.write_memory(0xff02, 0x0)
        }

        sleep();
    }

    //    println!("End of Program");
}
