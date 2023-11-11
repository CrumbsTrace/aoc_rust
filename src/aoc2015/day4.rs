use divan::black_box;
use md5::{Digest, Md5};
use rayon::prelude::*;
use std::fs;

// MD5 can only really be brute forced for this realistically
// We use rayon for threading which is unnecessary here but was fun to implement
pub fn run(mut input: &str, skip_p2: bool) -> (i32, i32) {
    input = input.trim();

    let p1 = (0..i32::MAX)
        .into_par_iter()
        .find_first(|i| {
            let mut hasher = Md5::new();
            hasher.update(input.as_bytes());
            hasher.update(i.to_string().as_bytes());
            let output = hasher.finalize();
            output[0] == 0 && output[1] == 0 && output[2] < 16
        })
        .unwrap();

    let mut p2 = 0;

    if !skip_p2 {
        p2 = (p1..i32::MAX)
            .into_par_iter()
            .find_first(|i| {
                let mut hasher = Md5::new();
                hasher.update(input.as_bytes());
                hasher.update(i.to_string().as_bytes());
                let output = hasher.finalize();
                output[0] == 0 && output[1] == 0 && output[2] == 0
            })
            .unwrap();
    }

    (p1, p2)
}

#[test]
fn example() {
    let (p1, _) = run("abcdef", true);
    assert_eq!(p1, 609043);
}

#[test]
fn real_input() {
    let input = fs::read_to_string("inputs/2015/day4.txt").unwrap();
    let (p1, p2) = run(&input, false);
    assert_eq!(p1, 254575);
    assert_eq!(p2, 1038736);
}

#[divan::bench(threads = false)]
fn bench(bencher: divan::Bencher) {
    let input = fs::read_to_string("inputs/2015/day4.txt").unwrap();
    rayon::ThreadPoolBuilder::new().num_threads(1).build_global().unwrap();
    bencher.bench_local(|| run(black_box(&input), false));
}
