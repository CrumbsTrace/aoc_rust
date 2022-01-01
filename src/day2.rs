use std::fs;
pub fn run() {
    let (p1, p2) = follow_instructions();
    // println!("Part 1: {:?}", p1);
    // println!("Part 2: {:?}", p2);
    assert_eq!(p1, 1488669);
    assert_eq!(p2, 1176514794);
}

fn follow_instructions() -> (u32, u32) {
    let mut pos = 0;
    let mut aim_depth = 0;
    let mut real_depth = 0;

    fs::read_to_string("inputs/day2.txt")
        .unwrap()
        .lines()
        .map(|x| x.split(' ').collect::<Vec<&str>>())
        .for_each(|instr| {
            let n = instr[1].parse::<u32>().unwrap();
            match instr[0] {
                "forward" => {
                    pos += n;
                    real_depth += aim_depth * n
                }
                "down" => aim_depth += n,
                _ => aim_depth -= n,
            }
        });

    (pos * aim_depth, pos * real_depth)
}
