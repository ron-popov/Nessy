use std::fmt;

use std::collections::HashMap;

use crate::core::consts;
use crate::core::memory::Memory;
use crate::core::Byte;
use crate::core::Double;
use crate::mapper::Mapper;

use super::CpuError;
use super::instructions::{Instruction, get_instruction_set, get_unknown_instruction};

extern crate simplelog;
use simplelog::{ConfigBuilder, Level, CombinedLogger, TermLogger, LevelFilter, TerminalMode, Color};


// #[derive(Clone)]
pub struct Cpu {
    reg_a: Byte,
    reg_x: Byte,
    reg_y: Byte,
    program_counter: Double,
    stack_pointer: Byte,
    // memory: Memory,

    flag_carry: bool,
    flag_zero: bool,
    flag_interrupt_disable: bool,
    flag_decimal_mode: bool,
    flag_break: bool,
    flag_overflow: bool,
    flag_negative: bool,

    cycle_counter: usize,

    instruction_set: HashMap<u8, Instruction>,
    current_opcode: Byte,
    mapper: Box<dyn Mapper>,
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{} -> {}", self.program_counter, self.get_memory_addr(self.program_counter))
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{} -> {}", self.program_counter, self.get_memory_addr(self.program_counter))
    }
}

impl Cpu {
    pub fn new(mapper: Box<dyn Mapper>) -> Result<Cpu, CpuError> {
        // Calculate starting point
        let entry_point_least = mapper.get_memory_addr(0xFFFCu16.into());
        let entry_point_most = mapper.get_memory_addr(0xFFFDu16.into());

        if entry_point_least.is_err() || entry_point_most.is_err() {
            return Err(CpuError::FailedParsingEntryPoint)
        }

        let entry_point = Double::new_from_significant(entry_point_least.unwrap(), entry_point_most.unwrap());
        log::info!("Program Entry point is {}", entry_point);

        Ok(Cpu {
            reg_a: Byte::new(0x00),
            reg_x: Byte::new(0x00),
            reg_y: Byte::new(0x00),
            program_counter: entry_point,
            stack_pointer: Byte::new(consts::STACK_SIZE),
            mapper: mapper,
            flag_carry: false, // TODO : Verify flag start state
            flag_zero: false,
            flag_interrupt_disable: true,
            flag_decimal_mode: false,
            flag_break: false,
            flag_overflow: false,
            flag_negative: false,
            instruction_set: get_instruction_set(),
            cycle_counter:7,
            current_opcode: Byte::new(0x00),
        })
    }

    // Getters
    pub fn get_memory_addr(&self, index: Double) -> Byte {
        self.mapper.get_memory_addr(index).unwrap()
    }

    pub fn set_memory_addr(&mut self, index: Double, b: Byte) {
        self.mapper.set_memory_addr(index, b).unwrap()
    }

    pub fn get_program_counter(&self) -> Double {
        self.program_counter
    }

    pub fn set_program_counter(&mut self, program_counter: Double) {
        self.program_counter = program_counter;
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
        self.get_memory_addr(self.program_counter + 1)
    }

    fn get_second_arg(&self) -> Byte {
        self.get_memory_addr(self.program_counter + 2)
    }

    fn get_immediate_value(&self) -> Byte {
        // Return the value of the first argument
        // Immediate means literal, this is the final value of the operation
        self.get_first_arg()
    }

    fn get_zero_page_addr(&self) -> Byte {
        // Return the value of the first argument
        // Zero page means the address in the first page to refer to, your value will be there
        self.get_first_arg()
    }

    fn get_zero_page_x_addr(&self) -> Byte {
        // Like zero_page, but reg_x is appended to it
        Byte::new(self.get_zero_page_addr().get_value().wrapping_add(self.reg_x.get_value()))
    }

    fn get_zero_page_y_addr(&self) -> Byte {
        // Like zero_page, but reg_y is appended to it
        let start_addr = self.get_first_arg();
        log::trace!("Zero Page addr (For ZeroPage,Y) is {}", start_addr);

        let start_addr_y = Byte::new(start_addr.get_value().wrapping_add(self.reg_y.get_value()));
        log::trace!("Zero Page Y addr is {}", start_addr_y);

        return start_addr_y;
    }

    fn get_relative_addr(&self) -> i8 {
        // Return the value of the first arg as i8
        // This is a relative value representing where is your value compared to PC
        self.get_first_arg().get_i8()
    }

    fn get_absolute_addr(&self) -> Double {
        // A memory address represented as two little endian bytes
        let addr = Double::new_from_significant(self.get_first_arg(), self.get_second_arg());

        log::trace!("Absolute addr is {}", addr);
        log::trace!("Value at Absolute addr is {}", self.get_memory_addr(addr));

        return addr;
    }

    fn get_absolute_addr_x(&mut self) -> Double {
        // Like absolute value, with reg_x appended to it
        let absolute_addr = Double::new_from_significant(self.get_first_arg(), self.get_second_arg());
        let absolute_x_addr = Double::from(absolute_addr.get_value().wrapping_add(self.reg_x.get_value() as u16));

        if absolute_addr.get_most_significant() != absolute_x_addr.get_most_significant() {
            log::trace!("Crossed Page in absolute,X");
            if !consts::PAGE_CROSS_EXTRA_CYCLE_WHITELIST.contains(&self.current_opcode.get_value()) {
                log::trace!("Increasing cycle counter by one");
                self.cycle_counter += 1;
            } else {
                log::trace!("Opcode in extra cycle whitelist");
            }
        }

        return absolute_x_addr;
    }

    fn get_absolute_addr_y(&mut self) -> Double {
        // Like absolute value, with reg_y appended to it
        let absolute_addr = Double::new_from_significant(self.get_first_arg(), self.get_second_arg());
        let absolute_y_addr = Double::from(absolute_addr.get_value().wrapping_add(self.reg_y.get_value() as u16));

        if absolute_addr.get_most_significant() != absolute_y_addr.get_most_significant() {
            if !consts::PAGE_CROSS_EXTRA_CYCLE_WHITELIST.contains(&self.current_opcode.get_value()) {
                self.cycle_counter += 1;
            }
        }

        return absolute_y_addr;
    }

    fn get_indirect_addr(&self) -> Double {
        // The two argument bytes are the memory address of the memory address
        // This function return the latter

        let first_memory_addr = Double::new_from_significant(self.get_first_arg(), self.get_second_arg());
        let second_memory_addr = Double::new_from_significant(self.get_first_arg(), 
            self.get_second_arg().get_value().wrapping_add(1).into());

        let target_memory_addr = Double::new_from_significant(self.get_memory_addr(first_memory_addr), 
                    self.get_memory_addr(second_memory_addr));

        log::trace!("Indirect memory addr in {} -> {}", first_memory_addr, target_memory_addr);

        return target_memory_addr;
    }

    fn get_indexed_indirect_x_addr(&self) -> Double {
        let start_addr = self.get_zero_page_x_addr();
        
        log::trace!("ZeroPage,X Address (for Indirect,X) is {}", start_addr);

        let addr = Double::new_from_significant(self.get_memory_addr(start_addr.into()), 
            self.get_memory_addr(Byte::new(start_addr.get_value().wrapping_add(1)).into()));
        
        log::trace!("Indirect,X address is {} -> {}", addr, self.get_memory_addr(addr));
        return addr;
    }

    fn get_indirect_indexed_y_addr(&mut self) -> Double {
        let least_addr = self.get_first_arg();

        log::trace!("ZeroPage Address of Indirect,Y is {}", least_addr);

        let least = self.get_memory_addr(least_addr.into());
        let most = self.get_memory_addr(Byte::new(least_addr.get_value().wrapping_add(1)).into());

        let indirect_addr = Double::new_from_significant(least, most);
        log::trace!("Indirect address (of Indirect,Y) is {}", indirect_addr);

        let target_addr = Double::from(indirect_addr.get_value().wrapping_add(self.reg_y.get_value().into()));
        
        if most != target_addr.get_most_significant() {
            if !consts::PAGE_CROSS_EXTRA_CYCLE_WHITELIST.contains(&self.current_opcode.get_value()) {
                self.cycle_counter += 1;
            }
        }

        log::trace!("Indirect,Y address is {} -> {}", target_addr, self.get_memory_addr(target_addr));

        return target_addr;
    }

    // Utils for flag usage
    fn set_negative_flag(&mut self, b: Byte) {
        self.flag_negative = b.is_negative();
    }

    fn set_zero_flag(&mut self, b: Byte) {
        self.flag_zero = b.get_value() == 0;
    }

    fn push_stack(&mut self, value: Byte) -> std::result::Result<(), CpuError> {
        log::trace!("Pushing {} to stack", value);

        if self.stack_pointer.get_value() == 0 {
            return Err(CpuError::StackOverflow);
        }

        self.set_memory_addr(Double::from(consts::STACK_ADDR) + Double::from(self.stack_pointer), value);
        self.stack_pointer -= Byte::new(1);

        Ok(())
    }

    fn pop_stack(&mut self) -> std::result::Result<Byte, CpuError> {
        if self.stack_pointer.get_value() == consts::STACK_SIZE {
            return Err(CpuError::StackEmpty);
        }

        self.stack_pointer += Byte::new(1);
        let stack_value = self.get_memory_addr(Double::from(consts::STACK_ADDR) + Double::from(self.stack_pointer));
        
        log::trace!("Popped {} from stack", stack_value);

        Ok(stack_value)
    }

    fn get_processor_status_byte(&self) -> Byte {
        let mut new_byte_arr: [bool; 8] = [false; 8];

        new_byte_arr[0] = self.flag_carry;
        new_byte_arr[1] = self.flag_zero;
        new_byte_arr[2] = self.flag_interrupt_disable;
        new_byte_arr[3] = self.flag_decimal_mode;
        new_byte_arr[4] = self.flag_break;
        new_byte_arr[5] = true;
        new_byte_arr[6] = self.flag_overflow;
        new_byte_arr[7] = self.flag_negative;

        Byte::from_bool_array(new_byte_arr)
    }

