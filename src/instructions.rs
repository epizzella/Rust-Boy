use crate::cpu::*;

const Y_REG_MASK: u8 = 0x38;
const Z_REG_MASK: u8 = 0x07;
const P_REG_MASK: u8 = 0x30;
const LDH_ADDR_MSB_MASK: usize = 0xff00;

pub struct Opcode {
    opcode_byte: u8, //hex representatoin of the opcode
    opcode_name: String,
    number_of_cycles: u8,
    increase_pc_by: u8, //in bytes
    handler: fn(&Self, &mut Cpu),
}

//For information on the implementation of these opcodes please see Chapter 2 of Game Boy: Complete Technical Reference

/**********8 bit load instructions**********/
impl Opcode {
    pub fn new(
        opcode: u8,
        opcode_name: String,
        cycles: u8,
        length: u8,
        function: fn(&Self, &mut Cpu),
    ) -> Self {
        let test_opcode = Self {
            //error here
            opcode_name: opcode_name,
            opcode_byte: opcode,
            number_of_cycles: cycles,
            increase_pc_by: length,
            handler: function,
        };
        test_opcode
    }
    //8-bit load instructions transfer one byte of data between two 8-bit registers
    //0b01yyyzzz
    pub fn load_r_r(&self, cpu: &mut Cpu) {
        let register_y: usize = ((self.opcode_byte & Y_REG_MASK) >> 3) as usize;
        let register_z: usize = (self.opcode_byte & Z_REG_MASK) as usize;
        cpu.write_reg8_with_reg8(register_y, register_z);
    }

    //Load to the 8-bit register r, the immediate data n.
    //0b00yyy110
    pub fn load_r_n(&self, cpu: &mut Cpu) {
        let register: usize = ((self.opcode_byte & Y_REG_MASK) >> 3) as usize;
        let pc: usize = cpu.read_pc();
        cpu.write_reg8(register, cpu.read_memory(pc + 1));
    }

    //Load to the 8-bit register r, data from the absolute address specified by the 16-bit register HL.
    //0b01yyy110
    pub fn load_r_hl(&self, cpu: &mut Cpu) {
        let register: usize = ((self.opcode_byte & Y_REG_MASK) >> 3) as usize;
        let index = cpu.read_reg16(Reg16bit::HL as usize) as usize;
        cpu.write_reg8(register, cpu.read_memory(index));
    }

    //Load to the absolute address specified by the 16-bit register HL, data from the 8-bit register r.
    //0b01110zzz
    pub fn load_hl_r(&self, cpu: &mut Cpu) {
        let register_z: usize = (self.opcode_byte & Z_REG_MASK) as usize;
        let index = cpu.read_reg16(Reg16bit::HL as usize) as usize;
        cpu.write_memory(index, cpu.read_reg8(register_z));
    }

    //Load to the absolute address specified by the 16-bit register HL, the immediate data n.
    //0b00110110/0x36 + n
    pub fn load_hl_n(&self, cpu: &mut Cpu) {
        let pc: usize = cpu.read_pc();
        let index = cpu.read_reg16(Reg16bit::HL as usize) as usize;
        cpu.write_memory(index, cpu.read_memory(pc + 1));
    }

    //Load to the 8-bit A register, data from the absolute address specified by the 16-bit register BC.
    //0b00001010/0x0A
    pub fn load_a_bc(&self, cpu: &mut Cpu) {
        let index = cpu.read_reg16(Reg16bit::BC as usize) as usize;
        cpu.write_reg8(Reg8bit::A as usize, cpu.read_memory(index));
    }

    //Load to the 8-bit A register, data from the absolute address specified by the 16-bit register DE.
    //0b00011010/0x1A
    pub fn load_a_de(&self, cpu: &mut Cpu) {
        let index = cpu.read_reg16(Reg16bit::DE as usize) as usize;
        cpu.write_reg8(Reg8bit::A as usize, cpu.read_memory(index));
    }

