pub const TIMER_ADDR_START: usize = 0xff04;
pub const TIMER_ADDR_END: usize = 0xff07;

//These are the four register reprsented by the array
const DIV: usize = 0;
const TIMA: usize = 1;
const TMA: usize = 2;
const TAC: usize = 3;

pub struct Timer {
    source_timer: u8,
    div_prescaler: u8,
    update_source: u8,
    //0xff04 - 0xff07
    memory_registers: [u8; 4],
}

impl Timer {
    pub fn new() -> Self {
        Self {
            source_timer: 0,
            div_prescaler: 0,
            update_source: 0,
            memory_registers: [0, 0, 0, 0],
        }
    }

    //Reads the timer regsiters.  If an invalid register is read 0xED is returned
    pub fn read_memory(&self, index: usize) -> u8 {
        if index >= TIMER_ADDR_START && index <= TIMER_ADDR_END {
            self.memory_registers[index - TIMER_ADDR_START]
        } else {
            println!("Error, timer read index out of bounds!");
            0xED
        }
    }

    //Write to the timer registers.
    pub fn write_memory(&mut self, index: usize, value: u8) {
        if index >= TIMER_ADDR_START && index <= TIMER_ADDR_END {
            self.memory_registers[index - TIMER_ADDR_START] = value;
        } else {
            println!("Error, timer write index out of bounds!")
        }
    }

    //Returns true if an interrupt occured
    pub fn update_timers(&mut self, cycles_elasped: u8) -> bool {
        self.update_source = self.update_source.wrapping_add(cycles_elasped);
        let mut interrupt_request = false;

        //source timer runs at 1/4 the speed of the clock
        if self.update_source >= 4 {
            self.source_timer = self.source_timer.wrapping_add(1);
            self.update_source -= 4;

            //The divider timer runs at 1/16 the speed of the source timer
            self.div_prescaler = self.div_prescaler.wrapping_add(1);
            if self.div_prescaler == 16 {
                self.memory_registers[DIV] = self.memory_registers[DIV].wrapping_add(1);
                self.div_prescaler = 0;
            }

            //The tima timer's frequence is adjustable by the TAC register
            if self.memory_registers[TAC] & 0x4 > 0 {
                let counter_threshold = match self.memory_registers[TAC] & 0x3 {
                    0 => 64_u8,
                    1 => 1_u8,
                    2 => 4_u8,
                    3 => 16_u8,
                    _ => 0_u8,
                };

                if self.source_timer >= counter_threshold {
                    self.source_timer = 0;
                    if self.memory_registers[TIMA] == 255 {
                        //on overflow the counter gets reloaded with the value from the TMA register
                        self.memory_registers[TIMA] = self.memory_registers[TMA];
                        interrupt_request = true;
                    } else {
                        self.memory_registers[TIMA] = self.memory_registers[TIMA].wrapping_add(1);
                    }
                }
            }
        }
        interrupt_request
    }
}
