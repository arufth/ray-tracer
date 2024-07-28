use crate::{interval::Interval, vector3::Vector3};
use std::io::{self, Write};

pub type Color = Vector3; 

impl Color {
    pub fn write_color<W: Write>(out: &mut W, pixel_color: &Color) -> Result<(), io::Error> {
        let Color { x: r, y: g, z: b } = pixel_color;

        // Translate the [1,0] component values to the byte range [0,255]
        let intensity = Interval::new(0.000, 0.999);

        let (rbyte, gbyte, bbyte) = (
            (255.999 * intensity.clamp(*r)) as u8,
            (255.999 * intensity.clamp(*g)) as u8,
            (255.999 * intensity.clamp(*b)) as u8,
        );

        writeln!(out, "{rbyte} {gbyte} {bbyte}")?;
        Ok(())
    }
}
