use std::{collections::HashSet, fs};
use itertools::Itertools;

type Pos = (i32, i32, i32);


pub fn run() {
    let scanners = parse();
    let solved_scanners = solve(&scanners);
    let p1 = solved_scanners
        .iter()
        .flat_map(|s| s.beacons.iter().map(|b| b.relative_position))
        .unique()
        .count();
    let scanner_positions: Vec<Pos> = solved_scanners.iter().map(|s| s.position.unwrap()).collect();
    let p2 = maximum_distance(&scanner_positions);

    // println!("Part 1: {}", p1);
    // println!("Part 2: {}", p2);
    assert_eq!(p1, 308);
    assert_eq!(p2, 12124);
    // dbg!();
}

fn maximum_distance(positions: &[Pos]) -> i32 {
    let mut maximum_distance = 0;
    for i in 0..positions.len() {
        for j in i..positions.len() {
            let dist = manhattan(&positions[i], &positions[j]);
            if dist > maximum_distance {
                maximum_distance = dist
            }

        }
    }
    maximum_distance
}

fn solve(scanners: &[Scanner]) -> Vec<Scanner> {
    let mut scanner_zero = scanners[0].clone();
    scanner_zero.position = Some((0, 0, 0));
    scanner_zero.solved_beacon_positions = Some(scanner_zero.beacons.iter().map(|b| b.relative_position).collect());
    let mut solved = vec![scanner_zero];
    let mut unsolved = scanners[1..].to_vec();

    while !unsolved.is_empty() {
        for i in 0..unsolved.len() {
            let maybe_scanner = solve_scanner(&unsolved[i], &solved);

            if let Some(solved_scanner) = maybe_scanner {
                solved.push(solved_scanner);
                unsolved.remove(i);
                break;
            }
        }
    }
    solved
}

fn solve_scanner(scanner: &Scanner, solved: &[Scanner]) -> Option<Scanner> {
    for beacon in scanner.beacons.iter() {
        for solved_scanner in solved {
            let maybe_solved_beacon = solved_scanner.beacons
                .iter()
                .find(|s_b| {
                s_b.edge_distances.intersection(&beacon.edge_distances).count() >= 12
            });

            if let Some(solved_beacon) = maybe_solved_beacon {
                return determine_orientation(beacon, solved_beacon, scanner, solved_scanner)
            }
        }
    }
    None
}

fn determine_orientation(beacon: &Beacon, solved_beacon: &Beacon, scanner: &Scanner, solved_scanner: &Scanner) -> Option<Scanner> {
    let solved_positions = solved_scanner.solved_beacon_positions.as_ref().unwrap();
    ORIENTATION_FUNCTIONS
        .map(|f| (f, offset(&f(beacon.relative_position), &solved_beacon.relative_position)))
        .iter()
        .find(|(f, o)| {
            scanner.beacons.iter().map(|b| offset(&f(b.relative_position), o)).fold(0, |acc, p| {
                if solved_positions.contains(&p) {
                    acc + 1
                } else {
                    acc
                }
            }) >= 12
        })
        .map(|orientation| {
            Scanner {
                position: Some(orientation.1),
                beacons: scanner.beacons.iter().map(|b| {
                    Beacon {
                        relative_position: offset(&orientation.0(b.relative_position), &orientation.1),
                        edge_distances: b.edge_distances.clone()
                    }
                }).collect(),
                solved_beacon_positions: Some(scanner.beacons.iter().cloned().map(|b| {
                    offset(&orientation.0(b.relative_position), &orientation.1)
                }).collect())
            }
        })
}


fn parse() -> Vec<Scanner> {
    let input = fs::read_to_string("inputs/day19.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    parse_scanners(&lines)
}

fn parse_scanners(lines: &[&str]) -> Vec<Scanner> {
    lines[1..]
        .split(|l| l.contains('s'))
        .map(Scanner::new)
        .collect()
}

fn offset(p1: &Pos, p2: &Pos) -> Pos {
    ((p1.0 - p2.0), (p1.1 - p2.1), (p1.2 - p2.2))
}

fn manhattan(p1: &Pos, p2: &Pos) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs() + (p1.2 - p2.2).abs()
}

#[derive(Debug, Clone)]
struct Scanner {
    position: Option<Pos>,
    beacons: Vec<Beacon>,
    solved_beacon_positions: Option<HashSet<Pos>>
}

impl Scanner {
    pub fn new(slice: &[&str]) -> Scanner {
        let beacon_positions: Vec<Pos> = slice
            .iter()
            .filter(|s| !s.is_empty())
            .map(|s| {
                let coordinates: Vec<i32> = s.split(',').map(|i| i.parse().unwrap()).collect();
                let x = coordinates[0];
                let y = coordinates[1];
                let z = coordinates[2];
                (x, y, z)
            })
            .collect();

        let mut beacons = Vec::with_capacity(beacon_positions.len());
        let mut distances: Vec<i32> = vec![0;beacon_positions.len()];

        for b1 in &beacon_positions {
            for (j, b2) in beacon_positions.iter().enumerate() {
                distances[j] = manhattan(b1, b2)
            }
            beacons.push(Beacon {
                edge_distances: distances.iter().cloned().collect(),
                relative_position: *b1
            })
        }

        Scanner { position: None, beacons, solved_beacon_positions: None }

    }
}

#[derive(Debug, Clone)]
struct Beacon {
    relative_position: Pos,
    edge_distances: HashSet<i32>,
}

const ORIENTATION_FUNCTIONS: [&dyn Fn(Pos) -> Pos ; 24] = 
    [
        &|(x, y, z): Pos| (x, y, z),
        &|(x, y, z): Pos| (x, -z, y),
        &|(x, y, z): Pos| (x, -y, -z),
        &|(x, y, z): Pos| (x, z, -y),
        &|(x, y, z): Pos| (-x, -y, z),
        &|(x, y, z): Pos| (-x, -z, -y),
        &|(x, y, z): Pos| (-x, y, -z),
        &|(x, y, z): Pos| (-x, z, y),
        &|(x, y, z): Pos| (y, -z, -x),
        &|(x, y, z): Pos| (y, z, x),
        &|(x, y, z): Pos| (y, -x, z),
        &|(x, y, z): Pos| (y, x, -z),
        &|(x, y, z): Pos| (-y, x, z),
        &|(x, y, z): Pos| (-y, -x, -z),
        &|(x, y, z): Pos| (-y, -z, x),
        &|(x, y, z): Pos| (-y, z, -x),
        &|(x, y, z): Pos| (z, x, y),
        &|(x, y, z): Pos| (z, -x, -y),
        &|(x, y, z): Pos| (z, -y, x),
        &|(x, y, z): Pos| (z, y, -x),
        &|(x, y, z): Pos| (-z, x, -y),
        &|(x, y, z): Pos| (-z, -x, y),
        &|(x, y, z): Pos| (-z, y, x),
        &|(x, y, z): Pos| (-z, -y, -x),
    ];
