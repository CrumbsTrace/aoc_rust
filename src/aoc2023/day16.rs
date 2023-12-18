use itertools::Itertools;
use rayon::prelude::*;
use ndarray::Array2;
use rustc_hash::FxHashSet;

type VisitedItem = ((usize, usize), (i8, i8));

pub fn run(input: &str) -> (usize, usize) {
    let temp_grid = input.trim().lines().map(|l| l.as_bytes()).collect_vec();
    let mirror_grid = Array2::from_shape_fn((temp_grid[0].len(), temp_grid.len()), |(x, y)| temp_grid[y][x]);
    let mut visited = FxHashSet::default();
    traverse(&mirror_grid, (0, 0), (1, 0), &mut visited);
    let p1 = visited.iter().map(|(p, _)| *p).unique().count();
    let p2 = find_best_result(&mirror_grid);
    (p1, p2)
}

fn find_best_result(grid: &Array2<u8>) -> usize {
    let edge_positions = get_edge_positions(grid);
    edge_positions
        .into_par_iter()
        .map(|pos| {
            let mut valid_dirs = Vec::new();
            if pos.0 == 0 {
                valid_dirs.push((1, 0));
            } else if pos.0 == grid.dim().0 - 1 {
                valid_dirs.push((-1, 0));
            }

            if pos.1 == 0 {
                valid_dirs.push((0, 1));
            } else if pos.1 == grid.dim().1 - 1 {
                valid_dirs.push((0, -1));
            }

            let mut best_count = 0;
            for dir in valid_dirs {
                let mut visited = FxHashSet::default();
                traverse(grid, pos, dir, &mut visited);
                let unique_positions = visited.iter().map(|p| p.0).collect::<FxHashSet<_>>().len();
                if unique_positions > best_count {
                    best_count = unique_positions;
                }
            }
            best_count
        })
        .max()
        .unwrap()
}

fn traverse(
    grid: &Array2<u8>,
    pos: (usize, usize),
    dir: (i8, i8),
    visited: &mut FxHashSet<VisitedItem>,
) {
    if visited.contains(&(pos, dir)) {
        return;
    }
    visited.insert((pos, dir));
    let mut pos = pos;

    //Avoid a lot of extra recursion by traversing as far as possible in the current direction
    while grid[pos] == b'.' {
        if let Some(new_pos) = new_position(grid, pos, dir) {
            pos = new_pos;
            visited.insert((pos, dir));
        } else {
            break;
        }
    }

    match grid[pos] {
        b'\\' => maybe_traverse(grid, pos, (dir.1, dir.0), visited),
        b'/' => maybe_traverse(grid, pos, (-dir.1, -dir.0), visited),
        b'|' => {
            if dir.1 == 0 {
                maybe_traverse(grid, pos, (0, -1), visited);
                maybe_traverse(grid, pos, (0, 1), visited);
            } else {
                maybe_traverse(grid, pos, dir, visited);
            }
        }
        b'-' => {
            if dir.0 == 0 {
                maybe_traverse(grid, pos, (-1, 0), visited);
                maybe_traverse(grid, pos, (1, 0), visited);
            } else {
                maybe_traverse(grid, pos, dir, visited);
            }
        }
        _ => (),
    }
}

fn maybe_traverse(
    grid: &Array2<u8>,
    pos: (usize, usize),
    dir: (i8, i8),
    visited: &mut FxHashSet<VisitedItem>,
) {
    if let Some(new_pos) = new_position(grid, pos, dir) {
        traverse(grid, new_pos, dir, visited);
    }
}

fn new_position(grid: &Array2<u8>, pos: (usize, usize), dir: (i8, i8)) -> Option<(usize, usize)> {
    let new_pos = (pos.0 as i8 + dir.0, pos.1 as i8 + dir.1);
    if in_bounds(grid, new_pos) {
        Some((new_pos.0 as usize, new_pos.1 as usize))
    } else {
        None
    }
}

fn in_bounds(grid: &Array2<u8>, pos: (i8, i8)) -> bool {
    pos.0 >= 0 && pos.0 < grid.dim().0 as i8 && pos.1 >= 0 && pos.1 < grid.dim().1 as i8
}

fn get_edge_positions(grid: &Array2<u8>) -> Vec<(usize, usize)> {
    let mut edge_positions = Vec::new();
    for x in 0..grid.dim().0 {
        edge_positions.push((x, 0));
        edge_positions.push((x, grid.dim().0 - 1));
    }
    for y in 0..grid.dim().1 {
        edge_positions.push((0, y));
        edge_positions.push((grid.dim().1 - 1, y));
    }
    edge_positions
}

#[test]
fn example() {
    let input = r#"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
    assert_eq!(run(input), (46, 51));
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("inputs/2023/day16.txt").unwrap();
    assert_eq!(run(&input), (6514, 8089));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("inputs/2023/day16.txt").unwrap();
    bencher.bench(|| run(divan::black_box(&input)));
}
