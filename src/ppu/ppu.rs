use crate::mapper::Mapper;
use crate::core::consts;

use std::sync::{Arc, Mutex};
use crate::ppu::{Pixel, Bitmap};

pub struct PPU {
    mapper: Arc<Mutex<Box<dyn Mapper>>>,
    picture: Bitmap,
}

impl PPU {
    pub fn new(mapper: Arc<Mutex<Box<dyn Mapper>>>) -> PPU {
        PPU{mapper:mapper, picture: Bitmap::new(consts::NES_SCREEN_WIDTH as usize, consts::NES_SCREEN_HEIGHT as usize).unwrap()}
    }

    pub fn get_picture(&self) -> &Bitmap {
        &self.picture
    }
}