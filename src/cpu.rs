use crate::memory_bank_controller::*;
use crate::ppu::*;
use crate::rom::*;
use crate::timer::*;
use crate::vram::*;
use crate::windows_interface::*;
use crate::{instructions::Opcode, opcode_table::*};

pub struct Cpu {
    //registers -- note: that A is the accumulator.  All maths are done through this reg.
    // 0  1  2  3  4  5  6  7
    // B  C  D  E  H  L  F  A
    registers: [u8; 8],
    sp: u16,
    pc: u16,
    timer: Timer,
    //Memory
    mcb: Mcb,
    lcd: Lcd,
    memory: [u8; 0x10000],
    ime: bool,
    halt: bool,
    /*
    rom_bank_0: [u8; self.BANK_00_END - self.BANK_00_START], //16KB ROM Bank 00     (in cartridge, fixed at bank 00)
    rom_bank_1: [u8; self.BANK_01_END - self.BANK_01_START], //16KB ROM Bank 01..NN (in cartridge, switchable bank number)
    vram: [u8; self.VRAM_END - self.VRAM_START], //8KB Video RAM (VRAM) (switchable bank 0-1 in CGB Mode)
    external_ram: [u8; self.EXTERNAL_RAM_END - EXTERNAL_RAM_START], //8KB External RAM     (in cartridge, switchable bank, if any)
    wram_bank_0: [u8; self.WRAM_BANK_0_END - self.WRAM_BANK_0_START],
    wram_bank_1: [u8; self.WRAM_BANK_1_END - self.WRAM_BANK_1_START],
    echo: [u8; self.ECHO_END - self.ECHO_START],
    oam: [u8; self.OAM_END - self.OAM_START], //Object atribute map (sprit info)
    input_output: [u8; self.IO_END - self.IO_START],
    hram: [u8; self.HRAM_END - self.HRAM_START],
    */
}

#[derive(Copy, Clone)]
pub enum Reg8bit {
    B = 0,
    C,
    D,
    E,
    H,
    L,
    F,
    A,
}

#[derive(Copy, Clone)]
pub enum Reg16bit {
    BC = 0,
    DE = 2,
    HL = 4,
    AF = 6, //these registers are backwards in the array
}

//Public Methods
impl Cpu {
    pub fn new() -> Self {
        let mut cpu = Self {
            registers: [0x00, 0x13, 0x00, 0xD8, 0x01, 0x4D, 0xB0, 0x01],
            sp: 0xfffe, //Top of stack, stack grows down
            pc: 0x0100, //where rom execution starts after bootstrap
            timer: Timer::new(),
            mcb: Mcb::new(),
            lcd: Lcd::new(),
            memory: [0; 0x10000],
            ime: false,
            halt: false,
        };

        cpu.write_memory(LCD_Y_REG, 0x90);
        cpu.write_memory(0xff00, 0x0f);
        cpu.write_memory(INTERRUPT_ENABLE_REG, 0x00);
        cpu.write_memory(INTERRUPT_FLAG_REG, 0xe0);

        cpu
    }

    pub fn execute_step(
        &mut self,
        unprifxed_instruct: &OpcodeTable,
        prifxed_instruct: &OpcodeTable,
        windows: &mut WindowsInterface,
    ) {
        //Check for and executes pending interrupts
        self.check_interrupts();

        let mut current_opcode = self.read_memory(self.pc as usize) as usize;
        let instruction: &Opcode;

        //windows.print_log_file(self);

        //The cpu can be halted by instruction HALT (0x76), the cpu resumes if a timer interrupt is pending
        if self.halt == false || self.get_interrupt_flag(TIMER) {
            //incrment pc by the length of the instruction.  This will cause pc to be ahead of the instuction currently being executed.
            self.pc = self
                .pc
                .wrapping_add(unprifxed_instruct.table[current_opcode].get_length() as u16);

            if current_opcode != 0xCB {
                instruction = &unprifxed_instruct.table[current_opcode];
            } else {
                current_opcode = self.read_memory((self.pc - 1) as usize) as usize;
                instruction = &prifxed_instruct.table[current_opcode];
            }

            //execute the instruction
            (instruction.handler)(&instruction, self);
        } else {
            //this is the instriction if the cpu is halted
            instruction = &unprifxed_instruct.table[current_opcode];
        }

        if self.timer.update_timers(instruction.number_of_cycles) {
            self.set_interrupt_pending(TIMER);
        }
    }

    //Write 8 bit register with value n
    #[inline]
    pub fn write_reg8(&mut self, index: usize, n: u8) {
        self.registers[index] = n;
    }

    //Copy value from one 8 bit reg to another 8 bit reg
    #[inline]
    pub fn write_reg8_with_reg8(&mut self, y: usize, z: usize) {
        self.registers[y] = self.registers[z];
    }

    //Read 8 bit register
    #[inline]
    pub fn read_reg8(&self, y: usize) -> u8 {
        self.registers[y]
    }

    //read 16 bit register
    #[inline]
    pub fn read_reg16(&self, reg_16: usize) -> u16 {
        let reg: u16 = ((self.registers[reg_16] as u16) << 8) | (self.registers[reg_16 as usize + 1] as u16);
        reg
    }

