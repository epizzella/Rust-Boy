use crate::opcode_table::*;

pub struct Cpu {
    //registers -- note: that A is the accumulator.  All maths are done through this reg.
    // 0  1  2  3  4  5  6  7
    // B  C  D  E  H  L  F  A
    registers: [u8; 8],
    sp: usize,
    pc: usize,
    //Memory
    memory: [u8; 0xffff],
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
        Self {
            registers: [0; 8],
            sp: 0xfffe, //Top of stack, stack grows down
            pc: 0x0100, //where rom execution starts after bootstrap
            memory: [0; 0xffff],
        }
    }

    pub fn execute_step(&mut self, unprifxed_instructions: &OpcodeTable) {
        let current_opcode = self.memory[self.pc] as usize;

        if current_opcode != 0xCB {
            self.pc += unprifxed_instructions.table[current_opcode].get_length();
            let instruction = &unprifxed_instructions.table[current_opcode];
            (instruction.handler)(&instruction, self);
        } else {
            println!("Opcode 0xCD not impelented");
        }
    }

    //Write 8 bit register with value n
    #[inline]
    pub fn write_reg8(&mut self, y: usize, n: u8) {
        self.registers[y] = n;
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
        let reg: u16 =
            ((self.registers[reg_16] as u16) << 8) | (self.registers[reg_16 as usize + 1] as u16);
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
    pub fn read_pc(&self) -> usize {
        self.pc
    }

    //Write program counter
    #[inline]
    pub fn write_pc(&mut self, nn: usize) {
        self.pc = nn;
    }

    //Write to the stack pointer
    #[inline]
    pub fn write_sp(&mut self, nn: usize) {
        self.sp = nn;
    }

    //Read the stack pointer
    #[inline]
    pub fn read_sp(&mut self) -> usize {
        self.sp
    }

    //Write a byte to memory
    #[inline]
    pub fn write_memory(&mut self, index: usize, n: u8) {
        self.memory[index] = n;
    }

    //write two bytes to memory
    #[inline]
    pub fn write_memory_n_n(&mut self, index: usize, lsb: u8, msb: u8) {
        self.memory[index] = lsb;
        self.memory[index + 1] = msb;
    }

    //Read a byte from memory
    #[inline]
    pub fn read_memory(&self, index: usize) -> u8 {
        self.memory[index]
    }

    //Read two bytes from memory
    #[inline]
    pub fn read_memory_nn(&self, index: usize) -> u16 {
        let value = (self.memory[index + 1] as u16) << 8 + (self.memory[index] as u16);
        value
    }

    #[inline]
    pub fn get_zero_bit(&self) -> bool {
        (self.registers[Reg8bit::F as usize] & F_ZERO_SET) > 0
    }

    #[inline]
    pub fn get_carry_bit(&self) -> bool {
        (self.registers[Reg8bit::F as usize] & F_CARRY_SET) > 0
    }

    //push 16bit register onto the stack
    pub fn push_rr(&mut self, reg_16: Reg16bit) {
        self.sp = self.sp.wrapping_sub(1);
        self.memory[self.sp] = self.registers[reg_16 as usize];
        self.sp = self.sp.wrapping_sub(1);
        self.memory[self.sp] = self.registers[(reg_16 as usize) + 1];
    }

    //pop 16bit regsiter off of the stack
    pub fn pop_rr(&mut self, reg_16: Reg16bit) {
        self.sp = self.sp.wrapping_add(1);
        self.write_reg16_fast(reg_16 as usize, self.memory[self.sp], self.memory[self.sp + 1]);
        self.sp = self.sp.wrapping_add(1);
    }

    //add a register to register a
    pub fn add_a_r(&mut self, reg_index: usize, add_carry: bool) {
        let addend = self.registers[reg_index];
        let carry_flag = self.registers[Reg8bit::F as usize] & F_CARRY_SET > 0;
        let carry_value = (carry_flag && add_carry) as u8;

        //do the add
        self.registers[Reg8bit::A as usize] = self.registers[Reg8bit::A as usize]
            .wrapping_add(addend)
            .wrapping_add(carry_value); //if add_carry is true then this acts as adc instruction

        self.check_for_carry(addend, carry_value);
        self.check_for_half_carry(self.registers[Reg8bit::A as usize], addend, carry_value);
        self.check_a_for_zero();

        //set the add/sub flag
        self.registers[Reg8bit::F as usize] &= F_Add_SUB_CLR;
    }

    //add a value from memory to register a
    pub fn add_a_hl(&mut self, add_carry: bool) {
        let mem_index = self.read_reg16(Reg16bit::HL as usize) as usize;
        let addend = self.registers[mem_index];
        let carry_flag = self.registers[Reg8bit::F as usize] & F_CARRY_SET > 0;
        let carry_value = (carry_flag && add_carry) as u8;

        //do the add
        self.registers[Reg8bit::A as usize] = self.registers[Reg8bit::A as usize]
            .wrapping_add(addend)
            .wrapping_add(carry_value); //if add_carry is true then this acts as adc instruction

        self.check_for_carry(addend, carry_value);
        self.check_for_half_carry(self.registers[Reg8bit::A as usize], addend, carry_value);
        self.check_a_for_zero();

        //set the add/sub flag
        self.registers[Reg8bit::F as usize] &= F_Add_SUB_CLR;
    }

    //adds the next value in memory to A
    pub fn add_a_n(&mut self, add_carry: bool) {
        let addend = self.memory[self.pc + 1];
        let carry_flag = self.registers[Reg8bit::F as usize] & F_CARRY_SET > 0;
        let carry_value = (carry_flag && add_carry) as u8;

        //do the add
        self.registers[Reg8bit::A as usize] = self.registers[Reg8bit::A as usize]
            .wrapping_add(self.memory[self.pc + 1])
            .wrapping_add(carry_value); //if add_carry is true then this acts as adc instruction

        self.check_for_carry(addend, carry_value);
        self.check_for_half_carry(self.registers[Reg8bit::A as usize], addend, carry_value);
        self.check_a_for_zero();

        //set the add/sub flag
        self.registers[Reg8bit::F as usize] &= F_Add_SUB_CLR;
    }

    //sub a register to register A
    pub fn sub_a_r(&mut self, reg_index: usize, add_barrow: bool) {
        let subtrahend = self.registers[reg_index];
        let carry_flag = self.registers[Reg8bit::F as usize] & F_CARRY_SET > 0;
        let barrow_value = (carry_flag && add_barrow) as u8;

        //do the sub
        self.registers[Reg8bit::A as usize] = self.registers[Reg8bit::A as usize]
            .wrapping_sub(self.memory[self.pc + 1])
            .wrapping_sub(barrow_value); //if add_carry is true then this acts as sbc instruction

        self.check_for_carry_sub(subtrahend, barrow_value);
        self.check_for_half_carry_sub(self.registers[Reg8bit::A as usize], subtrahend, barrow_value);
        self.check_a_for_zero();

        self.registers[Reg8bit::F as usize] |= F_Add_SUB_SET;
    }

    //sub a register to register A
    pub fn sub_a_hl(&mut self, add_barrow: bool) {
        let mem_index = self.read_reg16(Reg16bit::HL as usize) as usize;
        let subtrahend = self.registers[mem_index];
        let carry_flag = self.registers[Reg8bit::F as usize] & F_CARRY_SET > 0;
        let barrow_value = (carry_flag && add_barrow) as u8;

        //do the sub
        self.registers[Reg8bit::A as usize] = self.registers[Reg8bit::A as usize]
            .wrapping_sub(self.memory[self.pc + 1])
            .wrapping_sub(barrow_value); //if add_carry is true then this acts as sbc instruction

        self.check_for_carry_sub(subtrahend, barrow_value);
        self.check_for_half_carry_sub(self.registers[Reg8bit::A as usize], subtrahend, barrow_value);
        self.check_a_for_zero();

        self.registers[Reg8bit::F as usize] |= F_Add_SUB_SET;
    }

    //sub a register to register A
    pub fn sub_a_n(&mut self, add_barrow: bool) {
        let subtrahend = self.memory[self.pc + 1];
        let carry_flag = self.registers[Reg8bit::F as usize] & F_CARRY_SET > 0;
        let barrow_value = (carry_flag && add_barrow) as u8;

        //do the sub
        self.registers[Reg8bit::A as usize] = self.registers[Reg8bit::A as usize]
            .wrapping_sub(self.memory[self.pc + 1])
            .wrapping_sub(barrow_value); //if add_carry is true then this acts as sbc instruction

        self.check_for_carry_sub(subtrahend, barrow_value);
        self.check_for_half_carry_sub(self.registers[Reg8bit::A as usize], subtrahend, barrow_value);
        self.check_a_for_zero();

        self.registers[Reg8bit::F as usize] |= F_Add_SUB_SET;
    }

    //and a register with Regsiter A
    pub fn and_a_r(&mut self, reg_index: usize) {
        self.registers[Reg8bit::A as usize] &= self.registers[reg_index];

        self.check_a_for_zero();
        self.registers[Reg8bit::F as usize] &= F_Add_SUB_CLR;
        self.registers[Reg8bit::F as usize] |= F_HALF_CARRY_SET;
        self.registers[Reg8bit::F as usize] &= F_CARRY_CLR;
    }

    pub fn and_a_n(&mut self) {
        self.registers[Reg8bit::A as usize] &= self.registers[self.pc + 1];

        self.check_a_for_zero();
        self.registers[Reg8bit::F as usize] &= F_Add_SUB_CLR;
        self.registers[Reg8bit::F as usize] |= F_HALF_CARRY_SET;
        self.registers[Reg8bit::F as usize] &= F_CARRY_CLR;
    }

    pub fn and_a_hl(&mut self) {
        let reg_index = self.read_reg16(Reg16bit::HL as usize) as usize;
        self.registers[Reg8bit::A as usize] &= self.registers[reg_index];

        self.check_a_for_zero();
        self.registers[Reg8bit::F as usize] &= F_Add_SUB_CLR;
        self.registers[Reg8bit::F as usize] |= F_HALF_CARRY_SET;
        self.registers[Reg8bit::F as usize] &= F_CARRY_CLR;
    }

    //XOR Functions
    pub fn xor_a_r(&mut self, reg_index: usize) {
        self.registers[Reg8bit::A as usize] ^= self.registers[reg_index];

        self.check_a_for_zero();
        self.registers[Reg8bit::F as usize] &= F_Add_SUB_CLR;
        self.registers[Reg8bit::F as usize] &= F_HALF_CARRY_CLR;
        self.registers[Reg8bit::F as usize] &= F_CARRY_CLR;
    }

    pub fn xor_a_n(&mut self) {
        self.registers[Reg8bit::A as usize] ^= self.registers[self.pc + 1];

        self.check_a_for_zero();
        self.registers[Reg8bit::F as usize] &= F_Add_SUB_CLR;
        self.registers[Reg8bit::F as usize] &= F_HALF_CARRY_CLR;
        self.registers[Reg8bit::F as usize] &= F_CARRY_CLR;
    }

    pub fn xor_a_hl(&mut self) {
        let reg_index = self.read_reg16(Reg16bit::HL as usize) as usize;
        self.registers[Reg8bit::A as usize] ^= self.registers[reg_index];

        self.check_a_for_zero();
        self.registers[Reg8bit::F as usize] &= F_Add_SUB_CLR;
        self.registers[Reg8bit::F as usize] &= F_HALF_CARRY_CLR;
        self.registers[Reg8bit::F as usize] &= F_CARRY_CLR;
    }

    //OR functions
    pub fn or_a_r(&mut self, reg_index: usize) {
        self.registers[Reg8bit::A as usize] |= self.registers[reg_index];

        self.check_a_for_zero();
        self.registers[Reg8bit::F as usize] &= F_Add_SUB_CLR;
        self.registers[Reg8bit::F as usize] &= F_HALF_CARRY_CLR;
        self.registers[Reg8bit::F as usize] &= F_CARRY_CLR;
    }

    pub fn or_a_n(&mut self) {
        self.registers[Reg8bit::A as usize] |= self.registers[self.pc + 1];

        self.check_a_for_zero();
        self.registers[Reg8bit::F as usize] &= F_Add_SUB_CLR;
        self.registers[Reg8bit::F as usize] &= F_HALF_CARRY_CLR;
        self.registers[Reg8bit::F as usize] &= F_CARRY_CLR;
    }

    pub fn or_a_hl(&mut self) {
        let reg_index = self.read_reg16(Reg16bit::HL as usize) as usize;
        self.registers[Reg8bit::A as usize] |= self.registers[reg_index];

        self.check_a_for_zero();
        self.registers[Reg8bit::F as usize] &= F_Add_SUB_CLR;
        self.registers[Reg8bit::F as usize] &= F_HALF_CARRY_CLR;
        self.registers[Reg8bit::F as usize] &= F_CARRY_CLR;
    }

    //Increment register
    pub fn increment_r(&mut self, y: usize) {
        self.registers[y] = self.registers[y].wrapping_add(1);

        self.check_a_for_zero();
        self.registers[Reg8bit::F as usize] &= F_Add_SUB_CLR;
        self.check_for_half_carry(self.registers[y], 1, 0);
    }

    //Increment memory
    pub fn increment_memory(&mut self, memory_index: usize) {
        self.memory[memory_index] = self.memory[memory_index].wrapping_add(1);

        self.check_value_for_zero(self.memory[memory_index]);
        self.registers[Reg8bit::F as usize] &= F_Add_SUB_CLR;
        self.check_for_half_carry(self.memory[memory_index], 1, 0);
    }

    //Decrement register
    pub fn decrement_r(&mut self, y: usize) {
        self.registers[y] = self.registers[y].wrapping_sub(1);

        self.check_a_for_zero();
        self.registers[Reg8bit::F as usize] |= F_Add_SUB_SET;
        self.check_for_half_carry_sub(self.registers[y], 1, 0);
    }

    //Decrement memory
    pub fn decrement_memory(&mut self, memory_index: usize) {
        self.memory[memory_index] = self.memory[memory_index].wrapping_sub(1);

        self.check_value_for_zero(self.memory[memory_index]);
        self.registers[Reg8bit::F as usize] |= F_Add_SUB_SET;
        self.check_for_half_carry_sub(self.memory[memory_index], 1, 0);
    }

    //flip the bits in register A
    pub fn complement_a(&mut self) {
        self.registers[Reg8bit::A as usize] ^= 0xff;
        self.registers[Reg8bit::F as usize] |= F_Add_SUB_SET;
        self.registers[Reg8bit::F as usize] |= F_HALF_CARRY_SET;
    }

    pub fn add_hl_rr(&mut self, register_index: usize) {
        let hl = self.read_reg16(Reg16bit::HL as usize);
        let addend = self.read_reg16(register_index);
        let sum = hl.wrapping_add(addend);

        self.write_reg16(Reg16bit::HL as usize, sum);

        self.check_for_carry_16bit(addend);
        self.check_for_half_carry_16bit(hl, addend);
        self.registers[Reg8bit::F as usize] &= F_Add_SUB_CLR;
    }

    pub fn add_hl_sp(&mut self) {
        let hl = self.read_reg16(Reg16bit::HL as usize);
        let addend = self.sp as u16;
        let sum = hl.wrapping_add(addend);

        self.write_reg16(Reg16bit::HL as usize, sum);

        self.check_for_carry_16bit(addend);
        self.check_for_half_carry_16bit(hl, addend);
        self.registers[Reg8bit::F as usize] &= F_Add_SUB_CLR;
    }

    pub fn increment_sp(&mut self) {
        self.sp = self.sp.wrapping_add(1);
    }

    pub fn decrement_sp(&mut self) {
        self.sp = self.sp.wrapping_sub(1);
    }

    pub fn add_sp_dd(&mut self) -> u16 {
        let addend = (self.memory[self.pc + 1] as i8) as i16; //this is a signed number
        let sum = (self.sp as i16).wrapping_add(addend) as u16; // do the math

        let z = (self.sp as i16).checked_add(addend);
        match z {
            Some(_) => {
                self.registers[Reg8bit::F as usize] &= F_CARRY_CLR;
            }
            None => {
                self.registers[Reg8bit::F as usize] |= F_CARRY_SET;
            }
        }

        if (sum & 0xf) < (self.sp as u16 & 0xf) {
            self.registers[Reg8bit::F as usize] |= F_HALF_CARRY_SET;
        } else {
            self.registers[Reg8bit::F as usize] |= F_HALF_CARRY_CLR;
        }

        self.registers[Reg8bit::F as usize] &= F_ZERO_CLR;
        self.registers[Reg8bit::F as usize] &= F_Add_SUB_CLR;

        sum
    }

    //Rotates register A to the left with bit 7 being moved to bit 0 and also stored into carry
    pub fn rlca(&mut self) {
        if self.registers[Reg8bit::A as usize] > 0x7f {
            self.set_carry_bit()
        } else {
            self.clear_carry_bit()
        }

        self.registers[Reg8bit::A as usize] = self.registers[Reg8bit::A as usize].wrapping_shl(1);

        self.registers[Reg8bit::F as usize] &= F_ZERO_CLR;
        self.registers[Reg8bit::F as usize] &= F_Add_SUB_CLR;
        self.registers[Reg8bit::F as usize] &= F_HALF_CARRY_CLR;
    }

    //Rotates register A to the left with the carry's value put into bit 0 and bit 7 is put into the carry.
    pub fn rla(&mut self) {
        let v: (u8, bool);
        let carry = self.get_carry_bit();

        //do the rotation
        v = self.registers[Reg8bit::A as usize].overflowing_shl(1);
        self.registers[Reg8bit::A as usize] = v.0 | carry as u8; //put the carry bit in position 0

        //check if we carried
        if v.1 {
            self.set_carry_bit();
        } else {
            self.clear_carry_bit();
        }

        self.registers[Reg8bit::F as usize] &= F_ZERO_CLR;
        self.registers[Reg8bit::F as usize] &= F_Add_SUB_CLR;
        self.registers[Reg8bit::F as usize] &= F_HALF_CARRY_CLR;
    }

    //Rotates register A to the right with bit 0 being moved to bit 7 and also stored into carry
    pub fn rrca(&mut self) {
        if self.registers[Reg8bit::A as usize] & 0x01 > 0 {
            self.set_carry_bit()
        } else {
            self.clear_carry_bit()
        }
        self.registers[Reg8bit::A as usize] = self.registers[Reg8bit::A as usize].wrapping_shr(1);

        self.registers[Reg8bit::F as usize] &= F_ZERO_CLR;
        self.registers[Reg8bit::F as usize] &= F_Add_SUB_CLR;
        self.registers[Reg8bit::F as usize] &= F_HALF_CARRY_CLR;
    }

    //Rotates register A to the right with the carry's value put into bit 7 and bit 0 is put into the carry.
    pub fn rra(&mut self) {
        let v: (u8, bool);
        let carry = self.get_carry_bit();

        //do the rotation
        v = self.registers[Reg8bit::A as usize].overflowing_shr(1);
        self.registers[Reg8bit::A as usize] = v.0 | ((carry as u8) << 7); //put the carry bit in position 7

        //check if we carried
        if v.1 {
            self.set_carry_bit();
        } else {
            self.clear_carry_bit();
        }

        self.registers[Reg8bit::F as usize] &= F_ZERO_CLR;
        self.registers[Reg8bit::F as usize] &= F_Add_SUB_CLR;
        self.registers[Reg8bit::F as usize] &= F_HALF_CARRY_CLR;
    }

    //Carry bit is xored and saved to itself
    pub fn ccf(&mut self) {
        let carry_flag = self.get_carry_bit();

        if carry_flag {
            self.clear_carry_bit()
        } else {
            self.set_carry_bit();
        }

        self.registers[Reg8bit::F as usize] &= F_Add_SUB_CLR;
        self.registers[Reg8bit::F as usize] &= F_HALF_CARRY_CLR;
    }

    //Sets the carry bit
    pub fn scf(&mut self) {
        self.set_carry_bit();
    }
}

