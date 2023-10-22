use std::{collections::HashSet, fs};

#[divan::bench] 
pub fn run() {
    let input = fs::read_to_string("inputs/2021/day13.txt").unwrap();

    let mut point_list = HashSet::new();
    let mut folds = Vec::new();

    input.lines().for_each(|line| {
        if line.contains("fold") {
            let split = line.split('=').collect::<Vec<&str>>();
            if split[0].contains('x') {
                folds.push(('x', split[1].trim().parse::<i32>().unwrap()));
            } else {
                folds.push(('y', split[1].trim().parse::<i32>().unwrap()));
            }
        } else if !line.is_empty() {
            let split = line.split(',').collect::<Vec<&str>>();
            let x = split[0].trim().parse::<i32>().unwrap();
            let y = split[1].trim().parse::<i32>().unwrap();
            point_list.insert(Point { x, y });
        }
    });
    let fold_count = folds.len();

    folds.iter().enumerate().for_each(|(index, (axis, coord))| {
        point_list = fold_along_axis(*axis, &point_list, *coord);

        if index == 0 {
            assert!(point_list.len() == 661);
        } else if index == fold_count - 1 {
            // print the list of points to the console as a grid of dots
            let width = point_list.iter().map(|p| p.x).max().unwrap();
            let height = point_list.iter().map(|p| p.y).max().unwrap();
            let mut grid = vec![vec![' '; width as usize + 1]; height as usize + 1];
            point_list.iter().for_each(|point| {
                grid[point.y as usize][point.x as usize] = '#';
            });
            grid.iter().for_each(|_row| {
                // Print commented away for debugging purposes
                // println!("{}", row.iter().collect::<String>());
            });
        }
    });
}

fn fold_along_axis(
    axis: char,
    point_list: &HashSet<Point>,
    fold_coordinate: i32,
) -> HashSet<Point> {
    let mut new_point_list = HashSet::new();
    if axis == 'x' {
        point_list.iter().for_each(|point| {
            if point.x < fold_coordinate {
                new_point_list.insert(*point);
            } else {
                new_point_list.insert(Point {
                    x: fold_coordinate - (point.x - fold_coordinate),
                    y: point.y,
                });
            }
        });
    } else {
        point_list.iter().for_each(|point| {
            if point.y < fold_coordinate {
                new_point_list.insert(*point);
            } else {
                new_point_list.insert(Point {
                    x: point.x,
                    y: fold_coordinate - (point.y - fold_coordinate),
                });
            }
        });
    }
    new_point_list
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}
