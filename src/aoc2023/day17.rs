use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, Copy, Eq, Ord, Hash)]
struct Step {
    steps: u32,
    x: usize,
    y: usize,
    last_dir: (i32, i32),
    streak: u8,
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.steps.cmp(&self.steps))
    }
}

impl PartialEq for Step {
    fn eq(&self, other: &Self) -> bool {
        other.steps.eq(&self.steps)
    }
}

pub fn run(input: &str) -> (u32, u32) {
    let grid = input
        .lines()
        .map(|l| l.as_bytes().iter().map(|b| b - b'0').collect_vec())
        .collect_vec();

    let p1 = bfs(&grid, (grid[0].len() - 1, grid.len() - 1), false);
    let p2 = bfs(&grid, (grid[0].len() - 1, grid.len() - 1), true);
    (p1, p2)
}

fn bfs(grid: &[Vec<u8>], goal: (usize, usize), ultra: bool) -> u32 {
    let mut queue = BinaryHeap::new();
    let mut visited = FxHashMap::default();
    queue.push(Step {
        steps: 0,
        x: 0,
        y: 0,
        last_dir: (0, 0),
        streak: 0,
    });

    while let Some(step) = queue.pop() {
        if let Some(best_streak) = visited.get_mut(&(step.x, step.y, step.last_dir)) {
            if !update_visited(ultra, best_streak, step) {
                continue;
            }
        } else {
            visited.insert((step.x, step.y, step.last_dir), step.streak);
        }
        if (step.x, step.y) == goal && (!ultra || step.streak >= 4) {
            return step.steps;
        }

        for direction in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let x = step.x as i32 + direction.0;
            let y = step.y as i32 + direction.1;
            if x < 0
                || y < 0
                || x >= grid[0].len() as i32
                || y >= grid.len() as i32
                || step.last_dir == (-direction.0, -direction.1)
                || (*direction == step.last_dir
                    && ((!ultra && step.streak >= 3) || (ultra && step.streak >= 10)))
                || (ultra
                    && *direction != step.last_dir
                    && step.last_dir != (0, 0)
                    && step.streak < 4)
            {
                continue;
            }

            let new_steps = step.steps + grid[y as usize][x as usize] as u32;
            queue.push(Step {
                steps: new_steps,
                x: x as usize,
                y: y as usize,
                last_dir: *direction,
                streak: if step.last_dir == *direction {
                    step.streak + 1
                } else {
                    1
                },
            });
        }
    }
    panic!("No path found");
}

fn update_visited(ultra: bool, best_streak: &mut u8, step: Step) -> bool {
    let mut check = true;
    if ultra {
        if *best_streak >= 4 && step.streak >= *best_streak {
            check = false;
        }

        if *best_streak < 3 && step.streak <= *best_streak {
            check = false;
        }

        if step.streak >= 4 && step.streak < *best_streak {
            *best_streak = step.streak;
        } else if *best_streak < 4 && step.streak > *best_streak {
            *best_streak = step.streak;
        }
    } else {
        if step.streak >= *best_streak {
            check = false;
        }
        *best_streak = step.streak.min(*best_streak);
    }
    check
}

#[test]
fn example() {
    let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
    assert_eq!(run(input), (102, 94));
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("inputs/2023/day17.txt").unwrap();
    assert_eq!(run(&input), (970, 1149));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("inputs/2023/day17.txt").unwrap();
    bencher.bench(|| run(divan::black_box(&input)));
}
