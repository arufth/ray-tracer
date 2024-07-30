use crate::{
    color::Color,
    interval::Interval,
    material::{Lambertian, Material},
    ray::Ray,
    vector3::{Point3, Vector3},
};
use std::rc::Rc;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vector3,
    pub mat: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn zero() -> Self {
        Self {
            p: Point3::zero(),
            normal: Vector3::zero(),
            mat: Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0))),
            t: 0.0,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vector3) {
        // Setst the hits record normal vector
        // ! NOTE: the parameter 'outward_normal' is assumed to have a unit length

        self.front_face = Vector3::dot(&ray.direction(), &outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}
