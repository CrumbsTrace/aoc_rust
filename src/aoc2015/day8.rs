use std::fs;
use divan::black_box;

pub fn run(input: &str) -> (usize, usize) {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut code_length = 0;
    let mut memory_length = 0;
    let mut memory_length2 = 0;
    for line in lines {
        let line = line.trim();
        code_length += line.len() + 2;
        memory_length += parse_line(line).len();
        memory_length2 += line.chars().filter(|c| *c == '\\' || *c == '"').count() + 2;
    }

    let p1 = code_length - memory_length;
    let p2 = memory_length2;
    (p1, p2)
}

fn parse_line(line: &str) -> String {
    let mut chars = line.chars();
    let mut result = String::new();
    while let Some(c) = chars.next() {
        if c == '\\' {
            let next = chars.next().unwrap();
            if next == 'x' {
                chars.next();
                chars.next();
            }
        }
        result.push(c);
    }
    result
}

#[test]
fn example() {
    assert_eq!(run(r#""""#), (2, 4));
    assert_eq!(run(r#""abc""#), (2, 4));
    assert_eq!(run(r#""aaa\"aaa""#), (3, 6));
    assert_eq!(run(r#""\x27""#), (5, 5));
}

#[test]
fn real_input() {
    let input = fs::read_to_string("inputs/2015/day8.txt").unwrap();
    let result = run(&input);
    assert_eq!(result, (1333, 2046));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = fs::read_to_string("inputs/2015/day8.txt").unwrap();
    bencher.bench(|| run(black_box(&input)));
}
