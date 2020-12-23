use std::path::Path;
use std::vec::Vec;
use std::{fs::File, time::Duration};
use std::{
    fs::OpenOptions,
    io::{BufWriter, Read, Write},
};

use crate::cpu::*;

//Loads the rom from a file to the cpu's memory array
pub fn load_rom(file_path: &str, cpu: &mut Cpu) {
    let mut rom = File::open(file_path).expect("Rom was not found");
    let mut buffer: Vec<u8> = Vec::new();
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

pub fn print_log_console(cpu: &Cpu) {
    let sp = cpu.read_sp();
    let pc = cpu.read_pc();

    println!(
        "A: {:02X} F: {:02X} B: {:02X} C: {:02X} D: {:02X} E: {:02X} H: {:02X} L: {:02X} SP: {:02X} PC: 00:{:02X} Mem({:02X} {:02X} {:02X} {:02X}) [STACK]({:02X} {:02X})",
        cpu.read_reg8(Reg8bit::A as usize),
        cpu.read_reg8(Reg8bit::F as usize),
        cpu.read_reg8(Reg8bit::B as usize),
        cpu.read_reg8(Reg8bit::C as usize),
        cpu.read_reg8(Reg8bit::D as usize),
        cpu.read_reg8(Reg8bit::E as usize),
        cpu.read_reg8(Reg8bit::H as usize),
        cpu.read_reg8(Reg8bit::L as usize),
        sp,
        pc,
        cpu.read_memory(pc as usize),
        cpu.read_memory(pc as usize + 1),
        cpu.read_memory(pc as usize + 2),
        cpu.read_memory(pc as usize + 3),
        cpu.read_memory(sp as usize),
        cpu.read_memory(sp as usize + 1),
    );
}

pub fn print_log_file(cpu: &Cpu) -> std::io::Result<()> {
    let path = Path::new("DMG_log.txt");
    let f: File;

    let sp = cpu.read_sp();
    let pc = cpu.read_pc();

    if path.exists() {
        f = OpenOptions::new()
            .append(true)
            .open(path)
            .expect("Cannot open file");
    } else {
        f = File::create("DMG_log.txt").expect("Unable to create file");
    }

    let mut f = BufWriter::new(f);

    writeln!(f,
        "A: {:02X} F: {:02X} B: {:02X} C: {:02X} D: {:02X} E: {:02X} H: {:02X} L: {:02X} SP: {:02X} PC: 00:{:04X} ({:02X} {:02X} {:02X} {:02X})",
        cpu.read_reg8(Reg8bit::A as usize),
        cpu.read_reg8(Reg8bit::F as usize),
        cpu.read_reg8(Reg8bit::B as usize),
        cpu.read_reg8(Reg8bit::C as usize),
        cpu.read_reg8(Reg8bit::D as usize),
        cpu.read_reg8(Reg8bit::E as usize),
        cpu.read_reg8(Reg8bit::H as usize),
        cpu.read_reg8(Reg8bit::L as usize),
        sp,
        pc,
        cpu.read_memory(pc as usize),
        cpu.read_memory(pc as usize + 1),
        cpu.read_memory(pc as usize + 2),
        cpu.read_memory(pc as usize + 3),
    )?;

    Ok(())
}

pub fn sleep() {
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 600));
}
