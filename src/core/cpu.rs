use std::fmt;

use byteorder::{ByteOrder, LittleEndian};

use super::memory::Memory;
use super::byte::Byte;

pub struct Cpu {
    reg_a: Byte,
    reg_x: Byte,
    reg_y: Byte,
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

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A : {}, X : {}, Y : {}\nPC : {}, ", self.reg_a, self.reg_x, self.reg_y, self.program_counter)
    }
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            reg_a: Byte::new(0x00),
            reg_x: Byte::new(0x00),
            reg_y: Byte::new(0x00),
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

    fn _get_first_arg(&self) -> Byte {
        self.memory[(self.program_counter + 1) as usize]
    }

    fn _get_second_arg(&self) -> Byte {
        self.memory[(self.program_counter + 2) as usize]
    }

    fn get_immediate_value(&self) -> Byte {
        self._get_first_arg()
    }

    fn get_zero_page_value(&self) -> Byte {
        self._get_first_arg()
    }

    fn get_zero_page_x_value(&self) -> u16 {
        self._get_first_arg().get_value() as u16 + self.reg_x.get_value() as u16
        // TODO : Wrap around when reaching u8::MAX like in get_zero_range_y_value ?
    }

    fn get_zero_page_y_value(&self) -> u8 {
        ((self._get_first_arg().get_value() as u16 + self.reg_y.get_value() as u16) % u8::MAX as u16) as u8
    }

    fn get_relative_value(&self) -> i8 {
        self._get_first_arg().get_i8()
    }

    fn get_absolute_value(&self) -> u16 {
        let memory_addr_slice: [u8; 2] = [self._get_first_arg().get_value(),
                                          self._get_second_arg().get_value()];
        LittleEndian::read_u16(&memory_addr_slice)
    }

    fn get_absolute_value_x(&self) -> u16 {
        self.get_absolute_value() + self.reg_x.get_value() as u16
    }

    fn get_absolute_value_y(&self) -> u16 {
        self.get_absolute_value() + self.reg_y.get_value() as u16
    }

    fn set_negative_flag(&mut self, b: Byte) {
        if b.is_negative() {
            self.flag_zero = true;
        }
        // TODO : Set to false if not ?
    }

    fn set_zero_flag(&mut self, b: Byte) {
        if b.get_value() == 0 {
            self.flag_zero = true;
        }
        // TODO : Set to false if not ?
    }

    pub fn execute_instruction(&mut self) {
        let opcode = self.memory[self.program_counter as usize];

        match opcode.get_value() {
            0x00 => { // BRK
                // TODO : Handle this better
                // TODO : Set flags accordingly
                panic!("Break !")
            },
            0xAA => { // TAX
                self.reg_x = self.reg_a.clone();
                self.set_negative_flag(self.reg_x);
                self.set_zero_flag(self.reg_x);

                self.program_counter += 1;
            },
            0xA8 => { // TAY
                self.reg_y = self.reg_a.clone();
                self.set_negative_flag(self.reg_y);
                self.set_zero_flag(self.reg_y);

                self.program_counter += 1;
            },
            0x8A => { // TXA
                self.reg_a = self.reg_x.clone();
                self.set_negative_flag(self.reg_a);
                self.set_zero_flag(self.reg_a);

                self.program_counter += 1;
            },
            0x98 => { // TYA
                self.reg_a = self.reg_y.clone();
                self.set_negative_flag(self.reg_a);
                self.set_zero_flag(self.reg_a);

                self.program_counter += 1;
            },
            0x78 => { // SEI
                self.flag_interrupt_disable = true;
            },
            0xF8 => { // SED
                self.flag_decimal_mode = true;
            },
            0x38 => { // SEC
                self.flag_carry = true;
            },
            0xEA => { // NOP
                ()
            },
            0xA9 => { // LDA - Immediate
                self.reg_a = self.get_immediate_value().clone();

                self.set_negative_flag(self.reg_a);
                self.set_zero_flag(self.reg_a);

                self.program_counter += 2;
            },
            0xA5 => { // LDA - Zero Pange
                self.reg_a = self.memory[self.get_zero_page_value()];

                self.set_negative_flag(self.reg_a);
                self.set_zero_flag(self.reg_a);

                self.program_counter += 2;
            },
            0x8D => { // STA - Absolute
                let memory_addr = self.get_absolute_value();

                self.memory[memory_addr as usize] = self.reg_a;
                self.program_counter += 3;
            }
            _ => {
                error!("Unknown opcode {}", opcode.get_value());
            }
        }
    }
}


// Tests

#[test]
fn lda() {
    // Immediate
    let mut cpu = Cpu::new();

    cpu.memory[0x00] = 0xA9.into();
    cpu.memory[0x01] = 0x23.into();

    let mut before_pc = cpu.program_counter;
    cpu.execute_instruction();
    assert_eq!(cpu.program_counter, before_pc + 2);

    assert_eq!(cpu.reg_a.get_value(), 0x23);

    // Zero page
    cpu.memory[0x02] = 0xA5.into();
    cpu.memory[0x03] = 0xF0.into();
    cpu.memory[0xF0] = 0xAA.into();

    before_pc = cpu.program_counter;
    cpu.execute_instruction();
    assert_eq!(cpu.program_counter, before_pc + 2);

    assert_eq!(cpu.reg_a.get_value(), 0xAA);
}

#[test]
fn sta() {
    // Absolute
    let mut cpu = Cpu::new();
    cpu.reg_a = 0x52.into();

    cpu.memory[0x00] = 0x8D.into();
    cpu.memory[0x01] = 0x20.into();
    cpu.memory[0x02] = 0x10.into();

    assert_eq!(cpu._get_first_arg().get_value(), 0x20);
    assert_eq!(cpu._get_second_arg().get_value(), 0x10);
    assert_eq!(cpu.get_absolute_value(), 0x1020);

    let before_pc = cpu.program_counter;
    cpu.execute_instruction();
    assert_eq!(cpu.program_counter, before_pc + 3);

    assert_eq!(cpu.memory[0x1020], 0x52.into());
}