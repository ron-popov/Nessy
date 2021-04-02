mod core;
mod rom_parser;
mod cpu;
mod mapper;
// mod nestest;
mod ppu;

#[macro_use] extern crate log;

use simplelog::{ConfigBuilder, Level, CombinedLogger, TermLogger, WriteLogger, LevelFilter, TerminalMode, Color};

use std::thread;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;

use crate::rom_parser::ines::InesRom;
use crate::cpu::cpu::Cpu;
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

    // Read sample file buffer
    let mut file = File::open(r"samples\nestest.nes").unwrap();
    let mut rom_buffer = Vec::<u8>::new();
    let bytes_read = file.read_to_end(&mut rom_buffer).unwrap();
    log::info!("Read {} from rom", bytes_read);

    // Parse rom and create a mapper
    let parser = InesRom::new(rom_buffer).unwrap();
    let mapper = match parser.get_mapper() {
        Ok(m) => m,
        Err(err) => panic!("Failed getting mapper from rom parser : {:?}", err),
    };
    
    let mut mapper_arc = Arc::new(mapper);

    // Init cpu
    let cpu_result = Cpu::new(&mut mapper_arc);

    if cpu_result.is_err() {
        panic!("Failed creating cpu instance : {:?}", cpu_result.unwrap_err());
    }

    let mut cpu = cpu_result.unwrap();

    // Init PPU
    // let ppu = PPU::new(mapper);

    // Start CPU Thread
    let cpu_thread = thread::spawn(move || {
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

    cpu_thread.join().unwrap();


    // Start PPU Thread

}