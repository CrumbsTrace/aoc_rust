use std::time::Duration;

use aoc_rust::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("All days");
    group.measurement_time(Duration::from_secs(20));

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
    group.bench_function("Day 12", |b| b.iter(day12::run));
    // group.bench_function("Day 13", |b| b.iter(day13::run));
    // group.bench_function("Day 14", |b| b.iter(day14::run));
    // group.bench_function("Day 15", |b| b.iter(day15::run));
    // group.bench_function("Day 16", |b| b.iter(day16::run));

    // group.bench_function("All days", |b| {
    //     b.iter(day1::run);
    //     b.iter(day2::run);
    //     b.iter(day3::run);
    //     b.iter(day4::run);
    //     b.iter(day5::run);
    //     b.iter(day6::run);
    //     b.iter(day7::run);
    //     b.iter(day8::run);
    //     b.iter(day9::run);
    //     b.iter(day10::run);
    //     b.iter(day11::run);
    //     b.iter(day12::run);
    //     b.iter(day13::run);
    //     b.iter(day14::run);
    //     b.iter(day15::run);
    //     b.iter(day16::run);
    // });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
