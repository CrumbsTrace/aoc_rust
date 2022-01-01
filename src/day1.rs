use std::fs;

pub fn run() {
    let lines: Vec<i32> = fs::read_to_string("inputs/day1.txt")
        .unwrap()
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut last_n = i32::max_value();
    let mut p1_count = 0;
    let mut p2_count = 0;
    for i in 0..lines.len() {
        let n = lines[i];
        if i > 2 && lines[i - 3] < n {
            p2_count += 1
        }

        if n > last_n {
            p1_count += 1;
        }

        last_n = n
    }

    // println!("Part 1: {:?}", p1_count);
    // println!("Part 2: {:?}", p2_count);
    assert!(p1_count == 1154);
    assert!(p2_count == 1127);
}
