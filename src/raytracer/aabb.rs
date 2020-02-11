use super::vec3::Vec3;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AABB {
    maxs: Vec3,
    mins: Vec3,
}

impl AABB {
    pub fn new(mins: Vec3, maxs: Vec3) -> AABB {
        AABB { maxs, mins }
    }

    pub fn contains_aabb(&self, other: &AABB) -> bool {
        self.contains_point(&other.mins) && self.contains_point(&other.maxs)
    }

    pub fn contains_point(&self, other: &Vec3) -> bool {
        self.mins.x >= other.x
            && self.maxs.x <= other.x
            && self.mins.y >= other.y
            && self.maxs.y <= other.y
            && self.mins.z >= other.z
            && self.maxs.z <= other.z
    }

    pub fn extend_to(&mut self, other: &AABB) {
        self.maxs.x = self.maxs.x.max(other.maxs.x);
        self.maxs.y = self.maxs.y.max(other.maxs.y);
        self.maxs.z = self.maxs.z.max(other.maxs.z);

        self.mins.x = self.mins.x.min(other.mins.x);
        self.mins.y = self.mins.y.min(other.mins.y);
        self.mins.z = self.mins.z.min(other.mins.z);
    }

    pub fn intersect(&self, other: &AABB) -> bool {
        self.maxs.x > other.mins.x && self.maxs.y > other.mins.y && self.maxs.z > other.mins.z
    }

    pub fn get_maxs(&self) -> Vec3 {
        self.maxs
    }

    pub fn get_mins(&self) -> Vec3 {
        self.mins
    }

    pub fn get_width(&self) -> f32 {
        self.maxs.x - self.mins.x
    }

    pub fn get_height(&self) -> f32 {
        self.maxs.y - self.mins.y
    }

    pub fn get_depth(&self) -> f32 {
        self.maxs.z - self.mins.z
    }

    pub fn get_size(&self) -> Vec3 {
        Vec3::new(self.get_width(), self.get_height(), self.get_depth())
    }
}
