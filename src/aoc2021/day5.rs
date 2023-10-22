use std::{collections::HashMap, fs};

type Point = (i32, i32);
type Pipe = (Point, Point);

#[divan::bench] 
pub fn run() {
    let input = fs::read_to_string("inputs/2021/day5.txt").unwrap();
    let pipes: Vec<Pipe> = input
        .lines()
        .map(|line| {
            let parsed = line
                .split(" -> ")
                .flat_map(|s| s.split(',').map(|c| c.parse::<i32>().unwrap()))
                .collect::<Vec<i32>>();

            ((parsed[0], parsed[1]), (parsed[2], parsed[3]))
        })
        .collect();

    let p1 = count_intersections(&pipes, true);
    let p2 = count_intersections(&pipes, false);
    // println!("Part 1: {}", p1);
    // println!("Part 2: {}", p2);
    assert_eq!(p1, 7142);
    assert_eq!(p2, 20012);
}

pub fn count_intersections(pipes: &[Pipe], ignore_diagonal: bool) -> usize {
    let mut counts: HashMap<u32, u16> = HashMap::with_capacity(100_000);

    for pipe in pipes {
        let ((start_x, start_y), (end_x, end_y)) = pipe;
        if ignore_diagonal && start_x != end_x && start_y != end_y {
            continue;
        }
        let mut x = *start_x;
        let mut y = *start_y;

        loop {
            *counts.entry(cantor_pairing(x, y)).or_insert(0) += 1;
            if x == *end_x && y == *end_y {
                break;
            }
            x += i32::signum(end_x - x);
            y += i32::signum(end_y - y);
        }
    }
    counts.values().filter(|x| **x > 1).count()
}

fn cantor_pairing(x: i32, y: i32) -> u32 {
    (((x + y) * (x + y + 1)) / 2 + y) as u32
}
