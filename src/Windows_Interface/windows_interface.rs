use std::path::Path;
use std::vec::Vec;
use std::{fs::File, time::Duration};
use std::{
    fs::OpenOptions,
    io::{BufWriter, Read, Write},
};

use crate::cpu::*;
use crate::vram::*;

pub struct WindowsInterface {
    buff: BufWriter<File>,
}

impl WindowsInterface {
    pub fn new() -> Self {
        let log_path = Path::new("DMG_log.txt");
        let log_file: File;

        if log_path.exists() {
            log_file = OpenOptions::new()
                .append(true)
                .open(log_path)
                .expect("Cannot open file");
        } else {
            log_file = File::create("DMG_log.txt").expect("Unable to create file");
        }

        let buffer = BufWriter::new(log_file);

        Self { buff: buffer }
    }

    pub fn print_log_file(&mut self, cpu: &Cpu) -> std::io::Result<()> {
        let sp = cpu.read_sp();
        let pc = cpu.read_pc();

        writeln!(self.buff,
           //"A: {:02X} F: {:02X} B: {:02X} C: {:02X} D: {:02X} E: {:02X} H: {:02X} L: {:02X} SP: {:04X} PC: 00:{:04X} ({:02X} {:02X} {:02X} {:02X}) (HL) ({:02X}) ",
            "A: {:02X} F: {:02X} B: {:02X} C: {:02X} D: {:02X} E: {:02X} H: {:02X} L: {:02X} SP: {:04X} PC: 00:{:04X} ({:02X} {:02X} {:02X} {:02X})",
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
        //    cpu.read_memory(0x45fb),
        //    cpu.read_memory(cpu.read_reg16(Reg16bit::HL as usize) as usize),
        )?;

        Ok(())
    }

    //Loads the rom from a file to the cpu's memory array
    pub fn load_rom(file_path: &str, cpu: &mut Cpu) {
        let mut rom = File::open(file_path).expect("Rom was not found");
        let mut buffer: Vec<u8> = Vec::new();
        let buffer_size = rom.read_to_end(&mut buffer).expect("Error when reading rom");

        if buffer_size < VRAM_END {
            //transfer rom into memory
            for i in 0..buffer_size {
                cpu.load_read_only_data(i, buffer[i]);
            }
        } else {
            println!("Rom size ({} bytes) greater than end of vram.", buffer_size);
        }
    }

    pub fn sleep() {
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 1000));
    }
}
