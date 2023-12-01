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
    let mut digits_p1 = vec![];
    let mut digits_p2 = vec![];
    for line in input.lines() {
        collect_digits(line, &mut digits_p1, &mut digits_p2);
        p1 += digits_p1[0] * 10 + digits_p1[digits_p1.len() - 1];
        p2 += digits_p2[0] * 10 + digits_p2[digits_p2.len() - 1];
        digits_p1.clear();
        digits_p2.clear();
    }
    (p1, p2)
}

fn collect_digits(mut line: &str, digits_p1: &mut Vec<u32>, digits_p2: &mut Vec<u32>) {
    while !line.is_empty() {
        if let Some((_, digit)) = DIGITS.iter().find(|(word, digit)| line.starts_with(word) || line.starts_with(digit))
        {
            let parsed_digit = digit.parse::<u32>().unwrap();
            if line.starts_with(digit) {
                digits_p1.push(parsed_digit);
            }
            digits_p2.push(parsed_digit);
        }
        line = &line[1..];
    }
    if digits_p1.is_empty() {
        digits_p1.push(0)
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
