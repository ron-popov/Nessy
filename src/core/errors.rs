use super::cpu::Cpu;

// Errors enum
pub enum CpuError {
    BreakError(Cpu),
    UnknownOpcodeError(Cpu),
}