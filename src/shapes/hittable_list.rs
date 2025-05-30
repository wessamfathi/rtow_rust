use super::sphere::Sphere;
use crate::core::hit_record::HitRecord;
use crate::core::ray::Ray;

pub struct HittableList {
    hittables: Vec<Sphere>,
}

impl<'a> HittableList {
    pub fn new() -> HittableList {
        HittableList {
            hittables: Vec::new(),
        }
    }

    pub fn allocate(capacity: usize) -> HittableList {
        HittableList {
            hittables: Vec::with_capacity(capacity),
        }
    }

    fn clear(&mut self) {
        self.hittables.clear();
    }

    pub fn add(&mut self, object: Sphere) {
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
