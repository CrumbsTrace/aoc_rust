use std::fs;
use divan::black_box;

struct Reindeer {
    speed: i32,
    fly_time_in_seconds: i32,
    rest_time_in_seconds: i32,
}

impl Reindeer {
    pub fn new(speed: i32, fly_time: i32, rest_time: i32) -> Reindeer {
        Reindeer {
            speed,
            fly_time_in_seconds: fly_time,
            rest_time_in_seconds: rest_time,
        }
    }
}

pub fn run(input: &str, t: i32) -> (i32, i32) {
    let mut reindeers = Vec::new();
    for line in input.lines() {
        let mut words = line.split_whitespace();
        let speed = words.nth(3).unwrap().parse::<i32>().unwrap();
        let fly_time = words.nth(2).unwrap().parse::<i32>().unwrap();
        let rest_time = words.nth(6).unwrap().parse::<i32>().unwrap();
        reindeers.push(Reindeer::new(speed, fly_time, rest_time));
    }

    let mut points = vec![0; reindeers.len()];
    let mut distances = vec![0; reindeers.len()];
    let mut winning_reindeers = Vec::new();
    for time in 0..t {
        let mut max_distance = 0;
        for (i, reindeer) in reindeers.iter().enumerate() {
            if time % (reindeer.fly_time_in_seconds + reindeer.rest_time_in_seconds) < reindeer.fly_time_in_seconds {
                distances[i] += reindeer.speed;
            }
            if distances[i] > max_distance {
                max_distance = distances[i];
                winning_reindeers.clear();
                winning_reindeers.push(i);
            } else if distances[i] == max_distance {
                winning_reindeers.push(i);
            }
        }
        for i in winning_reindeers.iter() {
            points[*i] += 1;
        }
        winning_reindeers.clear();
    }
    let p1 = *distances.iter().max().unwrap();
    let p2 = *points.iter().max().unwrap();
    (p1, p2)
}

#[test]
fn example() {
    let input = r"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";
    assert_eq!(run(input, 1000), (1120, 689));
}

#[test]
fn real_input() {
    let input = fs::read_to_string("inputs/2015/day14.txt").unwrap();
    let result = run(&input, 2503);
    assert_eq!(result, (2640, 1102))
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = fs::read_to_string("inputs/2015/day14.txt").unwrap();
    bencher.bench(|| run(black_box(&input), 2503));
}

