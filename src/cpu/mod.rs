pub mod instructions;
pub mod cpu;

use crate::core::Byte;

#[derive(Debug)]
pub enum CpuError {
    BreakError,
    UnknownOpcodeError(Byte),
    StackOverflow,
    StackEmpty,
}