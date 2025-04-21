use crate::core;
use crate::core::vec3::Vec3;
use crate::core::ray::Ray;
use crate::core::hit_record::HitRecord;

use super::reflect;

#[derive(Copy, Clone)]
pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Self { ref_idx }
    }

    pub fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let outward_normal: Vec3;
        let reflected = reflect(ray_in.direction, hit_record.normal);
        let ni_over_nt: f64;
        let mut refracted = Vec3::new(0.0, 0.0, 0.0);

        if core::dot(ray_in.direction, hit_record.normal) > 0.0 {
            outward_normal = -hit_record.normal;
            ni_over_nt = self.ref_idx;
        } else {
            outward_normal = hit_record.normal;
            ni_over_nt = 1.0 / self.ref_idx;
        }

        let scattered: Ray;
        if self.refract(&ray_in.direction, &outward_normal, ni_over_nt, &mut refracted) {
            scattered = Ray::new(hit_record.p, refracted);
            return Some((Vec3::new(1.0, 1.0, 1.0), scattered));
        } else {
            scattered = Ray::new(hit_record.p, reflected);
            return None;
        }
    }

    fn refract(&self, v: &Vec3, n: &Vec3, ni_over_nt: f64, refracted: &mut Vec3) -> bool {
        let uv = v.unit_vector();
        let dt = core::dot(uv, *n);
        let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
        if discriminant > 0.0 {
            *refracted = ni_over_nt * (uv - n * dt) - n * discriminant.sqrt();
            true
        } else {
            false
        }
    }
}