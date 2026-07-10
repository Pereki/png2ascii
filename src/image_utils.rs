use image::{ImageReader, RgbaImage};

pub struct ImageUtils {}

impl ImageUtils {
    pub fn load_image(path: &str) -> Result<RgbaImage, Box<dyn std::error::Error>> {
        Ok(ImageReader::open(path)?.decode()?.to_rgba8())
    }
}
