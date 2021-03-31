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
    assert_eq!(cpu.get_absolute_addr(), Double::new_from_u16(0xC000));

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
    assert_eq!(cpu.get_absolute_addr(), Double::new_from_u16(0xC000));
    assert_eq!(cpu.get_absolute_addr_x(), Double::new_from_u16(0xC001));

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
    assert_eq!(cpu.get_absolute_addr(), Double::new_from_u16(0xC000));
    assert_eq!(cpu.get_absolute_addr_y(), Double::new_from_u16(0xC002));

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
    assert_eq!(cpu.get_absolute_addr(), Double::new_from_u16(0x1020));

    let before_pc = cpu.program_counter;
    let _ = cpu.execute_instruction();
    assert_eq!(cpu.program_counter, before_pc + 3);

    assert_eq!(cpu.memory[0x1020 as u16], 0x52.into());

    // Indirect, X
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x03 as u16] = 0x81.into();
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x04 as u16] = 0x02.into();
    cpu.reg_x = Byte::new(0x04);
    cpu.memory[0x06 as u16] = 0x00.into();
    cpu.memory[0x07 as u16] = 0x80.into();

    assert_eq!(cpu.get_first_arg().get_value(), 0x02);
    assert_eq!(cpu.reg_x.get_value(), 0x04);
    assert_eq!(cpu.get_first_arg().get_value() + cpu.reg_x.get_value(), 0x06);
    assert_eq!(cpu.get_indexed_indirect_x_addr().get_value(), 0x8000);

    assert_eq!(cpu.reg_a.get_value(), 0x52);
    assert_eq!(cpu.get_memory_addr(Double::new_from_u16(0x8000)).get_value(), 0x00);
    let _ = cpu.execute_instruction();
    assert_eq!(cpu.get_memory_addr(Double::new_from_u16(0x8000)).get_value(), 0x52);

    // Indirect, y
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x05 as u16] = 0x91.into();
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x06 as u16] = 0x86.into();
    cpu.reg_y = Byte::new(0x10);
    cpu.memory[0x86 as u16] = 0x28.into();
    cpu.memory[0x87 as u16] = 0x40.into();

    assert_eq!(cpu.get_first_arg().get_value(), 0x86);
    assert_eq!(cpu.reg_y.get_value(), 0x10);
    assert_eq!(cpu.get_indirect_indexed_y_addr().get_value(), 0x4028 + 0x10);

    assert_eq!(cpu.reg_a.get_value(), 0x52);
    assert_eq!(cpu.get_memory_addr(Double::new_from_u16(0x4038)).get_value(), 0x00);
    let _ = cpu.execute_instruction();
    assert_eq!(cpu.get_memory_addr(Double::new_from_u16(0x4038)).get_value(), 0x52);

    assert_eq!(cpu.reg_a.get_value(), 0x52);
}

#[test]
fn adc() {
    // Immediate
    let mut cpu = Cpu::new();
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x00 as u16] = 0x69.into();
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x01 as u16] = 0x23.into();

    assert_eq!(cpu.get_immediate_value().get_value(), 0x23);
    let _ = cpu.execute_instruction();
    assert_eq!(cpu.reg_a.get_value(), 0x23);

    // Immediate - Test carry flag
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x02 as u16] = 0x69.into();
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x03 as u16] = 0xF0.into();

    assert_eq!(cpu.get_immediate_value().get_value(), 0xF0);
    assert_eq!(cpu.flag_carry, false);
    let _ = cpu.execute_instruction();
    assert_eq!(cpu.reg_a.get_value(), 0x13);
    assert_eq!(cpu.flag_carry, true);

    // Zero Page, X
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x04 as u16] = 0x75.into();
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x05 as u16] = 0xC0.into();

    cpu.reg_x = Byte::new(0x01);
    cpu.memory[0xC1 as u16] = 0x27.into();

    assert_eq!(cpu.get_zero_page_x_addr().get_value(), 0xC1);
    let _ = cpu.execute_instruction();
    assert_eq!(cpu.reg_a.get_value(), 0x3B);
    assert_eq!(cpu.flag_carry, false);

    // Absolute, Y
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x06 as u16] = 0x79.into();
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x07 as u16] = 0x00.into();
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x08 as u16] = 0x0A.into();

    cpu.reg_y = Byte::new(0x10);
    cpu.memory[0x0A10 as u16] = 0x30.into();

    assert_eq!(cpu.get_absolute_addr_y().get_value(), 0x0A10);
    let _ = cpu.execute_instruction();
    assert_eq!(cpu.reg_a.get_value(), 0x6B);
    assert_eq!(cpu.flag_carry, false);
}

