#![allow(unused_variables)]
mod raytracer;

extern crate minifb;
extern crate rand;

use minifb::{Key, Window, WindowOptions};
use rand::Rng;
use raytracer::hittable::Hittable;
use raytracer::ray::Ray;
use raytracer::sphere::Sphere;
use raytracer::vec3::Vec3;

const WIDTH: usize = 1200;
const HEIGHT: usize = 600;
const RAY_PER_PIXEL: u32 = 100;

fn color(r: u8, g: u8, b: u8) -> u32 {
    (r as u32) << 16 | (g as u32) << 8 | (b as u32)
}

fn background_color(ray: &Ray) -> u32 {
    let unit_x = Vec3::new(1.0, 0.0, 0.0);
    let unit_y = Vec3::new(0.0, 1.0, 0.0);

    let dot_x = 1.0 + Vec3::dot_product(&unit_x, ray.direction()) / 2.0;
    let dot_y = 1.0 + Vec3::dot_product(&unit_y, ray.direction()) / 2.0;

    let r = (dot_x * 100.0).min(255.0);
    let g = (dot_x * 100.0).min(255.0);
    let b = (100.0 + (dot_y * 100.0)).min(255.0);

    color(r as u8, g as u8, b as u8)
}

fn normal_to_color(normal: &Vec3) -> u32 {
    let r = ((normal.x + 1.0) / 2.0 * 255.0).min(255.0);
    let g = ((normal.y + 1.0) / 2.0 * 255.0).min(255.0);
    let b = ((normal.z + 1.0) / 2.0 * 255.0).min(255.0);

    color(r as u8, g as u8, b as u8)
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
    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();
    objects.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, -10020.0, -1.0),
        10000.0,
    )));

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let left_corner = Vec3::new(-2.0, 1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, -2.0, 0.0);

    let mut rng = rand::XorShiftRng::new_unseeded();

    for (index, value) in buffer.iter_mut().enumerate() {
        let x = index % WIDTH;
        let y = index / WIDTH;

        let mut color_value = 0u32;
        for i in 0..RAY_PER_PIXEL {
            let factor_x = (x as f32 + (rng.next_f32() * 2.0 - 1.0)) / WIDTH as f32;
            let factor_y = (y as f32 + (rng.next_f32() * 2.0 - 1.0)) / HEIGHT as f32;

            let ray = Ray::new(
                origin,
                left_corner + horizontal * factor_x + vertical * factor_y,
            );

            let mut has_hit = false;

            let mut closest_hit = std::f32::INFINITY;
            let mut closest_position = Vec3::zero();
            let mut closest_normal = Vec3::zero();
            for object in objects.iter() {
                let mut hit_distance = 0.0;
                let mut hit_position = Vec3::zero();
                let mut hit_normal = Vec3::zero();
                if object.compute_hit(&ray, &mut hit_distance, &mut hit_position, &mut hit_normal) {
                    if hit_distance < closest_hit {
                        closest_hit = hit_distance;
                        closest_position = hit_position;
                        closest_normal = hit_normal;
                        has_hit = true;
                    }
                }
            }

            if has_hit {
                closest_normal.normalize();
                color_value += normal_to_color(&closest_normal);
            } else {
                color_value += background_color(&ray);
            }
        }

        *value = color_value / RAY_PER_PIXEL;
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer).unwrap();
    }
}
