mod args;
mod image_utils;
mod rendering_utils;
mod user_interaction_utils;

use std::error::Error;

use crate::{
    image_utils::ImageUtils, rendering_utils::RenderingUtils,
    user_interaction_utils::UserInteractionUtils,
};

fn print(path: &str, ratio: u32) -> Result<(), Box<dyn Error>> {
    let img = ImageUtils::load_image(path)?;
    RenderingUtils::render_picture(&img, ratio);
    Ok(())
}

fn main() {
    let (path, ratio) = UserInteractionUtils::get_path_and_ratio();
    let _ = print(&path, ratio);


}
