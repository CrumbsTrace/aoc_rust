use itertools::Itertools;

pub fn run(input: &str) -> (usize, usize) {
    let patterns = input
        .split("\n\n")
        .map(|pattern| {
            let pattern = pattern
                .lines()
                .map(|line| line.chars().collect_vec())
                .collect_vec();

            (pattern.clone(), transpose(&pattern))
        })
        .collect_vec();

    let mut p1 = 0;
    let mut p2 = 0;
    for (pattern, transposed) in patterns {
        let (p1_y, p2_y) = find_reflection(&pattern);
        let (p1_x, p2_x) = find_reflection(&transposed);
        p1 += p1_y.unwrap_or(p1_x.unwrap_or(0) * 100);
        p2 += p2_y.unwrap_or(p2_x.unwrap_or(0) * 100);
    }
    (p1, p2)
}

fn find_reflection(pattern: &[Vec<char>]) -> (Option<usize>, Option<usize>) {
    let mut p1 = None;
    let mut p2 = None;
    for i in 1..pattern[0].len() {
        if p1.is_none() && is_mirrored(pattern, i, false) {
            p1 = Some(i);
        }
        if p2.is_none() && is_mirrored(pattern, i, true) {
            p2 = Some(i);
        }
    }
    (p1, p2)
}

fn is_mirrored(pattern: &[Vec<char>], x: usize, p2: bool) -> bool {
    let mut total_smudges = 0;
    pattern.iter().all(|line| {
        let (left, right) = line.split_at(x);
        for (a, b) in left.iter().rev().zip(right) {
            if a != b {
                if p2 && total_smudges == 0 {
                    total_smudges += 1;
                } else {
                    return false;
                }
            }
        }
        true
    }) && (!p2 || total_smudges == 1)
}

fn transpose(v: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = Vec::with_capacity(v[0].len());
    for x in 0..v[0].len() {
        result.push(Vec::with_capacity(v.len()));
        for row in v {
            result[x].push(row[x]);
        }
    }
    result
}

#[test]
fn example() {
    let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    assert_eq!(run(input), (405, 400));
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("inputs/2023/day13.txt").unwrap();
    assert_eq!(run(&input), (29213, 37453));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("inputs/2023/day13.txt").unwrap();
    bencher.bench(|| run(divan::black_box(&input)));
}
