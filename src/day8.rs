use std::{collections::HashSet, fs};

type Digit = HashSet<char>;

pub fn run() {
    let input = fs::read_to_string("inputs/day8.txt").unwrap();
    let lines: Vec<Vec<Digit>> = input
        .lines()
        .map(|line| line.split(' ').map(|w| w.chars().collect()).collect())
        .collect();

    let mut p1 = 0;
    let mut p2 = 0;
    for line in lines {
        let mut result: [Digit; 10] = Default::default();
        let right = line.iter().rev().take(4).rev();
        result[1] = get_string_by_length(&line, 2).clone();
        result[4] = get_string_by_length(&line, 4).clone();
        result[7] = get_string_by_length(&line, 3).clone();
        result[8] = get_string_by_length(&line, 7).clone();

        p1 += right.clone().filter(|w| result.contains(w)).count();

        result[9] = check_len_and_subset(&line, 6, &result[4]);
        result[0] = get_zero(&line, &result[9], &result[1]);
        result[6] = check_len_and_not_eq(&line, 6, &result[9], &result[0]);
        result[3] = check_len_and_subset(&line, 5, &result[1]);
        result[5] = check_len_and_subset(&line, 5, &result[6]);
        result[2] = check_len_and_not_eq(&line, 5, &result[3], &result[5]);

        p2 += right.fold(0, |acc, s| {
            acc * 10 + result.iter().position(|w| w.eq(s)).unwrap()
        })
    }
    // println!("Part 1: {}", p1);
    // println!("Part 2: {}", p2);
    assert_eq!(p1, 539);
    assert_eq!(p2, 1084606);
}

fn get_string_by_length(line: &[Digit], length: usize) -> &Digit {
    line.iter().find(|w| w.len() == length).unwrap()
}

fn check_len_and_subset(l: &[Digit], len: usize, s: &Digit) -> Digit {
    l.iter()
        .find(|w| w.len() == len && (s.is_subset(w) || s.is_superset(w)))
        .unwrap()
        .clone()
}

fn check_len_and_not_eq(l: &[Digit], len: usize, s1: &Digit, s2: &Digit) -> Digit {
    l.iter()
        .find(|w| w.len() == len && !s1.eq(w) && !s2.eq(w))
        .unwrap()
        .clone()
}

fn get_zero(line: &[Digit], nine_set: &Digit, one_set: &Digit) -> Digit {
    line.iter()
        .find(|w| w.len() == 6 && !nine_set.eq(w) && one_set.is_subset(w))
        .unwrap()
        .clone()
}
