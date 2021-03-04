use std::fmt;

use byteorder::{ByteOrder, LittleEndian};

use super::consts;
use super::memory::Memory;
use super::byte::Byte;
use super::double::Double;
use super::errors::CpuError;

#[derive(Clone)]
pub struct Cpu {
    reg_a: Byte,
    reg_x: Byte,
    reg_y: Byte,
    program_counter: Double,
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
        // TODO : Add flags state
        write!(f, "A : {} | X : {} | Y : {} | PC : {}", self.reg_a, self.reg_x, self.reg_y, self.program_counter)
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO : Add flags state
        write!(f, "A : {} | X : {} | Y : {} | PC : {}", self.reg_a, self.reg_x, self.reg_y, self.program_counter)
    }
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            reg_a: Byte::new(0x00),
            reg_x: Byte::new(0x00),
            reg_y: Byte::new(0x00),
            program_counter: Double::new_from_u16(consts::PROGRAM_MEMORY_ADDR), //TODO : Change this according to program location
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

    // Getters
    pub fn get_memory_addr(&self, index: Double) -> Byte {
        self.memory[index]
    }

    pub fn set_memory_addr(&mut self, index: Double, b: Byte) {
        self.memory[index] = b;
    }

    pub fn get_program_counter(&self) -> Double {
        self.program_counter
    }

    pub fn get_reg_a(&self) -> Byte {
        self.reg_a
    }

    pub fn get_reg_x(&self) -> Byte {
        self.reg_x
    }

    pub fn get_reg_y(&self) -> Byte {
        self.reg_y
    }

    // Arguments parsing
    fn get_first_arg(&self) -> Byte {
        self.memory[(self.program_counter.get_value() + 1) as usize]
    }

    fn get_second_arg(&self) -> Byte {
        self.memory[(self.program_counter.get_value() + 2) as usize]
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

    fn get_absolute_value(&self) -> Double {
        // A memory address represented as two little endian bytes
        Double::new_from_significant(self.get_first_arg(), self.get_second_arg())
    }

    fn get_absolute_value_x(&self) -> Double {
        // Like absolute value, with reg_x appended to it
        self.get_absolute_value() + self.reg_x.get_value() as u16
    }

    fn get_absolute_value_y(&self) -> Double {
        // Like absolute value, with reg_y appended to it
        self.get_absolute_value() + self.reg_y.get_value() as u16
    }

    fn get_indirect(&self) -> Double {
        // The two argument bytes are the memory address of the memory address
        // This function return the latter

        let first_memory_addr = Double::new_from_significant(self.get_first_arg(), self.get_second_arg());

        Double::new_from_significant(self.memory[first_memory_addr], self.memory[first_memory_addr + 1])
    }

    fn get_indexed_indirect_x(&self) -> Double {
        let first_memory_addr = Byte::new(self.get_first_arg().get_value() + self.reg_x.get_value()); // TODO : Zero page wrap
        let start_addr = self.memory[first_memory_addr];

        Double::new_from_significant(self.memory[start_addr], self.memory[start_addr.get_value() as u16 + 1])
    }

    fn get_indirect_indexed_y(&self) -> Double {
        let start_addr = self.get_first_arg();
        Double::new_from_significant(self.memory[start_addr], self.memory[start_addr.get_value() as u16 + 1]) 
            + self.reg_x.get_value() as u16
    }

    // Utils for flag usage
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

    // Instruction parser
    pub fn execute_instruction(&mut self) -> Result<(), CpuError> {
        let opcode = self.memory[self.program_counter];

        log::trace!("PC : {}, OP : {}", self.program_counter, opcode);

        match opcode.get_value() {
            0x00 => { // BRK
                // TODO : Set flags accordingly
                info!("Break opcode");
                return Err(CpuError::BreakError(self.clone()));
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
                self.program_counter += 1;
            },
            0xF8 => { // SED
                self.flag_decimal_mode = true;
                self.program_counter += 1;
            },
            0x38 => { // SEC
                self.flag_carry = true;
                self.program_counter += 1;
            },
            0xEA => { // NOP
                self.program_counter += 1;
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

                self.program_counter += 1;
            },
            0x68 => { // PLA
                self.stack_pointer = Byte::new(self.stack_pointer.get_value() + 1);
                self.reg_a = self.memory[self.stack_pointer];

                self.set_zero_flag(self.reg_a);
                self.set_negative_flag(self.reg_a);

                self.program_counter += 1;
            },
            0xE8 => { // INX
                self.reg_x.set_value(self.reg_x.get_value() + 1);

                self.set_zero_flag(self.reg_x);
                self.set_negative_flag(self.reg_x);

                self.program_counter += 1;
            },
            0x69 => { // ADC - Immediate
                let add_result = self.reg_a.get_value().overflowing_add(self.get_immediate_value().get_value());

                self.reg_a = Byte::new(add_result.0);
                self.flag_carry = add_result.1;
                
                self.program_counter += 2;
            },
            0x65 => { //ADC - Zero page
                let add_result = self.reg_a.get_value().overflowing_add(self.get_zero_page_value().get_value());

                self.reg_a = Byte::new(add_result.0);
                self.flag_carry = add_result.1;
                
                self.program_counter += 2;
            },
            0x75 => { //ADC - Zero page, X
                let add_result = self.reg_a.get_value().overflowing_add(self.get_zero_page_x_value().get_value());

                self.reg_a = Byte::new(add_result.0);
                self.flag_carry = add_result.1;
                
                self.program_counter += 2;
            },
            0x6D => { //ADC - Absolute
                let memory_addr = self.get_absolute_value();
                let add_result = self.reg_a.get_value().overflowing_add(self.get_memory_addr(memory_addr).get_value());

                self.reg_a = Byte::new(add_result.0);
                self.flag_carry = add_result.1;
                
                self.program_counter += 3;
            },
            0x7D => { //ADC - Absolute, X
                let memory_addr = self.get_absolute_value_x();
                let add_result = self.reg_a.get_value().overflowing_add(self.get_memory_addr(memory_addr).get_value());

                self.reg_a = Byte::new(add_result.0);
                self.flag_carry = add_result.1;
                
                self.program_counter += 3;
            },
            0x79 => { //ADC - Absolute, Y
                let memory_addr = self.get_absolute_value_y();
                let add_result = self.reg_a.get_value().overflowing_add(self.get_memory_addr(memory_addr).get_value());

                self.reg_a = Byte::new(add_result.0);
                self.flag_carry = add_result.1;
                
                self.program_counter += 3;
            },
            0x61 => { //ADC - (Indirect, X)
                let memory_addr = self.get_indexed_indirect_x();
                let add_result = self.reg_a.get_value().overflowing_add(self.get_memory_addr(memory_addr).get_value());

                self.reg_a = Byte::new(add_result.0);
                self.flag_carry = add_result.1;
                
                self.program_counter += 2;
            },
            0x71 => { //ADC - (Indirect), Y
                let memory_addr = self.get_indirect_indexed_y();
                let add_result = self.reg_a.get_value().overflowing_add(self.get_memory_addr(memory_addr).get_value());

                self.reg_a = Byte::new(add_result.0);
                self.flag_carry = add_result.1;
                
                self.program_counter += 2;
            },
            _ => {
                error!("Unknown opcode {}", opcode.get_value());
                return Err(CpuError::UnknownOpcodeError(self.clone()));
            }
        }

        Ok(())
    }
}


