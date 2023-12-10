use itertools::Itertools;
use ndarray::Array2;
use std::collections::{HashSet, VecDeque};

pub fn run(input: &str) -> (u32, u32) {
    let lines = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let map = Array2::from_shape_vec(
        (lines.len(), lines[0].len()),
        lines.into_iter().flatten().collect_vec(),
    )
    .unwrap();
    let start = find_start(&map);
    let distance_map = flood_fill_distance_map(&map, start);
    let p1 = distance_map.into_iter().max().unwrap();
    (p1, 0)
}

fn flood_fill_distance_map(map: &Array2<char>, start: (usize, usize)) -> Array2<u32> {
    let mut distance_map = Array2::zeros(map.dim());
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(start);
    visited.insert(start);
    distance_map[start] = 0;
    while let Some((x, y)) = queue.pop_front() {
        let distance = distance_map[[y, x]];
        let valid_neighbors = valid_neighbors(map, (x, y));
        for (x2, y2) in valid_neighbors {
            if visited.contains(&(x2, y2)) {
                continue;
            }
            visited.insert((x2, y2));
            distance_map[[y2, x2]] = distance + 1;
            queue.push_back((x2, y2));
        }
    }
    distance_map
}

fn valid_neighbors(map: &Array2<char>, pos: (usize, usize)) -> Vec<(usize, usize)> {
    let options = neighbors(map, pos);
    options
        .into_iter()
        .filter(|(x, y)| {
            let dirs = neighbors(map, (*x, *y));
            dirs.contains(&pos)
        })
        .collect_vec()
}

fn neighbors(map: &Array2<char>, pos: (usize, usize)) -> Vec<(usize, usize)> {
    let dirs = match map[[pos.1, pos.0]] {
        '|' => vec![(0, 1), (0, -1)],
        '-' => vec![(1, 0), (-1, 0)],
        'L' => vec![(0, -1), (1, 0)],
        'J' => vec![(0, -1), (-1, 0)],
        '7' => vec![(0, 1), (-1, 0)],
        'F' => vec![(0, 1), (1, 0)],
        'S' => vec![(0, 1), (0, -1), (1, 0), (-1, 0)],
        _ => vec![],
    };
    dirs.iter()
        .filter_map(|(dx, dy)| {
            let x = pos.0 as i32 + dx;
            let y = pos.1 as i32 + dy;
            if x < 0 || y < 0 || x >= map.dim().0 as i32 || y >= map.dim().1 as i32 {
                None
            } else {
                Some((x as usize, y as usize))
            }
        })
        .collect_vec()
}

fn find_start(map: &Array2<char>) -> (usize, usize) {
    for (y, row) in map.outer_iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'S' {
                return (x, y);
            }
        }
    }
    unreachable!()
}

#[test]
fn example() {
    let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
    assert_eq!(run(input), (8, 0));
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("inputs/2023/day10.txt").unwrap();
    assert_eq!(run(&input), (7012, 1012));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("inputs/2023/day10.txt").unwrap();
    bencher.bench(|| run(divan::black_box(&input)));
}
