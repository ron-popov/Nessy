mod ppu;
mod bitmap;

pub use ppu::PPU;

pub use bitmap::{Pixel, Bitmap};

#[derive(Debug)]
pub enum PPUError {
    InvalidPixelIndex
}