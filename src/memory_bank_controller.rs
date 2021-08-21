use crate::rom::*;

pub struct Mcb {
    //16 is not the final size of this array.  I need to be able to buffer a whole rom cart (so atleast 128, but really 512 for mbc 5)
    //Problem is that the stack overflows (on windows) with such a big array.  I could use a vector (moving the data to the heap) but I'm not sure
    //about how that would work on a microcontroller using an external ram chip to buffer the cart.
    //Another potential solution is to use just two banks and as the second bank is switch read in the values from the cart.  I'll need to see how long
    //this takes especially on the micro
    //bank: [Rom; 16],
    bank: Vec<Rom>,
    current_bank: usize,
}

impl Mcb {
    pub fn new() -> Self {
        Self {
            //bank: [Rom::new(); 16],
            bank: vec![Rom::new(), Rom::new()], //init two banks
            current_bank: 1,
        }
    }

    pub fn read_bank_00(&self, index: usize) -> u8 {
        self.bank[0].read_memory(index - ROM_BANK_00_START)
    }

    pub fn read_bank_n(&self, index: usize) -> u8 {
        self.bank[self.current_bank].read_memory(index - ROM_BANK_01_START)
    }

    pub fn write_bank_00(&mut self, index: usize, data: u8) {
        self.bank[0].write_memory(index - ROM_BANK_00_START, data);
    }

    pub fn write_bank_n(&mut self, index: usize, data: u8) {
        self.bank[self.current_bank].write_memory(index - ROM_BANK_01_START, data);
    }

    pub fn change_bank(&self) {
        //stub -- this code will update current_bank
        println!("Change_bank not implemented");
    }
}
