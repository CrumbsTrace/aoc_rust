pub fn run(input: &str) -> (i64, i64) {
    let lines = input.lines().collect::<Vec<_>>();
    let mut galaxies = Vec::new();
    let mut empty_rows = vec![true; lines.len()];
    let mut empty_cols = vec![true; lines[0].len()];
    for (y, row) in lines.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            if c == '#' {
                galaxies.push((x as i64, y as i64));
                empty_rows[y] = false;
                empty_cols[x] = false;
            }
        }
    }

    let mut p1 = 0;
    let mut p2 = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let (x1, y1) = galaxies[i];
            let (x2, y2) = galaxies[j];
            let empty_cols = (x1.min(x2)..x2.max(x1))
                .filter(|x| empty_cols[*x as usize])
                .count() as i64;
            let empty_rows = (y1.min(y2)..y2.max(y1))
                .filter(|y| empty_rows[*y as usize])
                .count() as i64;
            let empty_count = empty_cols + empty_rows;
            let base_distance = (x1 - x2).abs() + (y1 - y2).abs();
            p1 += base_distance + empty_count;
            p2 += base_distance + empty_count * 999_999;
        }
    }
    (p1, p2)
}

#[test]
fn example() {
    let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    assert_eq!(run(input), (374, 82000210));
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("inputs/2023/day11.txt").unwrap();
    assert_eq!(run(&input), (9599070, 842645913794));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("inputs/2023/day11.txt").unwrap();
    bencher.bench(|| run(divan::black_box(&input)));
}
