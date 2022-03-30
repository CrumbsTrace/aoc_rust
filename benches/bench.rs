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
            day6::run();
            day7::run();
            day8::run();
            day9::run();
            day10::run();
        })
    });

    // c.bench_function("test", |b| b.iter(|| day10::run()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
