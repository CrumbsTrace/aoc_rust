use ndarray::Array2;
use std::fs;

#[divan::bench] 
pub fn run() {
    let input = fs::read_to_string("inputs/day9.txt").unwrap();
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = lines.len();
    let width = lines[0].len();

    let mut grid = Array2::zeros((width, height));

    for (y, line) in lines.iter().enumerate() {
        for (x, value) in line.iter().enumerate() {
            grid[[x, y]] = char::to_digit(*value, 10).unwrap();
        }
    }

    let mut p1 = 0;
    let mut lowest_points = Vec::new();
    for x in 0..width {
        for y in 0..height {
            let original = grid[[x, y]];
            let mut min = u32::MAX;
            let neighbors = neighbors(width, height, x as i32, y as i32);

            for neighbor in neighbors {
                min = min.min(grid[neighbor]);
            }

            if original < min {
                lowest_points.push((x, y));
                p1 += original + 1;
            }
        }
    }

    let mut basins_sizes: Vec<usize> = get_all_basins(&grid, lowest_points)
        .iter()
        .map(|b| b.len())
        .collect();
    basins_sizes.sort_unstable();
    let p2: usize = basins_sizes.iter().rev().take(3).product();

    // println!("Part 1: {}", p1);
    // println!("Part 2: {}", p2);
    assert_eq!(p1, 585);
    assert_eq!(p2, 827904);
}

fn neighbors(width: usize, height: usize, x: i32, y: i32) -> Vec<[usize; 2]> {
    [[x, y + 1], [x - 1, y], [x + 1, y], [x, y - 1]]
        .into_iter()
        .filter(|[i, j]| {
            *i >= 0 && *i < width.try_into().unwrap() && *j >= 0 && *j < height.try_into().unwrap()
        })
        .map(|[i, j]| [i as usize, j as usize])
        .collect()
}

fn get_all_basins(grid: &Array2<u32>, lowest_points: Vec<(usize, usize)>) -> Vec<Vec<[usize; 2]>> {
    let mut basins = vec![];
    for (x, y) in lowest_points {
        if grid[[x, y]] == 9
            || basins
                .iter()
                .any(|basin: &Vec<[usize; 2]>| basin.contains(&[x, y]))
        {
            continue;
        }

        let basin = get_basin(grid, x, y);
        basins.push(basin);
    }
    basins
}

fn get_basin(grid: &Array2<u32>, x: usize, y: usize) -> Vec<[usize; 2]> {
    let mut connected = vec![[x, y]];
    let mut to_visit = vec![[x, y]];

    while !to_visit.is_empty() {
        let [x, y] = to_visit.pop().unwrap();
        let neighbors = neighbors(grid.shape()[0], grid.shape()[1], x as i32, y as i32);

        for [i, j] in neighbors {
            if grid[[i, j]] != 9 && !connected.contains(&[i, j]) {
                connected.push([i, j]);
                to_visit.push([i, j]);
            }
        }
    }

    connected
}
