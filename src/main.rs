mod cpu;
mod instructions;
mod memory_bank_controller;
mod opcode_table;
mod ppu;
mod rom;
mod timer;
mod user_interface;
mod vram;

#[path = "Windows_Interface/windows_interface.rs"]
mod windows_interface;

use crate::cpu::*;
use opcode_table::OpcodeTable;
//use std::env;
use ppu::*;
use windows_interface::*;

fn main() {
    let mut gameboy_cpu: Cpu = Cpu::new();
    let unprifxed_instructions = OpcodeTable::init_unprefix_instruction_table();
    let prifxed_instructions = OpcodeTable::init_prefix_instruction_table();
    let mut windows = WindowsInterface::new();

    //get command line arguments
    //let args: Vec<String> = env::args().collect();

    //load rom into cpu's memory array
    //load_rom(&args[1], &mut gameboy_cpu);

    //load tetris; hard coded to work with debug
    //load_rom("C:\\Repos\\GBCEmulator\\roms\\Tetris.gb", &mut gameboy_cpu);

    WindowsInterface::load_rom(
        "C:\\Repos\\GBCEmulator\\roms\\cpu_test\\08-misc instrs.gb",
        &mut gameboy_cpu,
    );

    loop {
        gameboy_cpu.execute_step(&unprifxed_instructions, &prifxed_instructions, &mut windows);

        //print anything from the serial port
        if gameboy_cpu.read_memory(0xff02) > 0 {
            let mut buff = [0; 4];
            let output = (gameboy_cpu.read_memory(0xff01) as char).encode_utf8(&mut buff);

            print!("{}", output);
            gameboy_cpu.write_memory(0xff02, 0x0)
        }

        WindowsInterface::sleep();
    }

    //    println!("End of Program");
}
