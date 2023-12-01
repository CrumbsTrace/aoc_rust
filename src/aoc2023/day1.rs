const DIGITS: [(&str, &str); 9] = [
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
];

pub fn run(input: &str) -> (u32, u32) {
    let mut p1 = 0;
    let mut p2 = 0;
    for line in input.lines() {
        p1 += get_digit(line, false, false) * 10 + get_digit(line, true, false);
        p2 += get_digit(line, false, true) * 10 + get_digit(line, true, true);
    }
    (p1, p2)
}

fn get_digit(line: &str, last: bool, p2: bool) -> u32 {
    if line.is_empty() {
        0
    } else if last {
        if let Some((_, digit)) = DIGITS.iter().find(|(word, digit)| line.ends_with(digit) || (p2 && line.ends_with(word))) {
            return digit.parse().unwrap();
        }
        get_digit(&line[..line.len() - 1], last, p2)
    } else {
        if let Some((_, digit)) = DIGITS.iter().find(|(word, digit)| line.starts_with(digit) || (p2 && line.starts_with(word))) {
            return digit.parse().unwrap();
        }
        get_digit(&line[1..], last, p2)
    }
}

#[test]
fn example() {
    let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
    let (p1, _) = run(input);
    assert_eq!(p1, 142);
    let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
    let (_, p2) = run(input);
    assert_eq!(p2, 281);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("inputs/2023/day1.txt").unwrap();
    assert_eq!(run(&input), (54304, 54418));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("inputs/2023/day1.txt").unwrap();
    bencher.bench(|| run(divan::black_box(&input)));
}
