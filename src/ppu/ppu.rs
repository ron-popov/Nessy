use crate::mapper::Mapper;

use std::sync::{Arc, Mutex};

pub struct PPU {
    mapper: Arc<Mutex<Box<dyn Mapper>>>,
}

impl PPU {
    pub fn new(mapper: Arc<Mutex<Box<dyn Mapper>>>) -> PPU {
        PPU{mapper:mapper}
    }
}