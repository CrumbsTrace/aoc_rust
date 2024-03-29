use ndarray::prelude::*;
use std::collections::HashMap;

pub fn run(input: &str) -> (u32, u32) {
    let grid = intialize_grid(input);
    let mut p1 = 0;
    let mut gear_parts = HashMap::new();
    for (y, row) in grid.outer_iter().enumerate() {
        let mut x = 0;
        while x < row.len() {
            let mut part_number = 0;
            let mut x_end = x;
            if row[x].is_ascii_digit() {
                (part_number, x_end) = get_part_number(x, row);
            }

            if part_number > 0 {
                let adjacent_symbols = adjacent_symbols(x, x_end - 1, y, &grid);
                if !adjacent_symbols.is_empty() {
                    p1 += part_number;
                    update_adjacent_gears(adjacent_symbols, &mut gear_parts, part_number);
                }
            }
            x = x_end + 1;
        }
    }

    let p2 = gear_parts
        .values()
        .filter(|parts| parts.len() == 2)
        .map(|parts| parts.iter().product::<u32>())
        .sum::<u32>();

    (p1, p2)
}

fn get_part_number(x: usize, row: ArrayView1<char>) -> (u32, usize) {
    let mut part_number = 0;
    let mut x_end = x;
    while x_end < row.len() && row[x_end].is_ascii_digit() {
        part_number = part_number * 10 + row[x_end].to_digit(10).unwrap();
        x_end += 1
    }
    (part_number, x_end)
}

fn update_adjacent_gears(
    adjacent_symbols: Vec<(i32, i32, char)>,
    gear_adjacent_parts: &mut HashMap<(i32, i32), Vec<u32>>,
    part_number: u32,
) {
    for (x_adj, y_adj, symbol) in adjacent_symbols {
        if symbol != '*' {
            continue;
        }
        gear_adjacent_parts
            .entry((x_adj, y_adj))
            .or_default()
            .push(part_number);
    }
}

fn intialize_grid(input: &str) -> Array2<char> {
    let grid_input = input.lines().collect::<Vec<_>>();
    Array2::from_shape_vec(
        (grid_input.len(), grid_input[0].len()),
        grid_input
            .iter()
            .flat_map(|line| line.chars())
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

fn adjacent_symbols(
    x_start: usize,
    x_end: usize,
    y: usize,
    grid: &Array2<char>,
) -> Vec<(i32, i32, char)> {
    let mut symbols = vec![];
    for y_adj in y.saturating_sub(1)..=y + 1 {
        for x_adj in x_start.saturating_sub(1)..=x_end + 1 {
            if let Some(&symbol) = grid.get((y_adj, x_adj)) {
                if symbol != '.' && !symbol.is_ascii_digit() {
                    symbols.push((x_adj as i32, y_adj as i32, symbol));
                }
            }
        }
    }
    symbols
}

#[test]
fn example() {
    let input = "467..114.
...*.....
..35..633
......#..
617*.....
.....+.58
..592....
......755
...$.*...
.664.598.";
    assert_eq!(run(input), (4361, 467835));
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("inputs/2023/day3.txt").unwrap();
    assert_eq!(run(&input), (544433, 76314915));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("inputs/2023/day3.txt").unwrap();
    bencher.bench(|| run(divan::black_box(&input)));
}
