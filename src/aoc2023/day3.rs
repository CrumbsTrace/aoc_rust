use ndarray::prelude::*;
use std::collections::{HashMap, HashSet};

pub fn run(input: &str) -> (u32, u32) {
    let grid_input = input.lines().collect::<Vec<_>>();
    let grid = Array2::from_shape_vec(
        (grid_input.len(), grid_input[0].len()),
        grid_input
            .iter()
            .flat_map(|line| line.chars())
            .collect::<Vec<_>>(),
    )
    .unwrap();

    let mut count = 0;
    let mut gear_adjacent_parts = HashMap::new();
    for (y, row) in grid.outer_iter().enumerate() {
        let mut in_number = false;
        let mut part_number = 0;
        let mut is_adjacent_to_symbol = false;
        let mut adjacent_gears = HashSet::new();
        for (x, &cell) in row.iter().enumerate() {
            if cell.is_ascii_digit() {
                if !in_number {
                    in_number = true;
                }

                part_number = part_number * 10 + cell.to_digit(10).unwrap();
                for i in -1..=1 {
                    for j in -1..=1 {
                        if i == 0 && j == 0 {
                            continue;
                        }
                        let (x2, y2) = (x as i32 + i, y as i32 + j);
                        if let Some(&cell2) = grid.get((y2 as usize, x2 as usize)) {
                            if cell2 != '.' && !cell2.is_ascii_digit() {
                                is_adjacent_to_symbol = true;
                                if cell2 == '*' {
                                    adjacent_gears.insert((x2, y2));
                                }
                            }
                        }
                    }
                }
            } else if in_number {
                if is_adjacent_to_symbol {
                    update_parts(
                        &mut count,
                        part_number,
                        &adjacent_gears,
                        &mut gear_adjacent_parts,
                    );
                }
                part_number = 0;
                in_number = false;
                is_adjacent_to_symbol = false;
                adjacent_gears.clear();
            }
        }
        if in_number && is_adjacent_to_symbol {
            update_parts(
                &mut count,
                part_number,
                &adjacent_gears,
                &mut gear_adjacent_parts,
            );
        }
    }

    let gear_ratio_sum = gear_adjacent_parts
        .values()
        .filter(|parts| parts.len() == 2)
        .map(|parts| parts.iter().product::<u32>())
        .sum::<u32>();

    (count, gear_ratio_sum)
}

fn update_parts(
    count: &mut u32,
    part_number: u32,
    adjacent_gears: &HashSet<(i32, i32)>,
    gear_adjacent_parts: &mut HashMap<(i32, i32), Vec<u32>>,
) {
    *count += part_number;
    for gear in adjacent_gears {
        gear_adjacent_parts
            .entry(*gear)
            .and_modify(|e: &mut Vec<u32>| (*e).push(part_number))
            .or_insert(vec![part_number]);
    }
}

#[test]
fn example() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
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
