use std::fs;
use divan::black_box;

pub fn run(input: &str) -> i64 {
    let line = input.lines().next().unwrap().split_whitespace().collect::<Vec<_>>();
    let target_row = line[15].trim_end_matches(',').parse::<i32>().unwrap();
    let target_col = line[17].trim_end_matches('.').parse::<i32>().unwrap();
    let mut row = 1;
    let mut col = 1;
    let mut code = 20151125;
    while row != target_row || col != target_col {
        if row == 1 {
            row = col + 1;
            col = 1;
        } else {
            row -= 1;
            col += 1;
        }
        code = (code * 252533) % 33554393;
    }
    code
}

#[test]
fn real_input() {
    let input = fs::read_to_string("inputs/2015/day25.txt").unwrap();
    let result = run(&input);
    assert_eq!(result, 9132360);
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = fs::read_to_string("inputs/2015/day25.txt").unwrap();
    bencher.bench(|| run(black_box(&input)));
}
