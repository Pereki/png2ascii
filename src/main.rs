mod args;
mod image_utils;
mod rendering_utils;
mod user_interaction_utils;

use std::{error::Error, process};

use crate::{
    image_utils::load_image, rendering_utils::render_picture,
    user_interaction_utils::get_path_and_ratio,
};

fn print(path: &str, ratio: u32) -> Result<(), Box<dyn Error>> {
    let img = load_image(path)?;
    render_picture(&img, ratio);
    Ok(())
}

fn main() {
    let (path, ratio) = get_path_and_ratio();
    match print(&path, ratio) {
        Ok(()) => return,
        Err(err) => {
            eprintln!("Png could not be rendered into ascii.\n{err}");
            process::exit(1);
        }
    };
}
