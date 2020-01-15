use super::color::Color;
use super::geometry::{Geometry, HitInfo};
use super::light::Light;
use super::ray::Ray;
use super::vec3::Vec3;
use rand::Rng;
use std::mem;

pub struct Scene {
    lights: Vec<Box<dyn Light>>,
    objects: Vec<Box<dyn Geometry>>,
}

fn background_color(ray: &Ray) -> Color {
    let unit_x = Vec3::new(1.0, 0.0, 0.0);
    let unit_y = Vec3::new(0.0, 1.0, 0.0);

    let dot_x = 1.0 + Vec3::dot_product(&unit_x, ray.get_direction()) / 2.0;
    let dot_y = 1.0 + Vec3::dot_product(&unit_y, ray.get_direction()) / 2.0;

    let r = (dot_x * 100.0).min(255.0);
    let g = (dot_x * 100.0).min(255.0);
    let b = (100.0 + (dot_y * 100.0)).min(255.0);

    Color {
        r: r / 255.0,
        g: g / 255.0,
        b: b / 255.0,
    }
}

fn reflect(r: &Vec3, n: &Vec3) -> Vec3 {
    2.0 * n * Vec3::dot_product(&n, &r) - r
}

fn refract(i: &Vec3, n: &Vec3, refractive_index: f32) -> Option<Vec3> {
    let dt = Vec3::dot_product(&i, &n);
    let discriminant = 1.0 - refractive_index * refractive_index * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(refractive_index * (i - n * dt) - n * discriminant.sqrt())
    } else {
        None
    }
}

