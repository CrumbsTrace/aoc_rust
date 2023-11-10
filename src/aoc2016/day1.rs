use divan::black_box;
use std::collections::HashSet;
use std::fs;

pub fn run(input: &str) -> (i32, Option<i32>) {
    let instructions = input.trim().split(", ");
    let mut position = (0, 0);
    let mut direction = (0, 1);
    let mut visited = HashSet::from([position]);
    let mut p2_position = None;
    for instruction in instructions {
        let (turn, distance) = instruction.split_at(1);
        let distance = distance.parse::<i32>().unwrap();
        direction = match turn {
            "R" => (direction.1, -direction.0),
            "L" => (-direction.1, direction.0),
            _ => panic!("Invalid turn"),
        };
        for i in 1..=distance {
            let new_position = (position.0 + direction.0 * i, position.1 + direction.1 * i);
            if !visited.insert(new_position) && p2_position.is_none() {
                p2_position = Some(new_position);
            }
        }

        position = (
            position.0 + direction.0 * distance,
            position.1 + direction.1 * distance,
        );
    }

    let p1 = position.0.abs() + position.1.abs();
    let p2 = p2_position.map(|position| position.0.abs() + position.1.abs());
    (p1, p2)
}

#[test]
fn example() {
    let input = "R2, L3";
    assert_eq!(run(input), (5, None));

    let input = "R8, R4, R4, R8";
    assert_eq!(run(input), (8, Some(4)));
}

#[test]
fn real_input() {
    let input = fs::read_to_string("inputs/2016/day1.txt").unwrap();
    assert_eq!(run(&input), (253, Some(126)));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = fs::read_to_string("inputs/2016/day1.txt").unwrap();
    bencher.bench(|| run(black_box(&input)));
}
