use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::{
    color::Color,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
    utils,
    vector3::{Point3, Vector3},
};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,

    pub vfov: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vector3,

    pub defocus_angle: f64,
    pub focus_dist: f64,

    image_height: i32,
    pixel_sample_scale: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vector3,
    pixel_delta_v: Vector3,
    u: Vector3,
    v: Vector3,
    w: Vector3,
    defocus_disk_u: Vector3,
    defocus_disk_v: Vector3,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,

            vfov: 90.0,
            lookfrom: Point3::zero(),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vector3::new(0.0, 1.0, 0.0),

            defocus_angle: 0.0,
            focus_dist: 10.0,

            image_height: 0,
            pixel_sample_scale: 0.0,
            center: Point3::zero(),
            pixel00_loc: Point3::zero(),
            pixel_delta_u: Vector3::zero(),
            pixel_delta_v: Vector3::zero(),
            u: Vector3::zero(),
            v: Vector3::zero(),
            w: Vector3::zero(),
            defocus_disk_u: Vector3::zero(),
            defocus_disk_v: Vector3::zero(),
        }
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = self.image_height.max(1);

        self.pixel_sample_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = self.lookfrom;

        // Determime viewport dimensions

        let theta = utils::degrees_to_radians(self.vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the u, v, w unit basis vectors for the camera coordinate frame
        self.w = Vector3::unit_vector(&(&self.lookfrom - &self.lookat));
        self.u = Vector3::unit_vector(&&Vector3::cross(&self.vup, &self.w));
        self.v = Vector3::cross(&self.w, &self.u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = viewport_width as f64 * &self.u;
        let viewport_v = viewport_height as f64 * &-self.v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta_u = &viewport_u / self.image_width as f64;
        self.pixel_delta_v = &viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = &(&(&self.center - &(self.focus_dist as f64 * &self.w))
            - &(&viewport_u / 2.0))
            - &(&viewport_v / 2.0);

        // Calculate the camera defocus disk basis vectors
        let defocus_radius =
            self.focus_dist * f64::tan(utils::degrees_to_radians(self.defocus_angle / 2.0));
        self.defocus_disk_u = &self.u * defocus_radius;
        self.defocus_disk_v = &self.v * defocus_radius;

        self.pixel00_loc =
            &viewport_upper_left + &(0.5 * &(&self.pixel_delta_u + &self.pixel_delta_v));
    }

    pub fn render<T: Hittable>(&mut self, world: &T) -> std::io::Result<()> {
        self.initialize();

        // File
        let file = File::create("output.ppm")?;
        let mut writer = BufWriter::new(file);

        // Render
        writeln!(
            writer,
            "P3\n{} {}\n255",
            self.image_width, self.image_height
        )?;

        for j in 0..self.image_height {
            println!("Scanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color += &Camera::ray_color(&ray, self.max_depth, world);
                }
                Color::write_color(&mut writer, &(self.pixel_sample_scale * &pixel_color))?;
            }
        }
        println!("Done c:\n");
        Ok(())
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        // Construct a camera ray originating from the defocus disk and directed at a randomly samples
        // point around the pixel location i, j

        let offset = Camera::sample_square();
        let pixel_sample = &(&self.pixel00_loc + &((i as f64 + offset.x) * &self.pixel_delta_u))
            + &((j as f64 + offset.y) * &self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = &pixel_sample - &ray_origin;

        Ray::new(&ray_origin, &ray_direction)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vector3::random_in_unit_disk();
        &(&self.center + &(p.x * &self.defocus_disk_u)) + &(p.y * &self.defocus_disk_v)
    }

    fn sample_square() -> Vector3 {
        Vector3::new(
            utils::canonical_random_number() - 0.5,
            utils::canonical_random_number() - 0.5,
            0.0,
        )
    }

    fn ray_color<T: Hittable>(ray: &Ray, depth: i32, world: &T) -> Color {
        // If we've exceeded the ray bounce, no more lights is gathered
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::zero();

        if world.hit(ray, Interval::new(0.001, f64::INFINITY), &mut rec) {
            let mut scattered = Ray::zero();
            let mut attenuation = Color::zero();
            let material = &rec.mat;

            if material.scatter(ray, &rec, &mut attenuation, &mut scattered) {
                return &attenuation * &Camera::ray_color(&scattered, depth - 1, world);
            }
            return Color::zero();
        }

        let unit_direction = Vector3::unit_vector(&ray.direction());
        let a = 0.5 * (&unit_direction.y + 1.0);
        &((1.0 - a) * &Color::new(1.0, 1.0, 1.0)) + &(a * &Color::new(0.5, 0.7, 1.0))
    }
}
