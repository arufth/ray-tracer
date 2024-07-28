use crate::{
    hittable::{Hittable, HitRecord},
    interval::Interval,
    ray::Ray,
    vector3::{Point3, Vector3},
};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = &self.center - &ray.origin();
        let a = ray.direction().length_squared();
        let h = Vector3::dot(&ray.direction(), &oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (h - sqrtd) / a;
        if !ray_t.surronds(root) {
            root = (h + sqrtd) / a;

            if !ray_t.surronds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = &(&rec.p - &self.center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);

        true
    }
}
