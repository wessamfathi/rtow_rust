#![allow(dead_code)]
#![allow(non_camel_case_types)]

use src::ray;
use src::vec3;
use src::dot;

fn hit_sphere(center: vec3, radius: f64, r: ray) -> f64 {
    let oc = r.origin - center;
    let a = r.direction.length_squared();
    let half_b = dot(oc, r.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        -1.0
    }
    else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_color(r: ray) -> vec3 {
    let t = hit_sphere(vec3::init(0.0, 0.0, -1.0), 0.5, r);

    if t > 0.0 {
        let n = (r.at(t) - vec3::init(0.0, 0.0, -1.0)).unit_vector();
        0.5 * vec3::init(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0)
    }
    else {
        let unit_direction = r.direction.unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        
        (1.0 - t) * vec3::init(1.0, 1.0, 1.0) + t * vec3::init(0.5, 0.7, 1.0)
    }
}

fn main() {

    // Image
    let aspect_ratio = 16.0 / 9.0;
	let image_width = 400;
	let image_height = (image_width as f64 / aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = vec3::init(0.0, 0.0, 0.0);
    let horizontal = vec3::init(viewport_width, 0.0, 0.0);
    let vertical = vec3::init(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - vec3::init(0.0, 0.0, focal_length);

    // Render
	println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {}", j);
    	for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;

            let r = ray {
                origin: origin,
                direction: lower_left_corner + u * horizontal + v * vertical - origin,
            };

            let pixel_color = ray_color(r);

            pixel_color.print();
    	}
    }

    eprintln!("\nDone.");
}
