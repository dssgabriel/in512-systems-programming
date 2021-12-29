#[allow(dead_code)]

mod matrix;
extern crate statistical;

use matrix::Matrix;
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        panic!("Usage: {} <size> <nb_runs>", args[0]);
    }

    let n = args[1].parse::<usize>().unwrap();
    let runs = args[2].parse::<usize>().unwrap();

    let mut times = Vec::with_capacity(runs);
    let a = Matrix::random(n);
    let b = Matrix::random(n);

    for _ in 0..runs {
        let t = Instant::now();
        let _ = Matrix::multiply(&a, &b);
        times.push(t.elapsed());
    }

    let times: Vec<f64> = times.iter().map(|d| d.as_secs_f64()).collect();
    let mean = statistical::mean(&times);
    let stddev = statistical::standard_deviation(&times, None);
    println!("Size\tMean time (s)\tStandard deviation (s)");
    println!(
        "{}\t{:.6}\t{:.6}\t", n, mean, stddev);
}
