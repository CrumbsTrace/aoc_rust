use std::{collections::HashMap, fs};

#[divan::bench] 
pub fn run() {
    let input = fs::read_to_string("inputs/day14.txt").unwrap();
    let lines = input.lines().collect::<Vec<&str>>();
    let polymer = lines[0].chars().collect::<Vec<char>>();
    let instructions = lines[2..].to_vec();

    let first_char = polymer[0];
    let last_char = polymer[polymer.len() - 1];
    //get the frequency of each character pair in polymer
    let mut pair_freq = HashMap::new();
    for i in 0..polymer.len() - 1 {
        let c1 = polymer[i];
        let c2 = polymer[i + 1];
        let pair = format!("{}{}", c1, c2);
        let count = pair_freq.entry(pair).or_insert(0);
        *count += 1;
    }

    //create pair conversion map from instructions
    let mut pair_conversions = HashMap::new();
    instructions.iter().for_each(|instruction| {
        let split = instruction.split(" -> ").collect::<Vec<&str>>();
        let pair_one = format!("{}{}", split[0].chars().next().unwrap(), split[1]);
        let pair_two = format!("{}{}", split[1], split[0].chars().nth(1).unwrap());
        pair_conversions.insert(split[0].to_string(), (pair_one, pair_two));
    });

    let mut p1 = 0;
    for i in 0..40 {
        let mut new_pair_freq: HashMap<String, u64> = HashMap::new();
        for (pair, frequency) in pair_freq {
            let (pair_one, pair_two) = pair_conversions.get(&pair).unwrap();
            *new_pair_freq.entry(pair_one.to_string()).or_insert(0) += frequency;
            *new_pair_freq.entry(pair_two.to_string()).or_insert(0) += frequency;
        }

        if i == 9 {
            p1 = get_score(&new_pair_freq, first_char, last_char)
        }
        pair_freq = new_pair_freq;
    }
    let p2 = get_score(&pair_freq, first_char, last_char);

    // println!("Part 1: {}", p1);
    // println!("Part 2: {}", p2);
    assert_eq!(p1, 2360);
    assert_eq!(p2, 2967977072188);
}

fn get_score(pair_freq: &HashMap<String, u64>, first_char: char, last_char: char) -> u64 {
    let char_freq = count_char_freq(pair_freq, first_char, last_char);
    let (_, lowest) = char_freq.iter().min_by_key(|(_, v)| *v).unwrap();
    let (_, highest) = char_freq.iter().max_by_key(|(_, v)| *v).unwrap();
    (highest / 2) - (lowest / 2)
}

fn count_char_freq(
    pair_freq: &HashMap<String, u64>,
    first_char: char,
    last_char: char,
) -> HashMap<char, u64> {
    let mut char_freq: HashMap<char, u64> = HashMap::new();
    for (pair, frequency) in pair_freq.iter() {
        let chars: Vec<char> = pair.chars().collect();
        let c1_freq = char_freq.entry(chars[0]).or_insert(0);
        *c1_freq += frequency;
        let c2_freq = char_freq.entry(chars[1]).or_insert(0);
        *c2_freq += frequency;
    }
    *char_freq.entry(first_char).or_insert(0) += 1;
    *char_freq.entry(last_char).or_insert(0) += 1;
    char_freq
}
