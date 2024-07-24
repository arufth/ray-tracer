use std::fs::File;
use std::io::{BufWriter, Write};

use color::Color;

mod color;
mod vector3;

fn main() -> std::io::Result<()> {
    let image_width = 256;
    let image_height = 256;

    let file = File::create("example.ppm")?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "P3\n{} {}\n255", image_width, image_height)?;

    for j in 0..image_height {
        println!("Scanlines remaining: {}", image_height - j);
        for i in 0..image_width {

            let pixel_color = Color::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.0,
            );

            Color::write_color(&mut writer, &pixel_color)?;
        }
    }
    println!("Done c:\n");
    Ok(())
}
