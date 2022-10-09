use aoc_rust::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("All days");
    // group.measurement_time(std::time::Duration::from_secs(60));

    // group.bench_function("Day 1", |b| b.iter(day1::run));
    // group.bench_function("Day 2", |b| b.iter(day2::run));
    // group.bench_function("Day 3", |b| b.iter(day3::run));
    // group.bench_function("Day 4", |b| b.iter(day4::run));
    // group.bench_function("Day 5", |b| b.iter(day5::run));
    // group.bench_function("Day 6", |b| b.iter(day6::run));
    // group.bench_function("Day 7", |b| b.iter(day7::run));
    // group.bench_function("Day 8", |b| b.iter(day8::run));
    // group.bench_function("Day 9", |b| b.iter(day9::run));
    // group.bench_function("Day 10", |b| b.iter(day10::run));
    // group.bench_function("Day 11", |b| b.iter(day11::run));
    // group.bench_function("Day 12", |b| b.iter(day12::run));
    // group.bench_function("Day 12 ref_cell", |b| b.iter(day12_ref_cell::run));
    // group.bench_function("Day 13", |b| b.iter(day13::run));
    // group.bench_function("Day 14", |b| b.iter(day14::run));
    // group.bench_function("Day 15", |b| b.iter(day15::run));
    // group.bench_function("Day 16", |b| b.iter(day16::run));
    // group.bench_function("Day 17", |b| b.iter(day17::run));
    // group.bench_function("Day 18", |b| b.iter(day18::run));
    group.bench_function("Day 19", |b| b.iter(day19::run));

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
