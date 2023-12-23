use itertools::Itertools;
use rustc_hash::FxHashMap;

type Point = (usize, usize);

pub fn run(input: &str) -> (i32, i32) {
    let maze = input.lines().map(|line| line.as_bytes()).collect_vec();
    let start = (1, 0);
    let end = (maze[0].len() - 2, maze.len() - 1);
    let graph = build_graph(&maze, false);
    let p1 = longest_path(&graph, start, end);
    let graph = build_graph(&maze, true);
    let p2 = longest_path(&graph, start, end);
    (p1, p2)
}

fn build_graph(maze: &[&[u8]], allow_slope: bool) -> FxHashMap<Point, FxHashMap<Point, i32>> {
    let mut graph = FxHashMap::default();
    let mut queue = Vec::new();
    queue.push(((1, 0), (0, 1)));
    let end = (maze[0].len() - 2, maze.len() - 1);
    while let Some((pos, previous_dir)) = queue.pop() {
        let valid_dirs = valid_directions(maze, pos, previous_dir, allow_slope);
        for (dx, dy) in valid_dirs {
            let new_pos = ((pos.0 as i32 + dx) as usize, (pos.1 as i32 + dy) as usize);
            if let Some((distance, new_pos, previous_dir)) =
                to_next_intersection(maze, new_pos, (dx, dy), allow_slope)
            {
                graph
                    .entry(pos)
                    .or_insert(FxHashMap::default())
                    .insert(new_pos, distance + 1);
                if new_pos != end && !graph.contains_key(&new_pos) {
                    queue.push((new_pos, previous_dir));
                }
            }
        }
    }
    graph
}

fn to_next_intersection(
    maze: &[&[u8]],
    pos: (usize, usize),
    previous_dir: (i32, i32),
    allow_slope: bool,
) -> Option<(i32, Point, (i32, i32))> {
    let mut pos = pos;
    let mut previous_dir = previous_dir;
    let mut steps = 0;
    let end = (maze[0].len() - 2, maze.len() - 1);
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
        pos = (x as usize, y as usize);
    }
}

fn valid_directions(
    maze: &[&[u8]],
    pos: (usize, usize),
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
            let (x, y) = (x as usize, y as usize);
            match maze[y][x] {
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

fn longest_path(graph: &FxHashMap<Point, FxHashMap<Point, i32>>, start: Point, end: Point) -> i32 {
    let mut queue = Vec::new();
    queue.push((0, start, start, Vec::new()));
    let mut longest = 0;
    while let Some((distance, pos, previous_pos, mut visited)) = queue.pop() {
        if pos == end {
            longest = longest.max(distance);
            continue;
        }
        let next_options = graph[&pos]
            .iter()
            .filter(|(p, _)| previous_pos != **p && !visited.contains(p))
            .collect_vec();

        //Avoid a clone if there is only one option
        if next_options.len() == 1 {
            let (next_pos, next_distance) = next_options[0];
            let new_distance = distance + next_distance;
            visited.push(next_pos);
            queue.push((new_distance, *next_pos, pos, visited));
        } else {
            for (next_pos, next_distance) in next_options {
                let new_distance = distance + next_distance;
                let mut visited = visited.clone();
                visited.push(next_pos);
                queue.push((new_distance, *next_pos, pos, visited));
            }
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
