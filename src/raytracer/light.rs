use super::color::Color;
use super::geometry::HitInfo;
use super::ray::Ray;
use super::scene::Scene;
use super::vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct DirectionalLight {
    color: (f32, f32, f32),
    direction: Vec3,
}

pub trait Light: Sync + Send {
    fn compute_light(&self, scene: &Scene, hit_info: &HitInfo, pixel_color: &mut Color, ray: &Ray);
}