    // Instruction shortcuts
    fn execute_sbc(&mut self, value: Byte) -> Result<(), CpuError> {
        self.execute_adc(Byte::new(0xFF) - value)?;
        Ok(())
    }

    fn execute_inc(&mut self, target_addr: Double) -> Result<(), CpuError> {
        let new_value = Byte::new(self.get_memory_addr(target_addr).get_value().wrapping_add(1));
        self.set_memory_addr(target_addr, new_value);

        self.set_zero_flag(self.get_memory_addr(target_addr));
        self.set_negative_flag(self.get_memory_addr(target_addr));

        Ok(())
    }

    fn execute_asl(&mut self, mut value: Byte) -> Result<Byte, CpuError> {
        self.flag_carry = value[7];

        value <<= 1;

        self.set_negative_flag(value);
        self.set_zero_flag(value);

        Ok(value)
    }

    fn execute_ora(&mut self, value: Byte) -> Result<(), CpuError> {
        self.reg_a |= value;

        self.set_negative_flag(self.reg_a);
        self.set_zero_flag(self.reg_a);

        Ok(())
    }

    fn execute_rol(&mut self, value: Byte) -> Result<Byte, CpuError> {
        let value_arr: [bool; 8] = value.clone().as_array();

        let mut new_value_arr: [bool; 8] = [false; 8];
        new_value_arr[0] = self.flag_carry;
        for (i,x) in value_arr[0..7].iter().enumerate() {
            new_value_arr[i + 1] = *x;
        }

        self.flag_carry = value_arr[7];
        let new_value = Byte::from_bool_array(new_value_arr);

        self.set_negative_flag(new_value);
        self.set_zero_flag(new_value);

        Ok(new_value)
    }

    fn execute_ror(&mut self, value: Byte) -> Result<Byte, CpuError> {
        let value_arr: [bool; 8] = value.clone().as_array();


        let mut new_value_arr: [bool; 8] = [false; 8];
        new_value_arr[7] = self.flag_carry;
        for (i,x) in value_arr[1..8].iter().enumerate() {
            new_value_arr[i] = *x;
        }

        
        self.flag_carry = value_arr[0];
        let new_value = Byte::from_bool_array(new_value_arr);

        self.set_negative_flag(new_value);
        self.set_zero_flag(new_value);

        Ok(new_value)
    }

    fn execute_rla(&mut self, memory_addr: Double) -> Result<(), CpuError> {
        let value = self.get_memory_addr(memory_addr);

        let rol_output = self.execute_rol(value)?;
        self.set_memory_addr(memory_addr, rol_output);

        self.reg_a &= self.get_memory_addr(memory_addr);

        self.set_zero_flag(self.reg_a);
        self.set_negative_flag(self.reg_a);

        Ok(())
    }

    fn execute_adc(&mut self, value: Byte) -> Result<(), CpuError> {
        let add_result = self.reg_a.get_value().overflowing_add(value.get_value());
        let add_result_2 = add_result.0.overflowing_add(self.flag_carry as u8);

        // Taken from : http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html#:~:text=The%20definition%20of%20the%206502,%3E%20127%20or%20%3C%20%2D128.
        self.flag_overflow = ((self.reg_a.get_value() ^ add_result_2.0) & (value.get_value() ^ add_result_2.0) & 0x80) != 0;
        
        
        self.reg_a = Byte::new(add_result_2.0);
        self.flag_carry = add_result.1 | add_result_2.1;
        
        self.set_negative_flag(self.reg_a);
        self.set_zero_flag(self.reg_a);

        Ok(())
    }

    fn execute_branch(&mut self, flag: bool, offset: i8) -> Result<(), CpuError> {
        if flag {
            self.cycle_counter += 1;
            let page_before = self.program_counter.get_most_significant();

            self.program_counter = Double::from((self.program_counter.get_value() as i16 + offset as i16) as u16);

            if self.program_counter.get_most_significant() != page_before {
                self.cycle_counter += 1;
            }
        }

        Ok(())
    }

    fn log_instruction(&self) {
        let target_instruction = self.get_memory_addr(self.program_counter);
        let instruction: Instruction = self.instruction_set.get(&target_instruction.get_value())
            .unwrap_or(&get_unknown_instruction()).clone();
        
        
        let mut instruction_args = Vec::<String>::new();
        for x in self.program_counter.get_value()..self.program_counter.get_value() + instruction.bytes as u16 {
            instruction_args.push(format!("{:02X}", self.get_memory_addr(Double::from(x)).get_value()));
        }

        let instruction_args_string = format!("{:width$}", instruction_args.join(" "), width=12);

        log::trace!("{:X} -> {} {} | A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} CYC:{}", self.program_counter.get_value(), 
            format!("{:width$}", instruction.name, width=3), instruction_args_string,self.reg_a.get_value(), 
            self.reg_x.get_value(), self.reg_y.get_value(), self.get_processor_status_byte().get_value(), 
            self.stack_pointer.get_value(), self.cycle_counter);
    }

    fn increment_cycle(&mut self, opcode: Byte) {
        let instruction: Instruction = self.instruction_set.get(&opcode.get_value())
            .unwrap_or(&get_unknown_instruction()).clone();

        self.cycle_counter += instruction.cycles as usize;
    }

