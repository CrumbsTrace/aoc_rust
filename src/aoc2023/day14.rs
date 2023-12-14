use itertools::Itertools;
use ndarray::Array2;

pub fn run(input: &str) -> (usize, usize) {
    let grid = input.lines().map(|line| line.as_bytes().to_owned()).collect_vec();
    let dims = (grid[0].len(), grid.len());
    let grid = Array2::from_shape_fn(dims, |(x, y)| grid[y][x]);
    let mut p1_grid = grid.clone();
    slide_up(&mut p1_grid);
    let p1 = weight(&p1_grid);
    let p2_result = slide_n_times(grid, 1000000000);
    let p2 = weight(&p2_result);
    (p1, p2)
}

fn weight(grid: &Array2<u8>) -> usize {
    let mut sum = 0;
    for x in 0..grid.dim().0 {
        for y in 0..grid.dim().1 {
            if grid[(x, y)] == b'O' {
                sum += grid.dim().1 - y;
            }
        }
    }
    sum
}

fn slide_n_times(mut grid : Array2<u8>, n: usize) -> Array2<u8> {
    if n == 0 {
        return grid;
    }
    let mut known_states = Vec::new();
    known_states.push(grid.clone());
    while known_states.len() < n {
        slide_up(&mut grid);
        slide_left(&mut grid);
        slide_down(&mut grid);
        slide_right(&mut grid);
        if known_states.contains(&grid) {
            break;
        }
        known_states.push(grid.clone());
    }

    let index = known_states
        .iter()
        .position(|state| state == grid)
        .unwrap();
    let cycle_length = known_states.len() - index;
    let index = (n - index) % cycle_length + index;
    known_states[index].clone()
}

fn slide_up(grid: &mut Array2<u8>) {
    let dims = grid.dim();
    for x in 0..dims.0 {
        let mut heighest_y = 0;
        for y in 0..dims.1 {
            if grid[(x, y)] == b'#' {
                heighest_y = y + 1;
            } else if grid[(x, y)] == b'O' {
                grid[(x, y)] = b'.';
                grid[(x, heighest_y)] = b'O';
                heighest_y += 1;
            }
        }
    }
}

fn slide_left(grid: &mut Array2<u8>) {
    let dims = grid.dim();
    for y in 0..dims.1 {
        let mut leftmost_x = 0;
        for x in 0..dims.0 {
            if grid[(x, y)] == b'#' {
                leftmost_x = x + 1;
            } else if grid[(x, y)] == b'O' {
                grid[(x, y)] = b'.';
                grid[(leftmost_x, y)] = b'O';
                leftmost_x += 1;
            }
        }
    }
}

fn slide_down(grid: &mut Array2<u8>) {
    let dims = grid.dim();
    for x in 0..dims.0 {
        let mut lowest_y = dims.1 - 1;
        for y in (0..dims.1).rev() {
            if grid[(x, y)] == b'#' {
                lowest_y = y.saturating_sub(1)
            } else if grid[(x, y)] == b'O' {
                grid[(x, y)] = b'.';
                grid[(x, lowest_y)] = b'O';
                lowest_y = lowest_y.saturating_sub(1);
            }
        }
    }
}

fn slide_right(grid: &mut Array2<u8>) {
    let dims = grid.dim();
    for y in 0..dims.1 {
        let mut rightmost_x = dims.0 - 1;
        for x in (0..dims.0).rev() {
            if grid[(x, y)] == b'#' {
                rightmost_x = x.saturating_sub(1)
            } else if grid[(x, y)] == b'O' {
                grid[(x, y)] = b'.';
                grid[(rightmost_x, y)] = b'O';
                rightmost_x = rightmost_x.saturating_sub(1);
            }
        }
    }
}

#[test]
fn example() {
    let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    assert_eq!(run(input), (136, 64));
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("inputs/2023/day14.txt").unwrap();
    assert_eq!(run(&input), (106517, 79723));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("inputs/2023/day14.txt").unwrap();
    bencher.bench(|| run(divan::black_box(&input)));
}