    //write to 16 bit register
    #[inline]
    pub fn write_reg16(&mut self, reg_16: usize, nn: u16) {
        let msb = ((0xff00 & nn) >> 8) as u8;
        let lsb = (0x00ff & nn) as u8;
        self.registers[reg_16 as usize] = msb;
        self.registers[reg_16 as usize + 1] = lsb;
    }

    //write two u8s to 16 bit register
    #[inline]
    pub fn write_reg16_fast(&mut self, reg_16: usize, lsb: u8, msb: u8) {
        self.registers[reg_16 as usize] = msb;
        self.registers[reg_16 as usize + 1] = lsb;
    }

    //Read the program counter
    #[inline]
    pub fn read_pc(&self) -> u16 {
        self.pc
    }

    //Write program counter
    #[inline]
    pub fn write_pc(&mut self, nn: u16) {
        self.pc = nn;
    }

    //Write to the stack pointer
    #[inline]
    pub fn write_sp(&mut self, nn: u16) {
        self.sp = nn;
    }

    //Read the stack pointer
    #[inline]
    pub fn read_sp(&self) -> u16 {
        self.sp
    }

    //Loads data in the rom buffers
    pub fn load_read_only_data(&mut self, index: usize, data: u8) {
        match index {
            ROM_BANK_00_START..=ROM_BANK_00_END => {
                self.mcb.write_bank_00(index, data);
            }
            ROM_BANK_01_START..=ROM_BANK_01_END => {
                self.mcb.write_bank_n(index, data);
            }
            _ => {}
        }
    }

    //Write a byte to memory
    #[inline]
    pub fn write_memory(&mut self, index: usize, n: u8) {
        match index {
            //Writes to this section of read only memory are used to update control registers of the memory bank controller
            ROM_BANK_00_START..=ROM_BANK_01_END => self.mcb.change_bank(),
            TIMER_ADDR_START..=TIMER_ADDR_END => self.timer.write_memory(index, n),
            VRAM_START..=VRAM_END => self.lcd.write_vram(index, n),
            LCD_ADDR_START..=LCD_ADDR_END => self.lcd.write_register(index, n),
            _ => self.memory[index] = n,
        }
    }

    //Read a byte from memory
    #[inline]
    pub fn read_memory(&self, index: usize) -> u8 {
        match index {
            ROM_BANK_00_START..=ROM_BANK_00_END => self.mcb.read_bank_00(index),
            ROM_BANK_01_START..=ROM_BANK_01_END => self.mcb.read_bank_n(index),
            TIMER_ADDR_START..=TIMER_ADDR_END => self.timer.read_memory(index),
            VRAM_START..=VRAM_END => self.lcd.read_vram(index),
            LCD_ADDR_START..=LCD_ADDR_END => self.lcd.read_register(index),
            _ => self.memory[index],
        }
    }

    //Read two bytes from memory
    #[inline]
    pub fn read_memory_nn(&self, index: usize) -> u16 {
        let value = ((self.read_memory(index + 1) as u16) << 8) | (self.read_memory(index) as u16);
        value
    }

    #[inline]
    pub fn get_zero_flag(&self) -> bool {
        (self.registers[Reg8bit::F as usize] & F_ZERO_SET) > 0
    }

    #[inline]
    pub fn get_carry_flag(&self) -> bool {
        (self.registers[Reg8bit::F as usize] & F_CARRY_SET) > 0
    }

    #[inline]
    pub fn get_half_carry_flag(&self) -> bool {
        (self.registers[Reg8bit::F as usize] & F_HALF_CARRY_SET) > 0
    }

    #[inline]
    pub fn get_add_sub_flag(&self) -> bool {
        (self.registers[Reg8bit::F as usize] & F_ADD_SUB_SET) > 0
    }

    pub fn enable_interupts(&mut self) {
        self.ime = true;
    }

    pub fn disable_interupts(&mut self) {
        self.ime = false;
    }

    //push 16bit register onto the stack
    pub fn push_rr(&mut self, reg_16: Reg16bit) {
        self.sp = self.sp.wrapping_sub(1);
        self.write_memory(self.sp as usize, self.registers[reg_16 as usize]); //msb
        self.sp = self.sp.wrapping_sub(1);
        self.write_memory(self.sp as usize, self.registers[(reg_16 as usize) + 1]);
        //lsb
    }

    //push 16bit AF register onto the stack
    pub fn push_af(&mut self) {
        self.sp = self.sp.wrapping_sub(1);
        self.write_memory(self.sp as usize, self.registers[Reg8bit::A as usize]); //Register A

        self.sp = self.sp.wrapping_sub(1);
        //Register F -- bottom four bits are supposed to always be 0
        self.write_memory(self.sp as usize, self.registers[Reg8bit::F as usize] & 0xf0);
    }

    //pop 16bit regsiter off of the stack
    pub fn pop_rr(&mut self, reg_16: Reg16bit) {
        self.write_reg16_fast(
            reg_16 as usize,
            self.read_memory(self.sp as usize),       //lsb
            self.read_memory((self.sp as usize) + 1), //msb
        );
        self.sp = self.sp.wrapping_add(2);
    }

    //pop 16bit AF regsiter off of the stack
    pub fn pop_af(&mut self) {
        //A and F are backwards in the array compared to other 16 bit registers
        self.write_reg16_fast(
            Reg16bit::AF as usize,
            self.read_memory((self.sp as usize) + 1),  //Register A
            self.read_memory(self.sp as usize) & 0xf0, //Register F -- bottom four bits are supposed to always be 0
        );
        self.sp = self.sp.wrapping_add(2);
    }

