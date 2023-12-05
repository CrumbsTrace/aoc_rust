use itertools::Itertools;

pub fn run(input: &str) -> (i64, i64) {
    let mut lines = input.split("\n\n").map(|section| {
        section
            .split_whitespace()
            .filter_map(|n| n.parse::<i64>().ok())
            .collect_vec()
    });

    let seeds = lines.next().unwrap();
    let maps = lines.collect_vec();
    let p1_lowest = seeds.iter().map(|s| location(*s, &maps)).min().unwrap();
    let lowest_map_range = lowest_map_range(&maps);

    let mut p2_lowest = i64::MAX;
    for pair in seeds.chunks(2) {
        let mut search_range = (pair[0], (pair[0] + pair[1]));
        let mut lowest = i64::MAX;
        let mut step = lowest_map_range;
        while step > 0 {
            (search_range, lowest) = refine_search(search_range, &maps, step);
            step >>= 1;
        }
        p2_lowest = p2_lowest.min(lowest);
    }
    (p1_lowest, p2_lowest)
}

fn refine_search((start, end): (i64, i64), maps: &[Vec<i64>], step: i64) -> ((i64, i64), i64) {
    let best_seed = (start..end)
        .step_by(step as usize)
        .min_by_key(|&seed| location(seed, maps))
        .unwrap();
    let location = location(best_seed, maps);
    let new_range = ((best_seed - step).max(start), (best_seed + step).min(end));
    (new_range, location)
}

fn location(seed: i64, maps: &[Vec<i64>]) -> i64 {
    let mut seed = seed;
    for category in maps.iter() {
        for map in category.chunks(3) {
            if map[1] <= seed && map[1] + map[2] > seed {
                seed += map[0] - map[1];
                break;
            }
        }
    }
    seed
}

fn lowest_map_range(maps: &[Vec<i64>]) -> i64 {
    maps.iter()
        .map(|category| category.chunks(3).map(|map| map[2]).min().unwrap())
        .min()
        .unwrap()
}

#[test]
fn example() {
    let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    assert_eq!(run(input), (35, 46));
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("inputs/2023/day5.txt").unwrap();
    assert_eq!(run(&input), (806029445, 59370572));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("inputs/2023/day5.txt").unwrap();
    bencher.bench(|| run(divan::black_box(&input)));
}
