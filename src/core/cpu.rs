use std::fmt;

use byteorder::{ByteOrder, LittleEndian};

use super::consts;
use super::memory::Memory;
use super::byte::Byte;

pub struct Cpu {
    reg_a: Byte,
    reg_x: Byte,
    reg_y: Byte,
    program_counter: u16,
    stack_pointer: Byte,
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
            stack_pointer: Byte::new(consts::STACK_SIZE),
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

    fn get_first_arg(&self) -> Byte {
        self.memory[(self.program_counter + 1) as usize]
    }

    fn get_second_arg(&self) -> Byte {
        self.memory[(self.program_counter + 2) as usize]
    }

    fn get_immediate_value(&self) -> Byte {
        // Return the value of the first argument
        // Immediate means literal, this is the final value of the operation
        self.get_first_arg()
    }

    fn get_zero_page_value(&self) -> Byte {
        // Return the value of the first argument
        // Zero page means the address in the first page to refer to, your value will be there
        self.get_first_arg()
    }

    fn get_zero_page_x_value(&self) -> Byte {
        // Like zero_page, but reg_x is appended to it

        Byte::new(((self.get_first_arg().get_value() as u16 + self.reg_x.get_value() as u16) % u8::MAX as u16) as u8)
    }

    fn get_zero_page_y_value(&self) -> Byte {
        // Like zero_page, but reg_y is appended to it
        Byte::new(((self.get_first_arg().get_value() as u16 + self.reg_y.get_value() as u16) % u8::MAX as u16) as u8)
    }

    fn get_relative_value(&self) -> i8 {
        // Return the value of the first arg as i8
        // This is a relative value representing where is your value compared to PC

        self.get_first_arg().get_i8()
    }

    fn get_absolute_value(&self) -> u16 {
        // A memory address represented as two little endian bytes

        let memory_addr_slice: [u8; 2] = [self.get_first_arg().get_value(),
                                          self.get_second_arg().get_value()];
        LittleEndian::read_u16(&memory_addr_slice)
    }

    fn get_absolute_value_x(&self) -> u16 {
        // Like absolute value, with reg_x appended to it

        self.get_absolute_value() + self.reg_x.get_value() as u16
    }

    fn get_absolute_value_y(&self) -> u16 {
        // Like absolute value, with reg_y appended to it
        self.get_absolute_value() + self.reg_y.get_value() as u16
    }


    fn get_indirect(&self) -> u16 {
        // The two argument bytes are the memory address of the memory address
        // This function return the latter

        let first_memory_addr_slice: [u8; 2] = [self.get_first_arg().get_value(),
                                          self.get_second_arg().get_value()];

        let first_memory_addr = LittleEndian::read_u16(&first_memory_addr_slice);

        let second_memory_addr_slice: [u8; 2] = [self.memory[first_memory_addr].get_value(),
                                                 self.memory[first_memory_addr + 1].get_value()];

        LittleEndian::read_u16(&second_memory_addr_slice)
    }

    fn get_indexed_indirect_x(&self) -> u16 {
        let first_memory_addr = self.get_first_arg().get_value() + self.reg_x.get_value(); // TODO : Zero page wrap
        let start_addr = self.memory[first_memory_addr as usize];

        let memory_addr_slice: [u8; 2] = [self.memory[start_addr].get_value(),
                                          self.memory[start_addr.get_value() as usize + 1].get_value()];
                                          
        LittleEndian::read_u16(&memory_addr_slice)
    }

