use super::hittable::HitInfo;
use super::hittable::Hittable;
use super::light::Light;
use super::ray::Ray;
use super::vec3::Vec3;

pub struct Scene {
    lights: Vec<Light>,
    pub objects: Vec<Box<dyn Hittable>>,
}

fn background_color(ray: &Ray) -> (f32, f32, f32) {
    let unit_x = Vec3::new(1.0, 0.0, 0.0);
    let unit_y = Vec3::new(0.0, 1.0, 0.0);

    let dot_x = 1.0 + Vec3::dot_product(&unit_x, ray.direction()) / 2.0;
    let dot_y = 1.0 + Vec3::dot_product(&unit_y, ray.direction()) / 2.0;

    let r = (dot_x * 100.0).min(255.0);
    let g = (dot_x * 100.0).min(255.0);
    let b = (100.0 + (dot_y * 100.0)).min(255.0);

    (r / 255.0, g / 255.0, b / 255.0)
}

fn reflect(r: &Vec3, n: &Vec3) -> Vec3 {
    2.0 * n * Vec3::dot_product(&n, &r) - &r
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

    pub fn add_object(&mut self, object: Box<dyn Hittable>) {
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

                let (mut r, mut g, mut b): (f32, f32, f32);
                if reflection_factor > 0.001 {
                    closest_hitinfo.normal.normalize();
                    let reflection = reflect(&-ray.direction(), &closest_hitinfo.normal);

                    let mut final_color_r = o_r as f32 * (1.0 - reflection_factor);
                    let mut final_color_g = o_g as f32 * (1.0 - reflection_factor);
                    let mut final_color_b = o_b as f32 * (1.0 - reflection_factor);

                    if max_iter > 0 {
                        let (hit, reflected_r, reflected_g, reflected_b) = self.trace(
                            Ray::new(closest_hitinfo.position + reflection * 0.01, reflection),
                            max_iter - 1,
                        );

                        final_color_r += reflected_r as f32 * reflection_factor;
                        final_color_g += reflected_g as f32 * reflection_factor;
                        final_color_b += reflected_b as f32 * reflection_factor;
                    }

                    r = final_color_r;
                    g = final_color_g;
                    b = final_color_b;
                } else {
                    r = o_r;
                    g = o_g;
                    b = o_b;
                }

                if max_iter > 0 {
                    for light in self.lights.iter() {
                        let mut direction = light.get_position() - closest_hitinfo.position;
                        let mut length = 0.0;

                        direction.normalize_out_length(&mut length);

                        let diffuse_factor =
                            Vec3::dot_product(&direction, &closest_hitinfo.normal).max(0.0);

                        r *= diffuse_factor;
                        g *= diffuse_factor;
                        b *= diffuse_factor;

                        match self.intersect(Ray::new(
                            closest_hitinfo.position + direction * 0.01,
                            direction,
                        )) {
                            Some(distance) => {
                                if distance < length {
                                    r *= 0.1;
                                    g *= 0.1;
                                    b *= 0.1;
                                }
                            }
                            _ => (),
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