    //add register r to register a
    pub fn add_a_r(&mut self, reg_index: usize, add_carry: bool) {
        let addend = self.registers[reg_index];
        let carry_flag = self.registers[Reg8bit::F as usize] & F_CARRY_SET > 0;
        let carry_value = (carry_flag && add_carry) as u8;

        self.check_for_carry(self.registers[Reg8bit::A as usize], addend, carry_value);
        self.check_for_half_carry(self.registers[Reg8bit::A as usize], addend, carry_value);

        //do the add
        self.registers[Reg8bit::A as usize] = self.registers[Reg8bit::A as usize]
            .wrapping_add(addend)
            .wrapping_add(carry_value); //if add_carry is true then this acts as adc instruction

        self.zero_check_reg(Reg8bit::A as usize);

        //set the add/sub flag
        self.registers[Reg8bit::F as usize] &= F_ADD_SUB_CLR;
    }

    //add a value from memory to register a
    pub fn add_a_hl(&mut self, add_carry: bool) {
        let mem_index = self.read_reg16(Reg16bit::HL as usize) as usize;
        let addend = self.read_memory(mem_index);
        let carry_flag = self.registers[Reg8bit::F as usize] & F_CARRY_SET > 0;
        let carry_value = (carry_flag && add_carry) as u8;

        self.check_for_carry(self.registers[Reg8bit::A as usize], addend, carry_value);
        self.check_for_half_carry(self.registers[Reg8bit::A as usize], addend, carry_value);

        //do the add
        self.registers[Reg8bit::A as usize] = self.registers[Reg8bit::A as usize]
            .wrapping_add(addend)
            .wrapping_add(carry_value); //if add_carry is true then this acts as adc instruction

        self.zero_check_reg(Reg8bit::A as usize);

        //set the add/sub flag
        self.registers[Reg8bit::F as usize] &= F_ADD_SUB_CLR;
    }

    //adds the next value in memory to A
    pub fn add_a_n(&mut self, add_carry: bool) {
        let addend = self.read_memory((self.pc as usize) - 1);
        let carry_flag = self.registers[Reg8bit::F as usize] & F_CARRY_SET > 0;
        let carry_value = (carry_flag && add_carry) as u8;

        self.check_for_carry(self.registers[Reg8bit::A as usize], addend, carry_value);
        self.check_for_half_carry(self.registers[Reg8bit::A as usize], addend, carry_value);

        //do the add
        self.registers[Reg8bit::A as usize] = self.registers[Reg8bit::A as usize]
            .wrapping_add(addend)
            .wrapping_add(carry_value); //if add_carry is true then this acts as adc instruction

        self.zero_check_reg(Reg8bit::A as usize);

        //set the add/sub flag
        self.registers[Reg8bit::F as usize] &= F_ADD_SUB_CLR;
    }

    //sub a register to register A
    pub fn sub_a_r(&mut self, reg_index: usize, add_barrow: bool) {
        let subtrahend = self.registers[reg_index];
        let carry_flag = self.registers[Reg8bit::F as usize] & F_CARRY_SET > 0;
        let barrow_value = (carry_flag && add_barrow) as u8;

        self.check_for_carry_sub(subtrahend, barrow_value);
        self.check_for_half_carry_sub(self.registers[Reg8bit::A as usize], subtrahend, barrow_value);

        //do the sub
        self.registers[Reg8bit::A as usize] = self.registers[Reg8bit::A as usize]
            .wrapping_sub(subtrahend)
            .wrapping_sub(barrow_value); //if add_carry is true then this acts as sbc instruction

        self.zero_check_reg(Reg8bit::A as usize);

        self.registers[Reg8bit::F as usize] |= F_ADD_SUB_SET;
    }

    //sub a register to register A
    pub fn sub_a_hl(&mut self, add_barrow: bool) {
        let mem_index = self.read_reg16(Reg16bit::HL as usize) as usize;

        let subtrahend = self.read_memory(mem_index);
        let carry_flag = self.registers[Reg8bit::F as usize] & F_CARRY_SET > 0;
        let barrow_value = (carry_flag && add_barrow) as u8;

        self.check_for_carry_sub(subtrahend, barrow_value);
        self.check_for_half_carry_sub(self.registers[Reg8bit::A as usize], subtrahend, barrow_value);

        //do the sub
        self.registers[Reg8bit::A as usize] = self.registers[Reg8bit::A as usize]
            .wrapping_sub(subtrahend)
            .wrapping_sub(barrow_value); //if add_carry is true then this acts as sbc instruction

        self.zero_check_reg(Reg8bit::A as usize);

        self.registers[Reg8bit::F as usize] |= F_ADD_SUB_SET;
    }

    //sub a register to register A
    pub fn sub_a_n(&mut self, add_barrow: bool) {
        let subtrahend = self.read_memory((self.pc as usize) - 1);
        let carry_flag = self.registers[Reg8bit::F as usize] & F_CARRY_SET > 0;
        let barrow_value = (carry_flag && add_barrow) as u8;

        self.check_for_carry_sub(subtrahend, barrow_value);
        self.check_for_half_carry_sub(self.registers[Reg8bit::A as usize], subtrahend, barrow_value);

        //do the sub
        self.registers[Reg8bit::A as usize] = self.registers[Reg8bit::A as usize]
            .wrapping_sub(subtrahend)
            .wrapping_sub(barrow_value); //if add_carry is true then this acts as sbc instruction

        self.zero_check_reg(Reg8bit::A as usize);

        self.registers[Reg8bit::F as usize] |= F_ADD_SUB_SET;
    }