//Privat methods
impl Cpu {
    #[inline]
    fn set_carry_bit(&mut self) {
        self.registers[Reg8bit::F as usize] |= F_CARRY_SET;
    }

    #[inline]
    fn clear_carry_bit(&mut self) {
        self.registers[Reg8bit::F as usize] &= F_CARRY_CLR;
    }

    //checks if register A is equal to zero and sets the zero flag in register F if true
    #[inline]
    fn check_a_for_zero(&mut self) {
        //check for zero
        if self.registers[Reg8bit::A as usize] == 0 {
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
    fn check_for_carry(&mut self, value: u8, carry: u8) {
        let z = self.registers[Reg8bit::A as usize]
            .checked_add(value)
            .and_then(|x| x.checked_add(carry));

        match z {
            Some(_) => {
                self.registers[Reg8bit::F as usize] &= F_CARRY_CLR;
            }
            None => {
                self.registers[Reg8bit::F as usize] |= F_CARRY_SET;
            }
        }
    }

    //checks if a carry occured for add with register A and sets the carry flag in register F if true
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

    //checks if a half carry occured for add with register A and sets the half carry flag in register F if true
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

    //checks if a half carry occured for add with register A and sets the half carry flag in register F if true
    #[inline]
    fn check_for_half_carry_16bit(&mut self, reg: u16, addend: u16) {
        //mask off the bottom of both numbers add them together; then mask off the top of the result
        let half_carry = (reg & 0xff) + (addend & 0xff);

        //Check for a half carry
        if half_carry > 0xff {
            self.registers[Reg8bit::F as usize] |= F_HALF_CARRY_SET;
        } else {
            self.registers[Reg8bit::F as usize] &= F_HALF_CARRY_CLR;
        }
    }
}

const F_ZERO_SET: u8 = 0x80; //1000 0000
const F_Add_SUB_SET: u8 = 0x40; //0100 0000
const F_HALF_CARRY_SET: u8 = 0x20; //0010 0000
const F_CARRY_SET: u8 = 0x10; //0001 0000

const F_ZERO_CLR: u8 = 0x7f; //0111 1111
const F_Add_SUB_CLR: u8 = 0xbf; //1011 1111
const F_HALF_CARRY_CLR: u8 = 0xdf; //1101 1111
const F_CARRY_CLR: u8 = 0xef; //1110 1111

const BANK_00_START: usize = 0x0000;
const BANK_00_END: usize = 0x3fff;
const BANK_01_START: usize = 0x4000;
const BANK_01_END: usize = 0x7fff;
const VRAM_START: usize = 0x8000;
pub const VRAM_END: usize = 0x9fff;
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
