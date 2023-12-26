use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use rayon::prelude::*;

type Point = (u16, u16);

pub fn run(input: &str) -> (i32, i32) {
    let maze = input.lines().map(|line| line.as_bytes()).collect_vec();
    let p1 = get_max_length(&maze, false);
    let p2 = get_max_length(&maze, true);
    (p1, p2)
}

fn get_max_length(maze: &[&[u8]], allow_slope: bool) -> i32 {
    let end = ((maze[0].len() - 2) as u16, (maze.len() - 1) as u16);
    let graph = build_graph(maze, allow_slope);
    //For multithreading, collect a few starting positions 
    let starts = collect_starts(&graph, 32.min(graph.len() - 2).max(1));
    starts.into_par_iter().map(|(p, d, visited)| {
        longest_path(&graph, p, end, visited) + d
    }).max().unwrap()
}

fn collect_starts(graph: &FxHashMap<Point, FxHashMap<Point, i32>>, wanted_start_count: usize) -> Vec<(Point, i32, FxHashSet<Point>)> {
    let mut starts = Vec::new();
    starts.push(((1,0), 0, FxHashSet::default()));
    while starts.len() < wanted_start_count {
        let mut queue = starts.clone();
        starts.clear();
        while let Some((pos, distance, mut visited)) = queue.pop() {
            visited.insert(pos);
            for (&next_pos, next_distance) in graph[&pos].iter() {
                if visited.contains(&next_pos) {
                    continue;
                }
                starts.push((next_pos, distance + next_distance, visited.clone()));
            }
        }
    }
    starts
}

fn build_graph(maze: &[&[u8]], allow_slope: bool) -> FxHashMap<Point, FxHashMap<Point, i32>> {
    let mut graph = FxHashMap::default();
    let mut queue = Vec::new();
    queue.push(((1, 0), (0, 1)));
    let end = ((maze[0].len() - 2) as u16, (maze.len() - 1) as u16);
    let mut node_with_end = (0, 0);
    while let Some((pos, previous_dir)) = queue.pop() {
        let valid_dirs = valid_directions(maze, pos, previous_dir, allow_slope);
        for (dx, dy) in valid_dirs {
            let new_pos = ((pos.0 as i32 + dx) as u16, (pos.1 as i32 + dy) as u16);
            if let Some((distance, new_pos, previous_dir)) =
                to_next_intersection(maze, new_pos, (dx, dy), allow_slope)
            {
                graph
                    .entry(pos)
                    .or_insert(FxHashMap::default())
                    .insert(new_pos, distance + 1);
                if new_pos != end && !graph.contains_key(&new_pos) {
                    queue.push((new_pos, previous_dir));
                } else if new_pos == end {
                    node_with_end = pos;
                }
            }
        }
    }
    //For the node that leads to the end, remove all other edges since it's guaranteed to cause it
    //to never reach the end
    graph.get_mut(&node_with_end)
        .unwrap()
        .retain(|&pos, _| pos == end);
    graph
}

fn to_next_intersection(
    maze: &[&[u8]],
    pos: (u16, u16),
    previous_dir: (i32, i32),
    allow_slope: bool,
) -> Option<(i32, Point, (i32, i32))> {
    let mut pos = pos;
    let mut previous_dir = previous_dir;
    let mut steps = 0;
    let end = ((maze[0].len() - 2) as u16, (maze.len() - 1) as u16);
    loop {
        if pos == end {
            return Some((steps, pos, previous_dir));
        }
        let valid_dirs = valid_directions(maze, pos, previous_dir, allow_slope);
        if valid_dirs.len() > 1 {
            return Some((steps, pos, previous_dir));
        }
        if valid_dirs.is_empty() {
            return None;
        }
        steps += 1;
        previous_dir = valid_dirs[0];
        let (x, y) = (pos.0 as i32 + previous_dir.0, pos.1 as i32 + previous_dir.1);
        pos = (x as u16, y as u16);
    }
}

fn valid_directions(
    maze: &[&[u8]],
    pos: (u16, u16),
    previous_dir: (i32, i32),
    allow_slope: bool,
) -> Vec<(i32, i32)> {
    [(0, 1), (0, -1), (1, 0), (-1, 0)]
        .into_iter()
        .filter(|(dx, dy)| {
            if previous_dir == (-dx, -dy) {
                return false;
            }
            let (x, y) = (pos.0 as i32 + dx, pos.1 as i32 + dy);
            if x < 0 || y < 0 {
                return false;
            }
            let (x, y) = (x as u16, y as u16);
            match maze[y as usize][x as usize] {
                b'#' => false,
                b'>' if !allow_slope && dx != &1 => false,
                b'<' if !allow_slope && dx != &-1 => false,
                b'^' if !allow_slope && dy != &-1 => false,
                b'v' if !allow_slope && dy != &1 => false,
                _ => true,
            }
        })
        .collect_vec()
}

fn longest_path(graph: &FxHashMap<Point, FxHashMap<Point, i32>>, start: Point, end: Point, visited: FxHashSet<Point>) -> i32 {
    let mut queue = Vec::new();
    queue.push((0, start, start, visited));
    let mut longest = 0;
    while let Some((distance, pos, previous_pos, mut visited)) = queue.pop() {
        visited.insert(pos);
        if pos == end {
            if distance > longest {
                longest = distance;
            }
            continue;
        }

        for (&next_pos, next_distance) in graph[&pos].iter() {
            if next_pos == previous_pos || visited.contains(&next_pos) {
                continue;
            }
            let new_distance = distance + next_distance;
            queue.push((new_distance, next_pos, pos, visited.clone()));
        }
    }
    longest
}

#[test]
fn example() {
    let input = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
    assert_eq!(run(input), (94, 154));
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("inputs/2023/day23.txt").unwrap();
    assert_eq!(run(&input), (2070, 6498));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("inputs/2023/day23.txt").unwrap();
    bencher.bench(|| run(divan::black_box(&input)));
}
