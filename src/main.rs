#![allow(dead_code)]

use src::Ray;
use src::Vec3;
use src::Sphere;
use src::INFINITY;
use src::HitRecord;
use src::HittableList;

use std::fs;

const FILE_PATH: &str = "./output/image.ppm";

fn ray_color(r: Ray, world: &HittableList) -> Vec3 {
    let mut rec = HitRecord::new();

    if world.hit(r, 0.0, INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Vec3::init(1.0, 1.0, 1.0))
    };

    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - t) * Vec3::init(1.0, 1.0, 1.0) + t * Vec3::init(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
	let image_width = 400;
	let image_height = (image_width as f64 / aspect_ratio) as i32;

    // World
    let sphere1 = Sphere {
        center: Vec3::init(0.0, 0.0, -1.0),
        radius: 0.5
    };

    let sphere2 = Sphere {
        center: Vec3::init(0.0, -100.5, -1.0),
        radius: 100.0
    };

    let mut world = HittableList::new();
    world.add(&sphere1);
    world.add(&sphere2);

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::init(0.0, 0.0, 0.0);
    let horizontal = Vec3::init(viewport_width, 0.0, 0.0);
    let vertical = Vec3::init(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::init(0.0, 0.0, focal_length);

    // Render
    let mut buffer = String::new();
    buffer.push_str(&format!(
        "P3\n{} {}\n255\n",
        image_width,
        image_height
    ));

    for j in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {}", j);
    	for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;

            let r = Ray {
                origin: origin,
                direction: lower_left_corner + u * horizontal + v * vertical - origin,
            };

            let pixel_color = ray_color(r, &world);

            buffer.push_str(&pixel_color.print());
    	}
    }

    let _ = fs::write(FILE_PATH, buffer);

    eprintln!("\nDone.");
}
