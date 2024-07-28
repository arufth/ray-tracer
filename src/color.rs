use crate::vector3::Vector3;
use std::io::{self, Write};

pub type Color = Vector3; 

impl Color {
    pub fn write_color<W: Write>(out: &mut W, pixel_color: &Color) -> Result<(), io::Error> {
        let Color { x: r, y: g, z: b } = pixel_color;

        let (rbyte, gbyte, bbyte) = (
            (255.999 * r) as u8,
            (255.999 * g) as u8,
            (255.999 * b) as u8,
        );

        writeln!(out, "{rbyte} {gbyte} {bbyte}")?;
        Ok(())
    }
}
