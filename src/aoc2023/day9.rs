use itertools::Itertools;

pub fn run(input: &str) -> (i64, i64) {
    let sequences = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .filter_map(|n| n.parse::<i64>().ok())
                .collect_vec()
        })
        .collect_vec();

    let p1 = sequences.iter().map(|s| next(s, false)).sum();
    let p2 = sequences.iter().map(|s| next(s, true)).sum();
    (p1, p2)
}

fn next(seq: &[i64], first: bool) -> i64 {
    let mut ns = vec![if first { seq[0] } else { seq[seq.len() - 1] }];
    let mut seq = seq.to_owned();
    let mut non_zero = true;
    while non_zero {
        non_zero = false;
        let mut diffs = Vec::new();
        seq.windows(2).for_each(|ns| {
            diffs.push(ns[1] - ns[0]);
            non_zero = non_zero || ns[1] - ns[0] != 0;
        });
        ns.push(if first { diffs[0] } else { diffs[diffs.len() - 1] });
        seq = diffs;
    }
    if first {
        let mut result = 0;
        ns.iter().rev().for_each(|n| {
            result = n - result;
        });
        result
    } else {
        ns.iter().sum()
    }
}

#[test]
fn example() {
    let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    assert_eq!(run(input), (114, 2));
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("inputs/2023/day9.txt").unwrap();
    assert_eq!(run(&input), (1992273652, 1012));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("inputs/2023/day9.txt").unwrap();
    bencher.bench(|| run(divan::black_box(&input)));
}
