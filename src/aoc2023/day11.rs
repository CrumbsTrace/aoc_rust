use std::collections::HashSet;
use itertools::Itertools;

pub fn run(input: &str) -> (i64, i64) {
    let lines = input.lines().map(|l| l.chars().collect_vec()).collect_vec();

    let mut galaxies = Vec::new();
    let mut empty_rows = HashSet::new();
    let mut empty_cols = HashSet::new();
    for y in 0..lines.len() {
        let mut empty_row = true;
        for x in 0..lines[0].len() {
            if lines[y][x] == '#' {
                galaxies.push((x as i64, y as i64));
                empty_row = false;
            }
        }
        if empty_row {
            empty_rows.insert(y as i64);
        }
    }

    for x in 0..lines[0].len() {
        let mut empty_col = true;
        for y in 0..lines.len() {
            if lines[y][x] == '#' {
                empty_col = false;
            }
        }
        if empty_col {
            empty_cols.insert(x as i64);
        }
    }

    let mut p1 = 0;
    let mut p2 = 0;
    for galaxy in galaxies.into_iter().combinations(2) {
        let (x1, y1) = galaxy[0];
        let (x2, y2) = galaxy[1];
        let base_distance = (x1 - x2).abs() + (y1 - y2).abs();
        let mut empty_count = if x1 <= x2 {
            (x1..x2).filter(|x| empty_cols.contains(x)).count() as i64
        } else {
            (x2..x1).filter(|x| empty_cols.contains(x)).count() as i64
        };

        empty_count += if y1 <= y2 {
            (y1..y2).filter(|y| empty_rows.contains(y)).count() as i64
        } else {
            (y2..y1).filter(|y| empty_rows.contains(y)).count() as i64
        };

        p1 += base_distance + empty_count;
        p2 += base_distance + empty_count * 999_999;
    }
    (p1, p2)
}

#[test]
fn example() {
    let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    assert_eq!(run(input), (374, 82000210));
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("inputs/2023/day11.txt").unwrap();
    assert_eq!(run(&input), (9599070, 842645913794));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("inputs/2023/day11.txt").unwrap();
    bencher.bench(|| run(divan::black_box(&input)));
}
