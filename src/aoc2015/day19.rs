use divan::black_box;
use std::collections::HashSet;
use std::fs;

pub fn run(input: &str) -> (usize, usize) {
    let replacements = parse_replacements(input);
    let mut molecule = parse_goal_molecule(input);
    let p1 = new_molecules(
        &HashSet::from([molecule.clone()]),
        &replacements,
        10000,
    )
    .len();

    let mut i = 0;
    //Greedily replace from as close to the end of the molecule as possible
    //This abuses an aspect of the input that isn't true for the example but it speeds it up
    //by several orders of magnitude
    'cycle: while molecule != "e" {
        for j in (0..molecule.len()).rev() {
            for (from, to) in replacements.iter() {
                if molecule[..=j].ends_with(to) {
                    i += 1;
                    molecule.replace_range(j - (to.len() - 1)..=j, from);
                    continue 'cycle;
                }
            }
        }
    }

    (p1, i)
}

fn new_molecules(
    molecules: &HashSet<String>,
    replacements: &[(String, String)],
    goal_length: usize,
) -> HashSet<String> {
    let mut new_molecules = HashSet::new();
    for molecule in molecules {
        for (from, to) in replacements.iter() {
            for (i, _) in molecule.match_indices(from) {
                let mut new_molecule = molecule.clone();
                new_molecule.replace_range(i..i + from.len(), to);
                if new_molecule.len() > goal_length || molecules.contains(&new_molecule) {
                    continue;
                }
                new_molecules.insert(new_molecule);
            }
        }
    }
    new_molecules
}

fn parse_replacements(input: &str) -> Vec<(String, String)> {
    input
        .lines()
        .filter_map(|line| {
            let mut words = line.split_whitespace();
            let from = words.next()?;
            let to = words.nth(1)?;
            let result = Some((from.to_string(), to.to_string()));
            result
        })
        .collect()
}

fn parse_goal_molecule(input: &str) -> String {
    input.lines().last().unwrap().to_string()
}

#[test]
fn real_input() {
    let input = fs::read_to_string("inputs/2015/day19.txt").unwrap();
    let result = run(&input);
    assert_eq!(result, (535, 212));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = fs::read_to_string("inputs/2015/day19.txt").unwrap();
    bencher.bench(|| run(black_box(&input)));
}
