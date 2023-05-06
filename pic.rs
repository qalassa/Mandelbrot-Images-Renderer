use image::{ImageBuffer, Rgb};
use image::ImageError;
use std::io::Result;

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub buffer: Vec<u8>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        Image {
            width,
            height,
            buffer: vec![0; (width * height * 3) as usize],
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: [u8; 3]) {
        let index = (x + y * self.width) as usize * 3;
        self.buffer[index..index + 3].copy_from_slice(&color);
    }

    pub fn save(&self, file_name: &str) -> Result<()> {
        let buffer = ImageBuffer::<Rgb<u8>, _>::from_raw(self.width, self.height, &self.buffer[..])
            .expect("Failed to create ImageBuffer");
        buffer.save(file_name).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    } 
}

