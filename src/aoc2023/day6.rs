use itertools::Itertools;

pub fn run(input: &str) -> (i64, i64) {
    let values = input
        .split_whitespace()
        .filter_map(|n| n.parse::<i64>().ok())
        .collect_vec();
    let (t, d) = values.split_at(values.len() / 2);
    let p1 = t.iter().zip(d).map(|(&t, &d)| solve(-1 * t, d)).product();
    let p2_time = t.iter().join("").parse::<i64>().unwrap();
    let p2_dist = d.iter().join("").parse::<i64>().unwrap();
    let p2 = solve(-1 * p2_time, p2_dist);
    (p1, p2)
}

pub fn solve(b: i64, c: i64) -> i64 {
    let sqrt_discriminant = ((b * b - 4 * c) as f64).sqrt();
    let b = b as f64;
    let x1 = ((-b + sqrt_discriminant) / 2. - 1.).ceil() as i64;
    let x2 = ((-b - sqrt_discriminant) / 2. + 1.).floor() as i64;
    (x1 - x2).abs() + 1
}

#[test]
fn example() {
    let input = "Time:      7  15   30
Distance:  9  40  200";
    assert_eq!(run(input), (288, 71503));
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("inputs/2023/day6.txt").unwrap();
    assert_eq!(run(&input), (3316275, 27102791));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("inputs/2023/day6.txt").unwrap();
    bencher.bench(|| run(divan::black_box(&input)));
}
