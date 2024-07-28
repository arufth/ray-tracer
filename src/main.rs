use {camera::Camera, sphere::Sphere, vector3::Point3};

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod sphere;
mod utils;
mod vector3;

fn main() -> std::io::Result<()> {
    // World
    let mut world = hittable_list::HittableList::zero();

    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    let mut cam = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;

    cam.render(&world)?;
    Ok(())
}
