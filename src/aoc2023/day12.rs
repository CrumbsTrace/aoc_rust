use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;

pub fn run(input: &str) -> (u64, u64) {
    input
        .lines()
        .par_bridge()
        .map(|line| {
            let mut line = line.split([' ', ',']);
            let mut springs = line.next().unwrap().chars().collect_vec();
            let groups = line.filter_map(|x| x.parse().ok()).collect_vec();
            let p1 = solve(&springs, &groups, 0, &mut HashMap::new());
            springs.push('?');
            let new_springs = &unfold(&springs)[..springs.len() * 5 - 1];
            let new_groups = unfold(&groups);
            let p2 = solve(new_springs, &new_groups, 0, &mut HashMap::new());
            (p1, p2)
        })
        .reduce(|| (0, 0), |(a, b), (c, d)| (a + c, b + d))
}

fn solve(springs: &[char], groups: &[u8], start: usize, known: &mut HashMap<(usize, usize), u64>) -> u64 {
    if start >= springs.len() || groups.is_empty() {
        let success = start >= springs.len() || springs[start..].iter().all(|x| *x != '#');
        return success as u64;
    }

    let mut max_start = springs.len() - (groups.iter().sum::<u8>() as usize + groups.len() - 1);
    if let Some(pos) = springs[start..].iter().position(|&x| x == '#') {
        max_start = max_start.min(start + pos);
    }

    springs[start..=max_start]
        .iter()
        .positions(|&pos| pos != '.')
        .map(|pos| {
            let new_start = start + pos;
            let end = new_start + groups[0] as usize;
            if let Some(&count) = known.get(&(end + 1, groups.len() - 1)) {
                return count;
            }

            if springs[new_start..end].iter().all(|&x| x != '.')
                && (end == springs.len() || springs[end] != '#')
            {
                let count = solve(springs, &groups[1..], end + 1, known);
                known.insert((end + 1, groups.len() - 1), count);
                count
            } else {
                0
            }
        })
        .sum()
}

fn unfold<T>(v: &[T]) -> Vec<T> where T: Copy {
    v.iter().copied().cycle().take(5 * v.len()).collect_vec()
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
