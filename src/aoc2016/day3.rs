use itertools::Itertools;

pub fn run(input: &str) -> (i32, i32) {
    let mut p1 = 0;
    input.lines().for_each(|line| {
        let mut sides = line
            .split_whitespace()
            .map(|side| side.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        sides.sort();
        if sides[0] + sides[1] > sides[2] {
            p1 += 1;
        }
    });
    let mut p2 = 0;
    input.lines().chunks(3).into_iter().for_each(|chunk| {
        let mut sides = chunk
            .map(|line| {
                line.split_whitespace()
                    .map(|side| side.parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        for i in 0..3 {
            sides.sort_by_key(|side| side[i]);
            if sides[0][i] + sides[1][i] > sides[2][i] {
                p2 += 1;
            }
        }
    });
    (p1, p2)
}

#[test]
fn test() {
    let input = "5 10 25\n5 10 11\n5 10 10";
    assert_eq!(run(input), (2, 2));
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("inputs/2016/day3.txt").unwrap();
    assert_eq!(run(&input), (917, 1649));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("inputs/2016/day3.txt").unwrap();
    bencher.bench(|| run(&input));
}
