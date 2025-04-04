#![allow(dead_code)]
#![allow(unused_variables)]

extern crate rand;
use rand::thread_rng;
use rand::Rng;

extern crate overload;
use overload::overload;

use std::ops;

pub const PI:f64 = 3.1415926535897932385;
pub const INFINITY:f64 = f64::INFINITY;

pub fn dot(u: Vec3, v: Vec3) -> f64 {
	u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

#[derive(Clone, Copy)]
pub struct Vec3 {
	pub e: [f64; 3],
}

overload!(- (u: ?Vec3) -> Vec3 { Vec3::init(-u.e[0], -u.e[1], -u.e[2]) } );

overload!((u: ?Vec3) + (v: ?Vec3) -> Vec3 {
	Vec3::init(u.e[0] + v.e[0], u.e[1] + v.e[1], u.e[2] + v.e[2])
} );

overload!((u: ?Vec3) - (v: ?Vec3) -> Vec3 {
	Vec3::init(u.e[0] - v.e[0], u.e[1] - v.e[1], u.e[2] - v.e[2])
} );

overload!((u: ?Vec3) * (v: ?Vec3) -> Vec3 {
	Vec3::init(u.e[0] * v.e[0], u.e[1] * v.e[1], u.e[2] * v.e[2])
} );

overload!((v: ?Vec3) * (t: f64) -> Vec3 {
	Vec3::init(v.e[0] * t, v.e[1] * t, v.e[2] * t)
} );

overload!((t: f64) * (v: ?Vec3) -> Vec3 {
	v * t
} );

overload!((v: ?Vec3) / (t: f64) -> Vec3 {
	(1.0/t) * v
} );

overload!((u: &mut Vec3) += (v: Vec3) {
	u.e[0] += v.e[0];
	u.e[1] += v.e[1];
	u.e[2] += v.e[2];
});

overload!((u: &mut Vec3) -= (v: Vec3) {
	u.e[0] -= v.e[0];
	u.e[1] -= v.e[1];
	u.e[2] -= v.e[2];
});

overload!((u: &mut Vec3) *= (v: Vec3) {
	u.e[0] *= v.e[0];
	u.e[1] *= v.e[1];
	u.e[2] *= v.e[2];
});

overload!((u: &mut Vec3) /= (v: Vec3) {
	u.e[0] /= v.e[0];
	u.e[1] /= v.e[1];
	u.e[2] /= v.e[2];
});


impl Vec3 {
	fn vec3() -> Vec3 {
		Vec3 {
			e: [0.0, 0.0, 0.0]
		}
	}

	pub fn init(e0: f64, e1: f64, e2: f64) -> Vec3 {
		Vec3 {
			e: [e0, e1, e2]
		}
	}

	fn get(&self, index: usize) -> f64 {
		self.e[index]
	}

	pub fn x(&self) -> f64 {
		self.e[0]
	}

	pub fn y(&self) -> f64 {
		self.e[1]
	}

	pub fn z(&self) -> f64 {
		self.e[2]
	}

	fn length(&self) -> f64 {
		self.length_squared().sqrt()
	}

	pub fn length_squared(&self) -> f64 {
		self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
	}

	fn cross(u: Vec3, v: Vec3) -> Vec3 {
		Vec3 {
			e: [
			u.e[1] * v.e[2] - u.e[2] * v.e[1],
			u.e[2] * v.e[0] - u.e[0] * v.e[2],
			u.e[0] * v.e[1] - u.e[1] * v.e[0]
			]
		}
	}

	pub fn unit_vector(&self) -> Vec3 {
		self / self.length()
	}

	pub fn print(&self) {
		println!(
			"{} {} {}",
			(255.99999 * self.e[0]) as i32,
			(255.99999 * self.e[1]) as i32,
			(255.99999 * self.e[2]) as i32)
	}
}

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

fn random() -> f64 {
	// Returns a random real in [0,1)
	let mut rng = thread_rng();
	rng.gen_range(0.0..1.0)
}

fn random_range(min: f64, max: f64) -> f64 {
	// Returns a random real in [min,max])
	let mut rng = thread_rng();
	rng.gen_range(min..max)
}

#[derive(Copy, Clone)]
pub struct HitRecord {
	pub p: Vec3,
	pub normal: Vec3,
	pub t: f64,
	pub front_face: bool,
}

impl HitRecord {
	pub fn new() -> HitRecord {
		HitRecord {
			p: Vec3::init(0.0, 0.0, 0.0),
			normal: Vec3::init(0.0, 0.0, 0.0),
			t: 0.0,
			front_face: false
		}
	}

	fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
		self.front_face = dot(r.direction, outward_normal) > 0.0;
		self.normal = if self.front_face { outward_normal } else { -outward_normal };
	}
}


// Base class for all Hittables
pub struct Hittable {

}


impl Hittable {
	pub fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
		false
	}
}

pub struct HittableHittableList<'a> {
	hittables: Vec<&'a Sphere>
}

impl<'a> HittableHittableList<'a> {
	pub fn new() -> HittableHittableList<'a> {
		HittableHittableList {
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



#[derive(Clone, Copy)]
pub struct Ray {
	pub origin: Vec3,
	pub direction: Vec3,
}


impl Ray {
	pub fn at(&self, t: f64) -> Vec3 {
		self.origin + t * self.direction
	}
}
