use std::{collections::HashSet, fs};

use itertools::Itertools;

type Point = (usize, usize);
type Bounds = (usize, usize, usize, usize);

#[divan::bench]
pub fn run() {
    let (enchancement_lookup, image, bounds) = parse();
    let p1 = run_cycles(image, &enchancement_lookup, bounds, 2);
    let p2 = run_cycles(image, &enchancement_lookup, bounds, 50);
    assert_eq!(p1, 5573);
    assert_eq!(p2, 20097);
}

fn parse() -> (Vec<bool>, [[bool; 300]; 300], Bounds) {
    let input = fs::read_to_string("inputs/2021/day20.txt").unwrap();
    let mut lines = input.lines();
    let enhancement = lines.next().unwrap().chars().map(|c| c == '#').collect();
    lines.next();
    let image_hash_set = lines
        .map(|l| {
            l.chars()
                .enumerate()
                .filter(|(_idx, c)| *c == '#')
                .collect::<Vec<_>>()
        })
        .enumerate()
        .fold(HashSet::new(), |acc, (y, line)| {
            line.iter().fold(acc, |mut acc2, (x, _c)| {
                acc2.insert((*x + 110, y + 110));
                acc2
            })
        });

    let bounds = bounds(&image_hash_set);
    let mut image = [[false; 300]; 300];
    for (x, y) in image_hash_set {
        image[y][x] = true;
    }

    (enhancement, image, bounds)
}

fn run_cycles(
    mut image: [[bool; 300]; 300],
    enchancement_lookup: &[bool],
    mut bounds: Bounds,
    cycles: i32,
) -> usize {
    let mut infinity_on = false;
    for _ in 0..cycles {
        image = update_image(&image, enchancement_lookup, bounds, infinity_on);
        infinity_on = enchancement_lookup[0] && !infinity_on;
        bounds = (bounds.0 - 1, bounds.1 + 1, bounds.2 - 1, bounds.3 + 1);
    }
    image.into_iter().flatten().filter(|p| *p).count()
}

#[allow(clippy::needless_range_loop)]
fn update_image(
    image: &[[bool; 300]; 300],
    enchancement_lookup: &[bool],
    bounds: Bounds,
    infinity_on: bool,
) -> [[bool; 300]; 300] {
    let mut new_image = [[false; 300]; 300];
    for y in bounds.2..=bounds.3 {
        for x in bounds.0..=bounds.1 {
            let point = (x, y);
            let index = determine_index(point, image, bounds, infinity_on);

            if enchancement_lookup[index] {
                new_image[y][x] = true;
            }
        }
    }
    new_image
}

#[allow(clippy::needless_range_loop)]
fn determine_index(
    (px, py): Point,
    image: &[[bool; 300]; 300],
    bounds: Bounds,
    infinity_on: bool,
) -> usize {
    let mut index = 0;
    for y in (py - 1)..=(py + 1) {
        for x in (px - 1)..=(px + 1) {
            if image[y][x] || (infinity_on && outside_bounds((x, y), bounds)) {
                index = (index << 1) | 1;
            } else {
                index <<= 1;
            }
        }
    }
    index
}

fn outside_bounds((x, y): Point, (x1, x2, y1, y2): Bounds) -> bool {
    x <= x1 || x >= x2 || y <= y1 || y >= y2
}

fn bounds(image: &HashSet<Point>) -> Bounds {
    let (min_x, max_x) = image
        .iter()
        .minmax_by(|(x1, _), (x2, _)| x1.cmp(x2))
        .into_option()
        .unwrap();
    let (min_y, max_y) = image
        .iter()
        .minmax_by(|(_, y1), (_, y2)| y1.cmp(y2))
        .into_option()
        .unwrap();

    (min_x.0 - 1, max_x.0 + 1, min_y.1 - 1, max_y.1 + 1)
}

#[allow(dead_code)]
fn print_image(image: &[[bool; 300]; 300], (x1, x2, y1, y2): Bounds) {
    for row in image.iter().take(y2 + 1).skip(y1) {
        for v in row.iter().take(x2 + 1).skip(x1) {
            if *v {
                print!("#");
            } else {
               print!(".");
            }
        }
        println!();
    }
}
