use crate::cpu::*;

const Y_REG_MASK: u8 = 0x38;
const Z_REG_MASK: u8 = 0x07;
const P_REG_MASK: u8 = 0x30;
const LDH_ADDR_MSB_MASK: usize = 0xff00;

struct Opcode<'a> {
    opcode_name: &'a str,
    opcode_byte: u8, //hex representatoin of the opcode
    number_of_cycles: u8,
    length: u8, //in bytes
    handler: fn(&Opcode, Cpu),
}

/*
fn test() {
    let mut cpu_1: Cpu = Cpu::new();
    let test_opcode = Opcode {
        opcode_name: String::from("Test"),
        opcode_byte: 0x00,
        number_of_cycles: 2,
        length: 2,
        handler: Opcode::load_immediate,
    };

    (test_opcode.handler)(&test_opcode, cpu_1);

}
*/

//For information on the implementation of these opcodes please see Chapter 2 of Game Boy: Complete Technical Reference

/**********8 bit load instructions**********/
impl Opcode<'_> {
    //8-bit load instructions transfer one byte of data between two 8-bit registers
    //0b01yyyzzz
    fn load_r_r(&self, cpu: &mut Cpu) {
        let register_y: usize = ((self.opcode_byte & Y_REG_MASK) >> 3) as usize;
        let register_z: usize = (self.opcode_byte & Z_REG_MASK) as usize;
        cpu.write_reg8_with_reg8(register_y, register_z);
    }

    //Load to the 8-bit register r, the immediate data n.
    //0b00yyy110
    fn load_r_n(&self, cpu: &mut Cpu) {
        let register: usize = ((self.opcode_byte & Y_REG_MASK) >> 3) as usize;
        let pc: usize = cpu.read_pc();
        cpu.write_reg8(register, cpu.read_memory(pc + 1));
    }

    //Load to the 8-bit register r, data from the absolute address specified by the 16-bit register HL.
    //0b01yyy110
    fn load_r_hl(&self, cpu: &mut Cpu) {
        let register: usize = ((self.opcode_byte & Y_REG_MASK) >> 3) as usize;
        let index = cpu.read_reg16(Reg16bit::HL) as usize;
        cpu.write_reg8(register, cpu.read_memory(index));
    }

    //Load to the absolute address specified by the 16-bit register HL, data from the 8-bit register r.
    //0b01110zzz
    fn load_hl_r(&self, cpu: &mut Cpu) {
        let register_z: usize = (self.opcode_byte & Z_REG_MASK) as usize;
        let index = cpu.read_reg16(Reg16bit::HL) as usize;
        cpu.write_memory(index, cpu.read_reg8(register_z));
    }

    //Load to the absolute address specified by the 16-bit register HL, the immediate data n.
    //0b00110110/0x36 + n
    fn load_hl_n(&self, cpu: &mut Cpu) {
        let pc: usize = cpu.read_pc();
        let index = cpu.read_reg16(Reg16bit::HL) as usize;
        cpu.write_memory(index, cpu.read_memory(pc + 1));
    }

    //Load to the 8-bit A register, data from the absolute address specified by the 16-bit register BC.
    //0b00001010/0x0A
    fn load_a_bc(&self, cpu: &mut Cpu) {
        let index = cpu.read_reg16(Reg16bit::BC) as usize;
        cpu.write_reg8(Reg8bit::A as usize, cpu.read_memory(index));
    }

    //Load to the 8-bit A register, data from the absolute address specified by the 16-bit register DE.
    //0b00011010/0x1A
    fn load_a_de(&self, cpu: &mut Cpu) {
        let index = cpu.read_reg16(Reg16bit::DE) as usize;
        cpu.write_reg8(Reg8bit::A as usize, cpu.read_memory(index));
    }

    //Load to the absolute address specified by the 16-bit register BC, data from the 8-bit A register.
    //0b00000010/0x02
    fn load_bc_a(&self, cpu: &mut Cpu) {
        let index = cpu.read_reg16(Reg16bit::BC) as usize;
        cpu.write_memory(index, cpu.read_reg8(Reg8bit::A as usize));
    }

    //Load to the absolute address specified by the 16-bit register DE, data from the 8-bit A register
    //0b00010010/0x12
    fn load_de_a(&self, cpu: &mut Cpu) {
        let index = cpu.read_reg16(Reg16bit::DE) as usize;
        cpu.write_memory(index, cpu.read_reg8(Reg8bit::A as usize));
    }

    //Load to the 8-bit A register, data from the absolute address specified by the 16-bit operand nn.
    //0b11111010/0xFA + LSB of nn + MSB of nn
    fn load_a_nn(&self, cpu: &mut Cpu) {
        let pc: usize = cpu.read_pc();
        let mut index = cpu.read_memory(pc + 2) as usize;
        index <<= 8;
        index |= cpu.read_memory(pc + 1) as usize;
        cpu.write_reg8(Reg8bit::A as usize, cpu.read_memory(index));
    }

    //Load to the absolute address specified by the 16-bit operand nn, data from the 8-bit A register.
    //0b11101010/0xEA + LSB of nn + MSB of nn
    fn load_nn_a(&self, cpu: &mut Cpu) {
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
    fn ldh_a_c(&self, cpu: &mut Cpu) {
        let mut index = LDH_ADDR_MSB_MASK | cpu.read_reg8(Reg8bit::C as usize) as usize;
        cpu.write_reg8(Reg8bit::A as usize, cpu.read_memory(index));
    }

    //Load to the address specified by the 8-bit C register, data from the 8-bit A register. The full 16-bit absolute
    //address is obtained by setting the most significant byte to 0xFF and the least significant byte to the value of C,
    //so the possible range is 0xFF00-0xFFFF.
    //0b11100010/0xE2
    fn ldh_c_a(&self, cpu: &mut Cpu) {
        let mut index = LDH_ADDR_MSB_MASK | cpu.read_reg8(Reg8bit::C as usize) as usize;
        cpu.write_memory(index, cpu.read_reg8(Reg8bit::A as usize));
    }

    //Load to the 8-bit A register, data from the address specified by the 8-bit immediate data n. The full 16-bit
    //absolute address is obtained by setting the most significant byte to 0xFF and the least significant byte to the
    //value of n, so the possible range is 0xFF00-0xFFFF.
    //0b11110000/0xF0
    fn ldh_a_n(&self, cpu: &mut Cpu) {
        let pc: usize = cpu.read_pc();
        let mut index = LDH_ADDR_MSB_MASK | cpu.read_memory(pc + 1) as usize;
        cpu.write_reg8(Reg8bit::A as usize, cpu.read_memory(index));
    }

    //Load to the address specified by the 8-bit immediate data n, data from the 8-bit A register. The full 16-bit
    //absolute address is obtained by setting the most significant byte to 0xFF and the least significant byte to the
    //value of n, so the possible range is 0xFF00-0xFFFF.
    //0b11100000/0xE0
    fn ldh_n_a(&self, cpu: &mut Cpu) {
        let pc: usize = cpu.read_pc();
        let mut index = LDH_ADDR_MSB_MASK | cpu.read_memory(pc + 1) as usize;
        cpu.write_memory(index, cpu.read_memory(pc + 1));
    }

    //Load to the 8-bit A register, data from the absolute address specified by the 16-bit register HL. The value of
    //HL is decremented after the memory read.
    //0b00111010/0x3A
    fn load_a_hl_decrement(&self, cpu: &mut Cpu) {
        let index = cpu.read_reg16(Reg16bit::HL) as usize;
        cpu.write_reg8(Reg8bit::A as usize, cpu.read_memory(index));
        cpu.write_reg16(Reg16bit::HL, index as u16 - 1);
    }

    //Load to the absolute address specified by the 16-bit register HL, data from the 8-bit A register. The value of
    //HL is decremented after the memory write.
    //0b00110010/0x32
    fn load_hl_a_decrement(&self, cpu: &mut Cpu) {
        let index = cpu.read_reg16(Reg16bit::HL) as usize;
        cpu.write_memory(index, cpu.read_reg8(Reg8bit::A as usize));
        cpu.write_reg16(Reg16bit::HL, index as u16 - 1);
    }

    //Load to the 8-bit A register, data from the absolute address specified by the 16-bit register HL. The value of
    //HL is incremented after the memory read.
    //0b00101010/0x2A
    fn load_a_hl_increment(&self, cpu: &mut Cpu) {
        let index = cpu.read_reg16(Reg16bit::HL) as usize;
        cpu.write_reg8(Reg8bit::A as usize, cpu.read_memory(index));
        cpu.write_reg16(Reg16bit::HL, index as u16 + 1);
    }

    //Load to the absolute address specified by the 16-bit register HL, data from the 8-bit A register. The value of
    //HL is incremented after the memory write.
    //0b00100010/0x22
    fn load_hl_a_incement(&self, cpu: &mut Cpu) {
        let index = cpu.read_reg16(Reg16bit::HL) as usize;
        cpu.write_memory(index, cpu.read_reg8(Reg8bit::A as usize));
        cpu.write_reg16(Reg16bit::HL, index as u16 + 1);
    }

    /**********16 bit load instructions**********/

    //note about 16 bit registers:  Look into rust's to_bytes and from_bytes functions to converting two u8s to and from u16
    //also, figure out how rusts to_be from_be know endian of an mcu

    //16-bit load instructions transfer two bytes of data between one 16-bit register
    //and two sequential locations in memory.
    //0b00000001 + LSB of nn + MSB of nn
    fn load_bc_nn(&self, cpu: &mut Cpu) {
        let pc = cpu.read_pc();
        cpu.write_reg16_fast(Reg16bit::BC, cpu.read_memory(pc), cpu.read_memory(pc + 1))
    }

    //16-bit load instructions transfer two bytes of data between one 16-bit register
    //and two sequential locations in memory.
    //0b00010001 + LSB of nn + MSB of nn
    fn load_de_nn(&self, cpu: &mut Cpu) {
        let pc = cpu.read_pc();
        cpu.write_reg16_fast(Reg16bit::DE, cpu.read_memory(pc), cpu.read_memory(pc + 1))
    }

    //16-bit load instructions transfer two bytes of data between one 16-bit register
    //and two sequential locations in memory.
    //0b00100001 + LSB of nn + MSB of nn
    fn load_hl_nn(&self, cpu: &mut Cpu) {
        let pc = cpu.read_pc();
        cpu.write_reg16_fast(Reg16bit::HL, cpu.read_memory(pc), cpu.read_memory(pc + 1))
    }

    //16-bit load instructions transfer two bytes of data between one 16-bit register
    //and two sequential locations in memory.
    //0b00110001 + LSB of nn + MSB of nn
    fn load_sp_nn(&self, cpu: &mut Cpu) {
        let pc = cpu.read_pc();
        let mut new_sp = cpu.read_memory(pc + 1) as usize;
        new_sp <<= 8;
        new_sp += cpu.read_memory(pc) as usize;
        cpu.write_sp(new_sp);
    }

    //Load to the absolute address specified by the 16-bit operand nn, data from the 16-bit SP register.
    //0b00001000/0x08 + LSB of nn + MSB of nn
    fn load_nn_sp(&self, cpu: &mut Cpu) {
        let pc = cpu.read_pc();
        let index = cpu.read_memory_nn(pc) as usize;

        let mut sp_lsb = cpu.read_sp();
        let sp_msb = sp_lsb >> 8;
        sp_lsb &= 0x00ff;

        cpu.write_memory_n_n(index, sp_lsb as u8, sp_msb as u8);
    }

    //Load to the 16-bit SP register, data from the 16-bit HL register.
    //0b11111001/0xF9
    fn load_sp_hl(&self, cpu: &mut Cpu) {
        cpu.write_sp(cpu.read_reg16(Reg16bit::HL) as usize);
    }

    //Push to the stack memory, data from the 16-bit register BC.
    //0b11000101
    fn push_bc(&self, cpu: &mut Cpu) {
        cpu.push_rr(Reg16bit::BC);
    }

    //Push to the stack memory, data from the 16-bit register DE.
    //0b11010101
    fn push_de(&self, cpu: &mut Cpu) {
        cpu.push_rr(Reg16bit::DE);
    }

    //Push to the stack memory, data from the 16-bit register HL.
    //0b11100101
    fn push_hl(&self, cpu: &mut Cpu) {
        cpu.push_rr(Reg16bit::HL);
    }

    //Push to the stack memory, data from the 16-bit register AF.
    //0b11100101
    fn push_af(&self, cpu: &mut Cpu) {
        cpu.push_rr(Reg16bit::AF);
    }

    //Pops to the 16-bit register BD, data from the stack memory.
    //0b11000001
    fn pop_bc(&self, cpu: &mut Cpu) {
        cpu.pop_rr(Reg16bit::BC);
    }

    //Pops to the 16-bit register DE, data from the stack memory.
    //0b11010001
    fn pop_de(&self, cpu: &mut Cpu) {
        cpu.pop_rr(Reg16bit::DE);
    }

    //Pops to the 16-bit register HL, data from the stack memory.
    //0b11100001
    fn pop_hl(&self, cpu: &mut Cpu) {
        cpu.pop_rr(Reg16bit::HL);
    }

    //Pops to the 16-bit register AF, data from the stack memory.
    //0b11110001
    fn pop_af(&self, cpu: &mut Cpu) {
        cpu.pop_rr(Reg16bit::AF);
    }

    /**********8bit-Arithmetic/logical Commands**********/

    //Add to the 8-bit register A, data from register zzz
    //0b10000zzz
    fn add_a_r(&self, cpu: &mut Cpu) {
        let register_z: usize = (self.opcode_byte & Z_REG_MASK) as usize;
        cpu.add_a_r(register_z as usize, false);
    }

    //Add to the 8-bit register A, the immediate data n
    //0b11000110
    fn add_a_n(&self, cpu: &mut Cpu) {
        cpu.add_a_n(false);
    }

    //Add to the 8-bit register A, data from the absolute address specified by the 16-bit register HL.
    //10000110
    fn add_a_hl(&self, cpu: &mut Cpu) {
        cpu.add_a_hl(false);
    }

    //Add to the 8-bit register A, data from register zzz
    //0b10000zzz
    fn adc_a_r(&self, cpu: &mut Cpu) {
        let register_z: usize = (self.opcode_byte & Z_REG_MASK) as usize;
        cpu.add_a_r(register_z as usize, true);
    }

    //Add to the 8-bit register A, the immediate data n
    //0b11001110
    fn adc_a_n(&self, cpu: &mut Cpu) {
        cpu.add_a_n(true);
    }

    //Add to the 8-bit register A, data from the absolute address specified by the 16-bit register HL.
    //10001110
    fn adc_a_hl(&self, cpu: &mut Cpu) {
        cpu.add_a_hl(true);
    }
}
