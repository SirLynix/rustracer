#![allow(unused_variables)]
mod raytracer;

extern crate minifb;

use minifb::{Key, Window, WindowOptions};
use raytracer::ray::Ray;
use raytracer::vec3::Vec3;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn background_color(ray: &Ray) -> u32 {
    let unit_x = Vec3::new(1.0, 0.0, 0.0);
    let unit_y = Vec3::new(0.0, 1.0, 0.0);

    let dot_x = 1.0 + Vec3::dot_product(&unit_x, ray.direction()) / 2.0;
    let dot_y = 1.0 + Vec3::dot_product(&unit_y, ray.direction()) / 2.0;

    let r: u32 = (dot_x * 100.0) as u32;
    let g: u32 = (dot_x * 100.0) as u32;
    let b: u32 = 100 + (dot_y * 100.0) as u32;

    r << 16 | g << 8 | b
}

fn main() {
    let mut window = Window::new(
        "Raytracer - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let origin = Vec3::zero();
    let left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);

    for (index, value) in buffer.iter_mut().enumerate() {
        let x = index / WIDTH;
        let y = index % WIDTH;

        let factor_x = x as f32 / WIDTH as f32;
        let factor_y = y as f32 / HEIGHT as f32;

        let ray = Ray::new(
            origin,
            left_corner + horizontal * factor_x + vertical * factor_y,
        );

        *value = background_color(&ray);
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer).unwrap();
    }
}
