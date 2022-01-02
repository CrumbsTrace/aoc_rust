use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/day6.txt").unwrap();
    let mut fish: [u64; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];

    input
        .replace('\n', "")
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .for_each(|n| {
            fish[n] += 1;
        });

    let mut p1 = 0;
    for i in 0..256 {
        fish = update_fish(fish);
        if i == 79 {
            p1 = fish.iter().sum();
        }
    }
    let p2: u64 = fish.iter().sum();

    // println!("Part 1: {}", p1);
    // println!("Part 2: {}", p2);
    assert_eq!(p1, 386536);
    assert_eq!(p2, 1732821262171);
}

fn update_fish(fish: [u64; 9]) -> [u64; 9] {
    let [f0, f1, f2, f3, f4, f5, f6, f7, f8] = fish;
    [f1, f2, f3, f4, f5, f6, f7 + f0, f8, f0]
}
