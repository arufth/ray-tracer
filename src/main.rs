use {
    camera::Camera,
    color::Color,
    hittable_list::HittableList,
    material::{Lambertian, Metal},
    sphere::Sphere,
    vector3::Point3,
};

use std::rc::Rc;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod sphere;
mod utils;
mod vector3;

fn main() -> std::io::Result<()> {
    // World
    let mut world = HittableList::zero();

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Metal::new(Color::new(0.8, 0.8, 0.8));
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2));

    world.add(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Rc::new(material_ground),
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        Rc::new(material_center),
    ));
    world.add(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        Rc::new(material_left),
    ));
    world.add(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        Rc::new(material_right),
    ));

    let mut cam = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.render(&world)?;
    Ok(())
}
