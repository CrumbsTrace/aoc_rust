use aoc_rust::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("All days", |b| {
        b.iter(|| {
            day1::run();
            day2::run();
            day3::run();
            day4::run();
            day5::run();
        })
    });

    // c.bench_function("test", |b| b.iter(|| day5::run()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
