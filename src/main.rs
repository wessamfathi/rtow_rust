use std::fs;
use rtow_rust::core::{self, PI};
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

const FILE_PATH: &str = "./output/14.ppm";
const MAX_DEPTH: i32 = 50;

fn ray_color(r: Ray, world: &HittableList, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::new();

    if world.hit(r, 0.001, INFINITY, &mut rec) {
        if let Some(mat) = rec.material {
            if let Some((attenuation, scattered)) = mat.scatter(&r, &rec) {
                return attenuation * ray_color(scattered, world, depth - 1);
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
    let samples_per_pixel = 100;

    let r = (PI / 4.0).cos();
    
    // World
    let sphere1 = Sphere {
        center: Vec3::new(-r, 0.0, -1.0),
        radius: r,
        material: Material::Lambertian(Lambertian::new(Vec3::new(0.0, 0.0, 1.0))),
    };

    let sphere2 = Sphere {
        center: Vec3::new(r, 0.0, -1.0),
        radius: r,
        material: Material::Lambertian(Lambertian::new(Vec3::new(1.0, 0.0, 0.0))),
    };

    let sphere3 = Sphere {
        center: Vec3::new(r * 2.0, 0.0, -1.0),
        radius: r,
        material: Material::Metal(Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0)),
    };

    let sphere4 = Sphere {
        center: Vec3::new(-r * 2.0, 0.0, -1.0),
        radius: r,
        material: Material::Dielectric(Dielectric::new(1.5)),
    };

    let sphere5 = Sphere {
        center: Vec3::new(-r * 2.0, 0.0, -1.0),
        radius: -r,
        material: Material::Dielectric(Dielectric::new(1.5)),
    };

    let mut world = HittableList::new();
    world.add(&sphere1);
    world.add(&sphere2);
    world.add(&sphere3);
    world.add(&sphere4);
    world.add(&sphere5);

    // Camera
    let look_from = Vec3::new(-3.0, 3.0, 2.0);
    let look_at = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (look_from - look_at).length();
    let camera = Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        50.0,
        image_width as f64 / image_height as f64,
        2.0,
        dist_to_focus,
    );

    // Render
    let mut buffer = String::new();
    buffer.push_str(&format!("P3\n{} {}\n255\n", image_width, image_height));

    for j in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);

            for _ in 0..samples_per_pixel {
                let u = ((i as f64) + core::random()) / (image_width - 1) as f64;
                let v = ((j as f64) + core::random()) / (image_height - 1) as f64;

                let r = camera.get_ray(u, v);
                pixel_color += ray_color(r, &world, MAX_DEPTH);
            }

            pixel_color /= samples_per_pixel as f64;
            pixel_color.sqrt();

            buffer.push_str(&pixel_color.print());
        }
    }

    let _ = fs::write(FILE_PATH, buffer);

    eprintln!("\nDone.");
}