#[test]
fn and() {
    // Absolute
    let mut cpu = Cpu::new();
    cpu.reg_a = 0x52.into();

    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x00 as u16] = 0x29.into();
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x01 as u16] = 0x30.into();

    assert_eq!(cpu.get_first_arg().get_value(), 0x30);
    assert_eq!(cpu.get_immediate_value().get_value(), 0x30);

    let _ = cpu.execute_instruction();

    assert_eq!(cpu.reg_a.get_value(), 0x10);
    assert_eq!(cpu.flag_zero, false);
    assert_eq!(cpu.flag_negative, false);

    // (Indirect, X)
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x02 as u16] = 0x21.into();
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x03 as u16] = 0x30.into();

    cpu.reg_x = Byte::new(0x04);
    cpu.memory[0x34 as u16] = 0xA0.into();
    cpu.memory[0x35 as u16] = 0xB0.into();

    cpu.memory[0xB0A0 as u16] = 0x24.into();

    assert_eq!(cpu.get_indexed_indirect_x_addr().get_value(), 0xB0A0);

    let _ = cpu.execute_instruction();

    assert_eq!(cpu.reg_a.get_value(), 0x00);
    assert_eq!(cpu.flag_zero, true);
    assert_eq!(cpu.flag_negative, false);
}

#[test]
fn asl() {
    let mut cpu = Cpu::new();

    // Accumulator
    cpu.reg_a = Byte::new(0x23);
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x00 as u16] = 0x0A.into();

    let _ = cpu.execute_instruction();

    assert_eq!(cpu.reg_a.get_value(), 0x23 * 2);

    // Zero page
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x01 as u16] = 0x06.into();
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x02 as u16] = 0x10.into();
    cpu.memory[0x10 as u16] = 0xF0.into();

    assert_eq!(cpu.flag_carry, false);
    assert_eq!(cpu.flag_zero, false);
    assert_eq!(cpu.flag_negative, false);

    let _ = cpu.execute_instruction();
    
    assert_eq!(cpu.memory[0x10 as u16], 0xE0.into());
    assert_eq!(cpu.flag_carry, true);
    assert_eq!(cpu.flag_zero, false);
    assert_eq!(cpu.flag_negative, true);
}

#[test]
fn lsr() {
    let mut cpu = Cpu::new();

    // Accumulator
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x00 as u16] = 0x4A.into();
    cpu.reg_a = Byte::new(0x47);

    assert_eq!(cpu.flag_carry, false);
    assert_eq!(cpu.flag_negative, false);
    assert_eq!(cpu.flag_zero, false);

    let _ = cpu.execute_instruction();

    assert_eq!(cpu.reg_a.get_value(), 0x23);
    assert_eq!(cpu.flag_carry, true);
    assert_eq!(cpu.flag_negative, false);
    assert_eq!(cpu.flag_zero, false);

    // Absolute, X
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x01 as u16] = 0x5E.into();
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x02 as u16] = 0x50.into();
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x03 as u16] = 0x0A.into();

    cpu.reg_x = Byte::new(0x02);
    cpu.memory[0x0A52 as u16] = 0x0C.into();

    let _ = cpu.execute_instruction();

    assert_eq!(cpu.memory[0x0A52 as u16].get_value(), 0x06);
    assert_eq!(cpu.flag_carry, false);
    assert_eq!(cpu.flag_negative, false);
    assert_eq!(cpu.flag_zero, false);
}

