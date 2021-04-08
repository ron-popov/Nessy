use std::sync::{Arc, Mutex};
use std::thread;

use crate::mapper::Mapper;
use crate::cpu::Cpu;

pub fn start_cpu_thread(mapper_mutex: Arc::<Mutex::<Box::<dyn Mapper>>>) -> thread::JoinHandle<()> {
    // Start CPU Thread
    let cpu_thread = thread::spawn(move || {
        let mut cpu = Cpu::new(mapper_mutex).unwrap();

        // TODO : Count cycles and time to match CPU Frequency

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

    return cpu_thread;
}