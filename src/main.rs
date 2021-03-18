mod core;
mod rom_parser;

#[macro_use] extern crate log;

use simplelog::{ConfigBuilder, Level, CombinedLogger, TermLogger, WriteLogger, LevelFilter, TerminalMode, Color};

use std::fs::File;
use std::io::Read;

use crate::rom_parser::ines::InesRom;
use crate::core::cpu::Cpu;
use crate::core::byte::Byte;
use crate::core::double::Double;

fn main() {

   // Initialize logger
   let mut config_builder = ConfigBuilder::new();
   config_builder.set_level_color(Level::Info, Color::Green);

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

   let parser = InesRom::new(rom_buffer).unwrap();
   let mut cpu = parser.load_cpu();

   loop {
       let instruction_out = cpu.execute_instruction();
       log::trace!("CPU STATE : {}", cpu);
       if instruction_out.is_err() {
           log::info!("Stopping execution due to error {:?}", instruction_out.unwrap_err());
           break;
       }
   }
}