use divan::black_box;
use std::collections::HashMap;
use std::fs;

pub fn run(input: &str) -> (usize, usize) {
    let sues = parse_sues(input);

    let gift_sue_properties = [
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ];

    let mut best_sue = 0;
    let mut best_sue2 = 0;
    for (i, sue) in sues.iter().enumerate() {
        let mut match_count = 0;
        let mut match_count2 = 0;
        for (property, amount) in sue {
            for (gift_property, gift_amount) in &gift_sue_properties {
                if property == gift_property && amount == gift_amount {
                    match_count += 1;
                }
                match_count_p2(property, gift_property, amount, gift_amount, &mut match_count2);
            }
        }
        if match_count == 3 {
            best_sue = i + 1;
        }

        if match_count2 == 3 {
            best_sue2 = i + 1;
        }
    }

    (best_sue, best_sue2)
}

fn match_count_p2(property: &String, gift_property: &&str, amount: &i32, gift_amount: &i32, match_count2: &mut i32) {
    if property == gift_property {
        match property.as_str() {
            "cats" | "trees" => {
                if amount > gift_amount {
                    *match_count2 += 1;
                }
            }
            "pomeranians" | "goldfish" => {
                if amount < gift_amount {
                    *match_count2 += 1;
                }
            }
            _ if amount == gift_amount => {
                *match_count2 += 1;
            }
            _ => {}
        }
    }
}

fn parse_sues(input: &str) -> Vec<HashMap<String, i32>> {
    let mut sues = Vec::with_capacity(500);
    for line in input.lines() {
        let mut words = line.split_whitespace();
        let _ = words.nth(1).unwrap();
        let mut properties = HashMap::new();
        while let Some(property) = words.next() {
            let property = property.trim_end_matches(':');
            let amount = words
                .next()
                .unwrap()
                .trim_end_matches(',')
                .parse::<i32>()
                .unwrap();
            properties.insert(property.to_string(), amount);
        }
        sues.push(properties);
    }
    sues
}

#[test]
fn real_input() {
    let input = fs::read_to_string("inputs/2015/day16.txt").unwrap();
    let result = run(&input);
    assert_eq!(result, (373, 260))
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = fs::read_to_string("inputs/2015/day16.txt").unwrap();
    bencher.bench(|| run(black_box(&input)));
}
