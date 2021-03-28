use super::cpu::Cpu;

// Errors enum
#[derive(Debug)]
pub enum CpuError {
    BreakError(Cpu),
    UnknownOpcodeError(Cpu),
    StackOverflow(Cpu),
    StackEmpty(Cpu),
}