use divan::black_box;
use std::fs;

trait Keypad {
    fn new() -> Self;
    fn next(&mut self, direction: char);
    fn current(&self) -> char;
}

struct Keypad1 {
    position: (i32, i32),
}

// 1 2 3
// 4 5 6
// 7 8 9
impl Keypad for Keypad1 {
    fn new() -> Self {
        Self { position: (1, 1) }
    }

    fn next(&mut self, direction: char) {
        match direction {
            'U' => self.position.1 -= 1,
            'D' => self.position.1 += 1,
            'L' => self.position.0 -= 1,
            'R' => self.position.0 += 1,
            _ => panic!("Invalid direction"),
        }
        self.position.0 = self.position.0.max(0).min(2);
        self.position.1 = self.position.1.max(0).min(2);
    }
    fn current(&self) -> char {
        let key = self.position.1 * 3 + self.position.0 + 1;
        (key + '0' as i32) as u8 as char
    }
}

struct Keypad2 {
    position: (i32, i32),
}

//     1
//   2 3 4
// 5 6 7 8 9
//   A B C
//     D
impl Keypad for Keypad2 {
    fn new() -> Self {
        Self { position: (-2, 0) }
    }
    fn next(&mut self, direction: char) {
        let (x, y) = self.position;
        match direction {
            'U' if x.abs() + (y - 1).abs() <= 2 => {
                self.position.1 -= 1;
            },
            'D' if x.abs() + (y + 1).abs() <= 2 => {
                self.position.1 += 1;
            },
            'L' if (x - 1).abs() + y.abs() <= 2 => {
                self.position.0 -= 1;
            },
            'R' if (x + 1).abs() + y.abs() <= 2 => {
                self.position.0 += 1;
            },
            _ => ()
        }
    }
    fn current(&self) -> char {
        match self.position {
            (0, -2) => '1',
            (-1, -1) => '2',
            (0, -1) => '3',
            (1, -1) => '4',
            (-2, 0) => '5',
            (-1, 0) => '6',
            (0, 0) => '7',
            (1, 0) => '8',
            (2, 0) => '9',
            (-1, 1) => 'A',
            (0, 1) => 'B',
            (1, 1) => 'C',
            (0, 2) => 'D',
            _ => panic!("Invalid position"),
        }
    }
}

pub fn run(input: &str) -> (String, String) {
    let mut keypad1 = Keypad1::new();
    let mut keypad2 = Keypad2::new();
    let mut code1 = String::new();
    let mut code2 = String::new();
    for line in input.lines() {
        for direction in line.chars() {
            keypad1.next(direction);
            keypad2.next(direction);
        }
        code1.push(keypad1.current());
        code2.push(keypad2.current());
    }
    (code1, code2)
}

#[test]
fn example() {
    let input = "ULL\nRRDDD\nLURDL\nUUUUD";
    assert_eq!(run(input), ("1985".to_owned(), "5DB3".to_owned()));
}

#[test]
fn real_input() {
    let input = fs::read_to_string("inputs/2016/day2.txt").unwrap();
    assert_eq!(run(&input), ("14894".to_owned(), "26B96".to_owned()));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = fs::read_to_string("inputs/2016/day2.txt").unwrap();
    bencher.bench(|| run(black_box(&input)));
}