    //Load to the absolute address specified by the 16-bit register BC, data from the 8-bit A register.
    //0b00000010/0x02
    pub fn load_bc_a(&self, cpu: &mut Cpu) {
        let index = cpu.read_reg16(Reg16bit::BC as usize) as usize;
        cpu.write_memory(index, cpu.read_reg8(Reg8bit::A as usize));
    }

    //Load to the absolute address specified by the 16-bit register DE, data from the 8-bit A register
    //0b00010010/0x12
    pub fn load_de_a(&self, cpu: &mut Cpu) {
        let index = cpu.read_reg16(Reg16bit::DE as usize) as usize;
        cpu.write_memory(index, cpu.read_reg8(Reg8bit::A as usize));
    }

    //Load to the 8-bit A register, data from the absolute address specified by the 16-bit operand nn.
    //0b11111010/0xFA + LSB of nn + MSB of nn
    pub fn load_a_nn(&self, cpu: &mut Cpu) {
        let pc: usize = cpu.read_pc();
        let mut index = cpu.read_memory(pc + 2) as usize;
        index <<= 8;
        index |= cpu.read_memory(pc + 1) as usize;
        cpu.write_reg8(Reg8bit::A as usize, cpu.read_memory(index));
    }

    //Load to the absolute address specified by the 16-bit operand nn, data from the 8-bit A register.
    //0b11101010/0xEA + LSB of nn + MSB of nn
    pub fn load_nn_a(&self, cpu: &mut Cpu) {
        let pc: usize = cpu.read_pc();
        let mut index = cpu.read_memory(pc + 2) as usize;
        index <<= 8;
        index |= (cpu.read_memory(pc + 1)) as usize;
        cpu.write_memory(index, cpu.read_reg8(Reg8bit::A as usize));
    }

    //Load to the 8-bit A register, data from the address specified by the 8-bit C register. The full 16-bit absolute
    //address is obtained by setting the most significant byte to 0xFF and the least significant byte to the value of C,
    //so the possible range is 0xFF00-0xFFFF.
    //0b11110010/0xF2
    pub fn ldh_a_c(&self, cpu: &mut Cpu) {
        let mut index = LDH_ADDR_MSB_MASK | cpu.read_reg8(Reg8bit::C as usize) as usize;
        cpu.write_reg8(Reg8bit::A as usize, cpu.read_memory(index));
    }

    //Load to the address specified by the 8-bit C register, data from the 8-bit A register. The full 16-bit absolute
    //address is obtained by setting the most significant byte to 0xFF and the least significant byte to the value of C,
    //so the possible range is 0xFF00-0xFFFF.
    //0b11100010/0xE2
    pub fn ldh_c_a(&self, cpu: &mut Cpu) {
        let mut index = LDH_ADDR_MSB_MASK | cpu.read_reg8(Reg8bit::C as usize) as usize;
        cpu.write_memory(index, cpu.read_reg8(Reg8bit::A as usize));
    }

    //Load to the 8-bit A register, data from the address specified by the 8-bit immediate data n. The full 16-bit
    //absolute address is obtained by setting the most significant byte to 0xFF and the least significant byte to the
    //value of n, so the possible range is 0xFF00-0xFFFF.
    //0b11110000/0xF0
    pub fn ldh_a_n(&self, cpu: &mut Cpu) {
        let pc: usize = cpu.read_pc();
        let mut index = LDH_ADDR_MSB_MASK | cpu.read_memory(pc + 1) as usize;
        cpu.write_reg8(Reg8bit::A as usize, cpu.read_memory(index));
    }

    //Load to the address specified by the 8-bit immediate data n, data from the 8-bit A register. The full 16-bit
    //absolute address is obtained by setting the most significant byte to 0xFF and the least significant byte to the
    //value of n, so the possible range is 0xFF00-0xFFFF.
    //0b11100000/0xE0
    pub fn ldh_n_a(&self, cpu: &mut Cpu) {
        let pc: usize = cpu.read_pc();
        let mut index = LDH_ADDR_MSB_MASK | cpu.read_memory(pc + 1) as usize;
        cpu.write_memory(index, cpu.read_memory(pc + 1));
    }

