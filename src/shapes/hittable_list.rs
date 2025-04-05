use super::sphere::Sphere;
use crate::core::ray::Ray;
use crate::core::hit_record::HitRecord;

pub struct HittableList<'a> {
	hittables: Vec<&'a Sphere>
}

impl<'a> HittableList<'a> {
	pub fn new() -> HittableList<'a> {
		HittableList {
			hittables: Vec::new()
		}
	}

	fn clear(&mut self) {
		self.hittables.clear();
	}

	pub fn add(&mut self, object: &'a Sphere) {
		self.hittables.push(object);
	}

	pub fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
		let mut temp_rec = HitRecord::new();
		let mut hit_anything = false;
		let mut closest_so_far = t_max;

		for hittable in &self.hittables {
			if hittable.hit(r, t_min, closest_so_far, &mut temp_rec) {
				hit_anything = true;
				closest_so_far = temp_rec.t;
				*rec = temp_rec;
			}
		}

		hit_anything
	}
}


