pub mod instructions;
pub mod cpu;

#[derive(Debug)]
pub enum CpuError {
    BreakError(cpu::Cpu),
    UnknownOpcodeError(cpu::Cpu),
    StackOverflow(cpu::Cpu),
    StackEmpty(cpu::Cpu),
}