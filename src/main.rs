#![allow(unused_variables)]
mod raytracer;

extern crate minifb;
extern crate png;
extern crate rand;

use minifb::{Key, Window, WindowOptions};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
// To use encoder.set()
use rand::Rng;
use raytracer::camera::Camera;
use raytracer::light::Light;
use raytracer::scene::Scene;
use raytracer::sphere::Sphere;
use raytracer::textured_sphere::TexturedSphere;
use raytracer::vec3::Vec3;
use std::time::Instant;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
const BOX_SIDE: usize = 96;
const MAX_ITERATION: u32 = 5;
const RAY_PER_PIXEL: u32 = 100;
const RANDOM_OFFSET_COUNT: usize = RAY_PER_PIXEL as usize * 100;

fn color(r: u8, g: u8, b: u8) -> u32 {
    (r as u32) << 16 | (g as u32) << 8 | (b as u32)
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

    let origin = Vec3::new(0.0, 0.5, 0.0);
    let direction = Vec3::new(0.0, 0.0, -1.0);

    let camera = Camera::new(origin, direction, 4.0, 2.0);

    let mut scene = Scene::new();

    scene.add_light(Light::new(origin + Vec3::new(0.0, 1.5, 1.0)));

    // Ground
    scene.add_object(Box::new(TexturedSphere::new(
        Vec3::new(0.0, -10000.0, -1.0),
        10000.0,
        (1.0, 1.0, 1.0),
        0.2,
    )));

    // Left - Black
    scene.add_object(Box::new(Sphere::new(
        Vec3::new(-1.5, 0.5, -1.0),
        0.5,
        (0.0, 0.0, 0.0),
        0.9,
    )));

    // Middle - Yellow
    scene.add_object(Box::new(Sphere::new(
        Vec3::new(0.0, 0.75, -1.5),
        0.75,
        (1.0, 1.0, 0.0),
        0.5,
    )));

    // Right - Red
    scene.add_object(Box::new(Sphere::new(
        Vec3::new(1.5, 0.5, -1.0),
        0.5,
        (1.0, 0.0, 0.0),
        0.2,
    )));

    let mut rng = rand::XorShiftRng::new_unseeded();
    let mut random_offsets: Vec<f32> = vec![0.0; RANDOM_OFFSET_COUNT];
    if RAY_PER_PIXEL > 1 {
        for i in 0..RANDOM_OFFSET_COUNT {
            random_offsets[i] = rng.next_f32() * 2.0 - 1.0;
        }
    }
    let mut random_offset = 0usize;

    let box_count_x: usize = WIDTH / BOX_SIDE + if WIDTH % BOX_SIDE != 0 { 1 } else { 0 };
    let box_count_y: usize = HEIGHT / BOX_SIDE + if HEIGHT % BOX_SIDE != 0 { 1 } else { 0 };

    let mut boxes: Vec<usize> = (0..box_count_x * box_count_y).collect();
    rng.shuffle(&mut boxes);

    let start = Instant::now();

    for i in boxes.iter() {
        let x = i % box_count_x;
        let y = i / box_count_x;

        let min_x = x * BOX_SIDE;
        let min_y = y * BOX_SIDE;

        let max_x = (min_x + BOX_SIDE).min(WIDTH);
        let max_y = (min_y + BOX_SIDE).min(HEIGHT);

        for y in min_y..max_y {
            for x in min_x..max_x {
                let mut color_r = 0f32;
                let mut color_g = 0f32;
                let mut color_b = 0f32;
                for i in 0..RAY_PER_PIXEL {
                    let factor_x = (x as f32 + random_offsets[random_offset + 0]) / WIDTH as f32;
                    let factor_y = (y as f32 + random_offsets[random_offset + 1]) / HEIGHT as f32;
                    random_offset += 2;
                    if random_offset >= random_offsets.len() {
                        random_offset = 0;
                    }

                    let ray = camera.get_ray(factor_x, factor_y);
                    let (_, r, g, b) = scene.trace(ray, MAX_ITERATION);

                    color_r += r;
                    color_g += g;
                    color_b += b;
                }

                color_r /= RAY_PER_PIXEL as f32;
                color_g /= RAY_PER_PIXEL as f32;
                color_b /= RAY_PER_PIXEL as f32;

                let u8_r = (color_r * 255.0).min(255.0) as u8;
                let u8_g = (color_g * 255.0).min(255.0) as u8;
                let u8_b = (color_b * 255.0).min(255.0) as u8;

                buffer[(y * WIDTH + x) as usize] = color(u8_r, u8_g, u8_b);
            }
        }

        window.update_with_buffer(&buffer).unwrap();
    }

    let duration = start.elapsed();

    println!("Rendering took {}s", duration.as_secs_f32());

    // Save as PNG
    let path = Path::new(r"raytracer.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, WIDTH as u32, HEIGHT as u32); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();

    let mut png_data = vec![0u8; 0];
    png_data.reserve_exact(WIDTH * HEIGHT * 3);

    for value in buffer.iter() {
        let r = ((value & 0x00FF0000) >> 16) as u8;
        let g = ((value & 0x0000FF00) >> 8) as u8;
        let b = ((value & 0x000000FF) >> 0) as u8;

        png_data.push(r);
        png_data.push(g);
        png_data.push(b);
    }

    writer.write_image_data(&png_data).unwrap(); // Save

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update();
    }
}
