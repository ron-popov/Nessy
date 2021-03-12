use super::cpu::Cpu;

// Errors enum
pub enum CpuError {
    BreakError(Cpu),
    UnknownOpcodeError(Cpu),
    StackOverflow(Cpu),
    StackEmpty(Cpu),
}