use ndarray::Array2;
use std::fs;
use divan::black_box;

pub fn run(input: &str, steps: i32) -> (usize, usize) {
    let mut lights = parse_lights(input);
    let mut lights_p2 = lights.clone();
    for _ in 0..steps {
        step(&mut lights, false);
    }
    let p1 = lights.iter().filter(|&&l| l).count();

    let (width, height) = lights.dim();
    lights_p2[[0, 0]] = true;
    lights_p2[[0, height - 1]] = true;
    lights_p2[[width - 1, 0]] = true;
    lights_p2[[width - 1, height - 1]] = true;

    for _ in 0..steps {
        step(&mut lights_p2, true);
    }
    let p2 = lights_p2.iter().filter(|&&l| l).count();
    (p1, p2)
}

fn step(lights: &mut Array2<bool>, stuck_corners: bool) {
    let mut next_lights = lights.clone();
    let (width, height) = lights.dim();
    let (width_i32, height_i32) = (width as i32, height as i32);
    for ((x, y), on) in lights.indexed_iter() {
        if stuck_corners && (x == 0 || x == width - 1) && (y == 0 || y == height - 1) {
            continue;
        }
        let mut neighbors_on = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let n_x = x as i32 + dx;
                let n_y = y as i32 + dy;
                if n_x < 0 || n_x >= width_i32 || n_y < 0 || n_y >= height_i32 {
                    continue;
                }

                if lights[[n_x as usize, n_y as usize]] {
                    neighbors_on += 1;
                }
            }
        }
        if *on {
            next_lights[[x, y]] = neighbors_on == 2 || neighbors_on == 3;
        }
        else {
            next_lights[[x, y]] = neighbors_on == 3;
        }
    }
    *lights = next_lights;
}

fn parse_lights(input: &str) -> Array2<bool> {
    let mut lights = Array2::from_elem((input.lines().next().unwrap().len(), input.lines().count()), false);
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            lights[[x, y]] = match c {
                '#' => true,
                '.' => false,
                _ => panic!("unexpected char"),
            }
        }
    }
    lights
}

#[test]
fn example() {
    let input = ".#.#.#\n...##.\n#....#\n..#...\n#.#..#\n####..";
    let (result, _) = run(&input, 4);
    assert_eq!(result, 4);
    let (_, result) = run(&input, 5);
    assert_eq!(result, 17);
}

#[test]
fn real_input() {
    let input = fs::read_to_string("inputs/2015/day18.txt").unwrap();
    let result = run(&input, 100);
    assert_eq!(result, (768, 781))
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = fs::read_to_string("inputs/2015/day18.txt").unwrap();
    bencher.bench(|| run(black_box(&input), 150));
}
