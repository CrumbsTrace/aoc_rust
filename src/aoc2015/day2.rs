use std::fs;

pub fn run(input: &str) -> (i32, i32) {
    let result = input
        .lines()
        .map(|line| {
            let mut numbers = line.split('x').map(|n| n.parse::<i32>().unwrap());
            let l = numbers.next().unwrap();
            let w = numbers.next().unwrap();
            let h = numbers.next().unwrap();
            (calculate_paper_size(l, w, h), calculate_ribbon_size(l, w, h))
        })
        .fold((0, 0), |(acc1, acc2), (paper, ribbon)| (acc1 + paper, acc2 + ribbon));

    result
}

fn calculate_paper_size(l: i32, w: i32, h: i32) -> i32 {
    let lw = l * w;
    let wh = w * h;
    let hl = h * l;
    let min = lw.min(wh).min(hl);
    2 * lw + 2 * wh + 2 * hl + min
}

fn calculate_ribbon_size(l: i32, w: i32, h: i32) -> i32 {
    let min1 = l.min(w).min(h);
    let min2 = match min1 {
        s if s == l => w.min(h),
        s if s == w => l.min(h),
        s if s == h => l.min(w),
        _ => panic!("Impossible"),
    };

    2 * min1 + 2 * min2 + l * w * h
}

#[test]
fn example() {
    let (p1, p2) = run("2x3x4");
    assert_eq!(p1, 58);
    assert_eq!(p2, 34); 
}

#[test]
fn real_input() {
    let input = fs::read_to_string("inputs/2015/day2.txt").unwrap();
    let (p1, p2) = run(&input);
    assert_eq!(p1, 1586300);
    assert_eq!(p2, 3737498);
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    bencher.with_inputs(|| {
        fs::read_to_string("inputs/2015/day2.txt").unwrap()
    }).bench_refs(|s| {
        run(s);
    });
}
