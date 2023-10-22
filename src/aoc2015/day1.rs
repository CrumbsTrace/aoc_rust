use std::fs;

pub fn run(input: &str) -> (i32, Option<usize>) {
    let mut floor = 0;
    let mut basement = None;

    for (i, character) in input.trim().chars().enumerate() {
        match character {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("Invalid character"),
        }   

        if floor < 0 && basement.is_none() {
            basement = Some(i + 1);
        }
    }   

    (floor, basement)
}

#[test]
fn example() {
    assert_eq!(run("(()))"), (-1, Some(5)));
}

#[test]
fn real_input() {
    let input = fs::read_to_string("inputs/2015/day1.txt").unwrap();
    let (p1, p2) = run(&input);
    assert_eq!(p1, 74);
    assert_eq!(p2, Some(1795));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    bencher.with_inputs(|| {
        fs::read_to_string("inputs/2015/day1.txt").unwrap()
    }).bench_refs(|s| {
        run(s);
    });
}
