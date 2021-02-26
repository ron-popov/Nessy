mod core;

#[macro_use] extern crate log;
extern crate simplelog;

use simplelog::{ConfigBuilder, Level, CombinedLogger, TermLogger, LevelFilter, TerminalMode, Color};
// use crate::core::cpu::Cpu;

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
    // debug!("Initializing CPU");
    // let cpu = Cpu::new();
}