use std::fs::File;
use std::io::{Error, Write};

fn main() -> Result<(), Error> {
    let width: i32 = 200;
    let height: i32 = 100;
    let path = "image.ppm";

    let mut output = File::create(path)?;
    write!(output, "P3\n{} {}\n255\n", width, height)?;

    for y in 0..height {
        for x in 0..width {
            let u: f32 = x as f32 / width as f32;
            let v: f32 = (height as f32 - y as f32) / height as f32;

            let r: u8 = (u * 255.0) as u8;
            let g: u8 = (v * 255.0) as u8;
            let b: u8 = (0.2 * 255.0) as u8;
            write!(output, "{} {} {}\n", r, g, b)?;
        }
    }

    Ok(())
}