#[test]
fn cmp() {
    let mut cpu = Cpu::new();

    // CPX - Immediate
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x00u16] = 0xE0.into();
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x01u16] = 0x30.into();
    cpu.reg_x = 0x20.into();

    let _ = cpu.execute_instruction();

    assert_eq!(cpu.reg_x.get_value(), 0x20);
    assert_eq!(cpu.flag_carry, false);
    assert_eq!(cpu.flag_zero, false);
    assert_eq!(cpu.flag_negative, true);

    // CPX - Zero Page
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x02u16] = 0xE4.into();
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x03u16] = 0x01.into();
    cpu.memory[0x01u16] = 0x10.into();
    cpu.reg_x = 0x20.into();

    assert_eq!(cpu.reg_x.get_value(), 0x20);
    assert_eq!(cpu.get_zero_page_addr().get_value(), 0x01);
    assert_eq!(cpu.get_memory_addr(cpu.get_zero_page_addr().into()).get_value(), 0x10);

    let _ = cpu.execute_instruction();

    assert_eq!(cpu.reg_x.get_value(), 0x20);
    assert_eq!(cpu.flag_carry, true);
    assert_eq!(cpu.flag_zero, false);
    assert_eq!(cpu.flag_negative, false);


    // CPY - Immediate
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x04u16] = 0xC0.into();
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x05u16] = 0x50.into();
    cpu.reg_y = 0x50.into();

    let _ = cpu.execute_instruction();

    assert_eq!(cpu.reg_y.get_value(), 0x50);
    assert_eq!(cpu.flag_carry, true);
    assert_eq!(cpu.flag_zero, true);
    assert_eq!(cpu.flag_negative, false);

}

#[test]
fn stack() {
    let mut cpu = Cpu::new();

    // PHA - Push accumulator
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x00u16] = 0x48.into();
    cpu.reg_a = 0x20.into();

    let mut exec_out = cpu.execute_instruction();
    assert_eq!(exec_out.is_err(), false);

    assert_eq!(cpu.memory[consts::STACK_ADDR + cpu.stack_pointer.get_value() as u16 + 1].get_value(), 0x20);

    // PLA - Pull accumulator
    cpu.reg_a = 0x30.into();
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x01u16] = 0x68.into();

    assert_eq!(cpu.reg_a.get_value(), 0x30);
    exec_out = cpu.execute_instruction();
    assert_eq!(exec_out.is_err(), false);
    assert_eq!(cpu.reg_a.get_value(), 0x20);
}

#[test]
fn rotate() {
    // ROL - Accumulator
    let mut cpu = Cpu::new();

    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x00u16] = 0x2A.into();
    cpu.reg_a = 0xA3.into();

    assert_eq!(cpu.flag_carry, false);
    assert_eq!(cpu.reg_a.as_array(), [true, true, false, false, false, true, false, true]);

    let _ = cpu.execute_instruction();

    assert_eq!(cpu.flag_carry, true);
    assert_eq!(cpu.reg_a.as_array(), [false, true, true, false, false, false, true, false]);

    // ROR - Accumulator
    cpu.memory[consts::PROGRAM_MEMORY_ADDR + 0x01u16] = 0x6A.into();
    cpu.reg_a = 0xB2.into();

    assert_eq!(cpu.flag_carry, true);
    assert_eq!(cpu.reg_a.as_array(), [false, true, false, false, true, true, false, true]);

    let _ = cpu.execute_instruction();

    assert_eq!(cpu.flag_carry, false);
    assert_eq!(cpu.reg_a.as_array(), [true, false, false, true, true, false, true, true]);
}

