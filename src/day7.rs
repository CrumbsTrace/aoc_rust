use std::fs;

#[divan::bench] 
pub fn run() {
    let input = fs::read_to_string("inputs/day7.txt").unwrap();
    let mut crabs = input
        .replace('\n', "")
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    crabs.sort_unstable();

    let center = crabs[crabs.len() / 2 - 1];

    let p1: i32 = crabs.iter().map(|n| (n - center).abs()).sum();
    let p2: i32;

    let mut low: i32 = *crabs.iter().min().unwrap();
    let mut high: i32 = *crabs.iter().max().unwrap();
    let mut current: i32 = (low + high) / 2;
    loop {
        let now = calculate_fuel_linear(&crabs, current);
        let left = calculate_fuel_linear(&crabs, current - 1);
        let right = calculate_fuel_linear(&crabs, current + 1);

        if left < now {
            high = current;
            current = (low + current) / 2;
        } else if right < now {
            low = current;
            current = (high + current) / 2;
        } else {
            p2 = now;
            break;
        }
    }

    // println!("Part 1: {}", p1);
    // println!("Part 2: {}", p2);
    assert_eq!(p1, 336701);
    assert_eq!(p2, 95167302);
}

fn calculate_fuel_linear(crabs: &[i32], pos: i32) -> i32 {
    crabs.iter().map(|n| sum_of_integers((n - pos).abs())).sum()
}

fn sum_of_integers(number: i32) -> i32 {
    number * (number + 1) / 2
}
