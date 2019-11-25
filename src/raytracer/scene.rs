use super::geometry::Geometry;
use super::geometry::HitInfo;
use super::light::Light;
use super::ray::Ray;
use super::vec3::Vec3;
use std::mem;

pub struct Scene {
    lights: Vec<Light>,
    pub objects: Vec<Box<dyn Geometry>>,
}

fn background_color(ray: &Ray) -> (f32, f32, f32) {
    let unit_x = Vec3::new(1.0, 0.0, 0.0);
    let unit_y = Vec3::new(0.0, 1.0, 0.0);

    let dot_x = 1.0 + Vec3::dot_product(&unit_x, ray.get_direction()) / 2.0;
    let dot_y = 1.0 + Vec3::dot_product(&unit_y, ray.get_direction()) / 2.0;

    let r = (dot_x * 100.0).min(255.0);
    let g = (dot_x * 100.0).min(255.0);
    let b = (100.0 + (dot_y * 100.0)).min(255.0);

    (r / 255.0, g / 255.0, b / 255.0)
}

fn reflect(r: &Vec3, n: &Vec3) -> Vec3 {
    2.0 * n * Vec3::dot_product(&n, &r) - r
}

fn refract(i: &Vec3, n: &Vec3, refractive_index: f32) -> Option<Vec3> {
    let mut cosi = Vec3::dot_product(i, n).max(-1.0).min(1.0);
    let mut etai = 1.0;
    let mut etat = refractive_index;

    let mut n = *n;
    if cosi > 0.0 {
        mem::swap(&mut etai, &mut etat);
        n = -n;
    } else {
        cosi = -cosi;
    }

    let ratio = etai / etat;

    let k = 1.0 - ratio * ratio * (1.0 - cosi * cosi);
    if k > 0.0 {
        Some(ratio * i - (ratio * cosi + k.sqrt()) * n)
    } else {
        None
    }
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

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn add_object(&mut self, object: Box<dyn Geometry>) {
        self.objects.push(object);
    }

    pub fn intersect(&self, ray: Ray) -> Option<f32> {
        let mut closest_distance = std::f32::INFINITY;
        for (i, object) in self.objects.iter().enumerate() {
            match object.compute_hit(&ray, None) {
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

    pub fn intersect_dist(&self, ray: Ray, dist: f32) -> bool {
        for (i, object) in self.objects.iter().enumerate() {
            match object.compute_hit(&ray, None) {
                Some(hit_distance) => {
                    if hit_distance < dist {
                        return true;
                    }
                }
                None => (),
            }
        }

        false
    }

    pub fn trace(&self, ray: Ray, max_iter: u32) -> (f32, f32, f32, f32) {
        let mut closest_object: Option<usize> = None;
        let mut closest_distance = std::f32::INFINITY;
        let mut closest_hitinfo = HitInfo {
            position: Vec3::zero(),
            normal: Vec3::zero(),
        };

        for (i, object) in self.objects.iter().enumerate() {
            let mut hit_info = HitInfo {
                position: Vec3::zero(),
                normal: Vec3::zero(),
            };

            match object.compute_hit(&ray, Some(&mut hit_info)) {
                Some(distance) => {
                    if distance < closest_distance {
                        closest_distance = distance;
                        closest_hitinfo = hit_info;
                        closest_object = Some(i);
                    }
                }
                _ => (),
            }
        }

        match closest_object {
            Some(object) => {
                let object = &self.objects[object];

                let (o_r, o_g, o_b) = object.get_color(&closest_hitinfo.position);

                let reflection_factor = object.get_reflection_factor();
                let transparency_factor = object.get_transparency_factor();

                closest_hitinfo.normal.normalize();

                let (mut r, mut g, mut b): (f32, f32, f32);
                match reflection_factor {
                    Some(reflection_factor) => {
                        let reflection = reflect(&-ray.get_direction(), &closest_hitinfo.normal);

                        let mut final_color_r = o_r as f32 * (1.0 - reflection_factor);
                        let mut final_color_g = o_g as f32 * (1.0 - reflection_factor);
                        let mut final_color_b = o_b as f32 * (1.0 - reflection_factor);

                        if max_iter > 0 {
                            let reflection_origin =
                                if Vec3::dot_product(&reflection, &closest_hitinfo.normal) < 0.0 {
                                    closest_hitinfo.position - (reflection * 0.01)
                                } else {
                                    closest_hitinfo.position + reflection * 0.01
                                };

                            let (hit, reflected_r, reflected_g, reflected_b) =
                                self.trace(Ray::new(reflection_origin, reflection), max_iter - 1);

                            final_color_r += reflected_r as f32 * reflection_factor;
                            final_color_g += reflected_g as f32 * reflection_factor;
                            final_color_b += reflected_b as f32 * reflection_factor;
                        }

                        r = final_color_r;
                        g = final_color_g;
                        b = final_color_b;
                    }
                    _ => {
                        r = o_r;
                        g = o_g;
                        b = o_b;
                    }
                }

                if max_iter > 0 {
                    match transparency_factor {
                        Some(transparency_factor) => {
                            let (hit, refracted_r, refracted_g, refracted_b) = self.trace(
                                Ray::new(
                                    closest_hitinfo.position + ray.get_direction() * 0.01,
                                    *ray.get_direction(),
                                ),
                                max_iter - 1,
                            );

                            r = refracted_r;
                            g = refracted_g;
                            b = refracted_b;

                            /*let refractive_index = 1.3;
                            match refract(
                                &ray.get_direction(),
                                &closest_hitinfo.normal,
                                refractive_index,
                            ) {
                                Some(mut refraction_dir) => {
                                    refraction_dir.normalize();

                                    let refraction_origin = if Vec3::dot_product(
                                        &refraction_dir,
                                        &closest_hitinfo.normal,
                                    ) < 0.0
                                    {
                                        closest_hitinfo.position - &(refraction_dir * 0.01)
                                    } else {
                                        closest_hitinfo.position + &(refraction_dir * 0.01)
                                    };

                                    let (hit, refracted_r, refracted_g, refracted_b) = self.trace(
                                        Ray::new(refraction_origin, refraction_dir),
                                        max_iter - 1,
                                    );

                                    let fresnel_factor = fresnel(
                                        &ray.get_direction(),
                                        &closest_hitinfo.normal,
                                        refractive_index,
                                    );

                                    r = r * fresnel_factor + refracted_r * (1.0 - fresnel_factor);
                                    g = g * fresnel_factor + refracted_g * (1.0 - fresnel_factor);
                                    b = b * fresnel_factor + refracted_b * (1.0 - fresnel_factor);
                                }
                                _ => (),
                            }*/
                        }
                        _ => (),
                    }

                    for light in self.lights.iter() {
                        let mut direction = light.get_position() - closest_hitinfo.position;
                        let mut length = 0.0;

                        direction.normalize_out_length(&mut length);

                        let diffuse_factor =
                            Vec3::dot_product(&direction, &closest_hitinfo.normal).max(0.0);

                        r *= diffuse_factor;
                        g *= diffuse_factor;
                        b *= diffuse_factor;

                        if self.intersect_dist(
                            Ray::new(closest_hitinfo.position + direction * 0.01, direction),
                            length,
                        ) {
                            r *= 0.1;
                            g *= 0.1;
                            b *= 0.1;
                        }
                    }
                }

                (closest_distance, r, g, b)
            }
            None => {
                let (r, g, b) = background_color(&ray);
                (closest_distance, r, g, b)
            }
        }
    }
}
