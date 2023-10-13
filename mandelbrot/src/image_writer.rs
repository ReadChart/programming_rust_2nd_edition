use std::fs::File;
use image::codecs::png::PngEncoder;
use image::{ColorType, ImageEncoder, ImageError};

pub fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), ImageError>
{
    let output = File::create(filename)?;
    let encoder = PngEncoder::new(output);
    encoder.write_image(&pixels, bounds.0 as u32, bounds.1 as u32, ColorType::L8)?;
    Ok(())
}