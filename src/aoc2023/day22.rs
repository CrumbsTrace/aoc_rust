use itertools::Itertools;
use ndarray::{s, Array3};
use rayon::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;

pub fn run(input: &str) -> (u32, u32) {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;
    let blocks = input
        .lines()
        .filter_map(|line| {
            let coordinates = line
                .split([',', '~'])
                .filter_map(|n| n.parse::<usize>().ok())
                .chunks(3);
            let (x1, y1, z1) = coordinates.into_iter().next()?.collect_tuple()?;
            let (x2, y2, z2) = coordinates.into_iter().next()?.collect_tuple()?;
            max_x = max_x.max(x1).max(x2);
            max_y = max_y.max(y1).max(y2);
            max_z = max_z.max(z1).max(z2);
            Some((x1, x2, y1, y2, z1, z2))
        })
        .collect_vec();

    let (leans_on, supports) = drop_blocks(&blocks, max_x, max_y, max_z);
    let mut removable = 0;
    for (_block, supports) in supports.iter() {
        let mut can_be_removed = true;
        for &support in supports {
            if leans_on[&support].len() == 1 {
                can_be_removed = false;
                break;
            }
        }
        if can_be_removed {
            removable += 1;
        }
    }

    let supports_clone = supports.clone();
    let sum = supports_clone
        .into_par_iter()
        .map(|(block, _)| fall_count(&leans_on, &supports, block))
        .sum();

    (removable, sum)
}

fn fall_count(
    leans_on: &FxHashMap<usize, Vec<usize>>,
    supports: &FxHashMap<usize, Vec<usize>>,
    removed: usize,
) -> u32 {
    let mut removed_blocks = FxHashSet::default();
    let mut queue = VecDeque::default();
    queue.push_back(removed);
    removed_blocks.insert(removed);
    while let Some(block) = queue.pop_front() {
        for support in &supports[&block] {
            if leans_on[support]
                .iter()
                .filter(|s| !removed_blocks.contains(s))
                .count()
                == 0
            {
                queue.push_back(*support);
                removed_blocks.insert(*support);
            }
        }
    }
    removed_blocks.len() as u32 - 1
}

fn drop_blocks(
    blocks: &[(usize, usize, usize, usize, usize, usize)],
    highest_x: usize,
    highest_y: usize,
    highest_z: usize,
) -> (FxHashMap<usize, Vec<usize>>, FxHashMap<usize, Vec<usize>>) {
    let mut grid = Array3::from_elem((highest_x + 1, highest_y + 1, highest_z + 1), 0);
    let mut leans_on = FxHashMap::default();
    let mut supports = FxHashMap::default();
    let mut blocks = blocks.to_vec();
    blocks.sort_unstable_by_key(|(_, _, _, _, z1, _)| *z1);
    for (i, (x1, x2, y1, y2, z1, z2)) in blocks.into_iter().enumerate() {
        supports.insert(i + 1, Vec::new());
        for new_z in (0..z1).rev() {
            let non_zero_blocks = grid
                .slice(s![x1..=x2, y1..=y2, new_z])
                .into_iter()
                .filter(|&&b| b > 0)
                .copied()
                .unique()
                .collect_vec();

            if !non_zero_blocks.is_empty() || new_z == 0 {
                grid.slice_mut(s![x1..=x2, y1..=y2, (new_z + 1)..=((new_z + 1) + z2 - z1)])
                    .fill(i + 1);
                for &block in &non_zero_blocks {
                    supports.entry(block).or_insert_with(Vec::new).push(i + 1);
                }
                leans_on.insert(i + 1, non_zero_blocks);
                break;
            }
        }
    }
    (leans_on, supports)
}

#[test]
fn example() {
    let input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
    assert_eq!(run(input), (5, 7));
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("inputs/2023/day22.txt").unwrap();
    assert_eq!(run(&input), (522, 83519));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("inputs/2023/day22.txt").unwrap();
    bencher.bench(|| run(divan::black_box(&input)));
}
