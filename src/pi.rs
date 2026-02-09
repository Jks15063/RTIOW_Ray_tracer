use rand::Rng;

pub fn calc() {
    let mut inside_circle = 0;
    let mut inside_circle_stratified = 0;
    let sqrt_N = 1000;

    for i in 0..sqrt_N {
        for j in 0..sqrt_N {
            let x = rand::rng().random_range(-1.0..1.0);
            let y = rand::rng().random_range(-1.0..1.0);

            if x * x + y * y < 1.0 {
                inside_circle += 1;
            }

            let x = 2.0 * ((i as f64 + rand::rng().random::<f64>()) / sqrt_N as f64) - 1.0;
            let y = 2.0 * ((j as f64 + rand::rng().random::<f64>()) / sqrt_N as f64) - 1.0;

            if x * x + y * y < 1.0 {
                inside_circle_stratified += 1;
            }
        }
    }

    println!(
        "Estimate of Pi = {}",
        (4.0 * inside_circle as f64) / (sqrt_N * sqrt_N) as f64
    );

    println!(
        "Stratified estimate of Pi = {}",
        (4.0 * inside_circle_stratified as f64) / (sqrt_N * sqrt_N) as f64
    );
}
