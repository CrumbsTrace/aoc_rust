use divan::black_box;
use std::fs;

use super::*;
#[divan::bench]
fn bench(bencher: divan::Bencher) {
    bencher.bench(|| {
        let input = fs::read_to_string("inputs/2023/day1.txt").unwrap();
        day1::run(black_box(&input));
        let input = fs::read_to_string("inputs/2023/day2.txt").unwrap();
        day2::run(black_box(&input));
        let input = fs::read_to_string("inputs/2023/day3.txt").unwrap();
        day3::run(black_box(&input));
        let input = fs::read_to_string("inputs/2023/day4.txt").unwrap();
        day4::run(black_box(&input));
        let input = fs::read_to_string("inputs/2023/day5.txt").unwrap();
        day5::run(black_box(&input));
        let input = fs::read_to_string("inputs/2023/day6.txt").unwrap();
        day6::run(black_box(&input));
    });
}
