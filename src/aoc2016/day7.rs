use std::collections::HashSet;

pub fn run(input: &str) -> (i32, i32) {
    let p1 = input.lines().filter(|line| is_tls(line)).count();
    let p2 = input.lines().filter(|line| is_ssl(line)).count();
    (p1 as i32, p2 as i32)
}

fn is_tls(s: &str) -> bool { 
    let mut chars = s.chars();
    let mut current_sequence = vec![];
    let mut in_hypernet_sequence = false;
    let mut found_abba = false;
    loop {
        let next_char = chars.next();
        if next_char.is_none() {
            break;
        }
        if next_char.unwrap() == '[' {
            in_hypernet_sequence = true;
        } else if next_char.unwrap() == ']' {
            in_hypernet_sequence = false;
        }

        if current_sequence.len() == 4 {
            current_sequence.remove(0);
        }
        current_sequence.push(next_char.unwrap());

        if current_sequence.len() == 4 && current_sequence[0] == current_sequence[3] && current_sequence[1] == current_sequence[2] && current_sequence[0] != current_sequence[1] {
            if in_hypernet_sequence {
                return false;
            }
            found_abba = true;
        }

    }
    found_abba
}

fn is_ssl(s: &str) -> bool { 
    let mut chars = s.chars();
    let mut current_sequence = vec![];
    let mut in_hypernet_sequence = false;
    let mut found_abas = HashSet::new();
    let mut found_babs = HashSet::new();
    loop {
        let next_char = chars.next();
        if next_char.is_none() {
            break;
        }
        if next_char.unwrap() == '[' {
            in_hypernet_sequence = true;
        } else if next_char.unwrap() == ']' {
            in_hypernet_sequence = false;
        }
        if current_sequence.len() == 3 {
            current_sequence.remove(0);
        }
        current_sequence.push(next_char.unwrap());
        if current_sequence.len() == 3 && current_sequence[0] == current_sequence[2] && current_sequence[0] != current_sequence[1] {
            if in_hypernet_sequence {
                let corresponding_aba = vec![current_sequence[1], current_sequence[0], current_sequence[1]];
                found_babs.insert(corresponding_aba);
            } else {
                found_abas.insert(current_sequence.clone());
            }
        }
    }
    found_abas.intersection(&found_babs).count() > 0
}

#[test]
fn test() {
    let input = "abba[mnop]qrst\n\
                 abcd[bddb]xyyx\n\
                 aaaa[qwer]tyui\n\
                 ioxxoj[asdfgh]zxcvbn";
    assert_eq!(run(input), (2, 0));
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("inputs/2016/day7.txt").unwrap();
    assert_eq!(run(&input), (115, 44));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("inputs/2016/day7.txt").unwrap();
    bencher.bench(|| run(&input));
}
