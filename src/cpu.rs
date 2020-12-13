pub struct Cpu {
    //registers -- note: that A is the accumulator.  All maths are done through this reg.
    // 0  1  2  3  4  5  6  7
    // B  C  D  E  H  L  F  A
    registers: [u8; 8],
    sp: usize,
    pc: usize,
    //flag: FlagRegister,
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

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: [0; 8],
            sp: 0xfffe,
            pc: 0x0100,
            memory: [0; 0xffff],
        }
    }
    //Write 8 bit register with value n
    pub fn write_reg8(&mut self, y: usize, n: u8) {
        self.registers[y] = n;
    }

    //Copy value from one 8 bit reg to another 8 bit reg
    pub fn write_reg8_with_reg8(&mut self, y: usize, z: usize) {
        self.registers[y] = self.registers[z];
    }

    //Read 8 bit register
    pub fn read_reg8(&self, y: usize) -> u8 {
        self.registers[y]
    }

    //read 16 bit register
    pub fn read_reg16(&self, reg_16: Reg16bit) -> u16 {
        let reg: u16 = ((self.registers[reg_16 as usize] as u16) << 8)
            | (self.registers[reg_16 as usize + 1] as u16);
        reg
    }

    //write to 16 bit register
    pub fn write_reg16(&mut self, reg_16: Reg16bit, nn: u16) {
        let msb = ((0xff00 & nn) >> 8) as u8;
        let lsb = (0x00ff & nn) as u8;
        self.registers[reg_16 as usize] = msb;
        self.registers[reg_16 as usize + 1] = lsb;
    }

    //write two u8s to 16 bit register
    pub fn write_reg16_fast(&mut self, reg_16: Reg16bit, lsb: u8, msb: u8) {
        self.registers[reg_16 as usize] = msb;
        self.registers[reg_16 as usize + 1] = lsb;
    }

    //Read the program counter
    pub fn read_pc(&self) -> usize {
        self.pc
    }

    //Write to the stack pointer
    pub fn write_sp(&mut self, nn: usize) {
        self.sp = nn;
    }

    //Read the stack pointer
    pub fn read_sp(&mut self) -> usize {
        self.sp
    }

    //Write a byte to memory
    pub fn write_memory(&mut self, index: usize, n: u8) {
        self.memory[index] = n;
    }

    //write two bytes to memory
    pub fn write_memory_n_n(&mut self, index: usize, lsb: u8, msb: u8) {
        self.memory[index] = lsb;
        self.memory[index + 1] = msb;
    }

    //Read a byte from memory
    pub fn read_memory(&self, index: usize) -> u8 {
        self.memory[index]
    }

    //Read two bytes from memory
    pub fn read_memory_nn(&self, index: usize) -> u16 {
        let value = (self.memory[index + 1] as u16) << 8 + (self.memory[index] as u16);
        value
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
        self.write_reg16_fast(reg_16, self.memory[self.sp], self.memory[self.sp + 1]);
        self.sp = self.sp.wrapping_add(1);
    }

    //checks if register A is equal to zero and sets the zero flag in register F if true
    #[inline]
    fn check_for_zero(&mut self) {
        //check for zero
        if self.registers[Reg8bit::A as usize] == 0 {
            self.registers[Reg8bit::F as usize] |= F_ZERO_SET;
        } else {
            self.registers[Reg8bit::F as usize] &= F_ZERO_CLR;
        }
    }

    //checks if a carry occured for add with register A and sets the carry flag in register F if true
    #[inline]
    fn check_for_carry(&mut self) -> bool {
        //check for a carry
        if self.registers[Reg8bit::A as usize]
            > self.registers[Reg8bit::A as usize].wrapping_sub(0xff)
        {
            self.registers[Reg8bit::F as usize] |= F_CARRY_SET;
            true
        } else {
            self.registers[Reg8bit::F as usize] &= F_CARRY_CLR;
            false
        }
    }

    //checks if a half carry occured for add with register A and sets the half carry flag in register F if true
    #[inline]
    fn check_for_half_carry(&mut self, value: u8) {
        //mask off the bottom of both bytes add them together; then mask off the top of the result
        let half_carry = ((self.registers[Reg8bit::A as usize] & 0xf) + (value & 0xf)) & 0x10;

        //Check for a half carry
        if half_carry == 0x10 {
            self.registers[Reg8bit::F as usize] |= F_HALF_CARRY_SET;
        } else {
            self.registers[Reg8bit::F as usize] &= F_HALF_CARRY_CLR;
        }
    }

    //add a register to register a
    pub fn add_a_r(&mut self, reg_index: usize, add_carry: bool) {
        let carry = self.check_for_carry();
        self.check_for_half_carry(self.registers[reg_index]);

        //do the add
        self.registers[Reg8bit::A as usize] = self.registers[Reg8bit::A as usize]
            .wrapping_add(self.registers[reg_index])
            .wrapping_add((add_carry && carry) as u8);

        self.check_for_zero();

        //set the add/sub flag
        self.registers[Reg8bit::F as usize] &= F_Add_SUB_CLR;
    }

    //add a value from memory to register a
    pub fn add_a_hl(&mut self, add_carry: bool) {
        let carry = self.check_for_carry();
        let mem_index = self.read_reg16(Reg16bit::HL) as usize;
        self.check_for_half_carry(self.memory[mem_index]);

        //do the add
        self.registers[Reg8bit::A as usize] = self.registers[Reg8bit::A as usize]
            .wrapping_add(self.memory[mem_index])
            .wrapping_add((add_carry && carry) as u8);

        self.check_for_zero();

        //set the add/sub flag
        self.registers[Reg8bit::F as usize] &= F_Add_SUB_CLR;
    }

    //adds the next value in memory to A
    pub fn add_a_n(&mut self, add_carry: bool) {
        let carry = self.check_for_carry();
        self.check_for_half_carry(self.memory[self.pc + 1]);

        //do the add
        self.registers[Reg8bit::A as usize] = self.registers[Reg8bit::A as usize]
            .wrapping_add(self.memory[self.pc + 1])
            .wrapping_add((add_carry && carry) as u8);

        self.check_for_zero();

        //set the add/sub flag
        self.registers[Reg8bit::F as usize] &= F_Add_SUB_CLR;
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
const VRAM_END: usize = 0x9fff;
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
