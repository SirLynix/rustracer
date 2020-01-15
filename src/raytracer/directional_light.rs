use super::color::Color;
use super::geometry::HitInfo;
use super::light::Light;
use super::ray::Ray;
use super::scene::Scene;
use super::vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct DirectionalLight {
    color: (f32, f32, f32),
    direction: Vec3,
}

impl DirectionalLight {
    pub fn new(direction: Vec3, color: (f32, f32, f32)) -> DirectionalLight {
        DirectionalLight { color, direction }
    }

    pub fn get_color(&self) -> (f32, f32, f32) {
        self.color
    }

    pub fn get_direction(&self) -> &Vec3 {
        &self.direction
    }
}

impl Light for DirectionalLight {
    fn compute_light(&self, scene: &Scene, hit_info: &HitInfo, pixel_color: &mut Color, ray: &Ray) {
        let light_dir = -self.get_direction();

        let diffuse_factor = Vec3::dot_product(&light_dir, &hit_info.normal).max(0.0);

        let (light_color_r, light_color_g, light_color_b) = self.get_color();

        pixel_color.r *= diffuse_factor * light_color_r;
        pixel_color.g *= diffuse_factor * light_color_g;
        pixel_color.b *= diffuse_factor * light_color_b;

        if let Some(_) = scene.intersect(Ray::new(hit_info.position - light_dir * 0.01, light_dir))
        {
            pixel_color.r *= 0.1;
            pixel_color.g *= 0.1;
            pixel_color.b *= 0.1;
        }
    }
}
