use std::time::Instant;

use extism::{Manifest, Plugin, Wasm};
use rand::distributions::Uniform;
use rand::prelude::*;
use stats::Measurements;

mod stats;

fn main() {
    run_host();
    run_plugin();
}

fn run_plugin() {
    let file = Wasm::file("./plugin.wasm");
    let manifest = Manifest::new([file]);
    let mut plugin = Plugin::new(&manifest, [], true).unwrap();

    let mut measurements = Measurements::new();

    for _ in 0..100 {
        let mut rng = rand::thread_rng();
        let range = Uniform::new(-100, 100);

        let start = Instant::now();

        let vals: Vec<i64> = (0..10000).map(|_| rng.sample(&range)).collect();

        let extism::convert::Json(result) = plugin
            .call::<extism::convert::Json<Vec<i64>>, extism::convert::Json<Vec<i64>>>(
                "run",
                extism::convert::Json(vals),
            )
            .unwrap();

        let end = Instant::now();

        measurements.insert((end - start).as_micros());

        assert!(result.len() > 10);
    }

    measurements.print_results();
}

fn run_host() {
    let mut measurements = Measurements::new();

    for _ in 0..100 {
        let mut rng = rand::thread_rng();
        let range = Uniform::new(-100, 100);

        let start = Instant::now();

        let vals: Vec<i64> = (0..10000).map(|_| rng.sample(&range)).collect();
        let result = run(vals);

        let end = Instant::now();

        measurements.insert((end - start).as_micros());

        assert!(result.len() > 10);
    }

    measurements.print_results();
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