    //Load to the 8-bit A register, data from the absolute address specified by the 16-bit register HL. The value of
    //HL is decremented after the memory read.
    //0b00111010/0x3A
    pub fn load_a_hl_dec(&self, cpu: &mut Cpu) {
        let index = cpu.read_reg16(Reg16bit::HL as usize) as usize;
        cpu.write_reg8(Reg8bit::A as usize, cpu.read_memory(index));
        cpu.write_reg16(Reg16bit::HL as usize, index as u16 - 1);
    }

    //Load to the absolute address specified by the 16-bit register HL, data from the 8-bit A register. The value of
    //HL is decremented after the memory write.
    //0b00110010/0x32
    pub fn load_hl_a_dec(&self, cpu: &mut Cpu) {
        let index = cpu.read_reg16(Reg16bit::HL as usize) as usize;
        cpu.write_memory(index, cpu.read_reg8(Reg8bit::A as usize));
        cpu.write_reg16(Reg16bit::HL as usize, index as u16 - 1);
    }

    //Load to the 8-bit A register, data from the absolute address specified by the 16-bit register HL. The value of
    //HL is incremented after the memory read.
    //0b00101010/0x2A
    pub fn load_a_hl_inc(&self, cpu: &mut Cpu) {
        let index = cpu.read_reg16(Reg16bit::HL as usize) as usize;
        cpu.write_reg8(Reg8bit::A as usize, cpu.read_memory(index));
        cpu.write_reg16(Reg16bit::HL as usize, index as u16 + 1);
    }

    //Load to the absolute address specified by the 16-bit register HL, data from the 8-bit A register. The value of
    //HL is incremented after the memory write.
    //0b00100010/0x22
    pub fn load_hl_a_inc(&self, cpu: &mut Cpu) {
        let index = cpu.read_reg16(Reg16bit::HL as usize) as usize;
        cpu.write_memory(index, cpu.read_reg8(Reg8bit::A as usize));
        cpu.write_reg16(Reg16bit::HL as usize, index as u16 + 1);
    }

    /**********16 bit load instructions**********/

    //note about 16 bit registers:  Look into rust's to_bytes and from_bytes functions to converting two u8s to and from u16
    //also, figure out how rusts to_be from_be know endian of an mcu

    //16-bit load instructions transfer two bytes of data between one 16-bit register
    //and two sequential locations in memory.
    //0b00000001 + LSB of nn + MSB of nn
    pub fn load_bc_nn(&self, cpu: &mut Cpu) {
        let pc = cpu.read_pc();
        cpu.write_reg16_fast(
            Reg16bit::BC as usize,
            cpu.read_memory(pc),
            cpu.read_memory(pc + 1),
        )
    }

    //16-bit load instructions transfer two bytes of data between one 16-bit register
    //and two sequential locations in memory.
    //0b00010001 + LSB of nn + MSB of nn
    pub fn load_de_nn(&self, cpu: &mut Cpu) {
        let pc = cpu.read_pc();
        cpu.write_reg16_fast(
            Reg16bit::DE as usize,
            cpu.read_memory(pc),
            cpu.read_memory(pc + 1),
        )
    }

    //16-bit load instructions transfer two bytes of data between one 16-bit register
    //and two sequential locations in memory.
    //0b00100001 + LSB of nn + MSB of nn
    pub fn load_hl_nn(&self, cpu: &mut Cpu) {
        let pc = cpu.read_pc();
        cpu.write_reg16_fast(
            Reg16bit::HL as usize,
            cpu.read_memory(pc),
            cpu.read_memory(pc + 1),
        )
    }

    //16-bit load instructions transfer two bytes of data between one 16-bit register
    //and two sequential locations in memory.
    //0b00110001 + LSB of nn + MSB of nn
    pub fn load_sp_nn(&self, cpu: &mut Cpu) {
        let pc = cpu.read_pc();
        let mut new_sp = cpu.read_memory(pc + 1) as usize;
        new_sp <<= 8;
        new_sp += cpu.read_memory(pc) as usize;
        cpu.write_sp(new_sp);
    }