    //and a register with Regsiter A
    pub fn and_a_r(&mut self, reg_index: usize) {
        self.registers[Reg8bit::A as usize] &= self.registers[reg_index];

        self.zero_check_reg(Reg8bit::A as usize);
        self.registers[Reg8bit::F as usize] &= F_ADD_SUB_CLR;
        self.registers[Reg8bit::F as usize] |= F_HALF_CARRY_SET;
        self.registers[Reg8bit::F as usize] &= F_CARRY_CLR;
    }

    pub fn and_a_n(&mut self) {
        self.registers[Reg8bit::A as usize] &= self.read_memory((self.pc as usize) - 1);

        self.zero_check_reg(Reg8bit::A as usize);
        self.registers[Reg8bit::F as usize] &= F_ADD_SUB_CLR;
        self.registers[Reg8bit::F as usize] |= F_HALF_CARRY_SET;
        self.registers[Reg8bit::F as usize] &= F_CARRY_CLR;
    }

    pub fn and_a_hl(&mut self) {
        let index = self.read_reg16(Reg16bit::HL as usize) as usize;
        self.registers[Reg8bit::A as usize] &= self.read_memory(index);

        self.zero_check_reg(Reg8bit::A as usize);
        self.registers[Reg8bit::F as usize] &= F_ADD_SUB_CLR;
        self.registers[Reg8bit::F as usize] |= F_HALF_CARRY_SET;
        self.registers[Reg8bit::F as usize] &= F_CARRY_CLR;
    }

    //XOR Functions
    pub fn xor_a_r(&mut self, reg_index: usize) {
        self.registers[Reg8bit::A as usize] ^= self.registers[reg_index];

        self.zero_check_reg(Reg8bit::A as usize);
        self.registers[Reg8bit::F as usize] &= F_ADD_SUB_CLR;
        self.registers[Reg8bit::F as usize] &= F_HALF_CARRY_CLR;
        self.registers[Reg8bit::F as usize] &= F_CARRY_CLR;
    }

    pub fn xor_a_n(&mut self) {
        self.registers[Reg8bit::A as usize] ^= self.read_memory((self.pc as usize) - 1);

        self.zero_check_reg(Reg8bit::A as usize);
        self.registers[Reg8bit::F as usize] &= F_ADD_SUB_CLR;
        self.registers[Reg8bit::F as usize] &= F_HALF_CARRY_CLR;
        self.registers[Reg8bit::F as usize] &= F_CARRY_CLR;
    }

    pub fn xor_a_hl(&mut self) {
        let index = self.read_reg16(Reg16bit::HL as usize) as usize;
        self.registers[Reg8bit::A as usize] ^= self.read_memory(index);

        self.zero_check_reg(Reg8bit::A as usize);
        self.registers[Reg8bit::F as usize] &= F_ADD_SUB_CLR;
        self.registers[Reg8bit::F as usize] &= F_HALF_CARRY_CLR;
        self.registers[Reg8bit::F as usize] &= F_CARRY_CLR;
    }

    //OR functions
    pub fn or_a_r(&mut self, reg_index: usize) {
        self.registers[Reg8bit::A as usize] |= self.registers[reg_index];

        self.zero_check_reg(Reg8bit::A as usize);
        self.registers[Reg8bit::F as usize] &= F_ADD_SUB_CLR;
        self.registers[Reg8bit::F as usize] &= F_HALF_CARRY_CLR;
        self.registers[Reg8bit::F as usize] &= F_CARRY_CLR;
    }

    pub fn or_a_n(&mut self) {
        self.registers[Reg8bit::A as usize] |= self.read_memory((self.pc as usize) - 1);

        self.zero_check_reg(Reg8bit::A as usize);
        self.registers[Reg8bit::F as usize] &= F_ADD_SUB_CLR;
        self.registers[Reg8bit::F as usize] &= F_HALF_CARRY_CLR;
        self.registers[Reg8bit::F as usize] &= F_CARRY_CLR;
    }

    pub fn or_a_hl(&mut self) {
        let index = self.read_reg16(Reg16bit::HL as usize) as usize;
        self.registers[Reg8bit::A as usize] |= self.read_memory(index);

        self.zero_check_reg(Reg8bit::A as usize);
        self.registers[Reg8bit::F as usize] &= F_ADD_SUB_CLR;
        self.registers[Reg8bit::F as usize] &= F_HALF_CARRY_CLR;
        self.registers[Reg8bit::F as usize] &= F_CARRY_CLR;
    }

    //Increment register
    pub fn increment_r(&mut self, y: usize) {
        self.check_for_half_carry(self.registers[y], 1, 0);
        self.registers[y] = self.registers[y].wrapping_add(1);

        self.zero_check_reg(y);
        self.registers[Reg8bit::F as usize] &= F_ADD_SUB_CLR;
    }