    // Instruction parser
    pub fn execute_instruction(&mut self) -> std::result::Result<(), CpuError> {
        //Precheks and logs
        let opcode = self.get_memory_addr(self.program_counter);
        self.current_opcode = opcode;

        self.log_instruction();
 
        // Executing instruction
        self.increment_cycle(opcode);

        match opcode.get_value() {
            0x00 => { //BRK
                // TODO : Set flags accordingly
                self.program_counter += 1;

                log::info!("Break opcode");
                return Err(CpuError::BreakError);
            },
            0xAA => { //TAX
                self.reg_x = self.reg_a.clone();
                self.set_negative_flag(self.reg_x);
                self.set_zero_flag(self.reg_x);

                self.program_counter += 1;
            },
            0xA8 => { //TAY
                self.reg_y = self.reg_a.clone();
                self.set_negative_flag(self.reg_y);
                self.set_zero_flag(self.reg_y);

                self.program_counter += 1;
            },
            0x8A => { //TXA
                self.reg_a = self.reg_x.clone();
                self.set_negative_flag(self.reg_a);
                self.set_zero_flag(self.reg_a);

                self.program_counter += 1;
            },
            0x98 => { //TYA
                self.reg_a = self.reg_y.clone();
                self.set_negative_flag(self.reg_a);
                self.set_zero_flag(self.reg_a);

                self.program_counter += 1;
            },
            0x78 => { //SEI
                self.flag_interrupt_disable = true;
                self.program_counter += 1;
            },
            0xF8 => { //SED
                self.flag_decimal_mode = true;
                self.program_counter += 1;
            },
            0x38 => { //SEC
                self.flag_carry = true;
                self.program_counter += 1;
            },
            0xEA => { //NOP
                self.program_counter += 1;
            },
            0xA9 => { //LDA - Immediate
                self.reg_a = self.get_immediate_value().clone();

                self.set_negative_flag(self.reg_a);
                self.set_zero_flag(self.reg_a);

                self.program_counter += 2;
            },
            0xA5 => { //LDA - Zero Page
                self.reg_a = self.get_memory_addr(self.get_zero_page_addr().into());

                self.set_negative_flag(self.reg_a);
                self.set_zero_flag(self.reg_a);

                self.program_counter += 2;
            },
            0xB5 => { //LDA - Zero Page, X
                self.reg_a = self.get_memory_addr(self.get_zero_page_x_addr().into());

                self.set_negative_flag(self.reg_a);
                self.set_zero_flag(self.reg_a);

                self.program_counter += 2;
            },
            0xAD => { //LDA - Absolute
                self.reg_a = self.get_memory_addr(self.get_absolute_addr());

                self.set_negative_flag(self.reg_a);
                self.set_zero_flag(self.reg_a);

                self.program_counter += 3;
            },
            0xBD => { //LDA - Absolute, X
                let memory_addr = self.get_absolute_addr_x();
                self.reg_a = self.get_memory_addr(memory_addr);

                self.set_negative_flag(self.reg_a);
                self.set_zero_flag(self.reg_a);

                self.program_counter += 3;
            },
            0xB9 => { //LDA - Absolute, Y
                let memory_addr = self.get_absolute_addr_y();
                self.reg_a = self.get_memory_addr(memory_addr);

                self.set_negative_flag(self.reg_a);
                self.set_zero_flag(self.reg_a);

                self.program_counter += 3;
            },
            0xA1 => { //LDA - (Indirect, X)
                self.reg_a = self.get_memory_addr(self.get_indexed_indirect_x_addr());
                
                self.set_negative_flag(self.reg_a);
                self.set_zero_flag(self.reg_a);

                self.program_counter += 2;
            },
            0xB1 => { //LDA - (Indirect), Y
                let target_addr = self.get_indirect_indexed_y_addr();
                self.reg_a = self.get_memory_addr(target_addr);

                self.set_negative_flag(self.reg_a);
                self.set_zero_flag(self.reg_a);

                self.program_counter += 2;
            },
            0x85 => { //STA - Zero page
                let memory_addr = self.get_zero_page_addr();

                self.set_memory_addr(memory_addr.into(), self.reg_a);
                self.program_counter += 2;

            },
            0x95 => { //STA - Zero page, X
                let memory_addr = self.get_zero_page_x_addr();

                self.set_memory_addr(memory_addr.into(), self.reg_a);
                self.program_counter += 2;
            },
            0x8D => { //STA - Absolute
                let memory_addr = self.get_absolute_addr();

                self.set_memory_addr(memory_addr, self.reg_a);
                self.program_counter += 3;
            },
            0x9D => { //STA - Absolute X
                let memory_addr = self.get_absolute_addr_x();

                self.set_memory_addr(memory_addr, self.reg_a);
                self.program_counter += 3;
            },
            0x99 => { //STA - Absolute Y
                let memory_addr = self.get_absolute_addr_y();

                self.set_memory_addr(memory_addr, self.reg_a);
                self.program_counter += 3;
            },
            0x81 => { //STA - (Indirect, X)
                let memory_addr = self.get_indexed_indirect_x_addr();

                self.set_memory_addr(memory_addr, self.reg_a);
                self.program_counter += 2;
            },
            0x91 => { //STA - (Indirect), Y
                let memory_addr = self.get_indirect_indexed_y_addr();

                self.set_memory_addr(memory_addr, self.reg_a);
                self.program_counter += 2;
            },
            0x18 => { //CLS
                self.flag_carry = false;
                self.program_counter += 1;
            },
            0xD8 => { //CLD
                self.flag_decimal_mode = false;
                self.program_counter += 1;
            },
            0x58 => { //CLI
                self.flag_interrupt_disable = false;
                self.program_counter += 1;
            },
            0xB8 => { //CLV
                self.flag_overflow = false;
                self.program_counter += 1;
            },
            0xC6 => { //DEC - Zero page
                let memory_addr = self.get_zero_page_addr();

                let new_value = Byte::new(self.get_memory_addr(memory_addr.into()).get_value().wrapping_sub(1));
                self.set_memory_addr(memory_addr.into(), new_value);

                self.set_zero_flag(self.get_memory_addr(memory_addr.into()));
                self.set_negative_flag(self.get_memory_addr(memory_addr.into()));

                self.program_counter += 2;
            },
            0xD6 => { //DEC - Zero page X
                let memory_addr = self.get_zero_page_x_addr();

                let new_value = Byte::new(self.get_memory_addr(memory_addr.into()).get_value().wrapping_sub(1));
                self.set_memory_addr(memory_addr.into(), new_value);

                self.set_zero_flag(self.get_memory_addr(memory_addr.into()));
                self.set_negative_flag(self.get_memory_addr(memory_addr.into()));

                self.program_counter += 2;
            },
            0xCE => { //DEC - Absolute
                let memory_addr = self.get_absolute_addr();

                let new_value = Byte::new(self.get_memory_addr(memory_addr).get_value().wrapping_sub(1));
                self.set_memory_addr(memory_addr, new_value);

                self.set_zero_flag(self.get_memory_addr(memory_addr));
                self.set_negative_flag(self.get_memory_addr(memory_addr));

                self.program_counter += 3;
            },
            0xDE => { //DEC - Absolute X
                let memory_addr = self.get_absolute_addr_x();

                let new_value = Byte::new(self.get_memory_addr(memory_addr).get_value().wrapping_sub(1));
                self.set_memory_addr(memory_addr, new_value);

                self.set_zero_flag(self.get_memory_addr(memory_addr));
                self.set_negative_flag(self.get_memory_addr(memory_addr));

                self.program_counter += 3;
            },
            0x48 => { //PHA
                self.push_stack(self.reg_a)?;
                self.program_counter += 1;
            },
            0x68 => { //PLA
                self.reg_a = self.pop_stack()?;

                self.set_zero_flag(self.reg_a);
                self.set_negative_flag(self.reg_a);

                self.program_counter += 1;
            },
            0xE8 => { //INX
                self.reg_x = Byte::new(self.reg_x.get_value().overflowing_add(1).0);

                self.set_zero_flag(self.reg_x);
                self.set_negative_flag(self.reg_x);

                self.program_counter += 1;
            },
            0x69 => { //ADC - Immediate
                let value = self.get_immediate_value();

                self.execute_adc(value)?;

                self.program_counter += 2;
            },
            0x65 => { //ADC - Zero page
                let memory_addr = self.get_zero_page_addr();
                let value = self.get_memory_addr(memory_addr.into());

                self.execute_adc(value)?;
                
                self.program_counter += 2;
            },
            0x75 => { //ADC - Zero page, X
                let memory_addr = self.get_zero_page_x_addr();
                let value = self.get_memory_addr(memory_addr.into());

                self.execute_adc(value)?;
                
                self.program_counter += 2;
            },
            0x6D => { //ADC - Absolute
                let memory_addr = self.get_absolute_addr();
                let value = self.get_memory_addr(memory_addr);

                self.execute_adc(value)?;
                
                self.program_counter += 3;
            },
            0x7D => { //ADC - Absolute, X
                let memory_addr = self.get_absolute_addr_x();
                let value = self.get_memory_addr(memory_addr);

                self.execute_adc(value)?;
                
                self.program_counter += 3;
            },
            0x79 => { //ADC - Absolute, Y
                let memory_addr = self.get_absolute_addr_y();
                let value = self.get_memory_addr(memory_addr);

                self.execute_adc(value)?;
                
                self.program_counter += 3;
            },
            0x61 => { //ADC - (Indirect, X)
                let memory_addr = self.get_indexed_indirect_x_addr();
                let value = self.get_memory_addr(memory_addr);

                self.execute_adc(value)?;
                
                self.program_counter += 2;
            },
            0x71 => { //ADC - (Indirect), Y
                let memory_addr = self.get_indirect_indexed_y_addr();
                let value = self.get_memory_addr(memory_addr);

                self.execute_adc(value)?;
                
                self.program_counter += 2;
            },
            0x29 => { //AND - Immediate
                self.reg_a &= self.get_immediate_value();

                self.set_zero_flag(self.reg_a);
                self.set_negative_flag(self.reg_a);

                self.program_counter += 2;
            },
            0x25 => { //AND - Zero page
                self.reg_a &= self.get_memory_addr(self.get_zero_page_addr().into());

                self.set_zero_flag(self.reg_a);
                self.set_negative_flag(self.reg_a);

                self.program_counter += 2;
            },
            0x35 => { //AND - Zero page, X
                self.reg_a &= self.get_memory_addr(self.get_zero_page_x_addr().into());

                self.set_zero_flag(self.reg_a);
                self.set_negative_flag(self.reg_a);

                self.program_counter += 2;
            },
            0x2D => { //AND - Absolute
                self.reg_a &= self.get_memory_addr(self.get_absolute_addr());

                self.set_zero_flag(self.reg_a);
                self.set_negative_flag(self.reg_a);

                self.program_counter += 3;
            },
            0x3D => { //AND - Absolute, X
                let memory_addr = self.get_absolute_addr_x();
                self.reg_a &= self.get_memory_addr(memory_addr);

                self.set_zero_flag(self.reg_a);
                self.set_negative_flag(self.reg_a);

                self.program_counter += 3;
            },
            0x39 => { //AND - Absolute, Y
                let memory_addr = self.get_absolute_addr_y();
                self.reg_a &= self.get_memory_addr(memory_addr);

                self.set_zero_flag(self.reg_a);
                self.set_negative_flag(self.reg_a);

                self.program_counter += 3;
            },
            0x21 => { //AND - (Indirect, X)
                self.reg_a &= self.get_memory_addr(self.get_indexed_indirect_x_addr());

                self.set_zero_flag(self.reg_a);
                self.set_negative_flag(self.reg_a);

                self.program_counter += 2;
            },
            0x31 => { //AND - (Indirect), Y
                let target_addr = self.get_indirect_indexed_y_addr();
                self.reg_a &= self.get_memory_addr(target_addr);

                self.set_zero_flag(self.reg_a);
                self.set_negative_flag(self.reg_a);

                self.program_counter += 2;
            },
            0x0A => { //ASL - Accumulator
                self.reg_a = self.execute_asl(self.reg_a)?;

                self.program_counter += 1;
            },
            0x06 => { //ASL - Zero Page
                let target_memory_addr: Double = self.get_zero_page_addr().into();
                let asl_output = self.execute_asl(self.get_memory_addr(target_memory_addr))?;
                self.set_memory_addr(target_memory_addr, asl_output);

                self.program_counter += 2;
            },
            0x16 => { //ASL - Zero Page, X
                let target_memory_addr: Double = self.get_zero_page_x_addr().into();
                let asl_output = self.execute_asl(self.get_memory_addr(target_memory_addr))?;
                self.set_memory_addr(target_memory_addr, asl_output);

                self.program_counter += 2;
            },
            0x0E => { //ASL - Absolute
                let target_memory_addr: Double = self.get_absolute_addr();
                let asl_output = self.execute_asl(self.get_memory_addr(target_memory_addr))?;
                self.set_memory_addr(target_memory_addr, asl_output);

                self.program_counter += 3;
            },
            0x1E => { //ASL - Absolute, X
                let target_memory_addr: Double = self.get_absolute_addr_x();
                let asl_output = self.execute_asl(self.get_memory_addr(target_memory_addr))?;
                self.set_memory_addr(target_memory_addr, asl_output);

                self.program_counter += 3;
            },
            0x4A => { //LSR - Accumulator
                self.flag_carry = self.reg_a[0];

                self.reg_a >>= 1;

                self.set_negative_flag(self.reg_a);
                self.set_zero_flag(self.reg_a);

                self.program_counter += 1;
            },
            0x46 => { //LSR - Zero page
                let target_memory_addr: Double = self.get_zero_page_addr().into();
                self.flag_carry = self.get_memory_addr(target_memory_addr)[0];

                self.set_memory_addr(target_memory_addr, self.get_memory_addr(target_memory_addr) >> 1);

                self.set_negative_flag(self.get_memory_addr(target_memory_addr));
                self.set_zero_flag(self.get_memory_addr(target_memory_addr));

                self.program_counter += 2;
            },
            0x56 => { //LSR - Zero page, X
                let target_memory_addr: Double = self.get_zero_page_x_addr().into();
                self.flag_carry = self.get_memory_addr(target_memory_addr)[0];

                self.set_memory_addr(target_memory_addr, self.get_memory_addr(target_memory_addr) >> 1);

                self.set_negative_flag(self.get_memory_addr(target_memory_addr));
                self.set_zero_flag(self.get_memory_addr(target_memory_addr));

                self.program_counter += 2;
            },
            0x4E => { //LSR - Absolute
                let target_memory_addr: Double = self.get_absolute_addr();
                self.flag_carry = self.get_memory_addr(target_memory_addr)[0];

                self.set_memory_addr(target_memory_addr, self.get_memory_addr(target_memory_addr) >> 1);

                self.set_negative_flag(self.get_memory_addr(target_memory_addr));
                self.set_zero_flag(self.get_memory_addr(target_memory_addr));

                self.program_counter += 3;
            },
            0x5E => { //LSR - Absolute, X
                let target_memory_addr: Double = self.get_absolute_addr_x();
                self.flag_carry = self.get_memory_addr(target_memory_addr)[0];

                self.set_memory_addr(target_memory_addr, self.get_memory_addr(target_memory_addr) >> 1);

                self.set_negative_flag(self.get_memory_addr(target_memory_addr));
                self.set_zero_flag(self.get_memory_addr(target_memory_addr));

                self.program_counter += 3;
            },
            0xA2 => { //LDX - Immediate
                self.reg_x = self.get_immediate_value();

                self.set_zero_flag(self.reg_x);
                self.set_negative_flag(self.reg_x);

                self.program_counter += 2;
            },
            0xA6 => { //LDX - Zero page
                self.reg_x = self.get_memory_addr(self.get_zero_page_addr().into());

                self.set_zero_flag(self.reg_x);
                self.set_negative_flag(self.reg_x);

                self.program_counter += 2;
            },
            0xB6 => { //LDX - Zero page, Y
                self.reg_x = self.get_memory_addr(Double::from(self.get_zero_page_y_addr()));

                self.set_zero_flag(self.reg_x);
                self.set_negative_flag(self.reg_x);

                self.program_counter += 2;
            },
            0xAE => { //LDX - Absolute
                self.reg_x = self.get_memory_addr(self.get_absolute_addr());

                self.set_zero_flag(self.reg_x);
                self.set_negative_flag(self.reg_x);

                self.program_counter += 3;
            },
            0xBE => { //LDX - Absolute, Y
                let memory_addr = self.get_absolute_addr_y();
                self.reg_x = self.get_memory_addr(memory_addr);

                self.set_zero_flag(self.reg_x);
                self.set_negative_flag(self.reg_x);

                self.program_counter += 3;
            },
            0xA0 => { //LDY - Immediate
                self.reg_y = self.get_immediate_value();

                self.set_zero_flag(self.reg_y);
                self.set_negative_flag(self.reg_y);

                self.program_counter += 2;
            },
            0xA4 => { //LDY - Zero Page
                self.reg_y = self.get_memory_addr(self.get_zero_page_addr().into());

                self.set_zero_flag(self.reg_y);
                self.set_negative_flag(self.reg_y);

                self.program_counter += 2;
            },
            0xB4 => { //LDY - Zero Page, Y
                self.reg_y = self.get_memory_addr(self.get_zero_page_x_addr().into());

                self.set_zero_flag(self.reg_y);
                self.set_negative_flag(self.reg_y);

                self.program_counter += 2;
            },
            0xAC => { //LDY - Absolute
                self.reg_y = self.get_memory_addr(self.get_absolute_addr());

                self.set_zero_flag(self.reg_y);
                self.set_negative_flag(self.reg_y);

                self.program_counter += 3;
            },
            0xBC => { //LDY - Absolute, X
                let memory_addr = self.get_absolute_addr_x();
                self.reg_y = self.get_memory_addr(memory_addr);

                self.set_zero_flag(self.reg_y);
                self.set_negative_flag(self.reg_y);

                self.program_counter += 3;
            },
            0xCA => { //DEX
                self.reg_x = Byte::new(self.reg_x.get_value().wrapping_sub(1));

                self.set_negative_flag(self.reg_x);
                self.set_zero_flag(self.reg_x);

                self.program_counter += 1;
            },
            0x88 => { //DEY
                self.reg_y = Byte::new(self.reg_y.get_value().wrapping_sub(1));

                self.set_negative_flag(self.reg_y);
                self.set_zero_flag(self.reg_y);

                self.program_counter += 1;
            },
            0x86 => { //STX - Zero Page
                self.set_memory_addr(self.get_zero_page_addr().into(), self.reg_x);

                self.program_counter += 2;
            },
            0x96 => { //STX - Zero Page, Y
                self.set_memory_addr(self.get_zero_page_y_addr().into(), self.reg_x);

                self.program_counter += 2;
            },
            0x8E => { //STX - Absolute
                self.set_memory_addr(self.get_absolute_addr(), self.reg_x);

                self.program_counter += 3;
            },
            0x84 => { //STY - Zero Page
                self.set_memory_addr(self.get_zero_page_addr().into(), self.reg_y);

                self.program_counter += 2;
            },
            0x94 => { //STY - Zero Page, X
                self.set_memory_addr(self.get_zero_page_x_addr().into(), self.reg_y);

                self.program_counter += 2;
            },
            0x8C => { //STY - Absolute
                self.set_memory_addr(self.get_absolute_addr(), self.reg_y);

                self.program_counter += 3;
            },
            0xE0 => { //CPX - Immediate
                let value = self.get_immediate_value();
                let result = self.reg_x - value;
                
                self.flag_zero = self.reg_x == value;
                self.flag_carry = self.reg_x >= value;
                self.flag_negative = result[7];

                self.program_counter += 2;
            },
            0xE4 => { //CPX - Zero Page
                let value = self.get_memory_addr(self.get_zero_page_addr().into());
                let result = self.reg_x - value;
                
                self.flag_zero = self.reg_x == value;
                self.flag_carry = self.reg_x >= value;
                self.flag_negative = result[7];

                self.program_counter += 2;
            },
            0xEC => { //CPX - Absolute
                let value = self.get_memory_addr(self.get_absolute_addr());
                let result = self.reg_x - value;
                
                self.flag_zero = self.reg_x == value;
                self.flag_carry = self.reg_x >= value;
                self.flag_negative = result[7];

                self.program_counter += 3;
            },
            0xC0 => { //CPY - Immediate
                let value = self.get_immediate_value();
                let result = self.reg_y - value;
                
                self.flag_zero = self.reg_y == value;
                self.flag_carry = self.reg_y >= value;
                self.flag_negative = result[7];

                self.program_counter += 2;
            },
            0xC4 => { //CPY - Zero Page
                let value = self.get_memory_addr(self.get_zero_page_addr().into());
                let result = self.reg_y - value;
                
                self.flag_zero = self.reg_y == value;
                self.flag_carry = self.reg_y >= value;
                self.flag_negative = result[7];

                self.program_counter += 2;
            },
            0xCC => { //CPY - Absolute
                let value = self.get_memory_addr(self.get_absolute_addr());
                let result = self.reg_y - value;
                
                self.flag_zero = self.reg_y == value;
                self.flag_carry = self.reg_y >= value;
                self.flag_negative = result[7];

                self.program_counter += 3;
            },
            0xB0 => { //BCS
                let offset = self.get_relative_addr();
                self.program_counter += 2;
                self.execute_branch(self.flag_carry, offset)?;
            },
            0x90 => { //BCC
                let offset = self.get_relative_addr();
                self.program_counter += 2;
                self.execute_branch(!self.flag_carry, offset)?;
            },
            0xF0 => { //BEQ
                let offset = self.get_relative_addr();
                self.program_counter += 2;
                self.execute_branch(self.flag_zero, offset)?;
            },
            0xD0 => { //BNE
                let offset = self.get_relative_addr();
                self.program_counter += 2;
                self.execute_branch(!self.flag_zero, offset)?;
            },
            0x30 => { //BMI
                let offset = self.get_relative_addr();
                self.program_counter += 2;
                self.execute_branch(self.flag_negative, offset)?;
            },
            0x10 => { //BPL
                let offset = self.get_relative_addr();
                self.program_counter += 2;
                self.execute_branch(!self.flag_negative, offset)?;
            },
            0x70 => { //BVS
                let offset = self.get_relative_addr();
                self.program_counter += 2;
                self.execute_branch(self.flag_overflow, offset)?;
            },
            0x50 => { //BVC
                let offset = self.get_relative_addr();
                self.program_counter += 2;
                self.execute_branch(!self.flag_overflow, offset)?;
            },
            0xC9 => { //CMP - Immediate
                let value = self.get_immediate_value();
                let result = self.reg_a - value;
                
                self.flag_zero = self.reg_a == value;
                self.flag_carry = self.reg_a >= value;
                self.flag_negative = result[7];

                self.program_counter += 2;
            },
            0xC5 => { //CMP - Zero Page
                let value = self.get_memory_addr(self.get_zero_page_addr().into());
                let result = self.reg_a - value;
                
                self.flag_zero = self.reg_a == value;
                self.flag_carry = self.reg_a >= value;
                self.flag_negative = result[7];

                self.program_counter += 2;
            },
            0xD5 => { //CMP - Zero Page, X
                let value = self.get_memory_addr(self.get_zero_page_x_addr().into());
                let result = self.reg_a - value;
                
                self.flag_zero = self.reg_a == value;
                self.flag_carry = self.reg_a >= value;
                self.flag_negative = result[7];

                self.program_counter += 2;
            },
            0xCD => { //CMP - Absolute
                let value = self.get_memory_addr(self.get_absolute_addr());
                let result = self.reg_a - value;
                
                self.flag_zero = self.reg_a == value;
                self.flag_carry = self.reg_a >= value;
                self.flag_negative = result[7];

                self.program_counter += 3;
            },
            0xDD => { //CMP - Absolute, X
                let memory_addr = self.get_absolute_addr_x();
                let value = self.get_memory_addr(memory_addr);
                let result = self.reg_a - value;
                
                self.flag_zero = self.reg_a == value;
                self.flag_carry = self.reg_a >= value;
                self.flag_negative = result[7];

                self.program_counter += 3;
            },
            0xD9 => { //CMP - Absolute, Y
                let memory_addr = self.get_absolute_addr_y();
                let value = self.get_memory_addr(memory_addr);
                let result = self.reg_a - value;
                
                self.flag_zero = self.reg_a == value;
                self.flag_carry = self.reg_a >= value;
                self.flag_negative = result[7];

                self.program_counter += 3;
            },
            0xC1 => { //CMP - Indirect, X
                let value = self.get_memory_addr(self.get_indexed_indirect_x_addr());
                let result = self.reg_a - value;
                
                self.flag_zero = self.reg_a == value;
                self.flag_carry = self.reg_a >= value;
                self.flag_negative = result[7];

                self.program_counter += 2;
            },
            0xD1 => { //CMP - Indirect, Y
                let target_addr = self.get_indirect_indexed_y_addr();
                let value = self.get_memory_addr(target_addr);
                let result = self.reg_a - value;
                
                self.flag_zero = self.reg_a == value;
                self.flag_carry = self.reg_a >= value;
                self.flag_negative = result[7];

                self.program_counter += 2;
            },
            0x4C => { //JMP - Absolute
                self.program_counter = self.get_absolute_addr();
            },
            0x6C => { //JMP - Indirect
                self.program_counter = self.get_indirect_addr();
            },
            0xC8 => { //INY
                self.reg_y = Byte::new(self.reg_y.get_value().overflowing_add(1).0);

                self.set_zero_flag(self.reg_y);
                self.set_negative_flag(self.reg_y);

                self.program_counter += 1;
            },
            0xE6 => { //INC - Zero Page
                let target_addr = self.get_zero_page_addr().into();
                self.execute_inc(target_addr)?;

                self.program_counter += 2;
            },
            0xF6 => { //INC - Zero Page, X
                let target_addr = self.get_zero_page_x_addr().into();
                self.execute_inc(target_addr)?;

                self.program_counter += 2;
            },
            0xEE => { //INC - Absolute
                let target_addr = self.get_absolute_addr();
                self.execute_inc(target_addr)?;

                self.program_counter += 3;
            },
            0xFE => { //INC - Absolute, X
                let target_addr = self.get_absolute_addr_x();
                self.execute_inc(target_addr)?;

                self.program_counter += 3;
            },
            0x49 => { //EOR - Immediate
                let value = self.get_immediate_value();
                self.reg_a ^= value;

                self.set_negative_flag(self.reg_a);
                self.set_zero_flag(self.reg_a);

                self.program_counter += 2;
            },
            0x45 => { //EOR - Zero Page
                let value = self.get_memory_addr(self.get_zero_page_addr().into());
                self.reg_a ^= value;

                self.set_negative_flag(self.reg_a);
                self.set_zero_flag(self.reg_a);

                self.program_counter += 2;
            },
            0x55 => { //EOR - Zero Page, X
                let value = self.get_memory_addr(self.get_zero_page_x_addr().into());
                self.reg_a ^= value;

                self.set_negative_flag(self.reg_a);
                self.set_zero_flag(self.reg_a);

                self.program_counter += 2;
            },
            0x4D => { //EOR - Absolute
                let value = self.get_memory_addr(self.get_absolute_addr());
                self.reg_a ^= value;

                self.set_negative_flag(self.reg_a);
                self.set_zero_flag(self.reg_a);

                self.program_counter += 3;
            },
            0x5D => { //EOR - Absolute, X
                let memory_addr = self.get_absolute_addr_x();
                let value = self.get_memory_addr(memory_addr);
                self.reg_a ^= value;

                self.set_negative_flag(self.reg_a);
                self.set_zero_flag(self.reg_a);

                self.program_counter += 3;
            },
            0x59 => { //EOR - Absolute, Y
                let memory_addr = self.get_absolute_addr_y();
                let value = self.get_memory_addr(memory_addr);
                self.reg_a ^= value;

                self.set_negative_flag(self.reg_a);
                self.set_zero_flag(self.reg_a);

                self.program_counter += 3;
            },
            0x41 => { //EOR - Indirect, X
                let value = self.get_memory_addr(self.get_indexed_indirect_x_addr());
                self.reg_a ^= value;

                self.set_negative_flag(self.reg_a);
                self.set_zero_flag(self.reg_a);

                self.program_counter += 2;
            },
            0x51 => { //EOR - Indirect, Y
                let target_addr = self.get_indirect_indexed_y_addr();
                let value = self.get_memory_addr(target_addr);
                self.reg_a ^= value;

                self.set_negative_flag(self.reg_a);
                self.set_zero_flag(self.reg_a);

                self.program_counter += 2;
            },
            0x09 => { //ORA - Immediate
                let value = self.get_immediate_value();
                self.execute_ora(value)?;

                self.program_counter += 2;
            },
            0x05 => { //ORA - Zero Page
                let value = self.get_memory_addr(self.get_zero_page_addr().into());
                self.execute_ora(value)?;

                self.program_counter += 2;
            },
            0x15 => { //ORA - Zero Page, X
                let value = self.get_memory_addr(self.get_zero_page_x_addr().into());
                self.execute_ora(value)?;

                self.program_counter += 2;
            },
            0x0D => { //ORA - Absolute
                let value = self.get_memory_addr(self.get_absolute_addr());
                self.execute_ora(value)?;

                self.program_counter += 3;
            },
            0x1D => { //ORA - Absolute, X
                let memory_addr = self.get_absolute_addr_x();
                let value = self.get_memory_addr(memory_addr);
                self.execute_ora(value)?;

                self.program_counter += 3;
            },
            0x19 => { //ORA - Absolute, Y
                let memory_addr = self.get_absolute_addr_y();
                let value = self.get_memory_addr(memory_addr);
                self.execute_ora(value)?;

                self.program_counter += 3;
            },
            0x01 => { //ORA - Indirect, X
                let value = self.get_memory_addr(self.get_indexed_indirect_x_addr());
                self.execute_ora(value)?;

                self.program_counter += 2;
            },
            0x11 => { //ORA - Indirect, Y
                let target_addr = self.get_indirect_indexed_y_addr();
                let value = self.get_memory_addr(target_addr);
                self.execute_ora(value)?;

                self.program_counter += 2;
            },
            0x2A => { //ROL - Accumulator
                let value = self.reg_a;

                self.reg_a = self.execute_rol(value)?;
                
                self.program_counter += 1;
            },
            0x26 => { //ROL - Zero Page
                let target_memory_addr = Double::from(self.get_zero_page_addr());
                let value = self.get_memory_addr(target_memory_addr);             

                let rol_output = self.execute_rol(value)?;
                self.set_memory_addr(target_memory_addr, rol_output);
                
                self.program_counter += 2;
            },
            0x36 => { //ROL - Zero Page, X
                let target_memory_addr = Double::from(self.get_zero_page_x_addr());
                let value = self.get_memory_addr(target_memory_addr);

                let rol_output = self.execute_rol(value)?;
                self.set_memory_addr(target_memory_addr, rol_output);
                
                self.program_counter += 2;
            },
            0x2E => { //ROL - Absolute
                let target_memory_addr = self.get_absolute_addr();
                let value = self.get_memory_addr(target_memory_addr);

                let rol_output = self.execute_rol(value)?;
                self.set_memory_addr(target_memory_addr, rol_output);
                
                self.program_counter += 3;
            },
            0x3E => { //ROL - Absolute, X
                let target_memory_addr = self.get_absolute_addr_x();
                let value = self.get_memory_addr(target_memory_addr);

                let rol_output = self.execute_rol(value)?;
                self.set_memory_addr(target_memory_addr, rol_output);
                
                self.program_counter += 3;
            },
            0x6A => { //ROR - Accumulator
                let value = self.reg_a;

                self.reg_a = self.execute_ror(value)?;
                
                self.program_counter += 1;
            },
            0x66 => { //ROR - Zero Page
                let target_memory_addr = Double::from(self.get_zero_page_addr());
                let value = self.get_memory_addr(target_memory_addr);

                let ror_output = self.execute_ror(value)?;
                self.set_memory_addr(target_memory_addr, ror_output);
                
                self.program_counter += 2;
            },
            0x76 => { //ROR - Zero Page, X
                let target_memory_addr = Double::from(self.get_zero_page_x_addr());
                let value = self.get_memory_addr(target_memory_addr);

                let ror_output = self.execute_ror(value)?;
                self.set_memory_addr(target_memory_addr, ror_output);
                
                self.program_counter += 2;
            },
            0x6E => { //ROR - Absolute
                let target_memory_addr = self.get_absolute_addr();
                let value = self.get_memory_addr(target_memory_addr);

                let ror_output = self.execute_ror(value)?;
                self.set_memory_addr(target_memory_addr, ror_output);
                
                self.program_counter += 3;
            },
            0x7E => { //ROR - Absolute, X
                let target_memory_addr = self.get_absolute_addr_x();
                let value = self.get_memory_addr(target_memory_addr);

                let ror_output = self.execute_ror(value)?;
                self.set_memory_addr(target_memory_addr, ror_output);
                
                self.program_counter += 3;
            },
            0x20 => { //JSR
                let target_addr = self.get_absolute_addr();

                self.program_counter += 3; // Because the opcode uses absolute indexing mode
                self.program_counter -= 1; // Because the value pushed has to be (return_addr - 1)

                self.push_stack(self.program_counter.get_most_significant())?;
                self.push_stack(self.program_counter.get_least_significant())?;

                self.program_counter = target_addr;
            },
            0x60 => { //RTS
                let least_significant = self.pop_stack()?;
                let most_significant = self.pop_stack()?;

                let target_memory_addr = Double::new_from_significant(least_significant, most_significant);
                self.program_counter = target_memory_addr + 1;
            },
            0xBA => { //TSX
                self.reg_x = self.stack_pointer.clone();

                self.set_negative_flag(self.reg_x);
                self.set_zero_flag(self.reg_x);

                self.program_counter += 1;
            }
            0x9A => { //TXS
                self.stack_pointer = self.reg_x.clone();
                self.program_counter += 1;
            },
            0x24 => { //BIT - Zero Page
                let mask_pattern = self.get_memory_addr(self.get_zero_page_addr().into());
                let and_result = mask_pattern & self.reg_a;

                self.set_zero_flag(and_result);
                self.flag_overflow = mask_pattern[6];
                self.flag_negative = mask_pattern[7];

                self.program_counter += 2;
            },
            0x2C => { //BIT - Absolute
                let mask_pattern = self.get_memory_addr(self.get_absolute_addr());
                let and_result = mask_pattern & self.reg_a;

                self.set_zero_flag(and_result);
                self.flag_overflow = mask_pattern[6];
                self.flag_negative = mask_pattern[7];

                self.program_counter += 3;
            },
            0x08 => { //PHP
                let mut new_byte_arr: [bool; 8] = [false; 8];

                new_byte_arr[0] = self.flag_carry;
                new_byte_arr[1] = self.flag_zero;
                new_byte_arr[2] = self.flag_interrupt_disable;
                new_byte_arr[3] = self.flag_decimal_mode;
                new_byte_arr[4] = true; // Further explanation : https://stackoverflow.com/questions/52017657/6502-emulator-testing-nestest
                new_byte_arr[5] = true;
                new_byte_arr[6] = self.flag_overflow;
                new_byte_arr[7] = self.flag_negative;

                self.push_stack(Byte::from_bool_array(new_byte_arr))?;
                
                self.program_counter += 1;
            },
            0x28 => { //PLP
                let cpu_flags = self.pop_stack()?;

                self.flag_carry = cpu_flags[0];
                self.flag_zero = cpu_flags[1];
                self.flag_interrupt_disable = cpu_flags[2];
                self.flag_decimal_mode = cpu_flags[3];
                self.flag_break = false;
                // false = cpu_flags[5];
                self.flag_overflow = cpu_flags[6];
                self.flag_negative = cpu_flags[7];

                self.program_counter += 1;
            },
            0xE9 => { //SBC - Immediate
                self.execute_sbc(self.get_immediate_value())?;

                self.program_counter += 2;
            },
            0xE5 => { //SBC - Zero Page
                let target_memory_addr = self.get_zero_page_addr();
                let value = self.get_memory_addr(Double::from(target_memory_addr));

                self.execute_sbc(value)?;

                self.program_counter += 2;
            },
            0xF5 => { //SBC - Zero Page, X
                let target_memory_addr = self.get_zero_page_x_addr();
                let value = self.get_memory_addr(Double::from(target_memory_addr));

                self.execute_sbc(value)?;

                self.program_counter += 2;
            },
            0xED => { //SBC - Absolute
                let target_memory_addr = self.get_absolute_addr();
                let value = self.get_memory_addr(target_memory_addr);

                self.execute_sbc(value)?;

                self.program_counter += 3;
            },
            0xFD => { //SBC - Absolute, X
                let target_memory_addr = self.get_absolute_addr_x();
                let value = self.get_memory_addr(target_memory_addr);

                self.execute_sbc(value)?;

                self.program_counter += 3;
            },
            0xF9 => { //SBC - Absolute, Y
                let target_memory_addr = self.get_absolute_addr_y();
                let value = self.get_memory_addr(target_memory_addr);

                self.execute_sbc(value)?;

                self.program_counter += 3;
            },
            0xE1 => { //SBC - Indirect, X
                let target_memory_addr = self.get_indexed_indirect_x_addr();
                let value = self.get_memory_addr(target_memory_addr);

                self.execute_sbc(value)?;

                self.program_counter += 2;
            },
            0xF1 => { //SBC - Indirect, Y
                let target_memory_addr = self.get_indirect_indexed_y_addr();
                let value = self.get_memory_addr(target_memory_addr);

                self.execute_sbc(value)?;

                self.program_counter += 2;
            },
            0x40 => { //RTI
                // Pull CPU Flags
                let cpu_flags = self.pop_stack()?;

                self.flag_carry = cpu_flags[0];
                self.flag_zero = cpu_flags[1];
                self.flag_interrupt_disable = cpu_flags[2];
                self.flag_decimal_mode = cpu_flags[3];
                self.flag_break = cpu_flags[4];
                // false = cpu_flags[5];
                self.flag_overflow = cpu_flags[6];
                self.flag_negative = cpu_flags[7];

                // Pull PC from stack
                let least_significant = self.pop_stack()?;
                let most_significant = self.pop_stack()?;

                self.program_counter = Double::new_from_significant(least_significant, most_significant);
            },
            0x1A => { //UNOFFICIAL-NOP
                self.program_counter += 1;
            }
            0x3A => { //UNOFFICIAL-NOP
                self.program_counter += 1;
            }
            0x5A => { //UNOFFICIAL-NOP
                self.program_counter += 1;
            }
            0x7A => { //UNOFFICIAL-NOP
                self.program_counter += 1;
            }
            0xDA => { //UNOFFICIAL-NOP
                self.program_counter += 1;
            }
            0xFA => { //UNOFFICIAL-NOP
                self.program_counter += 1;
            },
            0x0C => { //UNOFFICIAL-NOP-Absolute
                self.program_counter += 3;
            },
            0x1C => { //UNOFFICIAL-NOP-Absolute,X
                let _ = self.get_absolute_addr_x();
                self.program_counter += 3;
            },
            0x3C => { //UNOFFICIAL-NOP-Absolute,X
                let _ = self.get_absolute_addr_x();
                self.program_counter += 3;
            },
            0x5C => { //UNOFFICIAL-NOP-Absolute,X
                let _ = self.get_absolute_addr_x();
                self.program_counter += 3;
            },
            0x7C => { //UNOFFICIAL-NOP-Absolute,X
                let _ = self.get_absolute_addr_x();
                self.program_counter += 3;
            },
            0xDC => { //UNOFFICIAL-NOP-Absolute,X
                let _ = self.get_absolute_addr_x();
                self.program_counter += 3;
            },
            0xFC => { //UNOFFICIAL-NOP-Absolute,X
                let _ = self.get_absolute_addr_x();
                self.program_counter += 3;
            },
            0x04 => { //UNOFFICIAL-NOP-ZeroPage
                self.program_counter += 2;
            },
            0x44 => { //UNOFFICIAL-NOP-ZeroPage
                self.program_counter += 2;
            },
            0x64 => { //UNOFFICIAL-NOP-ZeroPage
                self.program_counter += 2;
            },
            0x14 => { //UNOFFICIAL-NOP-ZeroPage,X
                self.program_counter += 2;
            },
            0x34 => { //UNOFFICIAL-NOP-ZeroPage,X
                self.program_counter += 2;
            },
            0x54 => { //UNOFFICIAL-NOP-ZeroPage,X
                self.program_counter += 2;
            },
            0x74 => { //UNOFFICIAL-NOP-ZeroPage,X
                self.program_counter += 2;
            },
            0xD4 => { //UNOFFICIAL-NOP-ZeroPage,X
                self.program_counter += 2;
            },
            0xF4 => { //UNOFFICIAL-NOP-ZeroPage,X
                self.program_counter += 2;
            },
            0x80 => { //UNOFFICIAL-NOP-Immediate
                self.program_counter += 2;
            },
            0x82 => { //UNOFFICIAL-NOP-Immediate
                self.program_counter += 2;
            },
            0x89 => { //UNOFFICIAL-NOP-Immediate
                self.program_counter += 2;
            },
            0xC2 => { //UNOFFICIAL-NOP-Immediate
                self.program_counter += 2;
            },
            0xE2 => { //UNOFFICIAL-NOP-Immediate
                self.program_counter += 2;
            },
            0xA3 => { //UNOFFICIAL-LAX-Indirect,X
                let value = self.get_memory_addr(self.get_indexed_indirect_x_addr());

                self.reg_a = value;
                self.set_negative_flag(self.reg_a);
                self.set_zero_flag(self.reg_a);

                self.reg_x = self.reg_a;
                self.set_negative_flag(self.reg_x);
                self.set_zero_flag(self.reg_x);

                self.program_counter += 2;
            },
            0xA7 => { //UNOFFICIAL-LAX-ZeroPage
                let value = self.get_memory_addr(self.get_zero_page_addr().into());

                self.reg_a = value;
                self.set_negative_flag(self.reg_a);
                self.set_zero_flag(self.reg_a);

                self.reg_x = self.reg_a;
                self.set_negative_flag(self.reg_x);
                self.set_zero_flag(self.reg_x);

                self.program_counter += 2;
            },
            0xAF => { //UNOFFICIAL-LAX-Absolute
                let value = self.get_memory_addr(self.get_absolute_addr());

                self.reg_a = value;
                self.set_negative_flag(self.reg_a);
                self.set_zero_flag(self.reg_a);

                self.reg_x = self.reg_a;
                self.set_negative_flag(self.reg_x);
                self.set_zero_flag(self.reg_x);
                
                self.program_counter += 3;
            },
            0xB3 => { //UNOFFICIAL-LAX-Indirect,Y
                let target_addr = self.get_indirect_indexed_y_addr();
                let value = self.get_memory_addr(target_addr);

                self.reg_a = value;
                self.set_negative_flag(self.reg_a);
                self.set_zero_flag(self.reg_a);

                self.reg_x = self.reg_a;
                self.set_negative_flag(self.reg_x);
                self.set_zero_flag(self.reg_x);
                
                self.program_counter += 2;
            },
            0xB7 => { //UNOFFICIAL-LAX-ZeroPage,Y
                let value = self.get_memory_addr(self.get_zero_page_y_addr().into());

                self.reg_a = value;
                self.set_negative_flag(self.reg_a);
                self.set_zero_flag(self.reg_a);

                self.reg_x = self.reg_a;
                self.set_negative_flag(self.reg_x);
                self.set_zero_flag(self.reg_x);

                self.program_counter += 2;
            },
            0xBF => { //UNOFFICIAL-LAX-Absolute,Y
                let memory_addr = self.get_absolute_addr_y();
                let value = self.get_memory_addr(memory_addr);

                self.reg_a = value;
                self.set_negative_flag(self.reg_a);
                self.set_zero_flag(self.reg_a);

                self.reg_x = self.reg_a;
                self.set_negative_flag(self.reg_x);
                self.set_zero_flag(self.reg_x);

                self.program_counter += 3;
            },
            0x83 => { //UNOFFICIAL-SAX-Indirect,X
                let target_memory_addr = self.get_indexed_indirect_x_addr();

                self.set_memory_addr(target_memory_addr, self.reg_a & self.reg_x);

                self.program_counter += 2;
            },
            0x87 => { //UNOFFICIAL-SAX-ZeroPage
                let target_memory_addr = Double::from(self.get_zero_page_addr());

                self.set_memory_addr(target_memory_addr, self.reg_a & self.reg_x);

                self.program_counter += 2;
            },
            0x8F => { //UNOFFICIAL-SAX-Absolute
                let target_memory_addr = self.get_absolute_addr();

                self.set_memory_addr(target_memory_addr, self.reg_a & self.reg_x);

                self.program_counter += 3;
            },
            0x97 => { //UNOFFICIAL-SAX-ZeroPage,Y
                let target_memory_addr = Double::from(self.get_zero_page_y_addr());

                self.set_memory_addr(target_memory_addr, self.reg_a & self.reg_x);

                self.program_counter += 2;
            },
            0xEB => { //UNOFFICIAL-SBC-Immediate
                let value = self.get_immediate_value();

                self.execute_sbc(value)?;

                self.program_counter += 2;
            },
            0xC3 => { //UNOFFICAIL-DCP-Indirect,X
                let memory_addr = Double::from(self.get_indexed_indirect_x_addr());

                let new_value = Byte::new(self.get_memory_addr(memory_addr).get_value().wrapping_sub(1));
                self.set_memory_addr(memory_addr, new_value);

                self.set_zero_flag(self.get_memory_addr(memory_addr));
                self.set_negative_flag(self.get_memory_addr(memory_addr));

                let result = self.reg_a - new_value;
                
                self.flag_zero = self.reg_a == new_value;
                self.flag_carry = self.reg_a >= new_value;
                self.flag_negative = result[7];

                self.program_counter += 2;
            },
            0xC7 => { //UNOFFICIAL-DCP-ZeroPage
                let memory_addr = Double::from(self.get_zero_page_addr());

                let new_value = Byte::new(self.get_memory_addr(memory_addr).get_value().wrapping_sub(1));
                self.set_memory_addr(memory_addr, new_value);

                self.set_zero_flag(self.get_memory_addr(memory_addr));
                self.set_negative_flag(self.get_memory_addr(memory_addr));

                let result = self.reg_a - new_value;
                
                self.flag_zero = self.reg_a == new_value;
                self.flag_carry = self.reg_a >= new_value;
                self.flag_negative = result[7];

                self.program_counter += 2;
            },
            0xCF => { //UNOFFICIAL-DCP-Absolute
                let memory_addr = Double::from(self.get_absolute_addr());

                let new_value = Byte::new(self.get_memory_addr(memory_addr).get_value().wrapping_sub(1));
                self.set_memory_addr(memory_addr, new_value);

                self.set_zero_flag(self.get_memory_addr(memory_addr));
                self.set_negative_flag(self.get_memory_addr(memory_addr));

                let result = self.reg_a - new_value;
                
                self.flag_zero = self.reg_a == new_value;
                self.flag_carry = self.reg_a >= new_value;
                self.flag_negative = result[7];

                self.program_counter += 3;
            },
            0xD3 => { //UNOFFICIAL-DCP-Indirect,Y
                let memory_addr = Double::from(self.get_indirect_indexed_y_addr());

                let new_value = Byte::new(self.get_memory_addr(memory_addr).get_value().wrapping_sub(1));
                self.set_memory_addr(memory_addr, new_value);

                self.set_zero_flag(self.get_memory_addr(memory_addr));
                self.set_negative_flag(self.get_memory_addr(memory_addr));

                let result = self.reg_a - new_value;
                
                self.flag_zero = self.reg_a == new_value;
                self.flag_carry = self.reg_a >= new_value;
                self.flag_negative = result[7];

                self.program_counter += 2;
            },
            0xD7 => { //UNOFFICIAL-DCP-ZeroPage,X
                let memory_addr = Double::from(self.get_zero_page_x_addr());

                let new_value = Byte::new(self.get_memory_addr(memory_addr).get_value().wrapping_sub(1));
                self.set_memory_addr(memory_addr, new_value);

                self.set_zero_flag(self.get_memory_addr(memory_addr));
                self.set_negative_flag(self.get_memory_addr(memory_addr));

                let result = self.reg_a - new_value;
                
                self.flag_zero = self.reg_a == new_value;
                self.flag_carry = self.reg_a >= new_value;
                self.flag_negative = result[7];

                self.program_counter += 2;
            },
            0xDB => { //UNOFFICIAL-DCP-Absolute,Y
                let memory_addr = Double::from(self.get_absolute_addr_y());

                let new_value = Byte::new(self.get_memory_addr(memory_addr).get_value().wrapping_sub(1));
                self.set_memory_addr(memory_addr, new_value);

                self.set_zero_flag(self.get_memory_addr(memory_addr));
                self.set_negative_flag(self.get_memory_addr(memory_addr));

                let result = self.reg_a - new_value;
                
                self.flag_zero = self.reg_a == new_value;
                self.flag_carry = self.reg_a >= new_value;
                self.flag_negative = result[7];

                self.program_counter += 3;
            },
            0xDF => { //UNOFFICIAL-DCP-Absolute,X
                let memory_addr = Double::from(self.get_absolute_addr_x());

                let new_value = Byte::new(self.get_memory_addr(memory_addr).get_value().wrapping_sub(1));
                self.set_memory_addr(memory_addr, new_value);

                self.set_zero_flag(self.get_memory_addr(memory_addr));
                self.set_negative_flag(self.get_memory_addr(memory_addr));

                let result = self.reg_a - new_value;
                
                self.flag_zero = self.reg_a == new_value;
                self.flag_carry = self.reg_a >= new_value;
                self.flag_negative = result[7];

                self.program_counter += 3;
            },
            0x43 => { //UNOFFICIAL-SRE-Indirect,X
                //LSR
                let target_memory_addr = Double::from(self.get_indexed_indirect_x_addr());
                self.flag_carry = self.get_memory_addr(target_memory_addr)[0];

                self.set_memory_addr(target_memory_addr, self.get_memory_addr(target_memory_addr) >> 1);

                self.set_negative_flag(self.get_memory_addr(target_memory_addr));
                self.set_zero_flag(self.get_memory_addr(target_memory_addr));

                //EOR
                let value = self.get_memory_addr(target_memory_addr);
                self.reg_a ^= value;

                self.set_negative_flag(self.reg_a);
                self.set_zero_flag(self.reg_a);

                self.program_counter += 2;
            },
            0x47 => { //UNOFFICIAL-SRE-ZeroPage
                //LSR
                let target_memory_addr = Double::from(self.get_zero_page_addr());
                self.flag_carry = self.get_memory_addr(target_memory_addr)[0];

                self.set_memory_addr(target_memory_addr, self.get_memory_addr(target_memory_addr) >> 1);

                self.set_negative_flag(self.get_memory_addr(target_memory_addr));
                self.set_zero_flag(self.get_memory_addr(target_memory_addr));

                //EOR
                let value = self.get_memory_addr(target_memory_addr);
                self.reg_a ^= value;

                self.set_negative_flag(self.reg_a);
                self.set_zero_flag(self.reg_a);

                self.program_counter += 2;
            },
            0x4F => { //UNOFFICIAL-SRE-Absolute
                //LSR
                let target_memory_addr = self.get_absolute_addr();
                self.flag_carry = self.get_memory_addr(target_memory_addr)[0];

                self.set_memory_addr(target_memory_addr, self.get_memory_addr(target_memory_addr) >> 1);

                self.set_negative_flag(self.get_memory_addr(target_memory_addr));
                self.set_zero_flag(self.get_memory_addr(target_memory_addr));

                //EOR
                let value = self.get_memory_addr(target_memory_addr);
                self.reg_a ^= value;

                self.set_negative_flag(self.reg_a);
                self.set_zero_flag(self.reg_a);

                self.program_counter += 3;
            },
            0x53 => { //UNOFFICIAL-SRE-Indirect,Y
                //LSR
                let target_memory_addr = Double::from(self.get_indirect_indexed_y_addr());
                self.flag_carry = self.get_memory_addr(target_memory_addr)[0];

                self.set_memory_addr(target_memory_addr, self.get_memory_addr(target_memory_addr) >> 1);

                self.set_negative_flag(self.get_memory_addr(target_memory_addr));
                self.set_zero_flag(self.get_memory_addr(target_memory_addr));

                //EOR
                let value = self.get_memory_addr(target_memory_addr);
                self.reg_a ^= value;

                self.set_negative_flag(self.reg_a);
                self.set_zero_flag(self.reg_a);

                self.program_counter += 2;
            },
            0x57 => { //UNOFFICIAL-SRE-ZeroPage,X
                //LSR
                let target_memory_addr = Double::from(self.get_zero_page_x_addr());
                self.flag_carry = self.get_memory_addr(target_memory_addr)[0];

                self.set_memory_addr(target_memory_addr, self.get_memory_addr(target_memory_addr) >> 1);

                self.set_negative_flag(self.get_memory_addr(target_memory_addr));
                self.set_zero_flag(self.get_memory_addr(target_memory_addr));

                //EOR
                let value = self.get_memory_addr(target_memory_addr);
                self.reg_a ^= value;

                self.set_negative_flag(self.reg_a);
                self.set_zero_flag(self.reg_a);

                self.program_counter += 2;
            },
            0x5B => { //UNOFFICIAL-SRE-Absolute,Y
                //LSR
                let target_memory_addr = self.get_absolute_addr_y();
                self.flag_carry = self.get_memory_addr(target_memory_addr)[0];

                self.set_memory_addr(target_memory_addr, self.get_memory_addr(target_memory_addr) >> 1);

                self.set_negative_flag(self.get_memory_addr(target_memory_addr));
                self.set_zero_flag(self.get_memory_addr(target_memory_addr));

                //EOR
                let value = self.get_memory_addr(target_memory_addr);
                self.reg_a ^= value;

                self.set_negative_flag(self.reg_a);
                self.set_zero_flag(self.reg_a);

                self.program_counter += 3;
            },
            0x5F => { //UNOFFICIAL-SRE-Absolute,X
                //LSR
                let target_memory_addr = self.get_absolute_addr_x();
                self.flag_carry = self.get_memory_addr(target_memory_addr)[0];

                self.set_memory_addr(target_memory_addr, self.get_memory_addr(target_memory_addr) >> 1);

                self.set_negative_flag(self.get_memory_addr(target_memory_addr));
                self.set_zero_flag(self.get_memory_addr(target_memory_addr));

                //EOR
                let value = self.get_memory_addr(target_memory_addr);
                self.reg_a ^= value;

                self.set_negative_flag(self.reg_a);
                self.set_zero_flag(self.reg_a);

                self.program_counter += 3;
            },
            0xE3 => { //UNOFFICIAL-ISC-Indirect,X
                let target_addr = self.get_indexed_indirect_x_addr();

                self.execute_inc(target_addr)?;
                self.execute_sbc(self.get_memory_addr(target_addr))?;

                self.program_counter += 2;
            },
            0xE7 => { //UNOFFICIAL-ISC-ZeroPage
                let target_addr = Double::from(self.get_zero_page_addr());

                self.execute_inc(target_addr)?;
                self.execute_sbc(self.get_memory_addr(target_addr))?;

                self.program_counter += 2;
            },
            0xEF => { //UNOFFICIAL-ISC-Absolute
                let target_addr = self.get_absolute_addr();

                self.execute_inc(target_addr)?;
                self.execute_sbc(self.get_memory_addr(target_addr))?;

                self.program_counter += 3;
            },
            0xF3 => { //UNOFFICIAL-ISC-Indirect,Y
                let target_addr = self.get_indirect_indexed_y_addr();

                self.execute_inc(target_addr)?;
                self.execute_sbc(self.get_memory_addr(target_addr))?;

                self.program_counter += 2;
            },
            0xF7 => { //UNOFFICIAL-ISC-ZeroPage,X
                let target_addr = Double::from(self.get_zero_page_x_addr());

                self.execute_inc(target_addr)?;
                self.execute_sbc(self.get_memory_addr(target_addr))?;

                self.program_counter += 2;
            },
            0xFB => { //UNOFFICIAL-ISC-Absolute,Y
                let target_addr = self.get_absolute_addr_y();

                self.execute_inc(target_addr)?;
                self.execute_sbc(self.get_memory_addr(target_addr))?;

                self.program_counter += 3;
            },
            0xFF => { //UNOFFICIAL-ISC-Absolute,X
                let target_addr = self.get_absolute_addr_x();

                self.execute_inc(target_addr)?;
                self.execute_sbc(self.get_memory_addr(target_addr))?;

                self.program_counter += 3;
            },
            0x03 => { //UNOFFICIAL-SLO-Indirect,X
                let target_addr = self.get_indexed_indirect_x_addr();

                // ASL
                let asl_output = self.execute_asl(self.get_memory_addr(target_addr))?;
                self.set_memory_addr(target_addr, asl_output);

                // ORA
                self.execute_ora(self.get_memory_addr(target_addr))?;

                self.program_counter += 2;
            },
            0x07 => { //UNOFFICIAL-SLO-ZeroPage
                let target_addr = Double::from(self.get_zero_page_addr());

                // ASL
                let asl_output = self.execute_asl(self.get_memory_addr(target_addr))?;
                self.set_memory_addr(target_addr, asl_output);

                // ORA
                self.execute_ora(self.get_memory_addr(target_addr))?;

                self.program_counter += 2;
            },
            0x0F => { //UNOFFICIAL-SLO-Absolute
                let target_addr = self.get_absolute_addr();

                // ASL
                let asl_output = self.execute_asl(self.get_memory_addr(target_addr))?;
                self.set_memory_addr(target_addr, asl_output);

                // ORA
                self.execute_ora(self.get_memory_addr(target_addr))?;

                self.program_counter += 3;
            },
            0x13 => { //UNOFFICIAL-SLO-Indirect,Y
                let target_addr = self.get_indirect_indexed_y_addr();

                // ASL
                let asl_output = self.execute_asl(self.get_memory_addr(target_addr))?;
                self.set_memory_addr(target_addr, asl_output);

                // ORA
                self.execute_ora(self.get_memory_addr(target_addr))?;

                self.program_counter += 2;
            },
            0x17 => { //UNOFFICIAL-SLO-ZeroPage,X
                let target_addr = Double::from(self.get_zero_page_x_addr());

                // ASL
                let asl_output = self.execute_asl(self.get_memory_addr(target_addr))?;
                self.set_memory_addr(target_addr, asl_output);

                // ORA
                self.execute_ora(self.get_memory_addr(target_addr))?;

                self.program_counter += 2;
            },
            0x1B => { //UNOFFICIAL-SLO-Absolute,Y
                let target_addr = self.get_absolute_addr_y();

                // ASL
                let asl_output = self.execute_asl(self.get_memory_addr(target_addr))?;
                self.set_memory_addr(target_addr, asl_output);

                // ORA
                self.execute_ora(self.get_memory_addr(target_addr))?;

                self.program_counter += 3;
            },
            0x1F => { //UNOFFICIAL-SLO-Absolute,X
                let target_addr = self.get_absolute_addr_x();

                // ASL
                let asl_output = self.execute_asl(self.get_memory_addr(target_addr))?;
                self.set_memory_addr(target_addr, asl_output);

                // ORA
                self.execute_ora(self.get_memory_addr(target_addr))?;

                self.program_counter += 3;
            },
            0x23 => { //UNOFFICIAL-RLA-Indirect,X
                self.execute_rla(self.get_indexed_indirect_x_addr())?;

                self.program_counter += 2;
            },
            0x27 => { //UNOFFICIAL-RLA-ZeroPage
                self.execute_rla(Double::from(self.get_zero_page_addr()))?;

                self.program_counter += 2;
            },
            0x2F => { //UNOFFICIAL-RLA-Absolute
                self.execute_rla(self.get_absolute_addr())?;

                self.program_counter += 3;
            },
            0x33 => { //UNOFFICIAL-RLA-Indirect,Y
                let target_addr = self.get_indirect_indexed_y_addr();
                self.execute_rla(Double::from(target_addr))?;

                self.program_counter += 2;
            },
            0x37 => { //UNOFFICIAL-RLA-ZeroPage,X
                self.execute_rla(Double::from(self.get_zero_page_x_addr()))?;

                self.program_counter += 2;
            },
            0x3B => { //UNOFFICIAL-RLA-Absolute,Y
                let memory_addr = self.get_absolute_addr_y();
                self.execute_rla(memory_addr)?;

                self.program_counter += 3;
            },
            0x3F => { //UNOFFICIAL-RLA-Absolute,X
                let memory_addr = self.get_absolute_addr_x();
                self.execute_rla(memory_addr)?;

                self.program_counter += 3;
            },
            0x63 => { //UNOFFICIAL-RRA-Indirect,X
                let memory_addr: Double = self.get_indexed_indirect_x_addr();
                let value = self.get_memory_addr(memory_addr);

                let ror_output = self.execute_ror(value)?;
                self.set_memory_addr(memory_addr, ror_output);
                self.execute_adc(self.get_memory_addr(memory_addr))?;
                
                self.program_counter += 2;
            },
            0x67 => { //UNOFFICIAL-RRA-ZeroPage
                let memory_addr: Double = Double::from(self.get_zero_page_addr());
                let value = self.get_memory_addr(memory_addr);

                let ror_output = self.execute_ror(value)?;
                self.set_memory_addr(memory_addr, ror_output);
                self.execute_adc(self.get_memory_addr(memory_addr))?;

                self.program_counter += 2;
            },
            0x6F => { //UNOFFICIAL-RRA-Absolute
                let memory_addr: Double = self.get_absolute_addr();
                let value = self.get_memory_addr(memory_addr);

                let ror_output = self.execute_ror(value)?;
                self.set_memory_addr(memory_addr, ror_output);
                self.execute_adc(self.get_memory_addr(memory_addr))?;

                self.program_counter += 3;
            },
            0x73 => { //UNOFFICIAL-RRA-Indirect,Y
                let memory_addr: Double = Double::from(self.get_indirect_indexed_y_addr());
                let value = self.get_memory_addr(memory_addr);

                let ror_output = self.execute_ror(value)?;
                self.set_memory_addr(memory_addr, ror_output);
                self.execute_adc(self.get_memory_addr(memory_addr))?;

                self.program_counter += 2;
            },
            0x77 => { //UNOFFICIAL-RRA-ZeroPage,X
                let memory_addr: Double = Double::from(self.get_zero_page_x_addr());
                let value = self.get_memory_addr(memory_addr);

                let ror_output = self.execute_ror(value)?;
                self.set_memory_addr(memory_addr, ror_output);
                self.execute_adc(self.get_memory_addr(memory_addr))?;

                self.program_counter += 2;
            },
            0x7B => { //UNOFFICIAL-RRA-Absolute,Y
                let memory_addr: Double = self.get_absolute_addr_y();
                let value = self.get_memory_addr(memory_addr);

                let ror_output = self.execute_ror(value)?;
                self.set_memory_addr(memory_addr, ror_output);
                self.execute_adc(self.get_memory_addr(memory_addr))?;

                self.program_counter += 3;
            },
            0x7F => { //UNOFFICIAL-RRA_Absolute,X
                let memory_addr: Double = self.get_absolute_addr_x();
                let value = self.get_memory_addr(memory_addr);

                let ror_output = self.execute_ror(value)?;
                self.set_memory_addr(memory_addr, ror_output);
                self.execute_adc(self.get_memory_addr(memory_addr))?;

                self.program_counter += 3;
            },
            _ => {
                error!("Unknown opcode {}", opcode);
                return Err(CpuError::UnknownOpcodeError(opcode));
            }
        }

        Ok(())
    }
}
