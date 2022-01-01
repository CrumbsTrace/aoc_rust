use aoc_rust::day1;
use aoc_rust::day2;
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("All Days");
    group.bench_function("Day 1", |b| b.iter(|| day1::run()));
    group.bench_function("Day 2", |b| b.iter(|| day2::run()));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
