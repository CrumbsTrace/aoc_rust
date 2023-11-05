use divan::black_box;
use std::fs;

use super::*;
#[divan::bench]
#[ignore]
fn bench(bencher: divan::Bencher) {
    bencher.bench(|| {
        let input = fs::read_to_string("inputs/2015/day1.txt").unwrap();
        day1::run(black_box(&input));
        let input = fs::read_to_string("inputs/2015/day2.txt").unwrap();
        day2::run(black_box(&input));
        let input = fs::read_to_string("inputs/2015/day3.txt").unwrap();
        day3::run(black_box(&input));
        let input = fs::read_to_string("inputs/2015/day4.txt").unwrap();
        day4::run(black_box(&input), false);
        let input = fs::read_to_string("inputs/2015/day5.txt").unwrap();
        day5::run(black_box(&input));
        let input = fs::read_to_string("inputs/2015/day6.txt").unwrap();
        day6::run(black_box(&input));
        let input = fs::read_to_string("inputs/2015/day7.txt").unwrap();
        day7::run(black_box(&input));
        let input = fs::read_to_string("inputs/2015/day8.txt").unwrap();
        day8::run(black_box(&input));
        let input = fs::read_to_string("inputs/2015/day9.txt").unwrap();
        day9::run(black_box(&input));
        let input = fs::read_to_string("inputs/2015/day10.txt").unwrap();
        day10::run(black_box(&input));
        let input = fs::read_to_string("inputs/2015/day11.txt").unwrap();
        day11::run(black_box(&input));
        let input = fs::read_to_string("inputs/2015/day12.txt").unwrap();
        day12::run(black_box(&input));
        let input = fs::read_to_string("inputs/2015/day13.txt").unwrap();
        day13::run(black_box(&input));
        let input = fs::read_to_string("inputs/2015/day14.txt").unwrap();
        day14::run(black_box(&input), 2503);
        let input = fs::read_to_string("inputs/2015/day15.txt").unwrap();
        day15::run(black_box(&input));
        let input = fs::read_to_string("inputs/2015/day16.txt").unwrap();
        day16::run(black_box(&input));
        let input = fs::read_to_string("inputs/2015/day17.txt").unwrap();
        day17::run(black_box(&input), 150);
        let input = fs::read_to_string("inputs/2015/day18.txt").unwrap();
        day18::run(black_box(&input), 150);
        let input = fs::read_to_string("inputs/2015/day19.txt").unwrap();
        day19::run(black_box(&input));
        day20::run();
        let input = fs::read_to_string("inputs/2015/day21.txt").unwrap();
        day21::run(black_box(&input));
        let input = fs::read_to_string("inputs/2015/day22.txt").unwrap();
        day22::run(black_box(&input));
        let input = fs::read_to_string("inputs/2015/day23.txt").unwrap();
        day23::run(black_box(&input));
        let input = fs::read_to_string("inputs/2015/day24.txt").unwrap();
        day24::run(black_box(&input));
        let input = fs::read_to_string("inputs/2015/day25.txt").unwrap();
        day25::run(black_box(&input));
    });
}
