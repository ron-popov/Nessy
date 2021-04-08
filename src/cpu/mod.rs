pub mod instructions;
pub mod cpu;
// mod cpu_tests;

pub use cpu::Cpu;

use crate::core::Byte;

#[derive(Debug)]
pub enum CpuError {
    BreakError,
    UnknownOpcodeError(Byte),
    StackOverflow,
    StackEmpty,
    FailedParsingEntryPoint
}