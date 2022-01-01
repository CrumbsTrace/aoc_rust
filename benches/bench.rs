use aoc_rust::day1;
use aoc_rust::day2;
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Day 1", |b| b.iter(|| day1::run()));
    c.bench_function("Day 2", |b| b.iter(|| day2::run()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
