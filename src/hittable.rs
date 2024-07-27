pub mod hittable {
    use crate::interval::Interval;
    use crate::ray::Ray;
    use crate::vector3::{Point3, Vector3};

    #[derive(Clone)]
    pub struct HitRecord {
        pub p: Point3,
        pub normal: Vector3,
        pub t: f64,
        pub front_face: bool,
    }

    impl HitRecord {
        pub fn zero() -> Self {
            Self {
                p: Point3::zero(),
                normal: Vector3::zero(),
                t: 0.0,
                front_face: false
            }
        } 
        
        pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vector3) {
            // Setst the hits record normal vector
            // ! NOTE: the parameter 'outward_normal' is assumed to have a unit length

            self.front_face = Vector3::dot(&ray.direction(), &outward_normal) < 0.0;
            self.normal = if self.front_face {
                Vector3::new_from(outward_normal)
            } else {
                -Vector3::new_from(outward_normal)
            }
        }
    }

    pub trait Hittable {
        fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;

    }
}
