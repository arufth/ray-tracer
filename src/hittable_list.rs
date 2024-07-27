
use crate::{hittable::hittable::{HitRecord, Hittable}, interval::Interval};

pub struct HittableList {
  pub objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
  pub fn new<T: Hittable + 'static>(&self, object: T) -> Self {
    Self {
      objects: vec![Box::new(object)]
    }
  }

  pub fn zero() -> Self {
    Self {
      objects: vec![]
    }
  }

  pub fn clear(&mut self) {
    self.objects.clear();
  }

  pub fn add<T: Hittable + 'static>(&mut self, object: T) {
    self.objects.push(Box::new(object))
  } 
}

impl Hittable for HittableList {
  fn hit(&self, ray: &crate::ray::Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
      let mut temp_rec = HitRecord::zero();
      let mut hit_anything = false;
      let mut closest_so_far = ray_t.max;

      for object in &self.objects {
        
        if object.hit(ray, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
          hit_anything = true;
         *rec = temp_rec.clone();
          closest_so_far = rec.t;
        }
      }

      hit_anything
  }
} 