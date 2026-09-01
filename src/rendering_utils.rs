use colored::Colorize;
use image::RgbaImage;

const PIXEL_REPRESENTATION: &str = "██";

pub fn render_picture(img: &RgbaImage, ratio: u32) {
    let height = img.height();
    let width = img.width();

    for y in (0..height).step_by(ratio as usize) {
        for x in (0..width).step_by(ratio as usize) {
            let mut commulated = (0 as i32, 0 as i32, 0 as i32);
            let mut counter = 0;

            for delta_y in 0..ratio {
                for delta_x in 0..ratio {
                    if x + delta_x < width && y + delta_y < height {
                        let [r, g, b, _] = img.get_pixel(x + delta_x, y + delta_y).0;
                        commulated.0 += r as i32;
                        commulated.1 += g as i32;
                        commulated.2 += b as i32;
                        counter += 1;
                    }
                }
            }

            print!(
                "{}",
                PIXEL_REPRESENTATION.truecolor(
                    (commulated.0 / counter) as u8,
                    (commulated.1 / counter) as u8,
                    (commulated.2 / counter) as u8
                )
            );
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_image_works() {
        let rgba_image = RgbaImage::new(10, 10);
        render_picture(&rgba_image, 100);
    }
}