    //Load to the absolute address specified by the 16-bit operand nn, data from the 16-bit SP register.
    //0b00001000/0x08 + LSB of nn + MSB of nn
    pub fn load_nn_sp(&self, cpu: &mut Cpu) {
        let pc = cpu.read_pc();
        let index = cpu.read_memory_nn(pc) as usize;

        let mut sp_lsb = cpu.read_sp();
        let sp_msb = sp_lsb >> 8;
        sp_lsb &= 0x00ff;

        cpu.write_memory_n_n(index, sp_lsb as u8, sp_msb as u8);
    }

    //Load to the 16-bit SP register, data from the 16-bit HL register.
    //0b11111001/0xF9
    pub fn load_sp_hl(&self, cpu: &mut Cpu) {
        cpu.write_sp(cpu.read_reg16(Reg16bit::HL as usize) as usize);
    }

    //Push to the stack memory, data from the 16-bit register BC.
    //0b11000101
    pub fn push_bc(&self, cpu: &mut Cpu) {
        cpu.push_rr(Reg16bit::BC);
    }

    //Push to the stack memory, data from the 16-bit register DE.
    //0b11010101
    pub fn push_de(&self, cpu: &mut Cpu) {
        cpu.push_rr(Reg16bit::DE);
    }

    //Push to the stack memory, data from the 16-bit register HL.
    //0b11100101
    pub fn push_hl(&self, cpu: &mut Cpu) {
        cpu.push_rr(Reg16bit::HL);
    }

    //Push to the stack memory, data from the 16-bit register AF.
    //0b11100101
    pub fn push_af(&self, cpu: &mut Cpu) {
        cpu.push_rr(Reg16bit::AF);
    }

    //Pops to the 16-bit register BD, data from the stack memory.
    //0b11000001
    pub fn pop_bc(&self, cpu: &mut Cpu) {
        cpu.pop_rr(Reg16bit::BC);
    }

    //Pops to the 16-bit register DE, data from the stack memory.
    //0b11010001
    pub fn pop_de(&self, cpu: &mut Cpu) {
        cpu.pop_rr(Reg16bit::DE);
    }

    //Pops to the 16-bit register HL, data from the stack memory.
    //0b11100001
    pub fn pop_hl(&self, cpu: &mut Cpu) {
        cpu.pop_rr(Reg16bit::HL);
    }

    //Pops to the 16-bit register AF, data from the stack memory.
    //0b11110001
    pub fn pop_af(&self, cpu: &mut Cpu) {
        cpu.pop_rr(Reg16bit::AF);
    }

    /********** 8bit-Arithmetic/logical Commands **********/

    //Add to the 8-bit register A, data from register zzz
    //0b10000zzz
    pub fn add_a_r(&self, cpu: &mut Cpu) {
        let register_z: usize = (self.opcode_byte & Z_REG_MASK) as usize;
        cpu.add_a_r(register_z as usize, false);
    }

    //Add to the 8-bit register A, the immediate data n
    //0b11000110
    pub fn add_a_n(&self, cpu: &mut Cpu) {
        cpu.add_a_n(false);
    }

    //Add to the 8-bit register A, data from the absolute address specified by the 16-bit register HL.
    //10000110
    pub fn add_a_hl(&self, cpu: &mut Cpu) {
        cpu.add_a_hl(false);
    }

    //Add to the 8-bit register A, data from register zzz
    //0b10000zzz
    pub fn adc_a_r(&self, cpu: &mut Cpu) {
        let register_z: usize = (self.opcode_byte & Z_REG_MASK) as usize;
        cpu.add_a_r(register_z as usize, true);
    }

    //Add to the 8-bit register A, the immediate data n
    //0b11001110
    pub fn adc_a_n(&self, cpu: &mut Cpu) {
        cpu.add_a_n(true);
    }

    //Add to the 8-bit register A, data from the absolute address specified by the 16-bit register HL.
    //10001110
    pub fn adc_a_hl(&self, cpu: &mut Cpu) {
        cpu.add_a_hl(true);
    }

