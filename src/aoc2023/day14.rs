use std::collections::HashSet;

use itertools::Itertools;

pub fn run(input: &str) -> (i32, i32) {
    let mut square_rocks = HashSet::new();
    let mut circle_rocks = HashSet::new();
    let grid = input.lines().collect_vec();
    let dims = (grid[0].len() as i32, grid.len() as i32);
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                square_rocks.insert((x as i32, y as i32));
            } else if c == 'O' {
                circle_rocks.insert((x as i32, y as i32));
            }
        }
    }
    for x in 0..dims.0 {
        square_rocks.insert((x, -1));
        square_rocks.insert((x, dims.1));
    }
    for y in 0..dims.1 {
        square_rocks.insert((-1, y));
        square_rocks.insert((dims.0, y));
    }

    let p1_result = slide(&square_rocks, &circle_rocks, (0, -1));
    let p1 = p1_result.into_iter().map(|(_, y)| dims.1 - y).sum();
    let p2_result = slide_n_times(&square_rocks, &circle_rocks, 1000000000);
    let p2 = p2_result.into_iter().map(|(_, y)| dims.1 - y).sum();
    (p1, p2)
}

fn slide_n_times(
    walls: &HashSet<(i32, i32)>,
    circles: &HashSet<(i32, i32)>,
    n: usize,
) -> HashSet<(i32, i32)> {
    let mut direction = (0, -1);
    let mut known_states = Vec::new();
    let mut circles = circles.clone();
    known_states.push(circles.clone());
    while known_states.len() < n {
        for _ in 0..4 {
            circles = slide(walls, &circles, direction);
            direction = next_direction(direction);
        }

        if known_states.contains(&circles) {
            break;
        }
        known_states.push(circles.clone());
    }

    let index = known_states
        .iter()
        .position(|state| state == &circles)
        .unwrap();
    let cycle_length = known_states.len() - index;
    let index = (n - index) % cycle_length + index;
    known_states[index].clone()
}

fn next_direction(direction: (i32, i32)) -> (i32, i32) {
    match direction {
        (0, -1) => (-1, 0),
        (-1, 0) => (0, 1),
        (0, 1) => (1, 0),
        (1, 0) => (0, -1),
        _ => panic!("Invalid direction"),
    }
}

fn slide(
    walls: &HashSet<(i32, i32)>,
    circles: &HashSet<(i32, i32)>,
    direction: (i32, i32),
) -> HashSet<(i32, i32)> {
    let mut result = HashSet::new();
    for (x, y) in circles
        .iter()
        .sorted_by_key(|(x, y)| -x * direction.0 + -y * direction.1)
    {
        let mut pos = (*x, *y);
        let mut next_pos = (x + direction.0, y + direction.1);
        while !walls.contains(&next_pos) && !result.contains(&next_pos) {
            pos = next_pos;
            next_pos = (pos.0 + direction.0, pos.1 + direction.1);
        }
        result.insert(pos);
    }
    result
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
