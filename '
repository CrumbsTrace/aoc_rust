use std::collections::HashSet;
use std::fs;

pub fn run(input: &str) -> (usize, usize) {
    let mut p1_visited = HashSet::from([(0, 0)]);
    let mut p2_visited = HashSet::from([(0, 0)]);
    let mut p1_santa_position = (0, 0);
    let mut p2_santa_position = (0, 0);
    let mut p2_robot_santa_position = (0, 0);

    for (i, c) in input.trim().chars().enumerate() {
        let p2_position = if i % 2 == 0 {
            &mut p2_santa_position
        } else {
            &mut p2_robot_santa_position
        };
        let direction = match c {
            '^' => (0, 1),
            '>' => (1, 0),
            'v' => (0, -1),
            '<' => (-1, 0),
            _ => panic!("Unknown direction"),
        };

        p1_santa_position.0 += direction.0;
        p1_santa_position.1 += direction.1;
        p1_visited.insert(p1_santa_position);

        p2_position.0 += direction.0;
        p2_position.1 += direction.1;
        p2_visited.insert(*p2_position);
    }

    (p1_visited.len(), p2_visited.len())
}

#[test]
fn example() {
    assert_eq!(run("^>v<"), (4, 3));
}

#[test]
fn real_input() {
    let input = fs::read_to_string("inputs/2015/day3.txt").unwrap();
    assert_eq!(run(&input), (2081, 2341));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    bencher.with_inputs(|| {
        fs::read_to_string("inputs/2015/day3.txt").unwrap()
    }).bench_refs(|s| {
        run(s);
    });
}
