use image::{ImageReader, RgbaImage};

pub fn load_image(path: &str) -> Result<RgbaImage, Box<dyn std::error::Error>> {
    Ok(ImageReader::open(path)?.decode()?.to_rgba8())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn errors_on_missing_file() {
        assert!(load_image("does_not_exist.png").is_err());
    }
}
