mod mapper_nrom;

pub use mapper_nrom::NROMMapper;

use crate::core::Double;
use crate::core::Byte;

// Mapper Errors Enum
#[derive(Debug)]
pub enum MapperError {
    InvalidMemoryAddrRequseted(Double),
}

// Mapper Trait
pub trait Mapper {
    fn get_memory_addr(&self, addr: Double) -> Result<Byte, MapperError> ;
    fn set_memory_addr(&mut self, addr: Double, value: Byte) -> Result<(), MapperError>;
}