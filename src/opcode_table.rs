use crate::instructions::*;

pub struct OpcodeTable {
    table: [Opcode; 1],
}

impl OpcodeTable {
    pub fn new() -> Self {
        OpcodeTable {
            table: [Opcode::new(
                "No Op".to_string(),
                0b00000000,
                1,
                1,
                Opcode::nop,
            )],
        }
    }
}
