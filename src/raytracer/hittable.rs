use super::ray::Ray;
use super::vec3::Vec3;

pub trait Hittable {
    fn compute_hit(&self, ray: &Ray, t: &mut f32, hitpoint: &mut Vec3, normal: &mut Vec3) -> bool;
}
