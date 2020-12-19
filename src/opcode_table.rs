use crate::instructions::*;

pub struct OpcodeTable {
    table: [Opcode; 256],
}

impl OpcodeTable {
    pub fn init_unprefix_insturction_table() -> Self {
        Self {
            table: [
                Opcode::new(0x00, "No Op".to_string(), 1, 1, Opcode::nop),
                Opcode::new(0x01, "LD BC, u16".to_string(), 3, 3, Opcode::load_bc_nn),
                Opcode::new(0x02, "LD BC, A".to_string(), 2, 1, Opcode::load_bc_a),
                Opcode::new(0x03, "INC BC".to_string(), 2, 1, Opcode::inc_rr),
                Opcode::new(0x04, "INC B".to_string(), 1, 1, Opcode::inc_r),
                Opcode::new(0x05, "DEC B".to_string(), 1, 1, Opcode::dec_r),
                Opcode::new(0x06, "LD B, u8".to_string(), 1, 21, Opcode::load_r_n), //Place holder nop
                Opcode::new(0x07, "RLCA".to_string(), 2, 1, Opcode::rlca),
                Opcode::new(0x08, "LD (u16), SP".to_string(), 2, 1, Opcode::load_nn_sp),
                Opcode::new(0x09, "ADD HL, BC".to_string(), 2, 1, Opcode::add_hl_rr),
                Opcode::new(0x0A, "LD A, (BC)".to_string(), 2, 1, Opcode::load_a_bc),
                Opcode::new(0x0B, "DEC BC".to_string(), 2, 1, Opcode::dec_rr),
                Opcode::new(0x0C, "INC C".to_string(), 1, 1, Opcode::inc_r),
                Opcode::new(0x0D, "DEC C".to_string(), 1, 1, Opcode::dec_r),
                Opcode::new(0x0E, "LD C, u8".to_string(), 2, 2, Opcode::load_r_n),
                Opcode::new(0x0F, "RRCA".to_string(), 1, 1, Opcode::nop), //Place holder nop
                Opcode::new(0x10, "STOP".to_string(), 1, 2, Opcode::nop), //Place holder nop
                Opcode::new(0x11, "LD DE, u16".to_string(), 3, 3, Opcode::load_de_nn),
                Opcode::new(0x12, "LD (DE), A".to_string(), 2, 1, Opcode::load_de_a),
                Opcode::new(0x13, "INC DE".to_string(), 2, 1, Opcode::inc_rr),
                Opcode::new(0x14, "INC D".to_string(), 1, 1, Opcode::inc_r),
                Opcode::new(0x15, "DEC D".to_string(), 1, 1, Opcode::dec_r),
                Opcode::new(0x16, "LD D, u8".to_string(), 1, 1, Opcode::load_r_n),
                Opcode::new(0x17, "RLA".to_string(), 1, 1, Opcode::rla),
                Opcode::new(0x18, "JR i8".to_string(), 3, 2, Opcode::nop), //Place holder nop
                Opcode::new(0x19, "ADD HL, DE".to_string(), 2, 1, Opcode::add_hl_rr),
                Opcode::new(0x1A, "LD A, (DE)".to_string(), 2, 1, Opcode::load_a_de),
                Opcode::new(0x1B, "DEC DE".to_string(), 2, 1, Opcode::dec_rr),
                Opcode::new(0x1C, "INC E".to_string(), 1, 1, Opcode::inc_r),
                Opcode::new(0x1D, "DEC D".to_string(), 1, 1, Opcode::dec_r),
                Opcode::new(0x1E, "LD E, u8".to_string(), 4, 1, Opcode::load_r_n),
                Opcode::new(0x1F, "RRA".to_string(), 1, 1, Opcode::nop), //Place holder nop
                Opcode::new(0x20, "JR NZ, i8".to_string(), 3, 2, Opcode::nop), //Place holder nop
                Opcode::new(0x21, "LD HL, u16".to_string(), 3, 3, Opcode::load_hl_nn),
                Opcode::new(0x22, "LD (HL+), A".to_string(), 2, 1, Opcode::load_hl_a_inc),
                Opcode::new(0x23, "INC HL".to_string(), 2, 1, Opcode::inc_rr),
                Opcode::new(0x24, "INC H".to_string(), 1, 1, Opcode::inc_r),
                Opcode::new(0x25, "DEC H".to_string(), 1, 1, Opcode::dec_r),
                Opcode::new(0x26, "LD H, u8".to_string(), 2, 2, Opcode::load_r_n),
                Opcode::new(0x27, "DAA".to_string(), 1, 1, Opcode::daa),
                Opcode::new(0x28, "JR Z, i8".to_string(), 3, 2, Opcode::nop), //place holder no op
                Opcode::new(0x29, "ADD HL, HL".to_string(), 1, 1, Opcode::add_hl_rr),
                Opcode::new(0x2A, "LD A, (HL+)".to_string(), 2, 1, Opcode::load_a_hl_inc),
                Opcode::new(0x2B, "DEC HL".to_string(), 2, 1, Opcode::dec_rr),
                Opcode::new(0x2C, "INC L".to_string(), 1, 1, Opcode::inc_r),
                Opcode::new(0x2D, "DEC L".to_string(), 1, 1, Opcode::dec_r),
                Opcode::new(0x2E, "LD L, u8".to_string(), 2, 2, Opcode::load_r_n),
                Opcode::new(0x2F, "CPL".to_string(), 1, 1, Opcode::cpl),
                Opcode::new(0x30, "JR NC, i8".to_string(), 3, 2, Opcode::nop), //place holder nop
                Opcode::new(0x31, "LD SP, u16".to_string(), 3, 3, Opcode::load_sp_nn),
                Opcode::new(0x32, "LD (HL-), A".to_string(), 1, 1, Opcode::load_hl_a_dec),
                Opcode::new(0x33, "INC SP".to_string(), 2, 1, Opcode::inc_sp),
                Opcode::new(0x34, "INC (HL)".to_string(), 3, 1, Opcode::inc_hl),
                Opcode::new(0x35, "DEC (HL)".to_string(), 3, 1, Opcode::dec_hl),
                Opcode::new(0x36, "LD (HL), u8".to_string(), 3, 2, Opcode::load_hl_n),
                Opcode::new(0x37, "SCF".to_string(), 1, 1, Opcode::nop), //place holder nop
                Opcode::new(0x38, "JR C, i8".to_string(), 1, 1, Opcode::nop), //place holder nop
                Opcode::new(0x39, "ADD HL, SP".to_string(), 2, 1, Opcode::add_hl_sp),
                Opcode::new(0x3A, "LD A, (HL-)".to_string(), 1, 2, Opcode::load_a_hl_dec),
                Opcode::new(0x3B, "DEC SP".to_string(), 2, 1, Opcode::dec_sp),
                Opcode::new(0x3C, "INC A".to_string(), 1, 1, Opcode::inc_r),
                Opcode::new(0x3D, "DEC a".to_string(), 1, 1, Opcode::dec_r),
                Opcode::new(0x3E, "LD A, u8".to_string(), 2, 2, Opcode::load_r_n),
                Opcode::new(0x3F, "CCF".to_string(), 1, 1, Opcode::nop), //place holder nop
                Opcode::new(0x40, "LD B, B".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x41, "LD B, C".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x42, "LD B, D".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x43, "LD B, E".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x44, "LD B. H".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x45, "LD B, L".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x46, "LD B, (HL)".to_string(), 2, 1, Opcode::load_r_hl),
                Opcode::new(0x47, "LD B, A".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x48, "LD C, B".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x49, "LD C, C".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x4A, "LD C, D".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x4B, "LD C, E".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x4C, "LD C, H".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x4D, "LD C, L".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x4E, "LD C, (HL)".to_string(), 1, 1, Opcode::load_r_hl),
                Opcode::new(0x4F, "LD C, A".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x50, "LD D, B".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x51, "LD D, C".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x52, "LD D, D".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x53, "LD D, E".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x54, "LD D, H".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x55, "LD D, L".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x56, "LD D, (HL)".to_string(), 1, 1, Opcode::load_r_hl),
                Opcode::new(0x57, "LD D, A".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x58, "LD E, B".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x59, "LD E, C".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x5A, "LD E, D".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x5B, "LD E, E".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x5C, "LD E, H".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x5D, "LD E, L".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x5E, "LD E, HL".to_string(), 1, 1, Opcode::load_r_hl),
                Opcode::new(0x5F, "LD E, A".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x60, "LD H, B".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x61, "LD H, C".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x62, "LD H, D".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x63, "LD H, E".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x64, "LD H, H".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x65, "LD H, L".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x66, "LD H, HL".to_string(), 1, 1, Opcode::load_r_hl),
                Opcode::new(0x67, "LD H, A".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x68, "LD L, B".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x69, "LD L, C".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x6A, "LD L, D".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x6B, "LD L, E".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x6C, "LD L, H".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x6D, "LD L, L".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x6E, "LD L, HL".to_string(), 1, 1, Opcode::load_r_hl),
                Opcode::new(0x6F, "LD L, A".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x70, "LD HL, B".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x71, "LD HL, C".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x72, "LD HL, D".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x73, "LD HL, E".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x74, "LD HL, H".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x75, "LD HL, L".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x76, "HALT".to_string(), 1, 1, Opcode::nop), //place holder nop
                Opcode::new(0x77, "LD HL, A".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x78, "LD A, B".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x79, "LD A, C".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x7A, "LD A, D".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x7B, "LD A, E".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x7C, "LD A, H".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x7D, "LD A, L".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x7E, "LD A, HL".to_string(), 1, 1, Opcode::load_r_hl),
                Opcode::new(0x7F, "LD A, A".to_string(), 1, 1, Opcode::load_r_r),
                Opcode::new(0x80, "ADD A, B".to_string(), 1, 1, Opcode::add_a_r),
                Opcode::new(0x81, "ADD A, C".to_string(), 1, 1, Opcode::add_a_r),
                Opcode::new(0x82, "ADD A, D".to_string(), 1, 1, Opcode::add_a_r),
                Opcode::new(0x83, "ADD A, E".to_string(), 1, 1, Opcode::add_a_r),
                Opcode::new(0x84, "ADD A, H".to_string(), 1, 1, Opcode::add_a_r),
                Opcode::new(0x85, "ADD A, L".to_string(), 1, 1, Opcode::add_a_r),
                Opcode::new(0x86, "ADD A, (HL)".to_string(), 1, 1, Opcode::add_a_hl),
                Opcode::new(0x87, "ADD A, A".to_string(), 1, 1, Opcode::add_a_r),
                Opcode::new(0x88, "ADC A, B".to_string(), 1, 1, Opcode::adc_a_r),
                Opcode::new(0x89, "ADC A, C".to_string(), 1, 1, Opcode::adc_a_r),
                Opcode::new(0x8A, "ADC A, D".to_string(), 1, 1, Opcode::adc_a_r),
                Opcode::new(0x8B, "ADC A, E".to_string(), 1, 1, Opcode::adc_a_r),
                Opcode::new(0x8C, "ADC A, H".to_string(), 1, 1, Opcode::adc_a_r),
                Opcode::new(0x8D, "ADC A, L".to_string(), 1, 1, Opcode::adc_a_r),
                Opcode::new(0x8E, "ADC A, (HL)".to_string(), 1, 1, Opcode::adc_a_hl),
                Opcode::new(0x8F, "ADC A, A".to_string(), 1, 1, Opcode::sub_a_r),
                Opcode::new(0x90, "SUB A, B".to_string(), 1, 1, Opcode::sub_a_r),
                Opcode::new(0x91, "SUB A, C".to_string(), 1, 1, Opcode::sub_a_r),
                Opcode::new(0x92, "SUB A, D".to_string(), 1, 1, Opcode::sub_a_r),
                Opcode::new(0x93, "SUB A, E".to_string(), 1, 1, Opcode::sub_a_r),
                Opcode::new(0x94, "SUB A, H".to_string(), 1, 1, Opcode::sub_a_r),
                Opcode::new(0x95, "SUB A, L".to_string(), 1, 1, Opcode::sub_a_r),
                Opcode::new(0x96, "SUB A, (HL)".to_string(), 1, 1, Opcode::sub_a_hl),
                Opcode::new(0x97, "SUB A, A".to_string(), 1, 1, Opcode::sub_a_r),
                Opcode::new(0x98, "SBC A, B".to_string(), 1, 1, Opcode::sbc_a_r),
                Opcode::new(0x99, "SBC A, C".to_string(), 1, 1, Opcode::sbc_a_r),
                Opcode::new(0x9A, "SBC A, D".to_string(), 1, 1, Opcode::sbc_a_r),
                Opcode::new(0x9B, "SBC A, E".to_string(), 1, 1, Opcode::sbc_a_r),
                Opcode::new(0x9C, "SBC A, H".to_string(), 1, 1, Opcode::sbc_a_r),
                Opcode::new(0x9D, "SBC A, L".to_string(), 1, 1, Opcode::sbc_a_r),
                Opcode::new(0x9E, "SBC A, (HL)".to_string(), 1, 1, Opcode::sbc_a_hl),
                Opcode::new(0x9F, "SBC A, A".to_string(), 1, 1, Opcode::sbc_a_r),
                Opcode::new(0xA0, "AND A, B".to_string(), 1, 1, Opcode::and_a_r),
                Opcode::new(0xA1, "AND A, C".to_string(), 1, 1, Opcode::and_a_r),
                Opcode::new(0xA2, "AND A, D".to_string(), 1, 1, Opcode::and_a_r),
                Opcode::new(0xA3, "AND A, E".to_string(), 1, 1, Opcode::and_a_r),
                Opcode::new(0xA4, "AND A, H".to_string(), 1, 1, Opcode::and_a_r),
                Opcode::new(0xA5, "AND A, L".to_string(), 1, 1, Opcode::and_a_r),
                Opcode::new(0xA6, "AND A, (HL)".to_string(), 1, 1, Opcode::and_a_hl),
                Opcode::new(0xA7, "AND A, A".to_string(), 1, 1, Opcode::xor_a_r),
                Opcode::new(0xA8, "XOR A, B".to_string(), 1, 1, Opcode::xor_a_r),
                Opcode::new(0xA9, "XOR A, C".to_string(), 1, 1, Opcode::xor_a_r),
                Opcode::new(0xAA, "XOR A, D".to_string(), 1, 1, Opcode::xor_a_r),
                Opcode::new(0xAB, "XOR A, E".to_string(), 1, 1, Opcode::xor_a_r),
                Opcode::new(0xAC, "XOR A, H".to_string(), 1, 1, Opcode::xor_a_r),
                Opcode::new(0xAD, "XOR A, L".to_string(), 1, 1, Opcode::xor_a_r),
                Opcode::new(0xAE, "XOR A, (HL)".to_string(), 1, 1, Opcode::xor_a_hl),
                Opcode::new(0xAF, "XOR A, A".to_string(), 1, 1, Opcode::xor_a_r),
                Opcode::new(0xB0, "OR A, B".to_string(), 1, 1, Opcode::or_a_r),
                Opcode::new(0xB1, "OR A, C".to_string(), 1, 1, Opcode::or_a_r),
                Opcode::new(0xB2, "OR A, D".to_string(), 1, 1, Opcode::or_a_r),
                Opcode::new(0xB3, "OR A, E".to_string(), 1, 1, Opcode::or_a_r),
                Opcode::new(0xB4, "OR A, H".to_string(), 1, 1, Opcode::or_a_r),
                Opcode::new(0xB5, "OR A, L".to_string(), 1, 1, Opcode::or_a_r),
                Opcode::new(0xB6, "OR A, (HL)".to_string(), 1, 1, Opcode::or_a_hl),
                Opcode::new(0xB7, "OR A, A".to_string(), 1, 1, Opcode::or_a_r),
                Opcode::new(0xB8, "CP A, B".to_string(), 1, 1, Opcode::cp_a_r),
                Opcode::new(0xB9, "CP A, C".to_string(), 1, 1, Opcode::cp_a_r),
                Opcode::new(0xBA, "CP A, D".to_string(), 1, 1, Opcode::cp_a_r),
                Opcode::new(0xBB, "CP A, E".to_string(), 1, 1, Opcode::cp_a_r),
                Opcode::new(0xBC, "CP A, H".to_string(), 1, 1, Opcode::cp_a_r),
                Opcode::new(0xBD, "CP A, L".to_string(), 1, 1, Opcode::cp_a_r),
                Opcode::new(0xBE, "CP A, (HL)".to_string(), 1, 1, Opcode::cp_a_hl),
                Opcode::new(0xBF, "CP A, A".to_string(), 1, 1, Opcode::cp_a_r),
                Opcode::new(0xC0, "RET NZ".to_string(), 2, 1, Opcode::nop), //Place holder nop
                Opcode::new(0xC1, "POP BC".to_string(), 3, 1, Opcode::pop_bc),
                Opcode::new(0xC2, "JP NZ, u16".to_string(), 3, 1, Opcode::nop), //Place holder nop
                Opcode::new(0xC3, "JP u16".to_string(), 1, 1, Opcode::nop),     //Place holder nop
                Opcode::new(0xC4, "CALL NZ, u16".to_string(), 1, 1, Opcode::nop), //Place holder nop
                Opcode::new(0xC5, "PUSH BC".to_string(), 4, 1, Opcode::push_bc),
                Opcode::new(0xC6, "ADD A, u8".to_string(), 1, 1, Opcode::nop),
                Opcode::new(0xC7, "RST 00h".to_string(), 4, 1, Opcode::nop), // Place Holder nop
                Opcode::new(0xC8, "RET Z".to_string(), 2, 1, Opcode::nop),   //place holder nop
                Opcode::new(0xC9, "RET".to_string(), 4, 1, Opcode::nop),     //Place holder nop
                Opcode::new(0xCA, "JP Z, u16".to_string(), 3, 3, Opcode::nop), //place holder nop
                //CB Is a prefix for the second second of 256 instructions.  It should never be executed from this table
                Opcode::new(0xCB, "PREFIX CB".to_string(), 3, 1, Opcode::nop),
                Opcode::new(0xCC, "CALL Z, u16".to_string(), 3, 3, Opcode::nop), //place holder nop
                Opcode::new(0xCD, "CALL u16".to_string(), 6, 3, Opcode::nop),    //place holder nop
                Opcode::new(0xCE, "ADC A, u8".to_string(), 2, 2, Opcode::adc_a_n),
                Opcode::new(0xCF, "RST".to_string(), 4, 1, Opcode::nop), //place holder nop
                Opcode::new(0xD0, "RET NC".to_string(), 2, 1, Opcode::nop), //place holder nop
                Opcode::new(0xD1, "POP DE".to_string(), 3, 1, Opcode::pop_de),
                Opcode::new(0xD2, "JP NC, u16".to_string(), 3, 1, Opcode::nop), //place holder nop
                Opcode::new(0xD3, "No Op 0xD3".to_string(), 1, 1, Opcode::nop),
                Opcode::new(0xD4, "CALL NC, u16".to_string(), 3, 3, Opcode::nop), //place holder nop
                Opcode::new(0xD5, "PUSH DE".to_string(), 4, 1, Opcode::push_de),
                Opcode::new(0xD6, "SUB A, u8".to_string(), 2, 2, Opcode::sub_a_n),
                Opcode::new(0xD7, "RST 10h".to_string(), 4, 1, Opcode::nop), //place holder nop
                Opcode::new(0xD8, "RET C".to_string(), 4, 1, Opcode::nop),   //place holder
                Opcode::new(0xD9, "RETI".to_string(), 4, 1, Opcode::nop),    //place holder
                Opcode::new(0xDA, "JP C, u16".to_string(), 3, 3, Opcode::nop), //place holder
                Opcode::new(0xDB, "No Op DB".to_string(), 1, 1, Opcode::nop),
                Opcode::new(0xDC, "CALL C, u16".to_string(), 1, 1, Opcode::nop), //place holder nop
                Opcode::new(0xDD, "No Op DD".to_string(), 1, 1, Opcode::nop),
                Opcode::new(0xDE, "SBC A, u8".to_string(), 2, 2, Opcode::sbc_a_n),
                Opcode::new(0xDF, "RST".to_string(), 4, 1, Opcode::nop), //place holder nop
                Opcode::new(0xE0, "LD (FF00+u8), A".to_string(), 1, 1, Opcode::ldh_n_a),
                Opcode::new(0xE1, "POP HL".to_string(), 3, 1, Opcode::pop_hl),
                Opcode::new(0xE2, "LD (FF00+C), A".to_string(), 1, 1, Opcode::ldh_c_a),
                Opcode::new(0xE3, "No Op 0xE3".to_string(), 1, 1, Opcode::nop),
                Opcode::new(0xE4, "No Op 0xE4".to_string(), 1, 1, Opcode::nop),
                Opcode::new(0xE5, "PUSH HL".to_string(), 4, 1, Opcode::push_hl),
                Opcode::new(0xE6, "AND A, u8".to_string(), 2, 1, Opcode::and_a_n),
                Opcode::new(0xE7, "RST 20h".to_string(), 4, 1, Opcode::nop), //place holder
                Opcode::new(0xE8, "ADD SP, i8".to_string(), 4, 1, Opcode::add_sp_dd),
                Opcode::new(0xE9, "JP HL".to_string(), 1, 1, Opcode::nop), //Place holder
                Opcode::new(0xEA, "LD (u16), A".to_string(), 1, 1, Opcode::load_nn_a),
                Opcode::new(0xEB, "No Op 0xEB".to_string(), 1, 1, Opcode::nop),
                Opcode::new(0xEC, "No Op 0xEC".to_string(), 1, 1, Opcode::nop),
                Opcode::new(0xED, "No Op 0xED".to_string(), 1, 1, Opcode::nop),
                Opcode::new(0xEE, "XOR A, u8".to_string(), 2, 1, Opcode::xor_a_n),
                Opcode::new(0xEF, "RST 28h".to_string(), 4, 1, Opcode::nop), //place holder
                Opcode::new(0xF0, "LD A, (FF00+u8)".to_string(), 3, 2, Opcode::ldh_a_n),
                Opcode::new(0xF1, "POP AF".to_string(), 3, 1, Opcode::pop_af),
                Opcode::new(0xF2, "LD A, (FF00+C)".to_string(), 2, 1, Opcode::ldh_a_c),
                Opcode::new(0xF3, "DI".to_string(), 1, 1, Opcode::nop),
                Opcode::new(0xF4, "No Op 0xF4".to_string(), 1, 1, Opcode::nop),
                Opcode::new(0xF5, "PUSH AF".to_string(), 4, 1, Opcode::push_af),
                Opcode::new(0xF6, "OR A, u8".to_string(), 2, 1, Opcode::or_a_n),
                Opcode::new(0xF7, "RST 30h".to_string(), 4, 1, Opcode::nop), //place holder
                Opcode::new(0xF8, "LD HL, SP+i8".to_string(), 3, 1, Opcode::ld_hl_sp_dd),
                Opcode::new(0xF9, "LD SP, HL".to_string(), 2, 1, Opcode::load_sp_hl),
                Opcode::new(0xFA, "LD A, (u16)".to_string(), 4, 3, Opcode::load_a_nn),
                Opcode::new(0xFB, "FB".to_string(), 1, 1, Opcode::nop), //place holder
                Opcode::new(0xFC, "No Op 0xFC".to_string(), 1, 1, Opcode::nop),
                Opcode::new(0xFD, "No Op 0xFD".to_string(), 1, 1, Opcode::nop),
                Opcode::new(0xFE, "CP A, u8".to_string(), 2, 2, Opcode::cp_a_n),
                Opcode::new(0xFF, "RST 38h".to_string(), 4, 1, Opcode::nop), //Place holder
            ],
        }
    }
    /*
    pub fn init_prefix_insctruction_table() -> Self {
        Self { table: [] }
    }
    */
}
