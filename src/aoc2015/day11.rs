use std::fs;
use divan::black_box;

pub fn run(input: &str) -> (String, String) {
    let mut input = input.trim().to_owned().into_bytes();
    find_next_valid_password(&mut input);
    let p1 = String::from_utf8(input.clone()).unwrap();
    find_next_valid_password(&mut input);
    let p2 = String::from_utf8(input).unwrap();
    (p1, p2)
}

fn find_next_valid_password(password: &mut [u8]) {
    loop {
        cycle_through_passwords(password);
        if is_valid_password(password) {
            break;
        }
    }
}

fn is_valid_password(password: &[u8]) -> bool {
    let mut found_straight = false;
    let mut pairs = 0;
    let mut last_pair = None;
    for i in 0..password.len() {
        if password[i] == b'i' || password[i] == b'o' || password[i] == b'l' {
            return false;
        }
        if i > 0 {
            if password[i] == password[i - 1] && (last_pair.is_none() || last_pair.unwrap() != password[i]) {
                pairs += 1;
                last_pair = Some(password[i]);
            }
            if i > 1 && password[i] == password[i - 1] + 1 && password[i] == password[i - 2] + 2 {
                found_straight = true;
            }
        }
    }
    pairs >= 2 && found_straight
}

fn cycle_through_passwords(password: &mut [u8]) {
    let mut i = password.len() - 1;
    loop {
        if password[i] == b'z' {
            password[i] = b'a';
            i -= 1;
        } else {
            password[i] += 1;
            break;
        }
    }
}

#[test]
fn example() {
    let result = run("abcdefgh");
    assert_eq!(result, ("abcdffaa".to_owned(), "abcdffbb".to_owned()));
}

#[test]
fn real_input() {
    let input = fs::read_to_string("inputs/2015/day11.txt").unwrap();
    let result = run(&input);
    assert_eq!(result, ("vzbxxyzz".to_owned(), "vzcaabcc".to_owned()));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = fs::read_to_string("inputs/2015/day11.txt").unwrap();
    bencher.bench(|| run(black_box(&input)));
}
