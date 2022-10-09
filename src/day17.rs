use regex::Regex;
use std::{cmp::max, fs};

pub fn run() {
    let bounds = parse();
    let y_min = bounds[2];
    let p1 = (y_min + 1).abs() * y_min.abs() / 2;
    let p2 = p2(&bounds);
    // println!("Part 1: {}", p1);
    // println!("Part 2: {}", p2);
    assert_eq!(p1, 11781);
    assert_eq!(p2, 4531);
}

pub fn p2(bounds: &[i32; 4]) -> i32 {
    let lower_bound_x = (bounds[0] as f32).sqrt() as i32;
    let higher_bound_y = (bounds[2] + 1).abs();

    (lower_bound_x..=bounds[1]).fold(0, |acc, vx| {
        (bounds[2]..=higher_bound_y).fold(acc, |acc2, vy| {
            if hits(vx, vy, bounds) {
                return acc2 + 1;
            }
            acc2
        })
    })
}

fn hits(mut vx: i32, mut vy: i32, bounds: &[i32; 4]) -> bool {
    let mut px = 0;
    let mut py = 0;

    while px <= bounds[1] && py >= bounds[2] {
        px += vx;
        py += vy;
        vx = max(vx - 1, 0);
        vy -= 1;

        if px >= bounds[0] && px <= bounds[1] && py >= bounds[2] && py <= bounds[3] {
            return true;
        }
    }
    false
}

fn parse() -> [i32; 4] {
    let region = fs::read_to_string("inputs/day17.txt").unwrap();
    let r: Vec<_> = region[15..]
        .replace(", y=", " ")
        .replace("..", " ")
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();

    [r[0], r[1], r[2], r[3]]
}

//not a fast way to parse but I had fun doing it so it's ok
#[allow(dead_code)]
fn parse_regex_way() -> [i32; 4] {
    let region = fs::read_to_string("inputs/day17.txt").unwrap();
    let re = Regex::new(
        r"target area: x=(?P<x1>-?\d+)\.\.(?P<x2>-?\d+), y=(?P<y1>-?\d+)\.\.(?P<y2>-?\d+)",
    )
    .unwrap();

    let c = re.captures(&region).unwrap();

    let b: Vec<i32> = [&c["x1"], &c["x2"], &c["y1"], &c["y2"]]
        .iter()
        .map(|v| v.parse().unwrap())
        .collect();

    [b[0], b[1], b[2], b[3]]
}
