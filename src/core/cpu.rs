use super::memory::Memory;

pub struct Cpu {
    reg_a: u8,
    reg_x: u8,
    reg_y: u8,
    program_counter: u16,
    // TODO : Add stack pointer
    memory: Memory,

    flag_carry: bool,
    flag_zero: bool,
    flag_interrupt_disable: bool,
    flag_decimal_mode: bool,
    flag_break: bool,
    flag_overflow: bool,
    flag_negative: bool,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            reg_a: 0x00,
            reg_x: 0x00,
            reg_y: 0x00,
            program_counter: 0x00, //TODO : Change this according to program location
            memory: Memory::new(),
            flag_carry: false, // TODO : Verify flag start state
            flag_zero: false,
            flag_interrupt_disable: false,
            flag_decimal_mode: false,
            flag_break: false,
            flag_overflow: false,
            flag_negative: false,
        }
    }

    pub fn execute_instruction() {
        
    }
}

