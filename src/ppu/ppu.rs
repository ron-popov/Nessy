use crate::mapper::Mapper;
use crate::core::consts;

use std::sync::{Arc, Mutex};

pub struct PPU {
    mapper: Arc<Mutex<Box<dyn Mapper>>>,
}

impl PPU {
    pub fn new(mapper: Arc<Mutex<Box<dyn Mapper>>>) -> PPU {
        PPU{mapper:mapper}
    }

    // pub fn get_canvas(&self) -> &Image {
    //     &self.canvas
    // }
}