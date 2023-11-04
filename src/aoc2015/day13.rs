use std::collections::HashMap;
use itertools::Itertools;
use std::fs;
use divan::black_box;

//Explanation for optimizations
//Since the table is circular we only need to permutate n-1 of the list and insert the last person
//For part 2 we can abuse that everyone is ambivalent towards you and not rerun and simply place
//you between the worst of the two seatings.
//This brings the runtime down from roughly 300ms to 4ms
//We could in theory use rayon to thread this but little point for such a low runtime
pub fn run(input: &str) -> (i32, i32) {
    let happiness_deltas = parse_happiness_deltas(input);
    let people: Vec<String> = happiness_deltas.keys().map(|(p1, _)| p1.clone()).unique().collect();
    let last_person = people.last().unwrap().clone();
    let mut best_happiness = 0;
    let mut worst_seating_in_solution = 0;
    for mut permutation in people.iter().take(people.len() - 1).permutations(people.len() - 1) {
        let mut happiness = 0;
        let mut worst_seating_happiness = i32::MAX;
        permutation.push(&last_person);
        for i in 0..permutation.len() {
            let person1 = permutation[i];
            let person2 = permutation[(i + 1) % permutation.len()];
            let mut delta = 0;
            delta += happiness_deltas.get(&(person1.clone(), person2.clone())).unwrap();
            delta += happiness_deltas.get(&(person2.clone(), person1.clone())).unwrap();
            happiness += delta;
            worst_seating_happiness = worst_seating_happiness.min(delta);
        }
        best_happiness = best_happiness.max(happiness);
        if best_happiness == happiness {
            worst_seating_in_solution = worst_seating_happiness;
        }
    }

    (best_happiness, best_happiness - worst_seating_in_solution)
}

fn parse_happiness_deltas(input: &str) -> HashMap<(String, String), i32> {
    let mut result = HashMap::new();
    for line in input.lines() {
        let mut words = line.split_whitespace();
        let person1 = words.next().unwrap().to_string();
        let win_or_lose = words.nth(1).unwrap();
        let delta = words.next().unwrap().parse::<i32>().unwrap();
        let person2 = words.nth(6).unwrap().trim_end_matches('.').to_string();
        if win_or_lose == "lose" {
            result.insert((person1, person2), -delta);
        } else {
            result.insert((person1, person2), delta);
        }
    }
    result
}

#[test]
fn example() {
    let input = r#"Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol."#;

    assert_eq!(run(input), (330, 286))
}

#[test]
fn real_input() {
    let input = fs::read_to_string("inputs/2015/day13.txt").unwrap();
    let result = run(&input);
    assert_eq!(result, (618, 601))
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = fs::read_to_string("inputs/2015/day13.txt").unwrap();
    bencher.bench(|| run(black_box(&input)));
}

