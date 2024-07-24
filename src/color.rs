use crate::vector3;
use std::io::{self, Write};

pub type Color = vector3::Vector3;

impl Color {
    pub fn write_color<W: Write>(out: &mut W, pixel_color: &Color) -> Result<(), io::Error>{
        let r = pixel_color.x;
        let g = pixel_color.y;
        let b = pixel_color.z;

        let (rbyte, gbyte, bbyte) = (
            (255.999 * r) as u8,
            (255.999 * g) as u8,
            (255.999 * b) as u8,
        );
        writeln!(out, "{rbyte} {gbyte} {bbyte}")?;
        Ok(())
    }
}