    //Increment memory
    pub fn increment_memory(&mut self, memory_index: usize) {
        let addend = self.read_memory(memory_index);
        self.check_for_half_carry(addend, 1, 0);
        let sum = addend.wrapping_add(1);

        self.check_value_for_zero(sum);
        self.write_memory(memory_index, sum);

        self.registers[Reg8bit::F as usize] &= F_ADD_SUB_CLR;
    }

    //Decrement register
    pub fn decrement_r(&mut self, y: usize) {
        self.check_for_half_carry_sub(self.registers[y], 1, 0);
        self.registers[y] = self.registers[y].wrapping_sub(1);

        self.zero_check_reg(y);
        self.registers[Reg8bit::F as usize] |= F_ADD_SUB_SET;
    }

    //Decrement memory
    pub fn decrement_memory(&mut self, memory_index: usize) {
        let minuend = self.read_memory(memory_index);
        self.check_for_half_carry_sub(minuend, 1, 0);
        let difference = minuend.wrapping_sub(1);

        self.check_value_for_zero(difference);
        self.write_memory(memory_index, difference);

        self.registers[Reg8bit::F as usize] |= F_ADD_SUB_SET;
    }

    //flip the bits in register A
    pub fn complement_a(&mut self) {
        self.registers[Reg8bit::A as usize] ^= 0xff;
        self.registers[Reg8bit::F as usize] |= F_ADD_SUB_SET;
        self.registers[Reg8bit::F as usize] |= F_HALF_CARRY_SET;
    }

    pub fn add_hl_rr(&mut self, register_index: usize) {
        let hl = self.read_reg16(Reg16bit::HL as usize);
        let addend = self.read_reg16(register_index);
        let sum = hl.wrapping_add(addend);

        self.check_for_half_carry_16bit(hl, addend);
        self.check_for_carry_16bit(addend);

        self.write_reg16(Reg16bit::HL as usize, sum);

        self.registers[Reg8bit::F as usize] &= F_ADD_SUB_CLR;
    }

    pub fn add_hl_sp(&mut self) {
        let hl = self.read_reg16(Reg16bit::HL as usize);
        let addend = self.sp as u16;
        let sum = hl.wrapping_add(addend);

        self.check_for_carry_16bit(addend);
        self.check_for_half_carry_16bit(hl, addend);

        self.write_reg16(Reg16bit::HL as usize, sum);

        self.registers[Reg8bit::F as usize] &= F_ADD_SUB_CLR;
    }

    pub fn increment_sp(&mut self) {
        self.sp = self.sp.wrapping_add(1);
    }

    pub fn decrement_sp(&mut self) {
        self.sp = self.sp.wrapping_sub(1);
    }

    pub fn daa(&mut self) {
        let mut reg_a = self.read_reg8(Reg8bit::A as usize);

        if !self.get_add_sub_flag() {
            if self.get_carry_flag() || reg_a > 0x99 {
                reg_a = reg_a.wrapping_add(0x60);
                self.set_carry_flag();
            }
            if self.get_half_carry_flag() || reg_a & 0x0f > 0x09 {
                reg_a = reg_a.wrapping_add(0x06);
            }
        } else {
            if self.get_carry_flag() {
                reg_a = reg_a.wrapping_sub(0x60);
            }
            if self.get_half_carry_flag() {
                reg_a = reg_a.wrapping_sub(0x06);
            }
        }

        self.check_value_for_zero(reg_a);
        self.clear_half_carry_flag();

        self.write_reg8(Reg8bit::A as usize, reg_a);
    }

    pub fn add_sp_dd(&mut self) -> u16 {
        let addend = (self.read_memory((self.pc as usize) - 1) as i8) as i16; //this is a signed number

        //since dd is only 8bit half carry and carry are set as though this is an 8 bit operation
        self.check_for_half_carry(self.sp as u8, addend as u8, 0);
        self.check_for_carry(self.sp as u8, addend as u8, 0);

        let sum = (self.sp as i16).wrapping_add(addend) as u16; // do the math

        self.registers[Reg8bit::F as usize] &= F_ZERO_CLR;
        self.registers[Reg8bit::F as usize] &= F_ADD_SUB_CLR;

        sum
    }

    //Rotates value to the left with bit 7 being moved to bit 0 and also stored into carry
    pub fn rlc_n(&mut self, mut value: u8, check_zero: bool) -> u8 {
        if value > 0x7f {
            self.set_carry_flag()
        } else {
            self.clear_carry_flag()
        }

        value = value.rotate_left(1);

        if check_zero {
            self.zero_check_u8(value);
        } else {
            self.registers[Reg8bit::F as usize] &= F_ZERO_CLR
        }

        self.registers[Reg8bit::F as usize] &= F_ADD_SUB_CLR & F_HALF_CARRY_CLR;

        value
    }

    //Rotates value to the right with bit 0 being moved to bit 7 and also stored into carry
    pub fn rrc_n(&mut self, mut value: u8, check_zero: bool) -> u8 {
        if value & 0x01 == 1 {
            self.set_carry_flag()
        } else {
            self.clear_carry_flag()
        }

        value = value.rotate_right(1);

        if check_zero {
            self.zero_check_u8(value);
        } else {
            self.registers[Reg8bit::F as usize] &= F_ZERO_CLR
        }

        self.registers[Reg8bit::F as usize] &= F_ADD_SUB_CLR & F_HALF_CARRY_CLR;

        value
    }