    fn get_indirect_indexed_y(&self) -> u16 {
        let start_addr = self.get_first_arg();

        let memory_addr_slice: [u8; 2] = [self.memory[start_addr].get_value(),
                                          self.memory[start_addr.get_value() as usize + 1].get_value()];
        
        LittleEndian::read_u16(&memory_addr_slice) + self.reg_y.get_value() as u16
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
        let opcode = self.memory[self.program_counter];

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
            0xA5 => { // LDA - Zero Page
                self.reg_a = self.memory[self.get_zero_page_value()];

                self.set_negative_flag(self.reg_a);
                self.set_zero_flag(self.reg_a);

                self.program_counter += 2;
            },
            0xB5 => { // LDA - Zero Page, X
                self.reg_a = self.memory[self.get_zero_page_x_value()];

                self.set_negative_flag(self.reg_a);
                self.set_zero_flag(self.reg_a);

                self.program_counter += 2;
            },
            0xAD => { // LDA - Absolute
                self.reg_a = self.memory[self.get_absolute_value()];

                self.set_negative_flag(self.reg_a);
                self.set_zero_flag(self.reg_a);

                self.program_counter += 3;
            },
            0xBD => { // LDA - Absolute, X
                self.reg_a = self.memory[self.get_absolute_value_x()];

                self.set_negative_flag(self.reg_a);
                self.set_zero_flag(self.reg_a);

                self.program_counter += 3;
            },
            0xB9 => { // LDA - Absolute, Y
                self.reg_a = self.memory[self.get_absolute_value_y()];

                self.set_negative_flag(self.reg_a);
                self.set_zero_flag(self.reg_a);

                self.program_counter += 3;
            },
            0xA1 => { // LDA - (Indirect, X)
                self.reg_a = self.memory[self.get_indexed_indirect_x()];
                
                self.set_negative_flag(self.reg_a);
                self.set_zero_flag(self.reg_a);

                self.program_counter += 2;
            },
            0xB1 => { // LDA - (Indirect), Y
                self.reg_a = self.memory[self.get_indirect_indexed_y()];

                self.set_negative_flag(self.reg_a);
                self.set_zero_flag(self.reg_a);

                self.program_counter += 2;
            },
            0x85 => { // STA - Zero page
                let memory_addr = self.get_zero_page_value();

                self.memory[memory_addr.get_value() as usize] = self.reg_a;
                self.program_counter += 2;

            },
            0x95 => { // STA - Zero page, X
                let memory_addr = self.get_zero_page_x_value();

                self.memory[memory_addr.get_value() as usize] = self.reg_a;
                self.program_counter += 2;
            },
            0x8D => { // STA - Absolute
                let memory_addr = self.get_absolute_value();

                self.memory[memory_addr] = self.reg_a;
                self.program_counter += 3;
            },
            0x9D => { // STA - Absolute X
                let memory_addr = self.get_absolute_value_x();

                self.memory[memory_addr] = self.reg_a;
                self.program_counter += 3;
            },
            0x99 => { // STA - Absolute Y
                let memory_addr = self.get_absolute_value_y();

                self.memory[memory_addr] = self.reg_a;
                self.program_counter += 3;
            },
            0x81 => { // STA - (Indirect, X)
                let memory_addr = self.get_indexed_indirect_x();

                self.memory[memory_addr] = self.reg_a;
                self.program_counter += 2;
            },
            0x91 => { // STA - (Indirect), Y
                let memory_addr = self.get_indirect_indexed_y();

                self.memory[memory_addr] = self.reg_a;
                self.program_counter += 2;
            },
            0x18 => { // CLS
                self.flag_carry = false;
                self.program_counter += 1;
            },
            0xD8 => { // CLD
                self.flag_decimal_mode = false;
                self.program_counter += 1;
            },
            0x58 => { // CLI
                self.flag_interrupt_disable = false;
                self.program_counter += 1;
            },
            0xB8 => { // CLV
                self.flag_overflow = false;
                self.program_counter += 1;
            },
            0xC6 => { // DEC - Zero page
                let memory_addr = self.get_zero_page_value().get_value() as usize;

                self.memory[memory_addr] = Byte::new(self.memory[memory_addr].get_value() - 1);

                self.set_zero_flag(self.memory[memory_addr]);
                self.set_negative_flag(self.memory[memory_addr]);

                self.program_counter += 2;
            },
            0xD6 => { // DEC - Zero page X
                let memory_addr = self.get_zero_page_x_value().get_value() as usize;

                self.memory[memory_addr] = Byte::new(self.memory[memory_addr].get_value() - 1);

                self.set_zero_flag(self.memory[memory_addr]);
                self.set_negative_flag(self.memory[memory_addr]);

                self.program_counter += 2;
            },
            0xCE => { // DEC - Absolute
                let memory_addr = self.get_absolute_value();

                self.memory[memory_addr] = Byte::new(self.memory[memory_addr].get_value() - 1);

                self.set_zero_flag(self.memory[memory_addr]);
                self.set_negative_flag(self.memory[memory_addr]);

                self.program_counter += 3;
            },
            0xDE => { // DEC - Absolute X
                let memory_addr = self.get_absolute_value_x();

                self.memory[memory_addr] = Byte::new(self.memory[memory_addr].get_value() - 1);

                self.set_zero_flag(self.memory[memory_addr]);
                self.set_negative_flag(self.memory[memory_addr]);

                self.program_counter += 3;
            },
            0x48 => { // PHA
                self.memory[consts::STACK_ADDR + self.stack_pointer.get_value() as u16] = self.reg_a;
                self.stack_pointer = Byte::new(self.stack_pointer.get_value() - 1);
            },
            0x68 => { // PLA
                self.stack_pointer = Byte::new(self.stack_pointer.get_value() + 1);
                self.reg_a = self.memory[self.stack_pointer];

                self.set_zero_flag(self.reg_a);
                self.set_negative_flag(self.reg_a);
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

    cpu.memory[0x00 as usize] = 0xA9.into(); // Instruction
    cpu.memory[0x01 as usize] = 0x23.into(); // Literal

    let mut before_pc = cpu.program_counter;
    cpu.execute_instruction();
    assert_eq!(cpu.program_counter, before_pc + 2);

    assert_eq!(cpu.reg_a.get_value(), 0x23);

    // Zero page
    cpu.memory[0x02 as usize] = 0xA5.into(); // Instruction
    cpu.memory[0x03 as usize] = 0xF0.into(); // Zero page address
    cpu.memory[0xF0 as usize] = 0xAA.into(); // Zero page value

    before_pc = cpu.program_counter;
    cpu.execute_instruction();
    assert_eq!(cpu.program_counter, before_pc + 2);

    assert_eq!(cpu.reg_a.get_value(), 0xAA);

    // Zero page X
    cpu.memory[0x04 as usize] = 0xB5.into(); // Instruction
    cpu.memory[0x05 as usize] = 0xF0.into(); // Zero page address
    cpu.memory[0xF1 as usize] = 0xCC.into(); // Zero page value at (zero page address + x register)

    cpu.reg_x = 0x01.into(); // X register value

    before_pc = cpu.program_counter;
    cpu.execute_instruction();
    assert_eq!(cpu.program_counter, before_pc + 2);

    assert_eq!(cpu.reg_a.get_value(), 0xCC);

    // Absolute
    cpu.memory[0x06 as usize] = 0xAD.into(); // Instruction
    cpu.memory[0x07 as usize] = 0x00.into(); // Least significant byte of memory address
    cpu.memory[0x08 as usize] = 0xC0.into(); // Most significant byte of memory address
    
    cpu.memory[0xC000 as usize] = 0x53.into(); // Value at memory address

    before_pc = cpu.program_counter;
    cpu.execute_instruction();
    assert_eq!(cpu.program_counter, before_pc + 3);

    assert_eq!(cpu.reg_a.get_value(), 0x53);

    // Absolute X

    cpu.memory[0x09 as usize] = 0xBD.into(); // Instruction
    cpu.memory[0x0A as usize] = 0x00.into(); // Least significant byte of memory address
    cpu.memory[0x0B as usize] = 0xC0.into(); // Most significant byte of memory address

    cpu.reg_x = 0x01.into(); // x register value
    
    assert_eq!(cpu.get_first_arg().get_value(), 0x00);
    assert_eq!(cpu._get_second_arg().get_value(), 0xC0);
    assert_eq!(cpu.get_absolute_value(), 0xC000);
    assert_eq!(cpu.get_absolute_value_x(), 0xC001);

    cpu.memory[0xC001 as usize] = 0x80.into(); // Value at memory address + x register
    

    before_pc = cpu.program_counter;
    cpu.execute_instruction();
    assert_eq!(cpu.program_counter, before_pc + 3);

    assert_eq!(cpu.reg_a.get_value(), 0x80);

    // Absolute Y

    cpu.memory[0x0C as usize] = 0xB9.into(); // Instruction
    cpu.memory[0x0D as usize] = 0x00.into(); // Least significant byte of memory address
    cpu.memory[0x0E as usize] = 0xC0.into(); // Most significant byte of memory address

    cpu.reg_y = 0x02.into(); // y register value
    
    assert_eq!(cpu.get_first_arg().get_value(), 0x00);
    assert_eq!(cpu._get_second_arg().get_value(), 0xC0);
    assert_eq!(cpu.get_absolute_value(), 0xC000);
    assert_eq!(cpu.get_absolute_value_y(), 0xC002);

    cpu.memory[0xC002 as usize] = 0xF3.into(); // Value at memory address + y register
    
    before_pc = cpu.program_counter;
    cpu.execute_instruction();
    assert_eq!(cpu.program_counter, before_pc + 3);

    assert_eq!(cpu.reg_a.get_value(), 0xF3);
}

#[test]
fn sta() {
    // Absolute
    let mut cpu = Cpu::new();
    cpu.reg_a = 0x52.into();

    cpu.memory[0x00 as usize] = 0x8D.into();
    cpu.memory[0x01 as usize] = 0x20.into();
    cpu.memory[0x02 as usize] = 0x10.into();

    assert_eq!(cpu.get_first_arg().get_value(), 0x20);
    assert_eq!(cpu._get_second_arg().get_value(), 0x10);
    assert_eq!(cpu.get_absolute_value(), 0x1020);

    let before_pc = cpu.program_counter;
    cpu.execute_instruction();
    assert_eq!(cpu.program_counter, before_pc + 3);

    assert_eq!(cpu.memory[0x1020 as usize], 0x52.into());
}