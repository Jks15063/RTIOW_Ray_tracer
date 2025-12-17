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
            let r: f32 = i as f32 / (image_width as f32 - 1.0);
            let g: f32 = j as f32 / (image_height as f32 - 1.0);
            let b: f32 = 0.0;

            let ir: u32 = (255.999 * r) as u32;
            let ig: u32 = (255.999 * g) as u32;
            let ib: u32 = (255.999 * b) as u32;

            println!("{} {} {}", ir, ig, ib);
        }
    }

    eprintln!("Done.");
}
