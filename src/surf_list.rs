use crate::ray::Ray;
use crate::surf::{HitRecord, Sphere, Surface};

pub struct SurfList {
    list: Vec<Sphere>,
}

pub struct SurfListIntoIter {
    index: usize,
    list: Vec<Sphere>,
}

impl Iterator for SurfListIntoIter {
    type Item = Sphere;

    fn next(&mut self) -> Option<Sphere> {
        let temp_index = self.index;
        self.index += 1;
        Some(self.list[temp_index])
    }
}

impl IntoIterator for SurfList {
    type Item = Sphere;
    type IntoIter = SurfListIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        SurfListIntoIter {
            list: self.list,
            index: 0,
        }
    }
}

impl SurfList {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub fn add(&mut self, surf: Sphere) {
        self.list.push(surf);
    }

    pub fn hit(
        &self,
        ray: &Ray<f32>,
        t_min: f32,
        t_max: f32,
        rec: &mut HitRecord,
    ) -> bool {
        let mut temp_rec: HitRecord = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for surf in self.list.iter() {
            if surf.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.parameter;
            }
        }
        *rec = temp_rec;
        hit_anything
    }
}
