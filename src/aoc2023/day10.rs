use itertools::Itertools;
use ndarray::Array2;
use std::collections::VecDeque;
use rustc_hash::FxHashSet;

pub fn run(input: &str) -> (usize, usize) {
    let lines = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let map = Array2::from_shape_fn((lines[0].len(), lines.len()), |(x, y)| lines[y][x]);
    let start = find_start(&map);
    let scaled_up_map = scale_up(&map, start);
    let true_count = scaled_up_map.iter().filter(|b| **b).count();
    let outside = flood_fill_outside(&scaled_up_map);
    let inside_count = scaled_up_map
        .indexed_iter()
        .filter(|(pos, &v)| !v && !outside.contains(pos) && pos.0 % 2 == 0 && pos.1 % 2 == 0)
        .count();
    (true_count / 4, inside_count)
}

fn flood_fill_outside(map: &Array2<bool>) -> FxHashSet<(usize, usize)> {
    let edges = get_outside_edges(map);
    let mut queue = VecDeque::from(edges);
    let mut visited = FxHashSet::default();
    while let Some(pos) = queue.pop_front() {
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);
        for dx in -1..=1 {
            for dy in -1..=1 {
                let x = pos.0 as i32 + dx;
                let y = pos.1 as i32 + dy;
                if x < 0 || y < 0 || x >= map.dim().0 as i32 || y >= map.dim().1 as i32 {
                    continue;
                }
                if map[[x as usize, y as usize]] {
                    continue;
                }
                queue.push_back((x as usize, y as usize));
            }
        }
    }
    visited
}

fn get_outside_edges(map: &Array2<bool>) -> Vec<(usize, usize)> {
    let mut edges = Vec::new();
    for x in 0..map.dim().0 {
        if !map[[x, 0]] {
            edges.push((x, 0));
        }
        if !map[[x, map.dim().1 - 1]] {
            edges.push((x, map.dim().1 - 1));
        }
    }
    for y in 0..map.dim().1 {
        if !map[[0, y]] {
            edges.push((0, y));
        }
        if !map[[map.dim().0 - 1, y]] {
            edges.push((map.dim().0 - 1, y));
        }
    }
    edges
}

fn scale_up(map: &Array2<char>, start: (usize, usize)) -> Array2<bool> {
    let mut new_map = Array2::from_elem((map.dim().0 * 2 - 1, map.dim().1 * 2 - 1), false);
    let mut queue = VecDeque::from(vec![start]);
    let mut visited = FxHashSet::default();
    while let Some(pos) = queue.pop_front() {
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);
        let neighbors = valid_neighbors(map, pos);
        for (x, y) in neighbors {
            queue.push_back((x, y));
            let new_pos = (x * 2, y * 2);
            new_map[[new_pos.0, new_pos.1]] = true;
            let dx = pos.0 as i32 - x as i32;
            let dy = pos.1 as i32 - y as i32;
            let new_pos = (new_pos.0 as i32 + dx, new_pos.1 as i32 + dy);
            new_map[[new_pos.0 as usize, new_pos.1 as usize]] = true;
        }
    }
    new_map
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
    let dirs = match map[[pos.0, pos.1]] {
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
    for y in 0..map.dim().1 {
        for x in 0..map.dim().0 {
            if map[[x, y]] == 'S' {
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
    assert_eq!(run(input), (8, 1));
}

#[test]
fn example2() {
    let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
    assert_eq!(run(input), (80, 10));
}

#[test]
fn example3() {
    let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
    assert_eq!(run(input), (23, 4));
}

#[test]
fn example4() {
    let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
    assert_eq!(run(input), (70, 8));
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("inputs/2023/day10.txt").unwrap();
    assert_eq!(run(&input), (7102, 363));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("inputs/2023/day10.txt").unwrap();
    bencher.bench(|| run(divan::black_box(&input)));
}
