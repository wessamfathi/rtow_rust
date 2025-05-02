use crate::core::ray::Ray;
use crate::core::vec3::Vec3;

pub const ASPECT_RATIO: f64 = 16.0 / 9.0;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(v_fov: f64, aspect: f64) -> Camera {
        let theta = v_fov * std::f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let origin = Vec3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(2.0 * half_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, 2.0 * half_height, 0.0);
        let lower_left_corner = Vec3::new(-half_width, -half_height, -1.0);

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        }
    }
}
