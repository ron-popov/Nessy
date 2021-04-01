#[test]
fn nestest_rom() {
    use std::fs::File;
    use std::io::Read;

    use crate::rom_parser::ines::InesRom;
    use crate::cpu::cpu::Cpu;

    let mut file = File::open(format!(r"{}\samples\nestest.nes", std::env::current_dir().unwrap().to_str().unwrap())).unwrap();
    let mut rom_buffer = Vec::<u8>::new();
    let bytes_read = file.read_to_end(&mut rom_buffer).unwrap();
    log::info!("Read {} from rom", bytes_read);

    let parser = InesRom::new(rom_buffer).unwrap();
    let mapper = match parser.get_mapper() {
        Ok(m) => m,
        Err(err) => panic!("Failed getting mapper from rom parser : {:?}", err),
    };
    
    let cpu_result = Cpu::new(mapper);

    if cpu_result.is_err() {
        panic!("Failed creating cpu instance : {:?}", cpu_result.unwrap_err());
    }

    let mut cpu = cpu_result.unwrap();

    loop {
        let instruction_out = cpu.execute_instruction();
        if instruction_out.is_err() {
            let cpu_error = instruction_out.unwrap_err();
            log::info!("Stopping execution due to error {:?}", cpu_error);
            match cpu_error {
                crate::cpu::CpuError::StackEmpty => {
                    if cpu.get_program_counter().get_value() == 0xC66Eu16 {
                        log::info!("Test PASSED");
                        break;
                    } else {
                        log::error!("Test FAILED");
                        return;
                    }
                },
                _ => {
                    log::error!("Test FAILED");
                    return;
                }
            }
        }
    }

    log::info!("Memory addr 0x02 : {}", cpu.get_memory_addr(0x02u16.into()));
    log::info!("Memory addr 0x03 : {}", cpu.get_memory_addr(0x03u16.into()));
}