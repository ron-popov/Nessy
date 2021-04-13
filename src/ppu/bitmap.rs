use crate::ppu::PPUError;

#[derive(Copy, Clone, Debug)]
pub struct Pixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Pixel {
    pub fn black() -> Pixel {Pixel{red:255, green:255, blue:255}}
    pub fn white() -> Pixel {Pixel{red: 0, green: 0, blue: 0}}
}

#[derive(Clone, Debug)]
pub struct Bitmap {
    pixels: Vec<Pixel>,
    width: usize,
    height: usize,
}

impl Bitmap {
    pub fn new(width: usize, height: usize) -> Result<Bitmap, PPUError> {
        let mut bmp = Bitmap{width: width, height: height, pixels: Vec::<Pixel>::new()};

        for _ in 0..(bmp.width * bmp.height) {
            bmp.pixels.push(Pixel::black());
        }

        Ok(bmp)
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Result<Pixel, PPUError> {
        let i = y * self.width + x;
        let val = self.pixels.get(i);

        if val.is_none() {
            Err(PPUError::InvalidPixelIndex)
        } else {
            Ok(val.unwrap().clone())
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, p: Pixel) -> Result<(), PPUError> {
        let i = y * self.width + x;

        if i >= self.pixels.len() {
            Err(PPUError::InvalidPixelIndex)
        } else {
            self.pixels[i] = p;
            Ok(())
        }
    }
}