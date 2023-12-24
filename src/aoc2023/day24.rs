use itertools::Itertools;

const C_MIN: i64 = 200000000000000;
const C_MAX: i64 = 400000000000000;

#[derive(Debug, Copy, Clone)]
struct Vector {
    pos: (f64, f64, f64),
    vel: (f64, f64, f64),
}

pub fn run(input: &str) -> (u32, i64) {
    let vectors = input
        .lines()
        .map(|line| {
            let split = line
                .split([',', ' '])
                .filter_map(|s| s.parse().ok())
                .collect_vec();
            Vector {
                pos: (split[0], split[1], split[2]),
                vel: (split[3], split[4], split[5]),
            }
        })
        .collect_vec();

    let mut p1 = 0;
    for i in 0..vectors.len() {
        for j in i + 1..vectors.len() {
            if let Some(pos) = find_intersection(&vectors[i], &vectors[j], (0., 0.)) {
                if pos.0 >= C_MIN && pos.0 <= C_MAX && pos.1 >= C_MIN && pos.1 <= C_MAX {
                    p1 += 1;
                }
            }
        }
    }

    let rock_pos = find_rock(&vectors);
    let p2 = rock_pos.0 + rock_pos.1 + rock_pos.2;
    (p1, p2)
}

//Just use trial and error
fn find_rock(vectors: &[Vector]) -> (i64, i64, i64) {
    let mut n = 0;
    loop {
        for x in 0..n {
            let y = n - x;
            for neg_x in [-1, 1] {
                'searchloop: for neg_y in [-1, 1] {
                    let mut known_pos = None;
                    let dx = neg_x * x;
                    let dy = neg_y * y;
                    let i = 0;
                    for j in 1..vectors.len() {
                        if let Some(pos) =
                            find_intersection(&vectors[i], &vectors[j], (dx as f64, dy as f64))
                        {
                            if known_pos.is_none() || known_pos.unwrap() == pos {
                                known_pos = Some(pos);
                            } else {
                                continue 'searchloop;
                            }
                        } else {
                            continue 'searchloop;
                        }
                    }
                    if known_pos.is_some() {
                        let known_pos = known_pos.unwrap();
                        let known_pos_f64 = (known_pos.0 as f64, known_pos.1 as f64);
                        let mut v1 = vectors[0];
                        v1.vel.0 -= dx as f64;
                        v1.vel.1 -= dy as f64;
                        let mut v2 = vectors[1];
                        v2.vel.0 -= dx as f64;
                        v2.vel.1 -= dy as f64;
                        if let Some(dz) = find_z(&v1, &v2, known_pos_f64) {
                            let z = v1.pos.2 + find_t(&v1, known_pos_f64) * (v1.vel.2 - dz);
                            return (known_pos.0, known_pos.1, z.round() as i64);
                        }
                    }
                }
            }
        }
        n += 1;
    }
}

fn find_intersection(a: &Vector, b: &Vector, dv: (f64, f64)) -> Option<(i64, i64)> {
    let (x1, y1, _) = a.pos;
    let (x2, y2, _) = b.pos;
    let (dx1, dy1) = (a.vel.0 - dv.0, a.vel.1 - dv.1);
    let (dx2, dy2) = (b.vel.0 - dv.0, b.vel.1 - dv.1);
    let t = (dx2 * (y1 - y2) - dy2 * (x1 - x2)) / (dx1 * dy2 - dy1 * dx2);
    let s = (dx1 * (y1 - y2) - dy1 * (x1 - x2)) / (dx1 * dy2 - dy1 * dx2);
    if t >= 0. && s >= 0. {
        let x = x1 + t * dx1;
        let y = y1 + t * dy1;
        if x.is_infinite() || y.is_infinite() {
            return None;
        }
        //Deal with floating point errors
        let x = x.round() as i64;
        let y = y.round() as i64;
        Some((x, y))
    } else {
        None
    }
}

fn find_t(a: &Vector, p: (f64, f64)) -> f64 {
    if a.vel.1 == 0. {
        (p.0 - a.pos.0) / a.vel.0
    } else {
        (p.1 - a.pos.1) / a.vel.1
    }
}

fn find_z(a: &Vector, b: &Vector, p: (f64, f64)) -> Option<f64> {
    let t1 = find_t(a, p);
    let t2 = find_t(b, p);
    if t1 == t2 {
        None
    } else {
        Some((a.pos.2 - b.pos.2 + t1 * a.vel.2 - t2 * b.vel.2) / (t1 - t2))
    }
}

#[test]
fn example() {
    let input = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
    assert_eq!(run(input), (0, 47));
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("inputs/2023/day24.txt").unwrap();
    assert_eq!(run(&input), (15262, 6498));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("inputs/2023/day24.txt").unwrap();
    bencher.bench(|| run(divan::black_box(&input)));
}
