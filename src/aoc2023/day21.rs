use itertools::Itertools;
use ndarray::{Array2, s, ArrayView2};
use std::collections::VecDeque;

pub fn run(input: &str) -> (u64, u64) {
    let grid = input.lines().map(|line| line.as_bytes()).collect_vec();
    let grid = Array2::from_shape_fn((grid.len() * 5, grid.len() * 5), |(y, x)| grid[y % grid.len()][x % grid.len()]);
    let start = (grid.dim().0 / 2, grid.dim().1 / 2);
    let p1 = process(&solve(&grid, start, 64).view(), true);
    let p2 = p2(&grid);
    (p1, p2)
}

fn p2(grid: &Array2<u8>) -> u64 {
    let start = (grid.dim().0 / 2, grid.dim().1 / 2);
    let steps = 26501365;
    let q_size = grid.dim().0 / 5;
    // First we solve this on a smaller scenario reaching the edge of a 5x5 instead as a
    // generalization
    let distance_to_edge = grid.dim().0 / 2;
    let visited = solve(grid, start, distance_to_edge as u64);

    //We need to cut up the visited into the quadrants we have
    // - The middle quadrant in which we started. AKA the Odd full quadrants
    // - One of the adjacent full quadrants aka the even full quadrants
    // - The 4 corner quadrants
    // - The 4 barely visited quadrants next to the corners
    // - The 4 larger quadrants in between these barely visited quadrants
    let plots_odd = process(&visited.slice(s![q_size * 2..q_size * 3, q_size * 2..q_size * 3]), false);
    let plots_even = process(&visited.slice(s![q_size..q_size * 2, q_size * 2..q_size * 3]), true);

    let plots_top = process(&visited.slice(s![q_size * 2..q_size * 3, ..q_size]), false);
    let plots_bottom = process(&visited.slice(s![q_size * 2..q_size * 3, q_size * 4..]), false);
    let plots_left = process(&visited.slice(s![..q_size, q_size * 2..q_size * 3]), false);
    let plots_right = process(&visited.slice(s![q_size * 4.., q_size * 2..q_size * 3]), false);

    let corner_topleft = process(&visited.slice(s![q_size..q_size * 2, ..q_size]), true);
    let corner_topright = process(&visited.slice(s![q_size * 3..q_size * 4, ..q_size]), true);
    let corner_bottomleft = process(&visited.slice(s![q_size..q_size * 2, q_size * 4..]), true);
    let corner_bottomright = process(&visited.slice(s![q_size * 3..q_size * 4, q_size * 4..]), true);

    let mid_topleft = process(&visited.slice(s![q_size..q_size * 2, q_size..q_size * 2]), false);
    let mid_topright = process(&visited.slice(s![q_size * 3..q_size * 4, q_size..q_size * 2]), false);
    let mid_bottomleft = process(&visited.slice(s![q_size..q_size * 2, q_size * 3..q_size * 4]), false);
    let mid_bottomright = process(&visited.slice(s![q_size * 3..q_size * 4, q_size * 3..q_size * 4]), false);

    //This is the width with the "unit" being a full quadrant
    let width_in_quadrants = (steps / q_size - 1) as u64;
    let odd_step_count = (width_in_quadrants / 2 * 2 + 1).pow(2);
    let even_step_count = ((width_in_quadrants + 1) / 2 * 2).pow(2);

    odd_step_count * plots_odd
        + even_step_count * plots_even
        + plots_top
        + plots_bottom
        + plots_left
        + plots_right
        + (width_in_quadrants + 1) * (corner_topleft + corner_topright + corner_bottomleft + corner_bottomright)
        + width_in_quadrants * (mid_topleft + mid_topright + mid_bottomleft + mid_bottomright)
}

fn process(visited: &ArrayView2<bool>, keep_even: bool) -> u64 {
    let mut result = 0;
    for ((x, y), visited) in visited.indexed_iter() {
        if *visited && ((keep_even && (x + y) % 2 == 0) || (!keep_even && (x + y) % 2 == 1)) {
            result += 1;
        }
    }
    result
}

fn solve(grid: &Array2<u8>, (s_x, s_y): (usize, usize), max_steps: u64) -> Array2<bool> {
    let mut visited = Array2::from_elem(grid.dim(), false);
    let mut queue = VecDeque::new();
    queue.push_back(((s_x, s_y), 0));

    while let Some(((x, y), steps)) = queue.pop_front() {
        if steps >= max_steps {
            continue;
        }

        for neighbor in [
            (x, y.saturating_sub(1)),
            (x + 1, y),
            (x, y + 1),
            (x.saturating_sub(1), y),
        ] {
            if neighbor == (x, y)
                || neighbor.0 >= grid.dim().0
                || neighbor.1 >= grid.dim().1
                || grid[neighbor] == b'#'
            {
                continue;
            }

            if !visited[neighbor] {
                visited[neighbor] = true;
                queue.push_back((neighbor, steps + 1));
            }
        }
    }
    visited
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("inputs/2023/day21.txt").unwrap();
    assert_eq!(run(&input), (3699, 613391294577878));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("inputs/2023/day21.txt").unwrap();
    bencher.bench(|| run(divan::black_box(&input)));
}
