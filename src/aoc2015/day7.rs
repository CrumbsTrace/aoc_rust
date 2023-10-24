use divan::black_box;
use itertools::Itertools;
use std::{collections::HashMap, fs};

pub fn run(input: &str) -> (u16, u16) {
    let mut instructions = input
        .lines()
        .map(|line| {
            if let Some((left, right)) = line.split(" -> ").collect_tuple() {
                (left.to_string(), right.to_string())
            } else {
                panic!("Unknown instruction")
            }
        })
        .collect::<Vec<(String, String)>>();

    let original_instructions = instructions.clone();
    let mut wires = HashMap::<String, u16>::new();
    while !instructions.is_empty() {
        instructions = run_instructions(&instructions, &mut wires);
    }

    let p1 = wires.get("a").unwrap().clone();

    let mut wires = HashMap::<String, u16>::new();
    wires.insert("b".to_string(), p1);
    let mut instructions = original_instructions;
    while !instructions.is_empty() {
        instructions = run_instructions(&instructions, &mut wires);
    }
    let p2 = wires.get("a").unwrap().clone();

    (p1, p2)
}

fn run_instructions(
    instructions: &Vec<(String, String)>,
    wires: &mut HashMap<String, u16>,
) -> Vec<(String, String)> {
    let mut unparsed_instructions = Vec::new();
    for instruction in instructions {
        let (left, right) = instruction;
        let mut left = left.split(" ");
        let left1 = left.next();
        let v1 = parse_wire_or_number(left1.unwrap(), &wires);
        let operator = if v1.is_some() {
            left.next()
        } else {
            if left1 == Some("NOT") {
                left1
            } else {
                unparsed_instructions.push(instruction.clone());
                continue;
            }
        };

        if operator.is_none() {
            if !wires.contains_key(right) {
                wires.insert(right.to_string(), v1.unwrap());
            }
            continue;
        }

        let v1 = v1.unwrap_or(0);

        let v2 = parse_wire_or_number(left.next().unwrap(), &wires);
        if v2.is_none() {
            unparsed_instructions.push(instruction.clone());
            continue;
        }
        let v2 = v2.unwrap_or(0);
        let result = match operator {
            Some("AND") => v1 & v2,
            Some("OR") => v1 | v2,
            Some("LSHIFT") => v1 << v2,
            Some("RSHIFT") => v1 >> v2,
            Some("NOT") => !v2,
            _ => v1,
        };
        wires.insert(right.to_string(), result);
    }
    unparsed_instructions
}

fn parse_wire_or_number(s: &str, wires: &HashMap<String, u16>) -> Option<u16> {
    match s.parse::<u16>() {
        Ok(i) => Some(i),
        Err(_) => {
            let s = s.to_string();
            if wires.contains_key(&s) {
                Some(wires.get(&s).unwrap().clone())
            } else {
                None
            }
        }
    }
}

#[test]
fn example() {
    let input = "123 -> x\n\
                 456 -> y\n\
                 x AND y -> d\n\
                 x OR y -> e\n\
                 x LSHIFT 2 -> f\n\
                 y RSHIFT 2 -> g\n\
                 NOT x -> a\n\
                 NOT y -> i";
    let (p1, _) = run(input);
    assert_eq!(p1, 65412);
}

#[test]
fn real_input() {
    let input = fs::read_to_string("inputs/2015/day7.txt").unwrap();
    let (p1, p2) = run(&input);
    assert_eq!(p1, 3176);
    assert_eq!(p2, 14710);
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = fs::read_to_string("inputs/2015/day7.txt").unwrap();
    bencher.bench(|| run(black_box(&input)));
}
