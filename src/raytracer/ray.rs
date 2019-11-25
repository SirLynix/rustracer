use super::vec3::Vec3;

#[derive(Debug)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        let mut dir = direction;
        dir.normalize();

        Ray {
            origin,
            direction: dir,
        }
    }

    pub fn get_direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn get_origin(&self) -> &Vec3 {
        &self.origin
    }

    pub fn point_at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}
