mod core;
mod rom_parser;
mod cpu;
mod mapper;
mod nestest;
mod ppu;
mod cpu_thread;
mod ppu_thread;

#[macro_use] extern crate bmp;

#[macro_use] extern crate log;
use simplelog::{ConfigBuilder, Level, CombinedLogger, TermLogger, WriteLogger, LevelFilter, TerminalMode, Color};

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};

use crate::rom_parser::ines::InesRom;
use crate::mapper::Mapper;



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

    // Initialize threads
    let cpu_thread = cpu_thread::start_cpu_thread(mapper_cpu_mutex);
    let (ppu_thread, ui_thread) = ppu_thread::start_ppu_thread(mapper_ppu_mutex);

    // Wait for them to finish
    cpu_thread.join().unwrap();
    ppu_thread.join().unwrap();
}