    //Sub from the 8-bit register A, data from register zzz
    //0b10010zzz
    pub fn sub_a_r(&self, cpu: &mut Cpu) {
        let register_z: usize = (self.opcode_byte & Z_REG_MASK) as usize;
        cpu.sub_a_r(register_z as usize, false);
    }

    //Sub from the 8-bit register A, the immediate data n
    //0b11010111
    pub fn sub_a_n(&self, cpu: &mut Cpu) {
        cpu.sub_a_n(false);
    }

    //Sub from the 8-bit register A, data from the absolute address specified by the 16-bit register HL.
    //0b10010110
    pub fn sub_a_hl(&self, cpu: &mut Cpu) {
        cpu.sub_a_hl(false);
    }

    //Sub from the 8-bit register A, data from register zzz
    //0b10010zzz
    pub fn sbc_a_r(&self, cpu: &mut Cpu) {
        let register_z: usize = (self.opcode_byte & Z_REG_MASK) as usize;
        cpu.sub_a_r(register_z as usize, true);
    }

    //Sub from the 8-bit register A, the immediate data n
    //0b11011110
    pub fn sbc_a_n(&self, cpu: &mut Cpu) {
        cpu.sub_a_n(true);
    }

    //Sub from the 8-bit register A, data from the absolute address specified by the 16-bit register HL.
    //0b10011110
    pub fn sbc_a_hl(&self, cpu: &mut Cpu) {
        cpu.sub_a_hl(true);
    }

    //And to the 8-bit register A, data from register zzz
    //0b10100zzz
    pub fn and_a_r(&self, cpu: &mut Cpu) {
        let register_z: usize = (self.opcode_byte & Z_REG_MASK) as usize;
        cpu.and_a_r(register_z);
    }

    //And to the 8-bit register A, the immediate data n
    //0b11100zzz
    pub fn and_a_n(&self, cpu: &mut Cpu) {
        cpu.and_a_n();
    }

    //And to the 8-bit register A, data from the absolute address specified by the 16-bit register HL.
    //0b10100110
    pub fn and_a_hl(&self, cpu: &mut Cpu) {
        cpu.and_a_hl();
    }

    //Xor to the 8-bit register A, data from register zzz
    //0b10101zzz
    pub fn xor_a_r(&self, cpu: &mut Cpu) {
        let register_z: usize = (self.opcode_byte & Z_REG_MASK) as usize;
        cpu.xor_a_r(register_z);
    }

    //Xor to the 8-bit register A, the immediate data n
    //0b11101110
    pub fn xor_a_n(&self, cpu: &mut Cpu) {
        cpu.xor_a_n();
    }

    //Xor to the 8-bit register A, data from the absolute address specified by the 16-bit register HL.
    //0b10101110
    pub fn xor_a_hl(&self, cpu: &mut Cpu) {
        cpu.xor_a_hl();
    }

    //Or to the 8-bit register A, data from register zzz
    //0b10111zzz
    pub fn or_a_r(&self, cpu: &mut Cpu) {
        let register_z: usize = (self.opcode_byte & Z_REG_MASK) as usize;
        cpu.or_a_r(register_z);
    }

    //Or to the 8-bit register A, the immediate data n
    //0b11110110
    pub fn or_a_n(&self, cpu: &mut Cpu) {
        cpu.or_a_n();
    }

    //Or to the 8-bit register A, data from the absolute address specified by the 16-bit register HL.
    //10110110
    pub fn or_a_hl(&self, cpu: &mut Cpu) {
        cpu.or_a_hl();
    }

    //Compare to the 8-bit register A, data from register zzz
    pub fn cp_a_r(&self, cpu: &mut Cpu) {
        let value = cpu.read_reg8(Reg8bit::A as usize);
        let register_z: usize = (self.opcode_byte & Z_REG_MASK) as usize;
        cpu.sub_a_r(register_z as usize, false);
        cpu.write_reg8(Reg8bit::A as usize, value);
    }

    //Compare to the 8-bit register A, the immediate data n
    pub fn cp_a_n(&self, cpu: &mut Cpu) {
        let value = cpu.read_reg8(Reg8bit::A as usize);
        cpu.sub_a_n(false);
        cpu.write_reg8(Reg8bit::A as usize, value);
    }

