use crate::mapper::Mapper;
use crate::core::consts;

use std::sync::{Arc, Mutex};
use crate::ppu::{Pixel, Bitmap};

use rand::Rng;

pub struct PPU {
    mapper: Arc<Mutex<Box<dyn Mapper>>>,
    bitmap: Bitmap,
}

impl PPU {
    pub fn new(mapper: Arc<Mutex<Box<dyn Mapper>>>) -> PPU {
        PPU{mapper:mapper, bitmap: Bitmap::new(consts::NES_SCREEN_WIDTH as usize, consts::NES_SCREEN_HEIGHT as usize).unwrap()}
    }

    pub fn update_frame(&mut self) {
        log::debug!("Updating ppu frame");
        let mut rng = rand::thread_rng();
        
        for x in 0..self.bitmap.get_width() {
            for y in 0..self.bitmap.get_height() {                
                let is_white: bool = rng.gen_range(0..100) / 10 == 0;
                let i = y * self.bitmap.get_width() + x;

                if is_white {
                    self.bitmap.set_pixel(i, 0, Pixel::white());
                } else {
                    self.bitmap.set_pixel(i, 0, Pixel::black());
                }
                
            }
        }
    }

    pub fn get_picture(&self) -> &Bitmap {
        &self.bitmap
    }
}