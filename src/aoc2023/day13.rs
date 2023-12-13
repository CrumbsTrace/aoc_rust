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
        let mut pattern_p1 = None;
        let mut pattern_p2 = None;
        for x in 1..transposed.len() {
            if pattern_p1 == None && is_mirrored(&pattern, x, false) {
                pattern_p1 = Some(x);
            }
            if pattern_p2 == None && is_mirrored(&pattern, x, true) {
                pattern_p2 = Some(x);
            }
        }

        for y in 1..pattern.len() {
            if pattern_p1 == None && is_mirrored(&transposed, y, false) {
                pattern_p1 = Some(y * 100);
            }
            if pattern_p2 == None && is_mirrored(&transposed, y, true) {
                pattern_p2 = Some(y * 100);
            }
        }
        p1 += pattern_p1.expect(&format!("Didn't find a pattern for part 1, {:?}", pattern));
        p2 += pattern_p2.expect(&format!("Didn't find a pattern for part 2, {:?}", pattern));
    }
    (p1, p2)
}

fn is_mirrored(pattern: &Vec<Vec<char>>, x: usize, p2: bool) -> bool {
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
        for y in 0..v.len() {
            result[x].push(v[y][x]);
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
    assert_eq!(run(&input), (29213, 0));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("inputs/2023/day13.txt").unwrap();
    bencher.bench(|| run(divan::black_box(&input)));
}
