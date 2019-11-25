use super::geometry::Geometry;
use super::geometry::HitInfo;
use super::ray::Ray;
use super::vec3::Vec3;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Sphere {
    center: Vec3,
    color: (f32, f32, f32),
    radius: f32,
    reflection_factor: Option<f32>,
    transparency_factor: Option<f32>,
}

impl Sphere {
    pub fn new(
        center: Vec3,
        radius: f32,
        color: (f32, f32, f32),
        reflection_factor: f32,
        transparency_factor: f32,
    ) -> Sphere {
        Sphere {
            center,
            radius,
            color,
            reflection_factor: if reflection_factor > 0.001 {
                Some(reflection_factor)
            } else {
                None
            },
            transparency_factor: if transparency_factor > 0.001 {
                Some(transparency_factor)
            } else {
                None
            },
        }
    }

    pub fn get_center(&self) -> Vec3 {
        self.center
    }

    pub fn get_color(&self) -> (f32, f32, f32) {
        self.color
    }

    pub fn get_radius(&self) -> f32 {
        self.radius
    }
}

impl Geometry for Sphere {
    fn compute_hit(&self, ray: &Ray, hitinfo: Option<&mut HitInfo>) -> Option<f32> {
        let ray_to_sphere = ray.get_origin() - self.center;
        let a = Vec3::dot_product(ray.get_direction(), ray.get_direction());
        let b = Vec3::dot_product(ray.get_direction(), &ray_to_sphere);
        let c = Vec3::dot_product(&ray_to_sphere, &ray_to_sphere) - self.radius * self.radius;

        let delta = (b * b) - a * c;

        let compute_result = |param: f32, hit_info: &mut HitInfo| {
            hit_info.position = ray.point_at(param);
            hit_info.normal = hit_info.position - self.center;
        };

        if delta >= 0.0 {
            let sqr_delta = delta.sqrt();
            let distance = (-b - sqr_delta) / a;

            if distance > 0.0 {
                match hitinfo {
                    Some(hit_info) => compute_result(distance, hit_info),
                    _ => (),
                }

                return Some(distance);
            }

            let distance = (-b + sqr_delta) / a;
            if distance > 0.0 {
                match hitinfo {
                    Some(hit_info) => compute_result(distance, hit_info),
                    _ => (),
                }

                return Some(distance);
            }
        }

        None
    }

    fn get_color(&self, position: &Vec3) -> (f32, f32, f32) {
        self.get_color()
    }

    fn get_reflection_factor(&self) -> Option<f32> {
        self.reflection_factor
    }

    fn get_transparency_factor(&self) -> Option<f32> {
        self.transparency_factor
    }
}
