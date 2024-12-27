use criterion::{criterion_group, criterion_main, Criterion};
use std::fs::read_to_string;
use std::path::Path;
use std::sync::LazyLock;

macro_rules! benchmark {
    ($year:tt $($day:tt),* $(,)?) => {
        pub mod $year {
            use super::*;
            $(
                pub fn $day(c: &mut Criterion) {
                    let data = {
                        static DATA: LazyLock<String> = LazyLock::new(|| {
                            let path = Path::new("input")
                                .join(stringify!($year))
                                .join(stringify!($day))
                                .with_extension("txt");
                            read_to_string(path).unwrap()
                        });
                        &DATA
                    };

                    let mut group = c.benchmark_group(format!("{}_{}", stringify!($year), stringify!($day)));

                    // Individual benchmarks
                    group.bench_function("parse", |b| {
                        b.iter(|| aoc::$year::$day::parse(data))
                    });

                    let input = aoc::$year::$day::parse(data);

                    group.bench_function("part1", |b| {
                        b.iter(|| aoc::$year::$day::part1(&input))
                    });

                    group.bench_function("part2", |b| {
                        b.iter(|| aoc::$year::$day::part2(&input))
                    });

                    // Total time benchmark
                    group.bench_function("total", |b| {
                        b.iter(|| {
                            let input = aoc::$year::$day::parse(data);
                            let part1 = aoc::$year::$day::part1(&input);
                            let part2 = aoc::$year::$day::part2(&input);
                            (input, part1, part2)
                        })
                    });

                    group.finish();
                }
            )*

            criterion_group!(
                name = benches;
                config = Criterion::default();
                targets = $($day,)*
            );
        }
    }
}

benchmark!(year2023
    day01
);

criterion_main!(
    year2023::benches
);