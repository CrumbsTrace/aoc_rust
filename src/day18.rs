use std::fs;

pub fn run() {
    let snailfishes: Vec<_> = fs::read_to_string("inputs/day18.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let (_, snailfish) = parse_snailfish(line);
            snailfish
        })
        .collect();

    dbg!(snailfishes);
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

#[derive(Debug)]
enum SnailFish {
    Fish(i32),
    List(Box<SnailFish>, Box<SnailFish>),
}
