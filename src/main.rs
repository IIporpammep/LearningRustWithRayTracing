use rand::Rng;
use std::fs::File;
use std::io::{Error, Write};

pub mod camera;
pub mod hit_record;
pub mod hittables;
pub mod ray;
pub mod vector;
use camera::Camera;
use hittables::hittable::Hittable;
use hittables::hittable_list::HittableList;
use hittables::sphere::Sphere;
use ray::Ray;
use vector::{lerp, Vector};

fn main() -> Result<(), Error> {
    let mut world: HittableList = HittableList {
        hittables: Vec::new(),
    };
    world.hittables.push(Box::new(Sphere {
        centre: Vector {
            data: [0.0, 0.0, 1.0],
        },
        radius: 0.5,
    }));
    world.hittables.push(Box::new(Sphere {
        centre: Vector {
            data: [0.0, -100.5, 1.0],
        },
        radius: 100.0,
    }));

    let aspect_ratio: f32 = 16.0 / 9.0;

    let camera: Camera = Camera::new(aspect_ratio);

    let width: i32 = 200;
    let height: i32 = (width as f32 / aspect_ratio) as i32;
    let path = "image.ppm";

    let mut output = File::create(path)?;
    write!(output, "P3\n{} {}\n255\n", width, height)?;

    let samples_per_pixel: i32 = 100;
    let mut random = rand::thread_rng();

    for y in 0..height {
        for x in 0..width {
            let u: f32 = x as f32 / width as f32;
            let v: f32 = (height as f32 - y as f32) / height as f32;

            let mut result_color: Vector = Vector {
                data: [0.0, 0.0, 0.0],
            };

            for _i in 0..samples_per_pixel {
                let u_with_offset: f32 = u + random.gen::<f32>() / (width as f32);
                let v_with_offset: f32 = v + random.gen::<f32>() / (height as f32);

                let ray = camera.get_ray(u_with_offset, v_with_offset);
                result_color += calculate_color(&ray, &world);
            }

            match write_average_color(output, result_color, samples_per_pixel) {
                Ok(file) => output = file,
                Err(error) => {
                    print!("Error during file writing: {}", error);
                    return Err(error);
                }
            }
        }
    }

    Ok(())
}

fn calculate_color(ray: &Ray, world: &HittableList) -> Vector {
    match (*world).hit(ray, 0.0, f32::MAX) {
        Some(hit_result) => {
            return 0.5
                * (hit_result.normal
                    + Vector {
                        data: [1.0, 1.0, 1.0],
                    })
        }
        None => (),
    }

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

fn write_average_color(
    mut file: File,
    color: Vector,
    samples_per_pixel: i32,
) -> Result<File, Error> {
    let average_color = color / (samples_per_pixel as f32);

    let r: u8 = (average_color.r() * 255.99) as u8;
    let g: u8 = (average_color.g() * 255.99) as u8;
    let b: u8 = (average_color.b() * 255.99) as u8;
    match write!(file, "{} {} {}\n", r, g, b) {
        Ok(_it) => return Ok(file),
        Err(err) => return Err(err),
    };
}
