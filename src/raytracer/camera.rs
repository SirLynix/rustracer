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
    pub fn new(position: Vec3, direction: Vec3, fovx: f32, fovy: f32) -> Camera {
        let left_corner = Vec3::new(-fovx / 2.0, fovy / 2.0, -1.0);
        let horizontal = Vec3::new(fovx, 0.0, 0.0);
        let vertical = Vec3::new(0.0, -fovy, 0.0);

        Camera {
            direction,
            left_corner: Vec3::new(-fovx / 2.0, fovy / 2.0, -1.0),
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
