use divan::black_box;
use itertools::Itertools;
use ndarray::prelude::*;
use std::fs;

pub fn run(input: &str) -> (i32, i32) {
    let lines = input.trim().lines();

    let mut on_off_grid = Array2::<bool>::default((1000, 1000));
    let mut light_level_grid = Array2::<i32>::zeros((1000, 1000));

    for line in lines {
        let (x1, y1, x2, y2) = parse_coords(line);
        if line.starts_with("turn on") {
            on_off_grid.slice_mut(s![x1..=x2, y1..=y2]).fill(true);
            light_level_grid
                .slice_mut(s![x1..=x2, y1..=y2])
                .mapv_inplace(|x| x + 1)
        } else if line.starts_with("turn off") {
            on_off_grid.slice_mut(s![x1..=x2, y1..=y2]).fill(false);
            light_level_grid
                .slice_mut(s![x1..=x2, y1..=y2])
                .mapv_inplace(|x| (x - 1).max(0))
        } else if line.starts_with("toggle") {
            on_off_grid
                .slice_mut(s![x1..=x2, y1..=y2])
                .mapv_inplace(|x| !x);
            light_level_grid
                .slice_mut(s![x1..=x2, y1..=y2])
                .mapv_inplace(|x| x + 2);
        }
    }

    let p1 = on_off_grid.iter().filter(|x| **x).count() as i32;
    let p2 = light_level_grid.iter().sum();
    (p1, p2)
}

fn parse_coords(line: &str) -> (usize, usize, usize, usize) {
    line.split(|c: char| !c.is_numeric())
        .filter_map(|s| s.parse::<usize>().ok())
        .collect_tuple()
        .unwrap()
}

#[test]
fn example() {
    let (p1, _) = run("turn on 0,0 through 999,999");
    assert_eq!(p1, 1000000);
}

#[test]
fn real_input() {
    let input = fs::read_to_string("inputs/2015/day6.txt").unwrap();
    let (p1, p2) = run(&input);
    assert_eq!(p1, 400410);
    assert_eq!(p2, 15343601);
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = fs::read_to_string("inputs/2015/day6.txt").unwrap();
    bencher.bench(|| run(black_box(&input)));
}
