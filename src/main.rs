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

const FILE_PATH: &str = "./output/15.ppm";
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

    let num_spheres = 500;
    let mut world = HittableList::allocate(num_spheres);
    random_scene(&mut world);

    // Camera
    let look_from = Vec3::new(-3.0, 3.0, 2.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
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

            pixel_color = pixel_color / (samples_per_pixel as f64);
            pixel_color.sqrt();

            buffer.push_str(&pixel_color.print());
        }
    }

    let _ = fs::write(FILE_PATH, buffer);

    eprintln!("\nDone.");
}

fn random_scene(world: &mut HittableList) {
    // Ground
    let ground_material = Material::Lambertian(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    let ground = Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: ground_material,
    };
    world.add(ground);

    // Random spheres
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = core::random();
            let center = Vec3::new(
                a as f64 + 0.9 * core::random(),
                0.2,
                b as f64 + 0.9 * core::random(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material = if choose_mat < 0.8 {
                    // Diffuse
                    Material::Lambertian(Lambertian::new(
                        Vec3::new(
                            core::random() * core::random(),
                            core::random() * core::random(),
                            core::random() * core::random()
                            )))
                } else if choose_mat < 0.95 {
                    // Metal
                    Material::Metal(Metal::new(
                        Vec3::new(
                            core::random() * core::random(),
                             core::random() * core::random(),
                            core::random() * core::random()
                        ),
                        0.5 * core::random(),
                    ))
                } else {
                    // Glass
                    Material::Dielectric(Dielectric::new(1.5))
                };

                let sphere = Sphere {
                    center,
                    radius: 0.2,
                    material: sphere_material,
                };
                world.add(sphere);
            }
        }
    }

    // Big spheres
    let material1 = Material::Dielectric(Dielectric::new(1.5));
    let material2 = Material::Lambertian(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    let material3 = Material::Metal(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    let big_sphere1 = Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: material1,
    };
    let big_sphere2 = Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: material2,
    };
    let big_sphere3 = Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: material3,
    };

    world.add(big_sphere1);
    world.add(big_sphere2);
    world.add(big_sphere3);
}