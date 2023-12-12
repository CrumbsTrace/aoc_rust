use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;

pub fn run(input: &str) -> (u64, u64) {
    let results = input.lines().par_bridge().map(|line| {
        let (row, expected) = line.split(' ').collect_tuple().unwrap();
        let expected = expected
            .split(',')
            .filter_map(|x| x.parse::<u8>().ok())
            .collect_vec();
        let initial = row
            .chars()
            .map(|c| match c {
                '#' => 1,
                '.' => 0,
                '?' => 2,
                _ => unreachable!(),
            })
            .collect_vec();

        let p1 = count_solutions(&initial, &expected, 0, &mut HashMap::new());
        let (initial, expected) = unfold(&initial, &expected);
        let p2 = count_solutions(&initial, &expected, 0, &mut HashMap::new());
        (p1, p2)
    }).collect::<Vec<_>>();

    results.into_iter().fold((0, 0), |(p1, p2), (a, b)| (p1 + a, p2 + b))
}

fn unfold(initial: &[u8], expected: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let mut unfolded = Vec::new();
    let mut expected_unfolded = Vec::new();
    for cycle in 0..5 {
        for i in 0..initial.len() {
            unfolded.push(initial[i]);
        }
        if cycle != 4 {
            unfolded.push(2);
        }

        for i in 0..expected.len() {
            expected_unfolded.push(expected[i]);
        }
    }
    (unfolded, expected_unfolded)
}

fn count_solutions(initial: &[u8], expected_springs: &[u8], mut start: usize, known_scenarios: &mut HashMap<(usize, usize), u64>) -> u64 {
    if let Some(&count) = known_scenarios.get(&(start, expected_springs.len())) {
        return count;
    }

    if expected_springs.is_empty() {
        let success = start >= initial.len() || initial[start..].iter().all(|x| *x != 1);
        return success as u64;
    }

    if start >= initial.len() {
        return 0;
    }

    let expected_length = expected_springs.iter().sum::<u8>() as usize + expected_springs.len() - 1;
    let mut solution_count = 0;
    let current_expected = expected_springs[0];
    while start < initial.len() && initial[start] == 0 {
        start += 1;
    }
    while start <= initial.len() - expected_length {
        let mut end = start + 1;
        while end < initial.len() && initial[end] != 0 && end - start < current_expected as usize {
            end += 1;
        }

        if end - start == current_expected as usize && (end == initial.len() || initial[end] != 1) {
            let new_start = end + 1;
            let count = count_solutions(initial, &expected_springs[1..], end + 1, known_scenarios);
            known_scenarios.insert((new_start, expected_springs.len() - 1), count);
            solution_count += count;
        }
        if initial[start] == 1 {
            break;
        }

        start += 1;
        while start < initial.len() && initial[start] == 0 {
            start += 1;
        }
    }
    solution_count
}

#[test]
fn example() {
    let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    assert_eq!(run(input), (21, 525152));
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("inputs/2023/day12.txt").unwrap();
    assert_eq!(run(&input), (7622, 4964259839627));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("inputs/2023/day12.txt").unwrap();
    bencher.bench(|| run(divan::black_box(&input)));
}
