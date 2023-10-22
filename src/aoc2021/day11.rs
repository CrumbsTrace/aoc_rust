use ndarray::Array2;
use std::{fmt, fs};

#[divan::bench] 
pub fn run() {
    let input = fs::read_to_string("inputs/2021/day11.txt").unwrap();
    //read all digits in input into Array2
    let lines: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    //move into Array2
    let height = lines.len();
    let width = lines[0].len();
    let mut octopus_grid = Array2::from_shape_fn((height, width), |(x, y)| {
        Octopus::new(
            Point {
                x: x as i32,
                y: y as i32,
            },
            lines[y][x] as u8,
            width,
            height,
        )
    });

    let p1 = flash_for_step_count(&mut octopus_grid, 100);
    let p2 = flash_till_synchronous(&mut octopus_grid) + 100;

    // println!("Part 1: {}", p1);
    // println!("Part 2: {}", p2);
    assert_eq!(p1, 1655);
    assert_eq!(p2, 337)
}

fn flash_for_step_count(octopus_grid: &mut Array2<Octopus>, step_count: u32) -> usize {
    let mut total_flash_count = 0_usize;

    for _ in 0..step_count {
        Octopus::increment_brightness(octopus_grid);
        loop {
            let flash_count = Octopus::flash(octopus_grid);
            if flash_count == 0 {
                break;
            }
            total_flash_count += flash_count;
        }
        Octopus::reset_flashed(octopus_grid);
    }
    total_flash_count
}

fn flash_till_synchronous(octopus_grid: &mut Array2<Octopus>) -> usize {
    let mut step_count = 0_usize;

    loop {
        step_count += 1;
        let mut total_flash_count = 0_usize;

        Octopus::increment_brightness(octopus_grid);
        loop {
            let flash_count = Octopus::flash(octopus_grid);
            if flash_count == 0 {
                break;
            }
            total_flash_count += flash_count;
        }

        if total_flash_count == octopus_grid.len() {
            return step_count;
        }
        Octopus::reset_flashed(octopus_grid);
    }
}

// return 8 adjacent neighbors of Point
fn get_adjacent_neighbors(point: Point, width: usize, height: usize) -> Vec<Point> {
    let x = point.x;
    let y = point.y;

    let neighbors: Vec<Point> = [
        Point { x: x - 1, y: y - 1 },
        Point { x, y: y - 1 },
        Point { x: x + 1, y: y - 1 },
        Point { x: x - 1, y },
        Point { x: x + 1, y },
        Point { x: x - 1, y: y + 1 },
        Point { x, y: y + 1 },
        Point { x: x + 1, y: y + 1 },
    ]
    .into_iter()
    .filter(|point| bounds_check(point, width, height))
    .collect();

    neighbors
}

fn bounds_check(point: &Point, width: usize, height: usize) -> bool {
    point.x >= 0
        && point.x < width.try_into().unwrap()
        && point.y >= 0
        && point.y < height.try_into().unwrap()
}

struct Octopus {
    brightness: u8,
    neighbors: Vec<Point>,
    flashed: bool,
}

impl Octopus {
    pub fn new(position: Point, brightness: u8, width: usize, height: usize) -> Octopus {
        Octopus {
            brightness,
            neighbors: get_adjacent_neighbors(position, width, height),
            flashed: false,
        }
    }

    pub fn increment_brightness(grid: &mut Array2<Octopus>) {
        for x in 0..grid.shape()[0] {
            for y in 0..grid.shape()[1] {
                grid[[x, y]].brightness += 1;
            }
        }
    }

    pub fn flash(grid: &mut Array2<Octopus>) -> usize {
        let mut octopuses_to_increase: Vec<Point> = Vec::new();
        let mut flash_count: usize = 0;

        for x in 0..grid.shape()[0] {
            for y in 0..grid.shape()[1] {
                if grid[[x, y]].brightness > 9 && !grid[[x, y]].flashed {
                    grid[[x, y]].flashed = true;
                    grid[[x, y]].brightness = 0;
                    //add neighbors to octopuses to increase
                    octopuses_to_increase.extend(
                        grid[[x, y]]
                            .neighbors
                            .iter()
                            .filter(|point| !grid[[point.x as usize, point.y as usize]].flashed),
                    );

                    flash_count += 1;
                }
            }
        }

        for octopus in octopuses_to_increase {
            //if neighbor is not already flashed, increase brightness
            if !grid[[octopus.x as usize, octopus.y as usize]].flashed {
                grid[[octopus.x as usize, octopus.y as usize]].brightness += 1;
            }
        }
        flash_count
    }

    //reset flashed state in grid
    pub fn reset_flashed(grid: &mut Array2<Octopus>) {
        for x in 0..grid.shape()[0] {
            for y in 0..grid.shape()[1] {
                grid[[x, y]].flashed = false;
            }
        }
    }
}

//implement debug for Octopus as brightness
impl fmt::Debug for Octopus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.brightness)
    }
}

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}
