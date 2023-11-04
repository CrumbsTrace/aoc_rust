use divan::black_box;
use serde_json::Value;
use std::fs;

pub fn run(input: &str) -> (i32, i32) {
    let v: Value = serde_json::from_str(input).unwrap();
    let sum = sum_numbers(&v, false);
    let sum2 = sum_numbers(&v, true);
    (sum, sum2)
}

fn sum_numbers(v: &Value, ignore_red: bool) -> i32 {
    match v {
        Value::Number(n) => n.as_i64().unwrap() as i32,
        Value::Array(a) => a.iter().fold(0, |result, v| result + sum_numbers(v, ignore_red)),
        Value::Object(o) => {
            if ignore_red && o.values().any(|v| v == "red") {
                0
            } else {
                o.values().fold(0, |result, v| result + sum_numbers(v, ignore_red))
            }
        }
        _ => 0,
    }
}

#[test]
fn example() {
    assert_eq!(run("[1,2,3]"), (6, 6));
    assert_eq!(run(r#"{"a":3,"b":4}"#), (7, 7));
}

#[test]
fn real_input() {
    let input = fs::read_to_string("inputs/2015/day12.txt").unwrap();
    let result = run(&input);
    assert_eq!(result, (156366, 96852))
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = fs::read_to_string("inputs/2015/day12.txt").unwrap();
    bencher.bench(|| run(black_box(&input)));
}
