use super::ray::Ray;
use super::vec3::Vec3;
use rand::Rng;

pub struct Camera {
    left_corner: Vec3,
    lens_radius: f32,
    horizontal: Vec3,
    position: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(
        position: Vec3,
        lookat: Vec3,
        up: Vec3,
        aspect: f32,
        fovy: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Camera {
        let lens_radius = aperture / 2.0;
        let half_theta = fovy.to_radians() / 2.0;
        let half_height = half_theta.tan();
        let half_width = half_height * aspect;

        let mut w = position - lookat;
        w.normalize();
        let mut u = Vec3::cross_product(&up, &w);
        u.normalize();
        let v = Vec3::cross_product(&w, &u);

        let left_corner =
            position - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w;
        let horizontal = 2.0 * half_width * focus_dist * u;
        let vertical = 2.0 * half_height * focus_dist * v;

        Camera {
            left_corner,
            lens_radius,
            horizontal,
            position,
            vertical,
        }
    }

    pub fn get_ray(&self, rng: &mut rand::XorShiftRng, x: f32, y: f32) -> Ray {
        let origin = self.position;
        let offset = Vec3::new(
            self.lens_radius * (rng.next_f32() * 2.0 - 1.0),
            self.lens_radius * (rng.next_f32() * 2.0 - 1.0),
            0.0,
        );

        Ray::new(
            origin + offset,
            self.left_corner + self.horizontal * x + self.vertical * y - origin - offset,
        )
    }
}
