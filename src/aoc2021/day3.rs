use std::fs;

#[divan::bench] 
pub fn run() {
    let input = fs::read_to_string("inputs/2021/day3.txt").unwrap();
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let p1 = p1(&lines);
    let p2 = p2(&lines);
    // println!("Part 1: {:?}", p1);
    // println!("Part 2: {:?}", p2);
    assert_eq!(p1, 845186);
    assert_eq!(p2, 4636702);
}

fn p1(lines: &[Vec<char>]) -> usize {
    let mut gamma = 0;
    let length: usize = lines[0].len();
    for i in 0..length {
        match one_most_common(lines, i) {
            true => gamma = (gamma << 1) + 1,
            false => gamma <<= 1,
        }
    }
    gamma * ((1 << length) - 1 - gamma)
}

fn p2(lines: &[Vec<char>]) -> usize {
    let length: usize = lines[0].len();
    let mut o2_list = lines.to_owned();
    let mut co2_list = lines.to_owned();
    for i in 0..length {
        if o2_list.len() > 1 {
            match one_most_common(&o2_list, i) {
                true => o2_list.retain(|line| line[i] == '1'),
                false => o2_list.retain(|line| line[i] == '0'),
            }
        }

        if co2_list.len() > 1 {
            match one_most_common(&co2_list, i) {
                true => co2_list.retain(|line| line[i] == '0'),
                false => co2_list.retain(|line| line[i] == '1'),
            }
        }
    }
    to_number(&o2_list[0]) * to_number(&co2_list[0])
}

fn to_number(bits: &[char]) -> usize {
    let mut result = 0;
    for bit in bits {
        if *bit == '1' {
            result = (result << 1) + 1;
        } else {
            result <<= 1;
        }
    }
    result
}

fn one_most_common(lines: &[Vec<char>], i: usize) -> bool {
    let mut one_count = 0;
    let mut zero_count = 0;
    let line = lines.iter().map(|line| line[i]);
    for c in line {
        if c == '1' {
            one_count += 1;
        } else {
            zero_count += 1;
        }
    }

    one_count >= zero_count
}
