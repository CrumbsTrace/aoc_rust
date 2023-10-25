use divan::black_box;
use itertools::Itertools;
use std::fs;

pub fn run(input: &str) -> (usize, usize) {
    let mut input = input.trim().to_owned();
    let mut p1 = 0;
    for i in 0..50 {
        if i == 40 {
            p1 = input.len();
        }
        input = look_and_say(input);
    }
    let p2 = input.len();
    (p1, p2)
}

fn look_and_say(input: String) -> String {
    input.chars().group_by(|&c| c).into_iter().fold(
        String::with_capacity(input.len()),
        |mut acc, (c, group)| {
            acc.push_str(&group.count().to_string());
            acc.push(c);
            acc
        },
    )
}

#[test]
fn example() {
    assert_eq!(look_and_say("1".to_owned()), "11");
    assert_eq!(look_and_say("11".to_owned()), "21");
    assert_eq!(look_and_say("21".to_owned()), "1211");
    assert_eq!(look_and_say("1211".to_owned()), "111221");
    assert_eq!(look_and_say("111221".to_owned()), "312211");
}

#[test]
fn real_input() {
    let input = fs::read_to_string("inputs/2015/day10.txt").unwrap();
    let result = run(&input);
    assert_eq!(result, (252594, 3579328));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = fs::read_to_string("inputs/2015/day10.txt").unwrap();
    bencher.bench(|| run(black_box(&input)));
}
