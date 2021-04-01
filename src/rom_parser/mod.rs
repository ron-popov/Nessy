pub mod ines;

#[derive(Debug)]
pub enum ParserError {
    InvalidRom,
    UnknownMapperID(u8),
}