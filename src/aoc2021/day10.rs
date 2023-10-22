use std::fs;

const OPENING_BRACKETS: [char; 4] = ['(', '{', '[', '<'];

#[divan::bench] 
pub fn run() {
    let input = fs::read_to_string("inputs/2021/day10.txt").unwrap();
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let p1 = lines
        .iter()
        .map(|line| get_line_score(line, false))
        .sum::<u64>();

    let mut p2_line_scores: Vec<u64> = lines
        .iter()
        .map(|line| get_line_score(line, true))
        .filter(|score| score > &0)
        .collect();

    p2_line_scores.sort_unstable();
    let p2 = p2_line_scores[p2_line_scores.len() / 2];

    // println!("Part 1: {}", p1);
    // println!("Part 2: {}", p2);
    assert_eq!(p1, 319329);
    assert_eq!(p2, 3515583998);
}

fn get_line_score(line: &[char], p2: bool) -> u64 {
    let mut expected_characters: Vec<char> = Vec::new();

    for character in line {
        if OPENING_BRACKETS.contains(character) {
            expected_characters.push(get_expected_character(character));
        } else {
            let expected_character = expected_characters.pop().unwrap();
            if expected_character != *character {
                if p2 {
                    return 0;
                }

                return get_character_score(character, false);
            }
        }
    }

    if p2 {
        return get_autocomplete_score(&expected_characters);
    }
    0
}

fn get_expected_character(character: &char) -> char {
    match character {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("Unexpected character"),
    }
}

fn get_character_score(character: &char, p2: bool) -> u64 {
    match character {
        ')' if !p2 => 3,
        ')' => 1,
        ']' if !p2 => 57,
        ']' => 2,
        '}' if !p2 => 1197,
        '}' => 3,
        '>' if !p2 => 25137,
        '>' => 4,
        _ => panic!("Unexpected character"),
    }
}

fn get_autocomplete_score(expected_characters: &[char]) -> u64 {
    expected_characters
        .iter()
        .rev()
        .fold(0, |acc, c| acc * 5 + get_character_score(c, true))
}
