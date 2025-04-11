use crate::core::dot;
use crate::core::hit_record::HitRecord;
use crate::core::ray::Ray;
use crate::core::vec3::Vec3;
use crate::materials::Material;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
	pub material: Material,
}

impl Sphere {
    pub fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin - self.center;
		let a = dot(r.direction, r.direction);
		let b = dot(oc, r.direction);
		let c = dot(oc, oc) - self.radius * self.radius;

		let discriminant = b * b - a * c;
		if discriminant > 0.0 {
			let temp = (-b - discriminant.sqrt()) / a;
			if temp < t_max && temp > t_min {
				rec.t = temp;
				rec.p = r.at(rec.t);
				rec.normal = (rec.p - self.center) / self.radius;
				rec.material = Some(self.material);
				return true;
			}
			
			let temp = (-b + discriminant.sqrt()) / a;
			if temp < t_max && temp > t_min {
				rec.t = temp;
				rec.p = r.at(rec.t);
				rec.normal = (rec.p - self.center) / self.radius;
				rec.material = Some(self.material);
				return true;
			}
		}
		false
    }
}
