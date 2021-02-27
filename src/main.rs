mod core;

#[macro_use] extern crate log;
extern crate simplelog;

use simplelog::{ConfigBuilder, Level, CombinedLogger, TermLogger, LevelFilter, TerminalMode, Color};

use crate::core::cpu::Cpu;
use crate::core::consts;

fn main() {
    let mut config_builder = ConfigBuilder::new();
    config_builder.set_level_color(Level::Info, Color::Green);

    // Init logger
    CombinedLogger::init(
        vec![TermLogger::new(LevelFilter::Trace, config_builder.build(), TerminalMode::Mixed)]
    ).unwrap();

    info!("Logger initialized");
    info!("Starting Nessy {}", env!("CARGO_PKG_VERSION"));

    // Init cpu
    debug!("Initializing CPU");
    let mut cpu = Cpu::new();

    let program_string = "a9 01 8d 00 02 a9 05 8d 01 02 a9 08 8d 02 02";
    let program_hex_strings: Vec<&str> = program_string.split(" ").collect();

    for (index, value) in program_hex_strings.iter().enumerate() {
        cpu.set_memory_addr(consts::PROGRAM_MEMORY_ADDR + index as u16, u8::from_str_radix(value, 16).unwrap().into());
    }

    loop {
        log::info!("{}", cpu);
        cpu.execute_instruction();
    }
}