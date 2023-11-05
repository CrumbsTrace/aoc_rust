use std::fs;
use divan::black_box;

pub fn run(input: &str, total: i32) -> (i32, i32) {
    let mut containers = input.lines().map(|c| c.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    containers.sort_unstable();
    let p1 = count_combinations(&containers, total, containers.len());
    let minimum = minimum_containers(&containers, total, 0);
    let p2 = count_combinations(&containers, total, minimum as usize);
    (p1, p2)
}

fn count_combinations(sorted_containers: &[i32], amount: i32, max_container_count: usize) -> i32 {
    if sorted_containers.is_empty() || max_container_count == 0 {
        if amount == 0 {
            return 1;
        } else {
            return 0;
        }
    }
    if amount < sorted_containers[0] && amount != 0 {
        return 0;
    }
    count_combinations(&sorted_containers[1..], amount, max_container_count) + count_combinations(&sorted_containers[1..], amount - sorted_containers[0], max_container_count - 1)
}

fn minimum_containers(sorted_containers: &[i32], amount: i32, current_count: i32) -> i32 {
    if sorted_containers.is_empty() {
        return i32::MAX;
    }
    if amount < sorted_containers[0] && amount != 0 {
        return i32::MAX;
    }
    if amount == 0 {
        return current_count;
    }
    let min_without =minimum_containers(&sorted_containers[1..], amount, current_count);
    let min_with = minimum_containers(&sorted_containers[1..], amount - sorted_containers[0], current_count + 1);
    min_without.min(min_with)
}

#[test]
fn example() {
    let input = "20\n15\n10\n5\n5";
    let result = run(&input, 25);
    assert_eq!(result, (4, 3));
}

#[test]
fn real_input() {
    let input = fs::read_to_string("inputs/2015/day17.txt").unwrap();
    let result = run(&input, 150);
    assert_eq!(result, (1638, 17))
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = fs::read_to_string("inputs/2015/day17.txt").unwrap();
    bencher.bench(|| run(black_box(&input), 150));
}
