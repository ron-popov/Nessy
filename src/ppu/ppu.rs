use crate::mapper::Mapper;
use crate::core::consts;

use std::sync::{Arc, Mutex};
use bmp::{Image, Pixel};

pub struct PPU {
    mapper: Arc<Mutex<Box<dyn Mapper>>>,
    canvas: Image,
}

impl PPU {
    pub fn new(mapper: Arc<Mutex<Box<dyn Mapper>>>) -> PPU {
        PPU{mapper:mapper, canvas: Image::new(consts::NES_SCREEN_WIDTH, consts::NES_SCREEN_HEIGHT)}
    }

    pub fn get_canvas(&self) -> &Image {
        &self.canvas
    }
}