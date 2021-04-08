use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use crate::mapper::Mapper;
use crate::cpu::Cpu;


pub fn start_cpu_thread(mapper_mutex: Arc::<Mutex::<Box::<dyn Mapper>>>) -> thread::JoinHandle<()> {
    // Start CPU Thread
    let cpu_thread = thread::spawn(move || {
        let mut cpu = Cpu::new(mapper_mutex).unwrap();

        let start_time = Instant::now();

        log::info!("Starting CPU Thread");
        loop {
            // Execute instruction
            let instruction_out = cpu.execute_instruction();
            if instruction_out.is_err() {
                let cpu_error = instruction_out.unwrap_err();
                log::info!("Stopping execution due to error {:?}", cpu_error);
                break;
            }
        }

        let end_time = Instant::now();
        let cpu_run_duration = end_time - start_time;

        log::debug!("Ran a total of {} cycles for {}ns", cpu.get_cycle_counter(), cpu_run_duration.as_nanos());
        log::debug!("Averaget CPU Frequncy is {}HZ",    cpu.get_cycle_counter() as f64 / 
                                                        cpu_run_duration.as_nanos() as f64 * (1.0/1000000000.0));
        log::debug!("Averaget Cycle took {}ns to run",  cpu_run_duration.as_nanos() / cpu.get_cycle_counter() as u128);
        log::info!("Closing CPU Thread");
    });

    return cpu_thread;
}