// Tests

#[test]
fn lda() {
    // Immediate
    let mut cpu = Cpu::new();

    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x00 as u16] = 0xA9.into(); // Instruction
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x01 as u16] = 0x23.into(); // Literal

    let mut before_pc = cpu.program_counter;
    let _ = cpu.execute_instruction();
    assert_eq!(cpu.program_counter, before_pc + 2);

    assert_eq!(cpu.reg_a.get_value(), 0x23);

    // Zero page
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x02 as u16] = 0xA5.into(); // Instruction
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x03 as u16] = 0xF0.into(); // Zero page address
    cpu.memory[0xF0 as u16] = 0xAA.into(); // Zero page value

    before_pc = cpu.program_counter;
    let _ = cpu.execute_instruction();
    assert_eq!(cpu.program_counter, before_pc + 2);

    assert_eq!(cpu.reg_a.get_value(), 0xAA);

    // Zero page X
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x04 as u16] = 0xB5.into(); // Instruction
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x05 as u16] = 0xF0.into(); // Zero page address
    cpu.memory[0xF1 as u16] = 0xCC.into(); // Zero page value at (zero page address + x register)

    cpu.reg_x = 0x01.into(); // X register value

    before_pc = cpu.program_counter;
    let _ = cpu.execute_instruction();
    assert_eq!(cpu.program_counter, before_pc + 2);

    assert_eq!(cpu.reg_a.get_value(), 0xCC);

    // Absolute
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x06 as u16] = 0xAD.into(); // Instruction
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x07 as u16] = 0x00.into(); // Least significant byte of memory address
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x08 as u16] = 0xC0.into(); // Most significant byte of memory address
    
    cpu.memory[0xC000 as u16] = 0x53.into(); // Value at memory address

    assert_eq!(cpu.get_first_arg(), Byte::new(0x00));
    assert_eq!(cpu.get_second_arg(), Byte::new(0xC0));
    assert_eq!(cpu.get_absolute_value(), Double::new_from_u16(0xC000));

    before_pc = cpu.program_counter;
    let _ = cpu.execute_instruction();
    assert_eq!(cpu.program_counter, before_pc + 3);

    assert_eq!(cpu.reg_a.get_value(), 0x53);

    // Absolute X

    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x09 as u16] = 0xBD.into(); // Instruction
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x0A as u16] = 0x00.into(); // Least significant byte of memory address
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x0B as u16] = 0xC0.into(); // Most significant byte of memory address

    cpu.reg_x = 0x01.into(); // x register value
    
    assert_eq!(cpu.get_first_arg().get_value(), 0x00);
    assert_eq!(cpu.get_second_arg().get_value(), 0xC0);
    assert_eq!(cpu.get_absolute_value(), Double::new_from_u16(0xC000));
    assert_eq!(cpu.get_absolute_value_x(), Double::new_from_u16(0xC001));

    cpu.memory[0xC001 as u16] = 0x80.into(); // Value at memory address + x register
    

    before_pc = cpu.program_counter;
    let _ = cpu.execute_instruction();
    assert_eq!(cpu.program_counter, before_pc + 3);

    assert_eq!(cpu.reg_a.get_value(), 0x80);

    // Absolute Y

    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x0C as u16] = 0xB9.into(); // Instruction
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x0D as u16] = 0x00.into(); // Least significant byte of memory address
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x0E as u16] = 0xC0.into(); // Most significant byte of memory address

    cpu.reg_y = 0x02.into(); // y register value
    
    assert_eq!(cpu.get_first_arg().get_value(), 0x00);
    assert_eq!(cpu.get_second_arg().get_value(), 0xC0);
    assert_eq!(cpu.get_absolute_value(), Double::new_from_u16(0xC000));
    assert_eq!(cpu.get_absolute_value_y(), Double::new_from_u16(0xC002));

    cpu.memory[0xC002 as u16] = 0xF3.into(); // Value at memory address + y register
    
    before_pc = cpu.program_counter;
    let _ = cpu.execute_instruction();
    assert_eq!(cpu.program_counter, before_pc + 3);

    assert_eq!(cpu.reg_a.get_value(), 0xF3);
}

