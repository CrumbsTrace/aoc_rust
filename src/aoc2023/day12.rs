use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;

pub fn run(input: &str) -> (u64, u64) {
    input
        .lines()
        .par_bridge()
        .map(|line| {
            let mut line = line.split([' ', ',']);
            let mut springs = parse_row(line.next().unwrap());
            let expected = line.filter_map(|x| x.parse().ok()).collect_vec();
            let p1 = count_solutions(&springs, &expected, 0, &mut HashMap::new());
            springs.push(2);
            let new_springs = springs.iter().copied().cycle().take(5 * springs.len()).collect_vec();
            let new_expected = expected.iter().copied().cycle().take(5 * expected.len()).collect_vec();
            let p2 = count_solutions(&new_springs[..new_springs.len() - 1], &new_expected, 0, &mut HashMap::new());
            (p1, p2)
        })
        .reduce(|| (0, 0), |(a, b), (c, d)| (a + c, b + d))
}

fn count_solutions(
    initial: &[u8],
    expected: &[u8],
    start: usize,
    known_scenarios: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if let Some(&count) = known_scenarios.get(&(start, expected.len())) {
        return count;
    }

    if start >= initial.len() || expected.is_empty() {
        let success = start >= initial.len() || initial[start..].iter().all(|x| *x != 1);
        return success as u64;
    }

    let minimum_length = expected.iter().sum::<u8>() as usize + expected.len() - 1;
    let mut maximum_end = initial.len() - minimum_length;
    if let Some(pos) = initial[start..].iter().position(|&x| x == 1) {
        if start + pos < maximum_end {
            maximum_end = start + pos;
        }
    }
    initial[start..=maximum_end]
        .iter()
        .positions(|&pos| pos != 0)
        .map(|pos| {
            let new_start = start + pos;
            let end = new_start + expected[0] as usize;
            if initial[new_start..end].iter().all(|&x| x != 0)
                && (end == initial.len() || initial[end] != 1)
            {
                let count = count_solutions(initial, &expected[1..], end + 1, known_scenarios);
                known_scenarios.insert((end + 1, expected.len() - 1), count);
                count
            } else {
                0
            }
        })
        .sum()
}

fn parse_row(row: &str) -> Vec<u8> {
    row.chars()
        .map(|c| match c {
            '#' => 1,
            '.' => 0,
            '?' => 2,
            _ => unreachable!(),
        })
        .collect_vec()
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
