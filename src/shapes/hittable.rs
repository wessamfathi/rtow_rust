use crate::core::ray::Ray;
use crate::core::hit_record::HitRecord;

// Base class for all Hittables
pub struct Hittable {

}


impl Hittable {
	pub fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
		false
	}
}