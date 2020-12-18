use crate::instructions::*;

pub struct OpcodeTable {
    table: [Opcode; 64],
}

impl OpcodeTable {
    pub fn init_nonprefix_insturction_table() -> Self {
        Self {
            table: [
                Opcode::new(0x00, "No Op".to_string(), 1, 1, Opcode::nop),
                Opcode::new(0x01, "LD BC, u16".to_string(), 3, 3, Opcode::load_bc_nn),
                Opcode::new(0x02, "LD BC, A".to_string(), 2, 1, Opcode::load_bc_a),
                Opcode::new(0x03, "INC BC".to_string(), 2, 1, Opcode::inc_rr),
                Opcode::new(0x04, "INC B".to_string(), 1, 1, Opcode::inc_r),
                Opcode::new(0x05, "DEC B".to_string(), 1, 1, Opcode::dec_r),
                Opcode::new(0x06, "LD B, u8".to_string(), 1, 21, Opcode::nop), //Place holder nop
                Opcode::new(0x07, "RLCA".to_string(), 2, 1, Opcode::load_bc_a),
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
                Opcode::new(0x17, "RLA".to_string(), 1, 1, Opcode::nop), //Plaace holder nop
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
            ],
        }
    }
    /*
    pub fn init_prefix_insctruction_table() -> Self {
        Self { table: [] }
    }
    */
}