fn schlick(cosine: f32, refractive_index: f32) -> f32 {
    let mut r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
    r0 = r0 * r0;

    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

fn fresnel(i: &Vec3, n: &Vec3, refractive_index: f32) -> f32 {
    let mut cosi = Vec3::dot_product(i, n).max(-1.0).min(1.0);
    let mut etai = 1.0;
    let mut etat = refractive_index;

    if cosi > 0.0 {
        mem::swap(&mut etai, &mut etat);
    }

    let sint = etai / etat * ((1.0 - cosi * cosi).max(0.0)).sqrt();
    if sint >= 1.0 {
        1.0
    } else {
        let cost = ((1.0 - sint * sint).max(0.0)).sqrt();
        cosi = cosi.abs();
        let rs = ((etat * cosi) - (etai * cost)) / ((etat * cosi) + (etai * cost));
        let rp = ((etai * cosi) - (etat * cost)) / ((etai * cosi) + (etat * cost));

        (rs * rs + rp * rp) / 2.0
    }
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            lights: Vec::new(),
            objects: Vec::new(),
        }
    }

    pub fn add_light(&mut self, light: Box<dyn Light>) {
        self.lights.push(light);
    }

    pub fn add_object(&mut self, object: Box<dyn Geometry>) {
        self.objects.push(object);
    }

    pub fn intersect(&self, ray: Ray) -> Option<f32> {
        let mut closest_distance = std::f32::INFINITY;
        for (i, object) in self.objects.iter().enumerate() {
            match object.compute_hit(&ray, None, None) {
                Some(dist) => {
                    if dist < closest_distance {
                        closest_distance = dist
                    }
                }
                None => (),
            }
        }

        if closest_distance < std::f32::INFINITY {
            Some(closest_distance)
        } else {
            None
        }
    }

    pub fn intersect_dist(&self, ray: Ray, dist: f32, min_dist: f32) -> bool {
        for (i, object) in self.objects.iter().enumerate() {
            match object.compute_hit(&ray, None, None) {
                Some(hit_distance) => {
                    if hit_distance < dist && hit_distance > min_dist {
                        return true;
                    }
                }
                None => (),
            }
        }

        false
    }

    pub fn trace(
        &self,
        rng: &mut rand::XorShiftRng,
        ray: Ray,
        max_iter: u32,
        min_dist: f32,
    ) -> (f32, f32, Color) {
        let mut closest_object: Option<usize> = None;
        let mut closest_distance = std::f32::INFINITY;
        let mut closest_exit_distance = std::f32::INFINITY;
        let mut closest_hitinfo = HitInfo {
            position: Vec3::zero(),
            normal: Vec3::zero(),
        };

        for (i, object) in self.objects.iter().enumerate() {
            let mut hit_info = HitInfo {
                position: Vec3::zero(),
                normal: Vec3::zero(),
            };

            let mut exit_dist = 0f32;
            match object.compute_hit(&ray, Some(&mut hit_info), Some(&mut exit_dist)) {
                Some(distance) => {
                    if distance < closest_distance && distance > min_dist {
                        closest_distance = distance;
                        closest_hitinfo = hit_info;
                        closest_exit_distance = exit_dist;
                        closest_object = Some(i);
                    }
                }
                _ => (),
            }
        }

        match closest_object {
            Some(object) => {
                let object = &self.objects[object];

                let object_color = object.get_color(&closest_hitinfo.position);

                let reflection_factor = object.get_reflection_factor();
                let transparency_factor = object.get_transparency_factor();

                closest_hitinfo.normal.normalize();

                let mut color = Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                };

                match reflection_factor {
                    Some(reflection_factor) => {
                        let mut reflection =
                            reflect(&-ray.get_direction(), &closest_hitinfo.normal);

                        let fuzziness = 0.02f32;
                        if fuzziness > 0.0 {
                            reflection = reflection
                                + fuzziness
                                    * Vec3 {
                                        x: rng.next_f32() * 2.0 - 1.0,
                                        y: rng.next_f32() * 2.0 - 1.0,
                                        z: rng.next_f32() * 2.0 - 1.0,
                                    };
                        }

                        let mut final_color_r = object_color.r as f32 * (1.0 - reflection_factor);
                        let mut final_color_g = object_color.g as f32 * (1.0 - reflection_factor);
                        let mut final_color_b = object_color.b as f32 * (1.0 - reflection_factor);

                        if max_iter > 0 {
                            let reflection_origin =
                                if Vec3::dot_product(&reflection, &closest_hitinfo.normal) < 0.0 {
                                    closest_hitinfo.position - (reflection * 0.01)
                                } else {
                                    closest_hitinfo.position + reflection * 0.01
                                };

                            let (hit, _, reflected_color) = self.trace(
                                rng,
                                Ray::new(reflection_origin, reflection),
                                max_iter - 1,
                                0f32,
                            );

                            final_color_r += reflected_color.r as f32 * reflection_factor;
                            final_color_g += reflected_color.g as f32 * reflection_factor;
                            final_color_b += reflected_color.b as f32 * reflection_factor;
                        }

                        color.r = final_color_r;
                        color.g = final_color_g;
                        color.b = final_color_b;
                    }
                    _ => {
                        color = object_color;
                    }
                }

                if max_iter > 0 {
                    if let Some(transparency_factor) = transparency_factor {
                        let mut refractive_index = 1.8;

                        let mut outward_normal = closest_hitinfo.normal;
                        let cosine;

                        let dot_ray_normal =
                            Vec3::dot_product(&ray.get_direction(), &closest_hitinfo.normal);
                        if dot_ray_normal > 0.0 {
                            outward_normal = -outward_normal;
                            cosine = refractive_index * dot_ray_normal;
                        } else {
                            refractive_index = 1.0 / refractive_index;
                            cosine = -dot_ray_normal;
                        }

                        match refract(&ray.get_direction(), &outward_normal, refractive_index) {
                            Some(mut refraction_dir) => {
                                refraction_dir.normalize();

                                let (hit, _, refracted_color) = self.trace(
                                    rng,
                                    Ray::new(closest_hitinfo.position, refraction_dir),
                                    max_iter - 1,
                                    0.01,
                                );

                                let reflect_prob = schlick(cosine, refractive_index);
                                color.r = color.r * reflect_prob
                                    + refracted_color.r * (1.0 - reflect_prob);
                                color.g = color.g * reflect_prob
                                    + refracted_color.g * (1.0 - reflect_prob);
                                color.b = color.b * reflect_prob
                                    + refracted_color.b * (1.0 - reflect_prob);
                                /*let refraction_origin =
                                    if Vec3::dot_product(&refraction_dir, &closest_hitinfo.normal)
                                        < 0.0
                                    {
                                        closest_hitinfo.position - refraction_dir * 0.01
                                    } else {
                                        closest_hitinfo.position + refraction_dir * 0.01
                                    };

                                let (hit, _, refracted_r, refracted_g, refracted_b) = self.trace(
                                    Ray::new(refraction_origin, refraction_dir),
                                    max_iter - 1,
                                    0.01,
                                );

                                let fresnel_factor = fresnel(
                                    &ray.get_direction(),
                                    &closest_hitinfo.normal,
                                    refractive_index,
                                );

                                r = r * fresnel_factor + refracted_r * (1.0 - fresnel_factor);
                                g = g * fresnel_factor + refracted_g * (1.0 - fresnel_factor);
                                b = b * fresnel_factor + refracted_b * (1.0 - fresnel_factor);*/
                            }
                            _ => (),
                        }
                    }

                    for light in self.lights.iter() {
                        light.compute_light(&self, &closest_hitinfo, &mut color, &ray);
                    }
                }

                (closest_distance, closest_exit_distance, color)
            }
            None => {
                let color = background_color(&ray);
                (closest_distance, closest_exit_distance, color)
            }
        }
    }
}
