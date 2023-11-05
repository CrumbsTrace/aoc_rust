use std::fs;
use divan::black_box;

pub fn run(input: &str) -> (i64, i64) {
    let mut weights = input.lines().map(|line| line.parse::<i64>().unwrap()).collect::<Vec<_>>();
    weights.sort_unstable();

    let mut min_qe_p1 = std::i64::MAX;
    let total_weight = weights.iter().sum::<i64>();
    for group_size in 1..weights.len() {
        if let Some(qe) = get_min_qe(&weights, total_weight / 3, group_size as i64) {
            min_qe_p1 = min_qe_p1.min(qe);
            break;
        }
    }

    let mut min_qe_p2 = std::i64::MAX;
    for group_size in 1..weights.len() {
        if let Some(qe) = get_min_qe(&weights, total_weight / 4, group_size as i64) {
            min_qe_p2 = min_qe_p2.min(qe);
            break;
        }
    }
    (min_qe_p1, min_qe_p2)
}

fn get_min_qe(sorted_weights: &[i64], target_weight: i64, group_size: i64) -> Option<i64> {
    if group_size == 1 {
        if sorted_weights.contains(&target_weight) {
            return Some(target_weight);
        }
        return None;
    }
    let mut min_qe = std::i64::MAX;
    for (i, &weight) in sorted_weights.iter().enumerate() {
        if weight > target_weight {
            break;
        }
        if let Some(qe) = get_min_qe(&sorted_weights[i+1..], target_weight - weight, group_size - 1) {
            min_qe = min_qe.min(qe * weight);
        }
    }
    if min_qe == std::i64::MAX {
        None
    } else {
        Some(min_qe)
    }
}

#[test]
fn example() {
    let input = "1\n2\n3\n4\n5\n7\n8\n9\n10\n11";
    assert_eq!(run(input), (99, 44));
}

#[test]
fn real_input() {
    let input = fs::read_to_string("inputs/2015/day24.txt").unwrap();
    let result = run(&input);
    assert_eq!(result, (11846773891, 80393059));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = fs::read_to_string("inputs/2015/day24.txt").unwrap();
    bencher.bench(|| run(black_box(&input)));
}
