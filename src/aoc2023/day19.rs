use std::collections::VecDeque;

use itertools::Itertools;
use rustc_hash::FxHashMap;

struct Condition {
    variable: String,
    operator: String,
    amount: u64,
}

struct Rule {
    condition: Option<Condition>,
    destination: String,
}

pub fn run(input: &str) -> (u64, u64) {
    let (workflows, ratings) = input.split_at(input.find("\n\n").unwrap());
    let workflows = parse_workflows(workflows);
    let ratings = ratings
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.split([',', '=', '}'])
                .filter_map(|s| s.parse::<u64>().ok())
                .collect_vec()
        })
        .collect_vec();

    let mut p1 = 0_u64;
    let mut current = "in".to_string();
    for rating in ratings {
        while &current != "A" && &current != "R" {
            let rules = workflows.get(&current).unwrap();
            for rule in rules {
                if let Some(condition) = &rule.condition {
                    let index = match condition.variable.as_str() {
                        "x" => 0,
                        "m" => 1,
                        "a" => 2,
                        _ => 3,
                    };
                    let value = rating[index];
                    let result = match condition.operator.as_str() {
                        "<" => value < condition.amount,
                        _ => value > condition.amount,
                    };
                    if result {
                        current = rule.destination.clone();
                        break;
                    }
                } else {
                    current = rule.destination.clone();
                    break;
                }
            }
        }
        if &current == "A" {
            p1 += rating.iter().sum::<u64>();
        }
        current = "in".to_string();
    }
    let p2 = number_of_valid_combinations(&workflows);
    (p1, p2)
}

//The four ratings x, m, a and s can all be values between 0 and 4000.
//We want to find the number of unique combinations of these ratings that will result in the rating
//being accepted, aka it ends in the destination "A".
fn number_of_valid_combinations(workflows: &FxHashMap<String, Vec<Rule>>) -> u64 {
    let mut valid_combinations = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(("in".to_string().clone(), vec![(1, 4000), (1, 4000), (1, 4000), (1, 4000)]));
    while let Some((current, ratings)) = queue.pop_front() {
        if &current == "A" {
            valid_combinations.push(ratings);
            continue;
        } else if &current == "R" {
            continue;
        }
        let rules = workflows.get(&current).unwrap();
        let mut next_ratings = ratings.clone();
        for rule in rules {
            if let Some(condition) = &rule.condition {
                let index = match condition.variable.as_str() {
                    "x" => 0,
                    "m" => 1,
                    "a" => 2,
                    _ => 3,
                };
                let mut new_ratings = next_ratings.clone();
                let (min, max) = new_ratings.get_mut(index).unwrap();
                let (next_min, next_max) = next_ratings.get_mut(index).unwrap();
                if condition.operator == "<" {
                    *max = (*max).min(condition.amount - 1);
                    *next_min = (*next_min).max(condition.amount);
                } else {
                    *min = (*min).max(condition.amount + 1);
                    *next_max = (*next_max).min(condition.amount);
                }
                queue.push_back((rule.destination.clone(), new_ratings));
            } else {
                queue.push_back((rule.destination.clone(), next_ratings.clone()));
            }
        }
    }
    //Calculate the number of valid combinations by calculating the number of combinations for
    //each rating and multiplying them together.
    valid_combinations
        .iter()
        .map(|ratings| {
            ratings
                .iter()
                .map(|(min, max)| max - min + 1)
                .product::<u64>()
        })
        .sum()
}

fn parse_workflows(input: &str) -> FxHashMap<String, Vec<Rule>> {
    let mut workflows = FxHashMap::default();
    for line in input.lines() {
        let parts = line
            .split(['{', '}', ','])
            .filter(|s| !s.is_empty())
            .collect_vec();
        let name = parts[0].to_string();
        let mut rules = Vec::new();
        for rule in parts[1..].iter() {
            let split = rule.split(':').collect_vec();
            if split.len() == 1 {
                rules.push(Rule {
                    condition: None,
                    destination: split[0].to_string(),
                });
            } else {
                let mut condition = split[0].chars();
                rules.push(Rule {
                    condition: Some(Condition {
                        variable: condition.next().unwrap().to_string(),
                        operator: condition.next().unwrap().to_string(),
                        amount: condition.collect::<String>().parse().unwrap(),
                    }),
                    destination: split[1].to_string(),
                })
            }
        }
        workflows.insert(name, rules);
    }
    workflows
}

#[test]
fn example() {
    let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
    assert_eq!(run(input), (19114, 167409079868000));
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("inputs/2023/day19.txt").unwrap();
    assert_eq!(run(&input), (421983, 129249871135292));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("inputs/2023/day19.txt").unwrap();
    bencher.bench(|| run(divan::black_box(&input)));
}
