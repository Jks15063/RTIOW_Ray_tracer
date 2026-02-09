use rand::Rng;

fn icd(d: f64) -> f64 {
    (4.0 * d).sqrt()
}

fn pdf(x: f64) -> f64 {
    x / 2.0
}

pub fn calc() {
    let N = 1_000_000;
    let mut sum = 0.0;

    for _ in 0..N {
        let z = rand::rng().random();
        if z == 0.0 {
            continue;
        }
        let x = icd(z);
        sum += x * x / pdf(x);
    }

    println!("I = {}", (sum / N as f64));
}
