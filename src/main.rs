use std::fs::File;
use std::io::{Error, Write};

pub mod ray;
pub mod vector;
use ray::Ray;
use vector::{lerp, Vector};

fn generate_color_gradient(ray: &Ray) -> Vector {
    let direction_normalized: Vector = (*ray).direction.normalize();

    // Remap y = [-1..1] to [0..1] range.
    let t: f32 = 0.5 * (direction_normalized.y() + 1.0);

    return lerp(
        &Vector {
            data: [1.0, 1.0, 1.0],
        },
        &Vector {
            data: [0.5, 0.7, 1.0],
        },
        t,
    );
}

fn main() -> Result<(), Error> {
    let width: i32 = 200;
    let height: i32 = 100;
    let path = "image.ppm";

    let mut output = File::create(path)?;
    write!(output, "P3\n{} {}\n255\n", width, height)?;

    // Use Unity-like left hand Y-up, Z-forward coordinate system.
    let origin: Vector = Vector {
        data: [0.0, 0.0, 0.0],
    };
    let lower_left_corner: Vector = Vector {
        data: [-2.0, -1.0, 1.0],
    };

    let viewport_width: Vector = Vector {
        data: [4.0, 0.0, 0.0],
    };
    let viewport_height: Vector = Vector {
        data: [0.0, 2.0, 0.0],
    };

    for y in 0..height {
        for x in 0..width {
            let u: f32 = x as f32 / width as f32;
            let v: f32 = (height as f32 - y as f32) / height as f32;

            let ray: Ray = Ray {
                origin: origin,
                direction: lower_left_corner + u * viewport_width + v * viewport_height,
            };
            let color: Vector = generate_color_gradient(&ray);

            let r: u8 = (color.r() * 255.99) as u8;
            let g: u8 = (color.g() * 255.99) as u8;
            let b: u8 = (color.b() * 255.99) as u8;
            write!(output, "{} {} {}\n", r, g, b)?;
        }
    }

    Ok(())
}
