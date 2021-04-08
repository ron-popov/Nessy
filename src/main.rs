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

extern crate argparse;
use argparse::{ArgumentParser, StoreTrue, Store};

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::path::Path;

use crate::rom_parser::ines::InesRom;
use crate::mapper::Mapper;



fn main() {
    let mut verbosity_level: String = "Debug".to_string();
    let mut rom_path: String = "".to_string();

    { // Parse arguments
        let mut ap = ArgumentParser::new();
        ap.set_description("Greet somebody.");
        ap.refer(&mut verbosity_level)
            .add_option(&["-v", "--verbosity-level"], Store,
            "Verbosity Level (Error|Warn|Info|Debug|Trace)");
        ap.refer(&mut rom_path)
            .add_option(&["-r", "--rom-path"], Store,
            "Path of the rom file");
        ap.parse_args_or_exit();
    }

    // Input validation and parsing
    if rom_path.len() == 0 {
        panic!("Rom Path No Given");
    }

    if !Path::new(&rom_path).exists() {
        panic!("Rom File Path does not exist");
    }

    let verbosity_level_lowercase = verbosity_level.to_lowercase();
    let console_verbosity = match &*verbosity_level_lowercase {
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => panic!("Invalid Verbosity Level")
    };

    // Initialize logger
    let mut config_builder = ConfigBuilder::new();
    config_builder.set_level_color(Level::Info, Color::Green);
    config_builder.set_location_level(LevelFilter::Off);
    config_builder.set_target_level(LevelFilter::Off);

    let config = config_builder.build();

    let mut logging_vector: Vec<Box<dyn simplelog::SharedLogger>> = vec![TermLogger::new(console_verbosity, config.clone(), TerminalMode::Mixed)];
    logging_vector.push(WriteLogger::new(LevelFilter::Trace, config.clone(), File::create("nessy.log").unwrap()));

    let _ = CombinedLogger::init(logging_vector);

    info!("Logger initialized");
    info!("Starting Nessy {}", env!("CARGO_PKG_VERSION"));

    // Read rom file buffer
    let mut file = File::open(rom_path).unwrap();
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