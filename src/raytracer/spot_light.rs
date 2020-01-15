use super::color::Color;
use super::geometry::HitInfo;
use super::light::Light;
use super::ray::Ray;
use super::scene::Scene;
use super::vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct SpotLight {
    attenuation: f32,
    color: Color,
    direction: Vec3,
    inner_angle: f32,
    inner_angle_cosinus: f32,
    outer_angle: f32,
    outer_angle_cosinus: f32,
    inv_radius: f32,
    radius: f32,
    position: Vec3,
}

impl SpotLight {
    pub fn new(
        position: Vec3,
        direction: Vec3,
        color: Color,
        attenuation: f32,
        radius: f32,
        inner_angle: f32,
        outer_angle: f32,
    ) -> SpotLight {
        SpotLight {
            color,
            direction,
            position,
            attenuation,
            radius,
            inner_angle,
            outer_angle,
            inner_angle_cosinus: inner_angle.to_radians().cos(),
            outer_angle_cosinus: outer_angle.to_radians().cos(),
            inv_radius: 1.0 / radius,
        }
    }

    pub fn get_attenuation(&self) -> f32 {
        self.attenuation
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn get_direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn get_inner_angle(&self) -> f32 {
        self.inner_angle
    }

    pub fn get_inner_angle_cosinus(&self) -> f32 {
        self.inner_angle_cosinus
    }

    pub fn get_outer_angle(&self) -> f32 {
        self.outer_angle
    }

    pub fn get_outer_angle_cosinus(&self) -> f32 {
        self.outer_angle_cosinus
    }

    pub fn get_inv_radius(&self) -> f32 {
        self.inv_radius
    }

    pub fn get_radius(&self) -> f32 {
        self.radius
    }

    pub fn get_position(&self) -> &Vec3 {
        &self.position
    }
}

fn reflect(r: &Vec3, n: &Vec3) -> Vec3 {
    2.0 * n * Vec3::dot_product(&n, &r) - r
}

impl Light for SpotLight {
    fn compute_light(&self, scene: &Scene, hit_info: &HitInfo, pixel_color: &mut Color, ray: &Ray) {
        let mut direction = self.get_position() - hit_info.position;
        let mut length = 0.0;

        direction.normalize_out_length(&mut length);

        let diffuse_factor = Vec3::dot_product(&direction, &hit_info.normal).max(0.0);

        let light_color = self.get_color();

        let mut att = (self.get_attenuation() - self.get_inv_radius() * length).max(0.0);

        let curr_angle = Vec3::dot_product(self.get_direction(), &(-direction));
        let inner_minus_outer = self.get_inner_angle_cosinus() - self.get_outer_angle_cosinus();
        att *= ((curr_angle - self.get_outer_angle_cosinus()) / inner_minus_outer).max(0.0);

        pixel_color.r *= att * diffuse_factor * light_color.r;
        pixel_color.g *= att * diffuse_factor * light_color.g;
        pixel_color.b *= att * diffuse_factor * light_color.b;

        let mut eye_vec = ray.get_origin() - hit_info.position;
        eye_vec.normalize();

        let reflection = reflect(&direction, &hit_info.normal);
        let mut specular_factor = (Vec3::dot_product(&reflection, &eye_vec)).max(0.0);
        specular_factor = specular_factor.powf(100.0);

        pixel_color.r += specular_factor;
        pixel_color.g += specular_factor;
        pixel_color.b += specular_factor;

        if scene.intersect_dist(
            Ray::new(hit_info.position + direction * 0.01, direction),
            length,
            0.0,
        ) {
            pixel_color.r *= 0.1;
            pixel_color.g *= 0.1;
            pixel_color.b *= 0.1;
        }
    }
}
