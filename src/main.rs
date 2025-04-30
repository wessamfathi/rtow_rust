#![allow(dead_code)]

use std::fs;
use rtow_rust::core;
use rtow_rust::materials::dielectric::Dielectric;

use core::INFINITY;
use core::hit_record::HitRecord;
use core::ray::Ray;
use core::vec3::Vec3;
use core::camera::*;

use rtow_rust::materials::{self, Material};
use materials::lambertian::Lambertian;
use materials::metal::Metal;
use rtow_rust::shapes;
use shapes::hittable_list::HittableList;
use shapes::sphere::Sphere;

const FILE_PATH: &str = "./output/11.ppm";

fn ray_color(r: Ray, world: &HittableList, depth: i32) -> Vec3 {
    let mut rec = HitRecord::new();

    if world.hit(r, 0.001, INFINITY, &mut rec) {
        if let Some(mat) = rec.material {
            if let Some((attenuation, scattered)) = mat.scatter(&r, &rec) {
                if depth < 50 {
                    return attenuation * ray_color(scattered, world, depth + 1);
                }
            }
        }

        return Vec3::new(0.0, 0.0, 0.0);
    }

    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    let image_width = 400;
    let image_height = (image_width as f64 / ASPECT_RATIO) as i32;
    let samples_per_pixel = 1;
    
    // World
    let sphere1 = Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Material::Lambertian(Lambertian::new(Vec3::new(0.1, 0.2, 0.5))),
    };

    let sphere2 = Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Material::Lambertian(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),
    };

    let sphere3 = Sphere {
        center: Vec3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: Material::Metal(Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0)),
    };

    let sphere4 = Sphere {
        center: Vec3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: Material::Dielectric(Dielectric::new(1.5)),
    };

    let sphere5 = Sphere {
        center: Vec3::new(-1.0, 0.0, -1.0),
        radius: -0.45,
        material: Material::Dielectric(Dielectric::new(1.5)),
    };

    let mut world = HittableList::new();
    world.add(&sphere1);
    world.add(&sphere2);
    world.add(&sphere3);
    world.add(&sphere4);
    world.add(&sphere5);

    // Camera
    let camera = Camera::new();

    // Render
    let mut buffer = String::new();
    buffer.push_str(&format!("P3\n{} {}\n255\n", image_width, image_height));

    for j in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let u = ((i as f64) + core::random()) / (image_width - 1) as f64;
            let v = ((j as f64) + core::random()) / (image_height - 1) as f64;

            let r = camera.get_ray(u, v);

            let mut pixel_color = ray_color(r, &world, 0) / samples_per_pixel as f64;
            pixel_color.sqrt();

            buffer.push_str(&pixel_color.print());
        }
    }

    let _ = fs::write(FILE_PATH, buffer);

    eprintln!("\nDone.");
}
