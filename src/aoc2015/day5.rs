use std::fs;
use divan::black_box;

pub fn run(input: &str) -> (usize, usize) {
    let lines = input.trim().lines().collect::<Vec<_>>();

    let p1 = lines.iter().filter(|l| count_naughty_p1(l)).count();
    let p2 = lines.iter().filter(|l| count_naughty_p2(l)).count();
    (p1, p2)
}

fn count_naughty_p1(line: &str) -> bool {
    let mut vowels = 0;
    let mut last_char = ' ';
    let mut has_double = false;
    for c in line.chars() {
        if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
            vowels += 1;
        }
        if c == last_char {
            has_double = true;
        }
        if (last_char == 'a' && c == 'b')
            || (last_char == 'c' && c == 'd')
            || (last_char == 'p' && c == 'q')
            || (last_char == 'x' && c == 'y')
        {
            return false;
        }
        last_char = c;

    }
    vowels >= 3 && has_double
}

fn count_naughty_p2(line: &str) -> bool {
    let mut has_split_double = false;
    let mut has_repeat = false;
    let mut seen_pairs = std::collections::HashSet::new();
    let mut last_char = ' ';
    let mut second_last_char = ' ';
    for c in line.chars() {
        if !has_repeat {
            let pair = format!("{}{}", last_char, c);
            if seen_pairs.contains(&pair) && (second_last_char != c || last_char != c) {
                has_repeat = true;
            }
            seen_pairs.insert(pair);
        }
        if second_last_char == c {
            has_split_double = true;
        }
        second_last_char = last_char;
        last_char = c;
        if has_split_double && has_repeat {
            return true;
        }
    }
    has_split_double && has_repeat
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("inputs/2015/day5.txt").unwrap();
    let (p1, p2) = run(&input);
    assert_eq!(p1, 258);
    assert_eq!(p2, 53);
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = fs::read_to_string("inputs/2015/day5.txt").unwrap();
    bencher.bench(|| run(black_box(&input)));
}
