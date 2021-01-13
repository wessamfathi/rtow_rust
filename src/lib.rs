#![allow(dead_code)]
#![allow(non_camel_case_types)]

extern crate overload;
use overload::overload;
use std::ops;

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


