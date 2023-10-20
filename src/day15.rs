use ndarray::Array2;
use std::{
    collections::{BinaryHeap, HashMap},
    fs,
};

#[divan::bench] 
pub fn run() {
    let input = fs::read_to_string("inputs/day15.txt").unwrap();
    let lines: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();

    let height = lines.len();
    let width = lines[0].len();
    let mut grid = Array2::zeros((width * 5, height * 5));
    for (y, line) in lines.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            grid[[x, y]] = c;
        }
    }

    for x in 0..grid.shape()[0] {
        let actual_x = x % width;
        for y in 0..grid.shape()[1] {
            let actual_y = y % height;
            let offset = x / width + y / height;
            grid[[x, y]] = flow_over(grid[[actual_x, actual_y]] + offset as i32);
        }
    }

    let p1 = find_shortest_path(&grid, Point::new(width - 1, height - 1));
    let p2 = find_shortest_path(&grid, Point::new(grid.shape()[0] - 1, grid.shape()[1] - 1));

    // println!("Part 1: {}", p1);
    // println!("Part 2: {}", p2);
    assert_eq!(p1, 390);
    assert_eq!(p2, 2814);
}

fn find_shortest_path(grid: &Array2<i32>, end: Point) -> i32 {
    let mut priority_queue = BinaryHeap::new();

    let start = Point { x: 0, y: 0 };
    let width = grid.shape()[0];
    let height = grid.shape()[1];
    let width_bound = width as i32 - 1;
    let height_bound = height as i32 - 1;

    priority_queue.push(PathStep {
        point: start,
        steps: 0,
    });

    let mut g_scores = HashMap::new();
    g_scores.insert(cantor_pairing(start), 0);

    let mut neighbor: Point;
    loop {
        let step = priority_queue.pop().unwrap();
        let point = step.point;
        let g_score = g_scores[&cantor_pairing(point)];

        if point == end {
            return g_score;
        }

        if point.x > 0 {
            neighbor = Point::new2(point.x - 1, point.y);
            handle_neighbor(grid, neighbor, &mut g_scores, &mut priority_queue, g_score);
        }
        if point.x < width_bound {
            neighbor = Point::new2(point.x + 1, point.y);
            handle_neighbor(grid, neighbor, &mut g_scores, &mut priority_queue, g_score);
        }
        if point.y > 0 {
            neighbor = Point::new2(point.x, point.y - 1);
            handle_neighbor(grid, neighbor, &mut g_scores, &mut priority_queue, g_score);
        }
        if point.y < height_bound {
            neighbor = Point::new2(point.x, point.y + 1);
            handle_neighbor(grid, neighbor, &mut g_scores, &mut priority_queue, g_score);
        }
    }
}

#[inline]
fn handle_neighbor(
    grid: &Array2<i32>,
    neighbor: Point,
    g_scores: &mut HashMap<i32, i32>,
    priority_queue: &mut BinaryHeap<PathStep>,
    g_score: i32,
) {
    let neighbor_steps = g_score + get_cost(neighbor, grid);
    let old_steps = g_scores
        .entry(cantor_pairing(neighbor))
        .or_insert(i32::max_value());

    if neighbor_steps < *old_steps {
        *old_steps = neighbor_steps;

        priority_queue.push(PathStep {
            point: neighbor,
            steps: neighbor_steps,
        });
    }
}

fn get_cost(point: Point, grid: &Array2<i32>) -> i32 {
    grid[[point.x as usize, point.y as usize]]
}

fn flow_over(mut value: i32) -> i32 {
    while value > 9 {
        value -= 9;
    }
    value
}

#[inline]
fn cantor_pairing(Point { x, y }: Point) -> i32 {
    ((x + y) * (x + y + 1)) / 2 + y
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

//Implement constructor for Point
impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point {
            x: x.try_into().unwrap(),
            y: y.try_into().unwrap(),
        }
    }

    fn new2(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PathStep {
    point: Point,
    steps: i32,
}

//implement PartialOrd for PathStep based on steps
impl PartialOrd for PathStep {
    fn partial_cmp(&self, other: &PathStep) -> Option<std::cmp::Ordering> {
        Some(self.steps.cmp(&other.steps).reverse())
    }
}

//implement Ord for PathStep based on steps
impl Ord for PathStep {
    fn cmp(&self, other: &PathStep) -> std::cmp::Ordering {
        self.steps.cmp(&other.steps).reverse()
    }
}
