#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]

extern crate rand;
use rand::thread_rng;
use rand::Rng;

extern crate overload;
use overload::overload;

use std::ops;

pub const PI:f64 = 3.1415926535897932385;
pub const INFINITY:f64 = f64::INFINITY;

pub fn dot(u: vec3, v: vec3) -> f64 {
	u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

#[derive(Clone, Copy)]
pub struct vec3 {
	pub e: [f64; 3],
}

overload!(- (u: ?vec3) -> vec3 { vec3::init(-u.e[0], -u.e[1], -u.e[2]) } );

overload!((u: ?vec3) + (v: ?vec3) -> vec3 {
	vec3::init(u.e[0] + v.e[0], u.e[1] + v.e[1], u.e[2] + v.e[2])
} );

overload!((u: ?vec3) - (v: ?vec3) -> vec3 {
	vec3::init(u.e[0] - v.e[0], u.e[1] - v.e[1], u.e[2] - v.e[2])
} );

overload!((u: ?vec3) * (v: ?vec3) -> vec3 {
	vec3::init(u.e[0] * v.e[0], u.e[1] * v.e[1], u.e[2] * v.e[2])
} );

overload!((v: ?vec3) * (t: f64) -> vec3 {
	vec3::init(v.e[0] * t, v.e[1] * t, v.e[2] * t)
} );

overload!((t: f64) * (v: ?vec3) -> vec3 {
	v * t
} );

overload!((v: ?vec3) / (t: f64) -> vec3 {
	(1.0/t) * v
} );

overload!((u: &mut vec3) += (v: vec3) {
	u.e[0] += v.e[0];
	u.e[1] += v.e[1];
	u.e[2] += v.e[2];
});

overload!((u: &mut vec3) -= (v: vec3) {
	u.e[0] -= v.e[0];
	u.e[1] -= v.e[1];
	u.e[2] -= v.e[2];
});

overload!((u: &mut vec3) *= (v: vec3) {
	u.e[0] *= v.e[0];
	u.e[1] *= v.e[1];
	u.e[2] *= v.e[2];
});

overload!((u: &mut vec3) /= (v: vec3) {
	u.e[0] /= v.e[0];
	u.e[1] /= v.e[1];
	u.e[2] /= v.e[2];
});


impl vec3 {
	fn vec3() -> vec3 {
		vec3 {
			e: [0.0, 0.0, 0.0]
		}
	}

	pub fn init(e0: f64, e1: f64, e2: f64) -> vec3 {
		vec3 {
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

	fn cross(u: vec3, v: vec3) -> vec3 {
		vec3 {
			e: [
			u.e[1] * v.e[2] - u.e[2] * v.e[1],
			u.e[2] * v.e[0] - u.e[0] * v.e[2],
			u.e[0] * v.e[1] - u.e[1] * v.e[0]
			]
		}
	}

	pub fn unit_vector(&self) -> vec3 {
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
pub struct hit_record {
	pub p: vec3,
	pub normal: vec3,
	pub t: f64,
	pub front_face: bool,
}

impl hit_record {
	pub fn new() -> hit_record {
		hit_record {
			p: vec3::init(0.0, 0.0, 0.0),
			normal: vec3::init(0.0, 0.0, 0.0),
			t: 0.0,
			front_face: false
		}
	}

	fn set_face_normal(&mut self, r: ray, outward_normal: vec3) {
		self.front_face = dot(r.direction, outward_normal) > 0.0;
		self.normal = if self.front_face { outward_normal } else { -outward_normal };
	}
}


// Base class for all hittables
pub struct hittable {

}


impl hittable {
	pub fn hit(&self, r: ray, t_min: f64, t_max: f64, rec: &mut hit_record) -> bool {
		false
	}
}

pub struct hittable_list<'a> {
	hittables: Vec<&'a sphere>
}

impl<'a> hittable_list<'a> {
	pub fn new() -> hittable_list<'a> {
		hittable_list {
			hittables: Vec::new()
		}
	}

	fn clear(&mut self) {
		self.hittables.clear();
	}

	pub fn add(&mut self, object: &'a sphere) {
		self.hittables.push(object);
	}

	pub fn hit(&self, r: ray, t_min: f64, t_max: f64, rec: &mut hit_record) -> bool {
		let mut temp_rec = hit_record::new();
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

pub struct sphere {
	pub center: vec3,
	pub radius: f64,
}

impl sphere {
	pub fn hit(&self, r: ray, t_min: f64, t_max: f64, rec: &mut hit_record) -> bool {
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
pub struct ray {
	pub origin: vec3,
	pub direction: vec3,
}


impl ray {
	pub fn at(&self, t: f64) -> vec3 {
		self.origin + t * self.direction
	}
}
