use super::aabb::AABB;
use super::geometry::Geometry;
use super::light::Light;
use super::vec3::Vec3;
use std::sync::Arc;

pub struct Octree {
    aabb: Option<AABB>,
    children: Option<Vec<Octree>>,
    lights: Vec<Arc<dyn Light>>,
    objects: Vec<Arc<dyn Geometry>>,
}

impl Octree {
    pub fn new() -> Octree {
        Octree {
            aabb: None,
            children: None,
            lights: Vec::new(),
            objects: Vec::new(),
        }
    }

    pub fn new_with_aabb(aabb: AABB) -> Octree {
        Octree {
            aabb: Some(aabb),
            children: None,
            lights: Vec::new(),
            objects: Vec::new(),
        }
    }

    pub fn add_light(&mut self, light: Arc<dyn Light>) {
        self.lights.push(light);
    }

    pub fn add_object(&mut self, object: Arc<dyn Geometry>) {
        if let Some(aabb) = &mut self.aabb {
            aabb.extend_to(&object.get_aabb());
        } else {
            self.aabb = Some(object.get_aabb());
        }

        self.objects.push(object);
    }

    pub fn subdivide(&mut self) {
        assert!(self.children.is_none());
        assert!(self.aabb.is_some());

        let aabb = self.aabb.unwrap();

        let mins = aabb.get_mins();
        let half_size = aabb.get_size() * 0.5f32;

        let mut children = Vec::new();

        let mut push_child = |x: f32, y: f32, z: f32| {
            let min_x = mins.x + half_size.x * x;
            let min_y = mins.y + half_size.y * y;
            let min_z = mins.z + half_size.z * z;

            let mins = Vec3::new(min_x, min_y, min_z);
            let maxs = mins + half_size;

            children.push(Octree::new_with_aabb(AABB::new(mins, maxs)));
        };

        push_child(0.0f32, 0.0f32, 0.0f32);
        push_child(0.0f32, 0.0f32, 1.0f32);
        push_child(0.0f32, 1.0f32, 0.0f32);
        push_child(0.0f32, 1.0f32, 1.0f32);
        push_child(1.0f32, 0.0f32, 0.0f32);
        push_child(1.0f32, 0.0f32, 1.0f32);
        push_child(1.0f32, 1.0f32, 0.0f32);
        push_child(1.0f32, 1.0f32, 1.0f32);

        debug_assert_eq!(children.len(), 8);

        self.objects.retain(|object| {
            let aabb = object.get_aabb();
            for sub_octree in children.iter_mut() {
                if sub_octree.aabb.unwrap().contains_aabb(&aabb) {
                    sub_octree.add_object(object.clone());
                    return false;
                }
            }

            true
        });

        self.children = Some(children);
    }

    pub fn subdivide_until(&mut self, max_objects: usize) -> bool {
        if self.objects.len() > max_objects && self.children.is_none() {
            self.subdivide();
            true
        } else {
            false
        }
    }

    pub fn subdivide_until_recurse(&mut self, max_objects: usize) {
        if self.subdivide_until(max_objects) {
            for child in self.children.as_mut().unwrap().iter_mut() {
                child.subdivide_until_recurse(max_objects);
            }
        }
    }
}
