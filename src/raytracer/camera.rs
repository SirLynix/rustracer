use super::ray::Ray;
use super::vec3::Vec3;

pub struct Camera {
    direction: Vec3,
    left_corner: Vec3,
    horizontal: Vec3,
    position: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(position: Vec3, direction: Vec3, aspect: f32, fovy: f32) -> Camera {
        let half_theta = fovy.to_radians() / 2.0;
        let half_height = half_theta.tan();
        let half_width = half_height * aspect;

        let left_corner = Vec3::new(-half_width, half_height, -1.0);
        let horizontal = Vec3::new(2.0 * half_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, -2.0 * half_height, 0.0);

        Camera {
            direction,
            left_corner,
            horizontal,
            position,
            vertical,
        }
    }

    pub fn get_ray(&self, x: f32, y: f32) -> Ray {
        Ray::new(
            self.position,
            self.left_corner + self.horizontal * x + self.vertical * y,
        )
    }
}
