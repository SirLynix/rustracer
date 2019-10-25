use super::hittable::Hittable;
use super::ray::Ray;
use super::vec3::Vec3;
use std::cmp::Ordering;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }

    pub fn get_center(&self) -> Vec3 {
        self.center
    }

    pub fn get_radius(&self) -> f32 {
        self.radius
    }
}

impl Hittable for Sphere {
    fn compute_hit(&self, ray: &Ray, t: &mut f32, hitpoint: &mut Vec3, normal: &mut Vec3) -> bool {
        let ray_to_sphere = ray.origin() - self.center;
        let a = Vec3::dot_product(ray.direction(), ray.direction());
        let b = Vec3::dot_product(ray.direction(), &ray_to_sphere);
        let c = Vec3::dot_product(&ray_to_sphere, &ray_to_sphere) - self.radius * self.radius;

        let delta = (b * b) - a * c;

        let mut compute_result = |param: f32| {
            *t = param;
            *hitpoint = ray.point_at(param);
            *normal = *hitpoint - &self.center;
        };

        if delta >= 0.0 {
            let sqr_delta = delta.sqrt();
            let temp = (-b - sqr_delta) / a;
            if temp > 0.0 {
                compute_result(temp);
                return true;
            }

            let temp = (-b + sqr_delta) / a;
            if temp > 0.0 {
                compute_result(temp);
                return true;
            }
        }

        false
    }
}
