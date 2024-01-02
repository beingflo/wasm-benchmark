use rand::distributions::Uniform;
use rand::prelude::*;
use stats::Measurements;

mod stats;

fn main() {
    println!("Hello, world!");
}

fn run_host(measurements: &mut Measurements) {
    let mut rng = rand::thread_rng();
    let range = Uniform::new(0, 20);

    let vals: Vec<u64> = (0..100).map(|_| rng.sample(&range)).collect();
}

fn run(x: Vec<i64>) -> Vec<i64> {
    let mut values = Vec::new();

    for (idx, a) in x.iter().enumerate() {
        for b in x.iter() {
            if values.len() > idx {
                values[idx] += b;
            } else {
                values.push(a * b);
            }
        }
    }

    values
}
