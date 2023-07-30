use crate::otest::report;

use super::utils::linspace;
use std::{
    io::{stdout, Write},
    time::Duration,
};

pub struct Measurement {
    pub n: usize,
    pub time: Duration,
}

pub fn run_n<T>(
    test_fn: impl Fn(&mut T),
    data_bake_fn: impl Fn(usize) -> T,
    max_n: usize,
    points: usize,
) -> Vec<Measurement> {
    let ns: Vec<usize> = linspace(1, max_n as i32, points as i32)
        .iter()
        .map(|&x| x as usize)
        .collect();

    let mut res = Vec::<Measurement>::with_capacity(ns.len());

    for n in ns {
        let mut data = data_bake_fn(n);

        let start = std::time::Instant::now();
        std::hint::black_box(test_fn(std::hint::black_box(&mut data)));
        let elapsed = start.elapsed();

        res.push(Measurement { n, time: elapsed });
    }

    return res;
}

pub fn run_o_test(
    name: &str,
    test_fn: impl Fn() -> Vec<Measurement>,
    preheat: usize,
    repeates: usize,
) {
    print!("\x1b[s");
    for i in 0..preheat {
        print!("\x1b[u\x1b[Kpreheating {}: {}/{}", name, i + 1, preheat);
        stdout().flush().unwrap();
        test_fn();
    }

    report::remove_reports(name).unwrap();
    for i in 0..repeates {
        print!("\x1b[u\x1b[K{}: {}/{}", name, i + 1, repeates);
        stdout().flush().unwrap();
        let res = test_fn();
        report::write_report(&format!("{}_{}", name, i), res).unwrap();
    }
    println!();
    stdout().flush().unwrap();
}
