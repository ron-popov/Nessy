mod core;
mod rom_parser;
mod cpu;
mod mapper;
// mod nestest;
// mod ppu;

#[macro_use] extern crate log;

use simplelog::{ConfigBuilder, Level, CombinedLogger, TermLogger, WriteLogger, LevelFilter, TerminalMode, Color};

use std::thread;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use std::sync::Mutex;

use crate::rom_parser::ines::InesRom;
use crate::cpu::cpu::Cpu;
use crate::mapper::Mapper;
// use crate::ppu::PPU;

fn main() {

    // Initialize logger
    let mut config_builder = ConfigBuilder::new();
    config_builder.set_level_color(Level::Info, Color::Green);
    config_builder.set_location_level(LevelFilter::Off);
    config_builder.set_target_level(LevelFilter::Off);

    let config = config_builder.build();

    let _ = CombinedLogger::init(
        vec![TermLogger::new(LevelFilter::Debug, config.clone(), TerminalMode::Mixed),
            WriteLogger::new(LevelFilter::Trace, config.clone(), File::create("nessy.log").unwrap()),]);

    info!("Logger initialized");
    info!("Starting Nessy {}", env!("CARGO_PKG_VERSION"));

    // Read rom file buffer
    let mut file = File::open(r"samples\nestest.nes").unwrap();
    let mut rom_buffer = Vec::<u8>::new();
    let bytes_read = file.read_to_end(&mut rom_buffer).unwrap();
    log::info!("Read {} from rom", bytes_read);

    // Parse rom and create a mapper
    let parser: InesRom = InesRom::new(rom_buffer).unwrap();
    let mapper_result = parser.get_mapper();

    let mapper: Box<dyn Mapper> = match mapper_result {
        Ok(m) => m,
        Err(err) => panic!("Failed getting mapper from rom parser : {:?}", err),
    };

    // Thread safety stuff
    let mapper_cpu_mutex = Arc::new(Mutex::<Box<dyn Mapper>>::new(mapper));
    let mapper_ppu_mutex = Arc::clone(&mapper_cpu_mutex);

    // Start CPU Thread
    let cpu_thread = thread::spawn(move || {
        let mut cpu = Cpu::new(mapper_cpu_mutex).unwrap();

        log::info!("Starting CPU Thread");
        loop {
            let instruction_out = cpu.execute_instruction();
            if instruction_out.is_err() {
                let cpu_error = instruction_out.unwrap_err();
                log::info!("Stopping execution due to error {:?}", cpu_error);
                break;
            }
        }
        log::info!("Closing CPU Thread");
    });
    
    // Start ppu thread
    let ppu_thread = thread::spawn(move || {
        log::info!("Value at 0xC000 : {:?}", mapper_ppu_mutex.lock().unwrap().get_memory_addr(0xC000u16.into()))
    });

    // Wait for them to finish
    cpu_thread.join().unwrap();
    ppu_thread.join().unwrap();
}