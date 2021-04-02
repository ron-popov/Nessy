use crate::mapper::Mapper;

pub struct PPU {
    mapper: Box<dyn Mapper>,
}

impl PPU {
    pub fn new(mapper: Box<dyn Mapper>) -> PPU {
        PPU{mapper:mapper}
    }
}