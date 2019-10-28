use super::vec3::Vec3;

pub struct Light {
    position: Vec3,
}

impl Light {
    pub fn new(position: Vec3) -> Light {
        Light { position }
    }

    pub fn get_position(&self) -> &Vec3 {
        &self.position
    }
}
