use crate::{
    ray::Ray,
    surf::{HitRecord, Sphere, Surface},
};

// `SurfList` accepts a vector of `Sphere` of the same material
pub struct SurfList {
    list: Vec<Sphere>,
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
        ray: &Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut HitRecord,
    ) -> bool {
        let mut temp_rec: HitRecord = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for surf in self.list.iter() {
            if surf.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.parameter();
            }
        }
        *rec = temp_rec;
        hit_anything
    }
}

impl Default for SurfList {
    fn default() -> Self {
        Self::new()
    }
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