// General tests
fn _general_test_util(program: &str) -> Cpu {
    // Creates a cpu, loads the program to the correct memory address and run the program until a break occures
    let program_hex_strings: Vec<&str> = program.split(" ").collect();
    
    let mut cpu = Cpu::new();
    log::info!("Running program {}", program);

    for (index, value) in program_hex_strings.iter().enumerate() {
        cpu.set_memory_addr(Double::new_from_u16(consts::PROGRAM_MEMORY_ADDR + index as u16), 
                            u8::from_str_radix(value, 16).unwrap().into());
    }

    loop {
        let execute_result = cpu.execute_instruction();
        match execute_result {
            Ok(()) => (),
            Err(err) => {
                match err {
                    _ => {
                        panic!("An error occured : {:?}", err);
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
    let cpu = _general_test_util(program_string);

    assert_eq!(cpu.get_memory_addr(Double::new_from_u16(0x0200)), Byte::new(0x01));
    assert_eq!(cpu.get_memory_addr(Double::new_from_u16(0x0201)), Byte::new(0x05));
    assert_eq!(cpu.get_memory_addr(Double::new_from_u16(0x0202)), Byte::new(0x08));
    assert_eq!(cpu.reg_a, Byte::new(0x08));
    assert_eq!(cpu.reg_x, Byte::new(0x00));
    assert_eq!(cpu.reg_y, Byte::new(0x00));
    assert_eq!(cpu.stack_pointer, Byte::new(0xFD));
}

#[test]
fn general_test_2() {
    let program_string = "a9 c0 aa e8 69 c4 00";
    let cpu = _general_test_util(program_string);

    assert_eq!(cpu.reg_a, Byte::new(0x84));
    assert_eq!(cpu.reg_x, Byte::new(0xC1));
    assert_eq!(cpu.stack_pointer, Byte::new(0xFD));
    assert_eq!(cpu.flag_carry, true);
}

#[test]
fn general_test_3() {
    let program_string = "a9 80 85 01 65 01";
    let cpu = _general_test_util(program_string);

    assert_eq!(cpu.reg_a, Byte::new(0x00));
    assert_eq!(cpu.get_memory_addr(Double::new_from_u16(0x01)), Byte::new(0x80));
    assert_eq!(cpu.flag_carry, true);
}

#[test]
fn general_test_4() {
    let program_string = "a2 08 ca 8e 00 02 e0 03 d0 f8 8e 01 02 00";
    let cpu = _general_test_util(program_string);

    assert_eq!(cpu.reg_x.get_value(), 0x03);
    assert_eq!(cpu.get_memory_addr(Double::new_from_u16(0x0200)).get_value(), 0x03);
    assert_eq!(cpu.get_memory_addr(Double::new_from_u16(0x0201)).get_value(), 0x03);
}

#[test]
fn general_test_5() {
    let program_string = "a9 02 c9 02 d0 02 85 22 00";
    let cpu = _general_test_util(program_string);

    assert_eq!(cpu.reg_a.get_value(), 0x02);
    assert_eq!(cpu.get_memory_addr(Double::new_from_u16(0x022)).get_value(), 0x02);
}

#[test]
fn general_test_6() {
    
    let program_string = "a9 01 85 f0 a9 cc 85 f1 6c f0 00";
    let cpu = _general_test_util(program_string);

    assert_eq!(cpu.reg_a.get_value(), 0xcc);
    assert_eq!(cpu.program_counter.get_value(), 0xcc02);
}

#[test]
fn general_test_7() {
    let program_string = "a2 01 a9 05 85 01 a9 07 85 02 a0 0a 8c 05 07 a1 00";
    let cpu = _general_test_util(program_string);

    assert_eq!(cpu.get_memory_addr(Double::new_from_u16(0x0705)), Byte::new(0x0a));
    assert_eq!(cpu.get_memory_addr(Double::new_from_u16(0x01)), Byte::new(0x05));
    assert_eq!(cpu.get_memory_addr(Double::new_from_u16(0x02)), Byte::new(0x07));

    assert_eq!(cpu.reg_a, Byte::new(0x0a));
}

#[test]
fn general_test_8() {
    let program_string = "a0 01 a9 03 85 01 a9 07 85 02 a2 0a 8e 04 07 b1 01";
    let cpu = _general_test_util(program_string);

    assert_eq!(cpu.get_memory_addr(Double::new_from_u16(0x0704)).get_value(), 0x0A);
    assert_eq!(cpu.reg_a.get_value(), 0x0A);
    assert_eq!(cpu.reg_y.get_value(), 0x01);
}

#[test]
fn general_test_9() { //Subroutines
    let program_string = "20 09 06 20 0c 06 20 12 06 a2 00 60 e8 e0 05 d0 fb 60 00";
    let cpu = _general_test_util(program_string);
    
    assert_eq!(cpu.reg_x.get_value(), 0x05);
    assert_eq!(cpu.program_counter.get_value(), 0x0613);
    assert_eq!(cpu.stack_pointer.get_value(), 0xFB);
}