#[test]
fn sta() {
    // Absolute
    let mut cpu = Cpu::new();
    cpu.reg_a = 0x52.into();

    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x00 as u16] = 0x8D.into();
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x01 as u16] = 0x20.into();
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x02 as u16] = 0x10.into();

    assert_eq!(cpu.get_first_arg().get_value(), 0x20);
    assert_eq!(cpu.get_second_arg().get_value(), 0x10);
    assert_eq!(cpu.get_absolute_value(), Double::new_from_u16(0x1020));

    let before_pc = cpu.program_counter;
    let _ = cpu.execute_instruction();
    assert_eq!(cpu.program_counter, before_pc + 3);

    assert_eq!(cpu.memory[0x1020 as u16], 0x52.into());
}

fn general_test_util(program: &str) -> Cpu {
    // Creates a cpu, loads the program to the correct memory address and run the program until a break occures
    let program_hex_strings: Vec<&str> = program.split(" ").collect();

    let mut cpu = Cpu::new();

    for (index, value) in program_hex_strings.iter().enumerate() {
        cpu.set_memory_addr(Double::new_from_u16(consts::PROGRAM_MEMORY_ADDR + index as u16), 
                            u8::from_str_radix(value, 16).unwrap().into());
    }

    loop {
        log::info!("{}", cpu);
        let execute_result = cpu.execute_instruction();
        match execute_result {
            Ok(()) => (),
            Err(err) => {
                match err {
                    CpuError::BreakError(_) => {
                        break;
                    },
                    CpuError::UnknownOpcodeError(cpu) => {
                        panic!("Unknown opcode reached : {}, opcode : {}", cpu, cpu.memory[cpu.program_counter]);
                    }
                }
            }
        };
    }

    return cpu;
}


#[test]
fn general_test_1() {
    let program_string = "a9 01 8d 00 02 a9 05 8d 01 02 a9 08 8d 02 02";
    let cpu = general_test_util(program_string);

    assert_eq!(cpu.get_memory_addr(Double::new_from_u16(0x0200)), Byte::new(0x01));
    assert_eq!(cpu.get_memory_addr(Double::new_from_u16(0x0201)), Byte::new(0x05));
    assert_eq!(cpu.get_memory_addr(Double::new_from_u16(0x0202)), Byte::new(0x08));
    assert_eq!(cpu.reg_a, Byte::new(0x08));
    assert_eq!(cpu.reg_x, Byte::new(0x00));
    assert_eq!(cpu.reg_y, Byte::new(0x00));
    assert_eq!(cpu.stack_pointer, Byte::new(0xff));

    // TODO : Check flag state
}

#[test]
fn general_test_2() {
    let program_string = "a9 c0 aa e8 69 c4 00";
    let cpu = general_test_util(program_string);

    assert_eq!(cpu.reg_a, Byte::new(0x84));
    assert_eq!(cpu.reg_x, Byte::new(0xC1));
    assert_eq!(cpu.stack_pointer, Byte::new(0xff));
    assert_eq!(cpu.flag_carry, true);

    // TODO : Check flag state
}