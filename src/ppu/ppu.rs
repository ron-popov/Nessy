use crate::mapper::Mapper;

pub struct PPU<'ppu> {
    mapper: Box<dyn Mapper + 'ppu>,
}

impl<'ppu> PPU<'ppu> {
    pub fn new(mapper: Box<dyn Mapper + 'ppu>) -> PPU {
        PPU{mapper:mapper}
    }
}