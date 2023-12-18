use std::iter::empty;

use criterion::{criterion_group, criterion_main, Criterion};

#[derive(Debug, Clone)]
struct Benchmark {
    benchmark: fn(&mut Criterion),
}

macro_rules! benchmark {
    ($year:tt, $day:tt) => {
        Benchmark {
            benchmark: |c: &mut $crate::Criterion| {
                use aoc_solutions::$year::$day::*;

                let raw_input = include_str!(concat![
                    "../../resources/input/",
                    stringify!($year),
                    "/",
                    stringify!($day),
                    ".txt"
                ]);

                let mut group = c.benchmark_group(stringify!($year));

                group.bench_function(concat!(stringify!($day), " parser"), |b| {
                    b.iter(|| input(raw_input))
                });
                group.bench_function(concat!(stringify!($day), " part one"), |b| {
                    b.iter(|| {
                        let input = input(raw_input);
                        part_one(&input);
                    })
                });
                group.bench_function(concat!(stringify!($day), " part two"), |b| {
                    b.iter(|| {
                        let input = input(raw_input);
                        part_two(&input);
                    })
                });
            },
        }
    };
}

fn benchmarks_2023() -> Vec<Benchmark> {
    [
        benchmark!(year2023, day01),
        benchmark!(year2023, day02),
        benchmark!(year2023, day03),
        benchmark!(year2023, day17),
        benchmark!(year2023, day18),
    ]
    .to_vec()
}

fn benchmark_wrapper(c: &mut Criterion) {
    empty().chain(benchmarks_2023()).for_each(|b| {
        (b.benchmark)(c);
    });
}

criterion_group!(benches, benchmark_wrapper);
criterion_main!(benches);
