use super::color::Color;
use super::geometry::Geometry;
use super::geometry::HitInfo;
use super::ray::Ray;
use super::sphere::Sphere;
use super::vec3::Vec3;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TexturedSphere {
    sphere: Sphere,
}

impl TexturedSphere {
    pub fn new(
        center: Vec3,
        radius: f32,
        color: Color,
        reflection_factor: f32,
        transparency_factor: f32,
    ) -> TexturedSphere {
        TexturedSphere {
            sphere: Sphere::new(
                center,
                radius,
                color,
                reflection_factor,
                transparency_factor,
            ),
        }
    }
}

impl Geometry for TexturedSphere {
    fn compute_hit(
        &self,
        ray: &Ray,
        hitinfo: Option<&mut HitInfo>,
        exit_dist: Option<&mut f32>,
    ) -> Option<f32> {
        self.sphere.compute_hit(ray, hitinfo, exit_dist)
    }

    fn get_color(&self, position: &Vec3) -> Color {
        let up = Vec3::new(0.0, 1.0, 0.0);

        let size = 1.0;

        let is_even = (position.z % size).abs() > size / 2.0;

        if (position.x.rem_euclid(size) > size / 2.0) ^ is_even {
            self.sphere.get_color()
        } else {
            Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
            }
        }
    }

    fn get_reflection_factor(&self) -> Option<f32> {
        self.sphere.get_reflection_factor()
    }

    fn get_transparency_factor(&self) -> std::option::Option<f32> {
        self.sphere.get_transparency_factor()
    }
}
