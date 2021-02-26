use std::fmt;

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

    pub fn execute_instruction(&mut self) {
        let opcode = self.memory[self.program_counter as usize];

        match opcode.get_value() {
            0xa9 => {
                let new_value = self.memory[(self.program_counter + 1) as usize];
                self.reg_a = new_value.get_value();
            }
            _ => {
                error!("Unknown opcode {}", opcode.get_value());
            }
        }
    }
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A : {}, X : {}, Y : {}\nPC : {}, ", self.reg_a, self.reg_x, self.reg_y, self.program_counter)
    }
}

#[test]
fn lda() {
    // TODO : Test LDA with different addressing modes
    use super::byte::Byte;
    let mut cpu = Cpu::new();

    cpu.memory[0x00] = Byte::new(0xa9);
    cpu.memory[0x01] = Byte::new(0x23);

    cpu.execute_instruction();

    assert_eq!(cpu.reg_a, 0x23);
}