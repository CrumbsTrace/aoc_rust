use md5::{Digest, Md5};
use rayon::prelude::*;
pub fn run(mut input: &str) -> (String, String) {
    input = input.trim();
    let mut p1 = String::new();
    let mut p2 = vec!['_'; 8];
    //Precalculate 30_000_000 hashes
    let hashes = (0..30_000_000)
        .into_par_iter()
        .filter_map(|i| {
            let mut hasher = Md5::new();
            hasher.update(input.as_bytes());
            hasher.update(i.to_string().as_bytes());
            let hash = hasher.finalize();
            if hash[0] == 0 && hash[1] == 0 && hash[2] < 16 {
                Some(hash)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    for hash in hashes {
        let pos = hash[2] as usize;
        if p1.len() < 8 {
            p1.push_str(&format!("{:x}", hash[2]));
        }
        if pos < 8 && p2[pos] == '_' {
            p2[pos] = format!("{:x}", hash[3] >> 4).chars().next().unwrap();
        }

        if p1.len() == 8 && !p2.contains(&'_') {
            break;
        }
    }
    (p1, p2.iter().collect())
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("inputs/2016/day5.txt").unwrap();
    assert_eq!(run(&input), ("f97c354d".to_owned(), "863dde27".to_owned()));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("inputs/2016/day5.txt").unwrap();
    bencher.bench_local(|| run(&input));
}
