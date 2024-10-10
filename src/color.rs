use crate::{interval::Interval, vector3::Vector3};
use std::io::{self, Write};

pub type Color = Vector3;

impl Color {
    pub fn linear_to_gamma(linear_component: f64) -> f64 {
        if linear_component > 0.0 {
            return linear_component.sqrt();
        }
        0.0
    }
    pub fn write_color<W: Write>(out: &mut W, pixel_color: &Color) -> Result<(), io::Error> {
        let Color { x: r, y: g, z: b } = pixel_color;

        let r = Color::linear_to_gamma(*r);
        let g = Color::linear_to_gamma(*g);
        let b = Color::linear_to_gamma(*b);

        // Translate the [1,0] component values to the byte range [0,255]
        let intensity = Interval::new(0.000, 0.999);

        let (rbyte, gbyte, bbyte) = (
            (255.999 * intensity.clamp(r)) as u8,
            (255.999 * intensity.clamp(g)) as u8,
            (255.999 * intensity.clamp(b)) as u8,
        );

        writeln!(out, "{rbyte} {gbyte} {bbyte}")?;
        Ok(())
    }
}
