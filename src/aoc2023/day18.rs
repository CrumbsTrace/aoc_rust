use itertools::Itertools;

pub fn run(input: &str) -> (i64, i64) {
    let mut shoelace1 = 0;
    let mut shoelace2 = 0;
    let mut p1_circumference = 0;
    let mut p2_circumference = 0;
    let mut pos1 = (0, 0);
    let mut pos2 = (0, 0);
    for line in input.lines() {
        let parts = line.split([' ', '(', ')', '#']).filter(|l| !l.is_empty()).collect_vec();
        let new_pos1 = p1_parse(&parts[0..2], pos1);
        let new_pos2 = p2_parse(&parts[2], pos2);
        shoelace1 += shoelace(pos1, new_pos1);
        p1_circumference += magnitude(pos1, new_pos1);
        shoelace2 += shoelace(pos2, new_pos2);
        p2_circumference += magnitude(pos2, new_pos2);
        pos1 = new_pos1;
        pos2 = new_pos2;
    }
    ((shoelace1.abs() + p1_circumference) / 2 + 1, (shoelace2.abs() + p2_circumference) / 2 + 1)
}

fn shoelace(pos1: (i64, i64), pos2: (i64, i64)) -> i64 {
    (pos1.1 + pos2.1) * (pos1.0 - pos2.0) 
}

fn magnitude(pos1: (i64, i64), pos2: (i64, i64)) -> i64 {
    (pos1.0 - pos2.0).abs() + (pos1.1 - pos2.1).abs()
}

fn p1_parse(parts: &[&str], (x, y): (i64, i64)) -> (i64, i64) {
    match parts[0] {
        "R" => (x + i64::from_str_radix(parts[1], 16).unwrap(), y),
        "L" => (x - i64::from_str_radix(parts[1], 16).unwrap(), y),
        "U" => (x, y + i64::from_str_radix(parts[1], 16).unwrap()),
        _ => (x, y - i64::from_str_radix(parts[1], 16).unwrap()),
    }
}

fn p2_parse(color_hex: &str, (x, y): (i64, i64)) -> (i64, i64) {
    let (length, dir) = color_hex.split_at(color_hex.len() - 1);
    match dir {
        "0" => (x + i64::from_str_radix(length, 16).unwrap(), y),
        "1" => (x, y - i64::from_str_radix(length, 16).unwrap()),
        "2" => (x - i64::from_str_radix(length, 16).unwrap(), y),
        _ => (x, y + i64::from_str_radix(length, 16).unwrap()),
    }
}

#[test]
fn example() {
    let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
    assert_eq!(run(input), (62, 952408144115));
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("inputs/2023/day18.txt").unwrap();
    assert_eq!(run(&input), (40761, 106920098354636));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("inputs/2023/day18.txt").unwrap();
    bencher.bench_local(|| run(divan::black_box(&input)));
}
