use std::collections::HashMap;

pub fn run(input: &str) -> (usize, u32) {
    input
        .lines()
        .enumerate()
        .fold((0, 0), |(p1, p2), (i, line)| {
            let mut max_color_counts = HashMap::new();
            let mut game = line.split(':');
            for sample in game.nth(1).unwrap().split(';') {
                for cube in sample.split(',') {
                    let mut split = cube.split_whitespace();
                    let count = split.next().unwrap().parse().unwrap();
                    let color = split.next().unwrap();
                    max_color_counts
                        .entry(color)
                        .and_modify(|e: &mut u32| *e = (*e).max(count))
                        .or_insert(count);
                }
            }

            let power = max_color_counts.values().product::<u32>();
            if *max_color_counts.get("red").unwrap_or(&0) <= 12
                && *max_color_counts.get("green").unwrap_or(&0) <= 13
                && *max_color_counts.get("blue").unwrap_or(&0) <= 14
            {
                (p1 + i + 1, p2 + power)
            } else {
                (p1, p2 + power)
            }
        })
}

#[test]
fn example() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    assert_eq!(run(input), (8, 2286));
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("inputs/2023/day2.txt").unwrap();
    assert_eq!(run(&input), (2105, 72422));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("inputs/2023/day2.txt").unwrap();
    bencher.bench(|| run(divan::black_box(&input)));
}
