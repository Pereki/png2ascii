use colored::Colorize;
use image::ImageReader;

fn print() {
    let img = match ImageReader::open("test.png") {
        Ok(reader) => match reader.decode() {
            Ok(img) => img.to_rgba8(),
            Err(err) => {
                eprintln!("{err}");
                return;
            }
        },
        Err(err) => {
            eprintln!("First Level: {err}");
            return;
        }
    };

    let height = img.height();
    let width = img.width();

    for y in (0..height).step_by(10) {
        for x in (0..width).step_by(10) {
            let mut commulated = (0 as i32, 0 as i32, 0 as i32);
            let mut counter = 0;

            for delta_y in 0..10 {
                for delta_x in 0..10 {
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
                "██".truecolor(
                    (commulated.0 / counter) as u8,
                    (commulated.1 / counter) as u8,
                    (commulated.2 / counter) as u8
                )
            );
        }
        println!();
    }
}

fn main() {
    let _ = print();
}
