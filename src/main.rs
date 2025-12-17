use color::Color;

mod color;
mod vec3;

fn main() {
    let image_width: u32 = 256;
    let image_height: u32 = 256;

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in 0..image_height {
        eprintln!("Scanlines remaining: {}", (image_height - j));
        for i in 0..image_width {
            let pixel_color = Color::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.0,
            );

            println!("{}", color::write_color(pixel_color));
        }
    }

    eprintln!("Done.");
}
