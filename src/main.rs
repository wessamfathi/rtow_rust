#![allow(dead_code)]

use std::fs;
use rand::{Rng,thread_rng};
use rtow_rust::core;

use core::INFINITY;
use core::hit_record::HitRecord;
use core::ray::Ray;
use core::vec3::Vec3;
use core::camera::*;

use rtow_rust::shapes;
use shapes::hittable_list::HittableList;
use shapes::sphere::Sphere;

const FILE_PATH: &str = "./output/06.ppm";

fn ray_color(r: Ray, world: &HittableList) -> Vec3 {
    let mut rec = HitRecord::new();

    if world.hit(r, 0.0, INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Vec3::init(1.0, 1.0, 1.0));
    };

    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - t) * Vec3::init(1.0, 1.0, 1.0) + t * Vec3::init(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    let image_width = 400;
    let image_height = (image_width as f64 / ASPECT_RATIO) as i32;
    
    // World
    let sphere1 = Sphere {
        center: Vec3::init(0.0, 0.0, -1.0),
        radius: 0.5,
    };

    let sphere2 = Sphere {
        center: Vec3::init(0.0, -100.5, -1.0),
        radius: 100.0,
    };

    let mut world = HittableList::new();
    world.add(&sphere1);
    world.add(&sphere2);

    // Camera
    let camera = Camera::init();

    // Render
    let mut buffer = String::new();
    buffer.push_str(&format!("P3\n{} {}\n255\n", image_width, image_height));

    let mut rng = thread_rng();

    for j in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let u = ((i as f64) + rng.gen_range(0.0..1.0)) / (image_width - 1) as f64;
            let v = ((j as f64) + rng.gen_range(0.0..1.0)) / (image_height - 1) as f64;

            let r = camera.get_ray(u, v);

            let pixel_color = ray_color(r, &world);

            buffer.push_str(&pixel_color.print());
        }
    }

    let _ = fs::write(FILE_PATH, buffer);

    eprintln!("\nDone.");
}
