use crate::core::vec3::Vec3;
use crate::core::ray::Ray;
use crate::core::hit_record::HitRecord;
use crate::core::dot;

pub struct Sphere {
	pub center: Vec3,
	pub radius: f64,
}

impl Sphere {
	pub fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
	    let oc = r.origin - self.center;
	    let a = r.direction.length_squared();
	    let half_b = dot(oc, r.direction);
	    let c = oc.length_squared() - self.radius * self.radius;
	    let discriminant = half_b * half_b - a * c;

	    let mut result = true;

	    if discriminant < 0.0 {
	        false
	    }
	    else {
	        let sqrtd = discriminant.sqrt();

	        // Find the nearest root that lie in the acceptable range
	        let mut root = (-half_b - sqrtd) / a;

	        if root < t_min || root > t_max {
	            root = (-half_b + sqrtd) / a;

	            if root < t_min || root > t_max {
	                result = false;
	            }
	        }

	        if result {
		        rec.t = root;
		        rec.p = r.at(rec.t);
		        let outward_normal = (rec.p - self.center) / self.radius;
		        rec.set_face_normal(r, outward_normal);

		        true
	        }
	        else {
	        	false
	        }
		}
	}
}
