use itertools::Itertools;
use std::{
    fmt::{self, Debug},
    fs,
};

pub fn run() {
    p1();
    p2();
}

fn p1() {
    let result: Box<SnailFish> = fs::read_to_string("inputs/day18.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let (_, snailfish) = parse_snailfish(line);
            Box::new(snailfish)
        })
        .reduce(|left, right| {
            let mut snailfish = Box::new(SnailFish::List(left, right));
            evaluate(&mut snailfish);
            snailfish
        })
        .unwrap();
    assert_eq!(magnitude(&result), 3359);
}

fn p2() {
    let result: i32 = fs::read_to_string("inputs/day18.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let (_, snailfish) = parse_snailfish(line);
            Box::new(snailfish)
        })
        .permutations(2)
        .map(|perm| {
            let r = perm.into_iter().reduce(|left, right| {
                let mut snailfish = Box::new(SnailFish::List(left, right));
                evaluate(&mut snailfish);
                snailfish
            }).unwrap();
            magnitude(&r)
        })
        .max()
        .unwrap();

    assert_eq!(result, 4616)
}

fn evaluate(snailfish: &mut Box<SnailFish>) {
    let mut something_happened = true;

    while something_happened {
        let action = explode(snailfish, 0);
        something_happened = action != ExplodeAction::Nothing;
        if !something_happened {
            something_happened = split(snailfish)
        }
    }
}

fn explode(snailfish: &mut Box<SnailFish>, depth: i32) -> ExplodeAction {
    match snailfish.as_mut() {
        SnailFish::Fish(_) => ExplodeAction::Nothing,
        s if depth == 4 => {
            let (v1, v2) = match s {
                SnailFish::List(l, r) => {
                    let v1 = match l.as_ref() {
                        SnailFish::Fish(value) => *value,
                        _ => 0,
                    };
                    let v2 = match r.as_ref() {
                        SnailFish::Fish(value) => *value,
                        _ => 0,
                    };
                    (v1, v2)
                }
                _ => unreachable!(),
            };

            *s = SnailFish::Fish(0);
            ExplodeAction::Explode(v1, v2)
        }
        SnailFish::List(left, right) => {
            let action = explode(left, depth + 1);

            match action {
                ExplodeAction::Explode(v1, v2) => {
                    if v2 > 0 {
                        add_to_left_most(right, v2)
                    }
                    ExplodeAction::Explode(v1, 0)
                }
                ExplodeAction::Nothing => {
                    let action = explode(right, depth + 1);

                    match action {
                        ExplodeAction::Explode(v1, v2) => {
                            if v1 > 0 {
                                add_to_right_most(left, v1)
                            }
                            ExplodeAction::Explode(0, v2)
                        }
                        ExplodeAction::Nothing => ExplodeAction::Nothing,
                    }
                }
            }
        }
    }
}

fn add_to_left_most(snailfish: &mut Box<SnailFish>, value: i32) {
    match snailfish.as_mut() {
        SnailFish::List(l, _) => {
            add_to_left_most(l, value);
        }
        SnailFish::Fish(v) => *v += value,
    }
}

fn add_to_right_most(snailfish: &mut Box<SnailFish>, value: i32) {
    match snailfish.as_mut() {
        SnailFish::List(_, r) => {
            add_to_right_most(r, value);
        }
        SnailFish::Fish(v) => *v += value,
    }
}

fn split(snailfish: &mut Box<SnailFish>) -> bool {
    match snailfish.as_mut() {
        SnailFish::List(l, r) => split(l) || split(r),
        SnailFish::Fish(n) if *n < 10 => false,
        fish => {
            if let SnailFish::Fish(n) = fish {
                *fish = SnailFish::List(
                    Box::new(SnailFish::Fish(*n / 2)),
                    Box::new(SnailFish::Fish(*n - (*n / 2))),
                );
            }
            true
        }
    }
}

fn magnitude(snailfish: &SnailFish) -> i32 {
    match snailfish {
        SnailFish::Fish(n) => *n,
        SnailFish::List(l, r) => 3 * magnitude(l) + 2 * magnitude(r),
    }
}

fn parse_snailfish(input: &str) -> (&str, SnailFish) {
    if input.starts_with('[') {
        let (rem_input, left) = parse_snailfish(input.strip_prefix('[').unwrap());
        let (rem_input, right) = parse_snailfish(&rem_input[1..]);
        (
            &rem_input[1..],
            SnailFish::List(Box::new(left), Box::new(right)),
        )
    } else {
        let comma_index = input.find([',', ']']).unwrap();
        let number = input[..comma_index].parse().unwrap();
        (&input[comma_index..], SnailFish::Fish(number))
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ExplodeAction {
    Explode(i32, i32),
    Nothing,
}

#[derive(PartialEq, Eq, Clone)]
enum SnailFish {
    Fish(i32),
    List(Box<SnailFish>, Box<SnailFish>),
}

impl fmt::Debug for SnailFish {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SnailFish::Fish(n) => write!(f, "{}", n),
            SnailFish::List(l, r) => write!(f, "[{:?},{:?}]", l, r),
        }
    }
}
