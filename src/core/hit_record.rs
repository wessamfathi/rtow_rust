use crate::core::dot;
use crate::core::ray::Ray;
use crate::core::vec3::Vec3;
use crate::materials::Material;

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Option<Material>,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
            material: None,
        }
    }

    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = dot(r.direction, outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
