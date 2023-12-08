use itertools::Itertools;
use std::collections::HashMap;

pub fn run(input: &str) -> (u64, u64) {
    let mut line_iter = input.lines();
    let instr = line_iter.next().unwrap().chars().collect_vec();
    line_iter.next();
    let nodes = line_iter
        .map(|l| {
            let (name, left, right) = l
                .split([' ', '=', '(', ')', ','])
                .filter(|s| !s.is_empty())
                .collect_tuple()
                .unwrap();
            (name, (left, right))
        })
        .collect::<HashMap<_, _>>();

    let steps = traverse(&nodes, &instr, &["AAA"]);
    let xxa_nodes = nodes
        .clone()
        .into_keys()
        .filter(|name| name.ends_with('A'))
        .collect_vec();
    let steps_p2 = traverse(&nodes, &instr, &xxa_nodes);
    (steps, steps_p2)
}

pub fn traverse(
    nodes: &HashMap<&str, (&str, &str)>,
    instructions: &[char],
    starts: &[&str],
) -> u64 {
    let mut steps = 0;
    let mut i = 0;
    let mut current_locations = starts.iter().copied().collect_vec();
    let mut result = 1_u64;
    let mut finish_count = 0;
    while finish_count < current_locations.len() {
        let instruction = instructions[i];
        for j in 0..current_locations.len() {
            let current = current_locations[j];
            if current.ends_with('Z') {
                continue;
            }
            let (left, right) = nodes.get(current).unwrap();
            match instruction {
                'L' => current_locations[j] = left,
                'R' => current_locations[j] = right,
                _ => panic!("Unknown instruction {}", instruction),
            }
            if current_locations[j].ends_with('Z') {
                result = lcm(steps + 1, result);
                finish_count += 1;
            }
        }
        i = (i + 1) % instructions.len();
        steps += 1;
    }
    result
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[test]
fn example() {
    let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    assert_eq!(run(input), (6, 6));
}

#[test]
fn example_two() {
    let input = "LR

AAA = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
    let (_, steps) = run(input);
    assert_eq!(steps, 6);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("inputs/2023/day8.txt").unwrap();
    assert_eq!(run(&input), (17873, 0));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("inputs/2023/day8.txt").unwrap();
    bencher.bench(|| run(divan::black_box(&input)));
}
