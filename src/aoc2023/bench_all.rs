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
        let input = fs::read_to_string("inputs/2023/day7.txt").unwrap();
        day7::run(black_box(&input));
        let input = fs::read_to_string("inputs/2023/day8.txt").unwrap();
        day8::run(black_box(&input));
        let input = fs::read_to_string("inputs/2023/day9.txt").unwrap();
        day9::run(black_box(&input));
        let input = fs::read_to_string("inputs/2023/day10.txt").unwrap();
        day10::run(black_box(&input));
        let input = fs::read_to_string("inputs/2023/day11.txt").unwrap();
        day11::run(black_box(&input));
        let input = fs::read_to_string("inputs/2023/day12.txt").unwrap();
        day12::run(black_box(&input));
        let input = fs::read_to_string("inputs/2023/day13.txt").unwrap();
        day13::run(black_box(&input));
        let input = fs::read_to_string("inputs/2023/day14.txt").unwrap();
        day14::run(black_box(&input));
        let input = fs::read_to_string("inputs/2023/day15.txt").unwrap();
        day15::run(black_box(&input));
        let input = fs::read_to_string("inputs/2023/day16.txt").unwrap();
        day16::run(black_box(&input));
        let input = fs::read_to_string("inputs/2023/day17.txt").unwrap();
        day17::run(black_box(&input));
        let input = fs::read_to_string("inputs/2023/day18.txt").unwrap();
        day18::run(black_box(&input));
        let input = fs::read_to_string("inputs/2023/day19.txt").unwrap();
        day19::run(black_box(&input));
        let input = fs::read_to_string("inputs/2023/day20.txt").unwrap();
        day20::run(black_box(&input));
        let input = fs::read_to_string("inputs/2023/day21.txt").unwrap();
        day21::run(black_box(&input));
        let input = fs::read_to_string("inputs/2023/day22.txt").unwrap();
        day22::run(black_box(&input));
        let input = fs::read_to_string("inputs/2023/day23.txt").unwrap();
        day23::run(black_box(&input));
        let input = fs::read_to_string("inputs/2023/day24.txt").unwrap();
        day24::run(black_box(&input));
        let input = fs::read_to_string("inputs/2023/day25.txt").unwrap();
        day25::run(black_box(&input));
    });
}