    //Rotates value to the left with the carry's value put into bit 0 and bit 7 is put into the carry.
    pub fn rl_n(&mut self, value: u8, check_zero: bool) -> u8 {
        let carry = self.get_carry_flag();

        //do the rotation
        let mut ret = value.wrapping_shl(1);
        ret |= carry as u8; //put the carry bit in position 0

        //check if we carried
        if value > 127 {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        if check_zero {
            self.zero_check_u8(ret);
        } else {
            self.registers[Reg8bit::F as usize] &= F_ZERO_CLR
        }

        self.registers[Reg8bit::F as usize] &= F_ADD_SUB_CLR & F_HALF_CARRY_CLR;

        ret
    }

    //Rotates value to the right with the carry's value put into bit 7 and bit 0 is put into the carry.
    pub fn rr_n(&mut self, value: u8, check_zero: bool) -> u8 {
        let carry = self.get_carry_flag();

        //do the rotation
        let mut ret = value.wrapping_shr(1);
        ret |= (carry as u8) << 7; //put the carry bit in position 7

        //check if we carried
        if (value & 0x01) == 1 {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        if check_zero {
            self.zero_check_u8(ret);
        } else {
            self.registers[Reg8bit::F as usize] &= F_ZERO_CLR
        }

        self.registers[Reg8bit::F as usize] &= F_ADD_SUB_CLR & F_HALF_CARRY_CLR;

        ret
    }

    pub fn shift_left_arithmetic(&mut self, value: u8) -> u8 {
        let ret = value.wrapping_shl(1);

        self.zero_check_u8(ret);

        //check if we carried
        if value > 127 {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        self.registers[Reg8bit::F as usize] &= F_ADD_SUB_CLR & F_HALF_CARRY_CLR;

        ret
    }

    pub fn swap(&mut self, value: u8) -> u8 {
        let new_upper = value.wrapping_shl(4);
        let new_lower = value.wrapping_shr(4);

        self.registers[Reg8bit::F as usize] &= F_ADD_SUB_CLR & F_HALF_CARRY_CLR & F_CARRY_CLR;

        let ret = new_upper | new_lower;
        self.zero_check_u8(ret);
        ret
    }

    pub fn shift_right_arithmetic(&mut self, value: u8) -> u8 {
        let v = value.wrapping_shr(1);

        let mut ret = v;
        //set most siginifcant bit to 1 if it was 1 beofre the shift.
        if value > 127 {
            ret |= 0x80;
        }

        self.zero_check_u8(ret);

        //check if we carried
        if (value & 0x01) == 1 {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        self.registers[Reg8bit::F as usize] &= F_ADD_SUB_CLR & F_HALF_CARRY_CLR;
        ret
    }

    pub fn shift_right_logical(&mut self, value: u8) -> u8 {
        let v = value.wrapping_shr(1);

        self.zero_check_u8(v);

        //check if we carried
        if (value & 0x01) == 1 {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        self.registers[Reg8bit::F as usize] &= F_ADD_SUB_CLR & F_HALF_CARRY_CLR;
        v
    }

    //Carry bit is xored and saved to itself
    pub fn ccf(&mut self) {
        let carry_flag = self.get_carry_flag();

        if carry_flag {
            self.clear_carry_flag()
        } else {
            self.set_carry_flag();
        }

        self.clear_add_sub_flag();
        self.clear_half_carry_flag();
    }

    //Sets the carry bit
    pub fn scf(&mut self) {
        self.set_carry_flag();
        self.clear_add_sub_flag();
        self.clear_half_carry_flag();
    }

    #[inline]
    pub fn call(&mut self, addr: u16) {
        let mut sp = self.read_sp();
        let pc = self.read_pc();

        sp = sp.wrapping_sub(1);
        let temp_msb = (pc & 0xff00) >> 8;
        self.write_memory(sp as usize, temp_msb as u8); //msb of pc
        sp = sp.wrapping_sub(1);
        self.write_memory(sp as usize, pc as u8); //lsb of pc

        self.write_sp(sp);
        self.write_pc(addr);
    }

    pub fn bit_check(&mut self, value: u8, bit: u8) {
        if (value & (1 << bit)) > 0 {
            self.clear_zero_flag();
        } else {
            self.set_zero_flag();
        }

        self.clear_add_sub_flag();
        self.set_half_carry_flag();
    }

    pub fn bit_set(&mut self, value: u8, bit: u8) -> u8 {
        value | 1 << bit
    }

    pub fn bit_clear(&mut self, value: u8, bit: u8) -> u8 {
        value & !(1 << bit)
    }

    pub fn set_halt(&mut self) {
        self.halt = true;
    }

    pub fn check_interrupts(&mut self) {
        let enable_flag = self.read_memory(INTERRUPT_ENABLE_REG);
        let mut interrupt_flag = self.read_memory(INTERRUPT_FLAG_REG);

        //if all interupts are enabled
        if self.ime {
            if ((enable_flag & (1 << V_BLANK)) > 0) && ((interrupt_flag & (1 << V_BLANK)) > 0) {
                self.ime = false;
                interrupt_flag &= !(1 << V_BLANK);
                self.call(V_BLANK_ADDR);
            } else if ((enable_flag & (1 << LCD_STAT)) > 0) && ((interrupt_flag & (1 << LCD_STAT)) > 0) {
                self.ime = false;
                interrupt_flag &= !(1 << LCD_STAT);
                self.call(LCD_STAT_ADDR);
            } else if ((enable_flag & (1 << TIMER)) > 0) && ((interrupt_flag & (1 << TIMER)) > 0) {
                self.ime = false;
                interrupt_flag &= !(1 << TIMER);
                self.call(TIMER_ADDR);
            } else if ((enable_flag & (1 << SERIAL)) > 0) && ((interrupt_flag & (1 << SERIAL)) > 0) {
                self.ime = false;
                interrupt_flag &= !(1 << SERIAL);
                self.call(SERIAL_ADDR);
            } else if ((enable_flag & (1 << JOYPAD)) > 0) && ((interrupt_flag & (1 << JOYPAD)) > 0) {
                self.ime = false;
                interrupt_flag &= !(1 << JOYPAD);
                self.call(JOYPAD_ADDR);
            }

            //clear the flag
            self.write_memory(INTERRUPT_FLAG_REG, interrupt_flag);
        }
    }

    //Set the interrupt pending
    fn set_interrupt_pending(&mut self, interrupt: u8) {
        let mut interrupt_flag = self.read_memory(INTERRUPT_FLAG_REG);
        interrupt_flag |= 1 << interrupt;
        self.write_memory(INTERRUPT_FLAG_REG, interrupt_flag);
    }

    #[inline]
    fn get_interrupt_flag(&mut self, interrupt: u8) -> bool {
        let interrupt_flag = self.read_memory(INTERRUPT_FLAG_REG);
        interrupt_flag & (1 << interrupt) > 0
    }
}

//Privat methods
impl Cpu {
    //Helper methods
    #[inline]
    fn set_carry_flag(&mut self) {
        self.registers[Reg8bit::F as usize] |= F_CARRY_SET;
    }

    #[inline]
    fn clear_carry_flag(&mut self) {
        self.registers[Reg8bit::F as usize] &= F_CARRY_CLR;
    }

    #[inline]
    fn set_zero_flag(&mut self) {
        self.registers[Reg8bit::F as usize] |= F_ZERO_SET;
    }

    #[inline]
    fn clear_zero_flag(&mut self) {
        self.registers[Reg8bit::F as usize] &= F_ZERO_CLR;
    }

    #[inline]
    fn set_half_carry_flag(&mut self) {
        self.registers[Reg8bit::F as usize] |= F_HALF_CARRY_SET;
    }

    #[inline]
    fn clear_half_carry_flag(&mut self) {
        self.registers[Reg8bit::F as usize] &= F_HALF_CARRY_CLR;
    }

    #[inline]
    fn set_add_sub_flag(&mut self) {
        self.registers[Reg8bit::F as usize] |= F_ADD_SUB_SET;
    }

    #[inline]
    fn clear_add_sub_flag(&mut self) {
        self.registers[Reg8bit::F as usize] &= F_ADD_SUB_CLR;
    }

    //checks if register A is equal to zero and sets the zero flag in register F if true
    #[inline]
    fn zero_check_u8(&mut self, value: u8) {
        //check for zero
        if value == 0 {
            self.registers[Reg8bit::F as usize] |= F_ZERO_SET;
        } else {
            self.registers[Reg8bit::F as usize] &= F_ZERO_CLR;
        }
    }

    //checks if register A is equal to zero and sets the zero flag in register F if true
    #[inline]
    fn zero_check_reg(&mut self, reg_index: usize) {
        //check for zero
        if self.registers[reg_index] == 0 {
            self.registers[Reg8bit::F as usize] |= F_ZERO_SET;
        } else {
            self.registers[Reg8bit::F as usize] &= F_ZERO_CLR;
        }
    }

    //checks if register A is equal to zero and sets the zero flag in register F if true
    #[inline]
    fn check_value_for_zero(&mut self, value: u8) {
        //check for zero
        if value == 0 {
            self.registers[Reg8bit::F as usize] |= F_ZERO_SET;
        } else {
            self.registers[Reg8bit::F as usize] &= F_ZERO_CLR;
        }
    }

    //checks if a carry occured for add with register A and sets the carry flag in register F if true
    #[inline]
    fn check_for_carry(&mut self, addend_1: u8, addend_2: u8, carry: u8) {
        let z = addend_1.checked_add(addend_2).and_then(|x| x.checked_add(carry));

        match z {
            Some(_) => {
                self.registers[Reg8bit::F as usize] &= F_CARRY_CLR;
            }
            None => {
                self.registers[Reg8bit::F as usize] |= F_CARRY_SET;
            }
        }
    }

    //checks if a carry occured for sub with register A and sets the carry flag in register F if true
    #[inline]
    fn check_for_carry_sub(&mut self, value: u8, carry: u8) {
        let z = self.registers[Reg8bit::A as usize]
            .checked_sub(value)
            .and_then(|x| x.checked_sub(carry));

        match z {
            Some(_) => {
                self.registers[Reg8bit::F as usize] &= F_CARRY_CLR;
            }
            None => {
                self.registers[Reg8bit::F as usize] |= F_CARRY_SET;
            }
        }
    }

    //checks if a half carry occured for add with register A and sets the half carry flag in register F if true
    #[inline]
    fn check_for_half_carry(&mut self, addend_1: u8, addend_2: u8, carry: u8) {
        //mask off the bottom of both bytes add them together
        let half_carry = (addend_1 & 0xf) + (addend_2 & 0xf) + carry;

        //Check for a half carry
        if half_carry > 0xf {
            self.registers[Reg8bit::F as usize] |= F_HALF_CARRY_SET;
        } else {
            self.registers[Reg8bit::F as usize] &= F_HALF_CARRY_CLR;
        }
    }

    //checks if a half carry occured for sub with register A and sets the half carry flag in register F if true
    #[inline]
    fn check_for_half_carry_sub(&mut self, minuend: u8, subtrahend: u8, carry: u8) {
        //mask off the bottom of both bytes subtract them
        let half_carry = (minuend & 0xf) as i16 - (subtrahend & 0xf) as i16 - carry as i16;

        //Check for a half carry
        if half_carry < 0 {
            self.registers[Reg8bit::F as usize] |= F_HALF_CARRY_SET;
        } else {
            self.registers[Reg8bit::F as usize] &= F_HALF_CARRY_CLR;
        }
    }

    //checks if a carry occured for add with register A and sets the carry flag in register F if true
    #[inline]
    fn check_for_carry_16bit(&mut self, addend: u16) {
        let z = self.read_reg16(Reg16bit::HL as usize).checked_add(addend);

        match z {
            Some(_) => {
                self.registers[Reg8bit::F as usize] &= F_CARRY_CLR;
            }
            None => {
                self.registers[Reg8bit::F as usize] |= F_CARRY_SET;
            }
        }
    }

    //checks if a half carry occured for sub with register A and sets the half carry flag in register F if true
    #[inline]
    fn check_for_half_carry_16bit(&mut self, addend_1: u16, addend_2: u16) {
        //mask off the bottom of both numbers add them together; then mask off the top of the result
        let half_carry = (addend_1 & 0xfff) + (addend_2 & 0xfff);

        //Check for a half carry
        if half_carry > 0xfff {
            self.registers[Reg8bit::F as usize] |= F_HALF_CARRY_SET;
        } else {
            self.registers[Reg8bit::F as usize] &= F_HALF_CARRY_CLR;
        }
    }
}

const F_ZERO_SET: u8 = 0x80; //1000 0000
const F_ADD_SUB_SET: u8 = 0x40; //0100 0000
const F_HALF_CARRY_SET: u8 = 0x20; //0010 0000
const F_CARRY_SET: u8 = 0x10; //0001 0000

const F_ZERO_CLR: u8 = 0x7f; //0111 1111
const F_ADD_SUB_CLR: u8 = 0xbf; //1011 1111
const F_HALF_CARRY_CLR: u8 = 0xdf; //1101 1111
const F_CARRY_CLR: u8 = 0xef; //1110 1111

const V_BLANK: u8 = 0;
const LCD_STAT: u8 = 1;
const TIMER: u8 = 2;
const SERIAL: u8 = 3;
const JOYPAD: u8 = 4;

const V_BLANK_ADDR: u16 = 0x40;
const LCD_STAT_ADDR: u16 = 0x48;
const TIMER_ADDR: u16 = 0x50;
const SERIAL_ADDR: u16 = 0x58;
const JOYPAD_ADDR: u16 = 0x60;

const EXTERNAL_RAM_START: usize = 0xa000;
const EXTERNAL_RAM_END: usize = 0xbfff;
const WRAM_BANK_0_START: usize = 0xc000;
const WRAM_BANK_0_END: usize = 0xcfff;
const WRAM_BANK_1_START: usize = 0xd000;
const WRAM_BANK_1_END: usize = 0xdfff;
const ECHO_START: usize = 0xe000;
const ECHO_END: usize = 0xfdff;
const OAM_START: usize = 0xfe00;
const OAM_END: usize = 0xfe9f;
const NOT_USABLE_START: usize = 0xfea0;
const NOT_USABLE_END: usize = 0xfeff;
const IO_START: usize = 0xff00;
const IO_END: usize = 0xff7f;
const HRAM_START: usize = 0xff80;
const HRAM_END: usize = 0xfffe;
const INTERRUPT_ENABLE_REG: usize = 0xffff;

const INTERRUPT_FLAG_REG: usize = 0xff0f;

/*
struct FlagRegister {
    zero: bool,       //zero flag bit 7
    add_sub: bool,    //Add/Sub flag (BCD) bit 6
    half_carry: bool, //Half carry falg (BCD) bit 5
    carry: bool,      //Carry flag bit 4
}

impl FlagRegister {
    pub fn new() -> Self {
        Self {
            zero: false,
            add_sub: false,
            half_carry: false,
            carry: false,
        }
    }
    pub fn convert_to_byte(&self) -> u8 {
        let mut flags: u8 = 0;
        if self.zero {
            flags |= F_ZERO_SET
        }
        if self.add_sub {
            flags |= F_Add_SUB_SET
        }
        if self.half_carry {
            flags |= F_HALF_CARRY_SET
        }
        if self.carry {
            flags |= F_CARRY_SET
        }
        flags
    }

    pub fn convert_to_struct(&self, byte: u8) -> Self {
        Self {
            zero: byte & F_ZERO_SET == 1,
            add_sub: byte & F_Add_SUB_SET == 1,
            half_carry: byte & F_HALF_CARRY_SET == 1,
            carry: byte & F_CARRY_SET == 1,
        }
    }
}
*/