    //Compare the 8-bit register A, data from the absolute address specified by the 16-bit register HL.
    pub fn cp_a_hl(&self, cpu: &mut Cpu) {
        let value = cpu.read_reg8(Reg8bit::A as usize);
        cpu.sub_a_hl(false);
        cpu.write_reg8(Reg8bit::A as usize, value);
    }

    //Incerement register r by 1
    pub fn inc_r(&self, cpu: &mut Cpu) {
        let register_y: usize = ((self.opcode_byte & Y_REG_MASK) >> 3) as usize;
        cpu.increment_r(register_y);
    }

    //Increment data from the absolute address specified by the 16-bit register HL by 1.
    //0b00110100
    pub fn inc_hl(&self, cpu: &mut Cpu) {
        cpu.increment_memory(cpu.read_reg16(Reg16bit::HL as usize) as usize);
    }

    //Decerement register r by 1
    pub fn dec_r(&self, cpu: &mut Cpu) {
        let register_y: usize = ((self.opcode_byte & Y_REG_MASK) >> 3) as usize;
        cpu.decrement_r(register_y);
    }

    //Decrement data from the absolute address specified by the 16-bit register HL by 1.
    //0b00110101
    pub fn dec_hl(&self, cpu: &mut Cpu) {
        cpu.decrement_memory(cpu.read_reg16(Reg16bit::HL as usize) as usize);
    }

    //DAA
    //0b00100111
    pub fn daa(&self, cpu: &mut Cpu) {
        //to be figured out later
    }

    //Complement register A
    pub fn cpl(&self, cpu: &mut Cpu) {
        cpu.complement_a();
    }

    /********** 16bit-Arithmetic/logical Commands **********/

    //Add to the 16-bit register HL, data from register pp
    //0b00pp1001
    pub fn add_hl_rr(&self, cpu: &mut Cpu) {
        let register_p = (self.opcode_byte & P_REG_MASK >> 4) as usize;
        cpu.add_hl_rr(register_p);
    }

    //Add to the 16-bit register HL, data from register SP
    //0b00111001
    pub fn add_hl_sp(&self, cpu: &mut Cpu) {
        cpu.add_hl_sp();
    }

    //Incerement register rr by 1
    pub fn inc_rr(&self, cpu: &mut Cpu) {
        let register_p = (self.opcode_byte & P_REG_MASK >> 4) as usize;
        cpu.write_reg16(register_p, cpu.read_reg16(register_p).wrapping_add(1));
    }

    //Incerement register sp by 1
    pub fn inc_sp(&self, cpu: &mut Cpu) {
        cpu.increment_sp();
    }

    //Decerement register rr by 1
    pub fn dec_rr(&self, cpu: &mut Cpu) {
        let register_p = (self.opcode_byte & P_REG_MASK >> 4) as usize;
        cpu.write_reg16(register_p, cpu.read_reg16(register_p).wrapping_sub(1));
    }

    //Decerement register sp by 1
    pub fn dec_sp(&self, cpu: &mut Cpu) {
        cpu.decrement_sp();
    }

    //Add to the 16-bit register SP, the signed immediate data dd
    //0b11101000
    pub fn add_sp_dd(&self, cpu: &mut Cpu) {
        let value = cpu.add_sp_dd() as usize;
        cpu.write_sp(value);
    }

    //Load to the 16-bit register HL, the stack pointer plus the signed immediate data dd
    //0b11111000
    pub fn ld_hl_sp_dd(&self, cpu: &mut Cpu) {
        let value = cpu.add_sp_dd();
        cpu.write_reg16(Reg16bit::HL as usize, value);
    }

    /********** Rotate and Shift Commands **********/

    //0b00000111/0x07
    pub fn rlca(&self, cpu: &mut Cpu) {
        self.rlca(cpu);
    }

    pub fn rla(&self, cpu: &mut Cpu) {
        cpu.rla();
    }

    /********** CPU-Control commands **********/

    //No Operation Preformed
    //0b00000000
    pub fn nop(&self, cpu: &mut Cpu) {}
}
