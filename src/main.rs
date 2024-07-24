use std::fs::File;
use std::io::{BufWriter, Write};

use color::Color;
use ray::Ray;
use vector3::{Point3, Vector3};

mod color;
mod ray;
mod vector3;

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = center - ray.origin();

    let a = ray.direction().length_squared();
    let h = Vector3::dot(&ray.direction(), &oc);
    let c = oc.length_squared() - radius * radius;

    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        return -1.0
    }

    (h - discriminant.sqrt()) / a
}

fn ray_color(ray: &Ray) -> Color {
    let t = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let n = Vector3::unit_vector(&(&ray.at(t) - &Vector3::new(0.0, 0.0, -1.0)));
        return 0.5 * &Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0)
    }

    let unit_direction = Vector3::unit_vector(ray.direction());
    let a = 0.5 * &(&unit_direction.y + 1.0); 
    &((1.0 -a) * &Color::new(1.0, 1.0, 1.0)) + &(a * &Color::new(0.5, 0.7, 1.0))
}

fn main() -> std::io::Result<()> {

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // Calculate the image height, and ensure that it's at least 1.
    let image_height = image_width as i32 / aspect_ratio as i32;
    let image_height = if image_height < 1 { 1 } else { image_height };

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::zero();

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);
    
    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = &viewport_u / image_width as f64;
    let pixel_delta_v = &viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = &(&(&camera_center - &Vector3::new(0.0, 0.0, focal_length)) - &(&viewport_u / 2.0)) - &(&viewport_v / 2.0);
    let pixel00_loc = &viewport_upper_left + &(0.5 * &(&pixel_delta_u + &pixel_delta_v));

    // File
    let file = File::create("output.ppm")?;
    let mut writer = BufWriter::new(file);

    // Render
    writeln!(writer, "P3\n{} {}\n255", image_width, image_height)?;

    for j in 0..image_height {
        println!("Scanlines remaining: {}", image_height - j);
        for i in 0..image_width {

            let pixel_center = &pixel00_loc + &(&(i as f64 * &pixel_delta_u) + &(j as f64 * &pixel_delta_v));
            let ray_direction = &pixel_center - &camera_center;
            let ray = Ray::new(&camera_center, &ray_direction);

            let pixel_color = ray_color(&ray);

            Color::write_color(&mut writer, &pixel_color)?;
        }
    }
    println!("Done c:\n");
    Ok(())
}
