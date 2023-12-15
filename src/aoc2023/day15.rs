use itertools::Itertools;

pub fn run(input: &str) -> (usize, usize) {
    let steps = input.trim().as_bytes().split(|&c| c == b',').collect_vec();
    let p1 = steps.iter().map(|s| get_hash(s)).sum();
    let p2 = follow_steps(&steps);
    (p1, p2)
}

fn get_hash(step: &[u8]) -> usize {
    step.iter().fold(0, |a, &c| (a + c as usize) * 17 % 256)
}

fn follow_steps(steps: &[&[u8]]) -> usize {
    const EMPTY: Vec<(&[u8], u8)> = Vec::new();
    let mut boxes = [EMPTY; 256];
    for &step in steps {
        if step.contains(&b'=') {
            let (label, focal_length) = step.split_at(step.len() - 2);
            let focal_length = focal_length.last().unwrap();
            let box_nr = get_hash(label);
            let lbox = &mut boxes[box_nr];
            if let Some((_, current_focal_length)) = lbox.iter_mut().find(|(l, _)| l == &label) {
                *current_focal_length = focal_length - b'0';
            } else {
                lbox.push((label, *focal_length - b'0'));
            }
        } else if step.contains(&b'-') {
            let label = &step[..step.len() - 1];
            let box_nr = get_hash(label);
            if let Some(index) = boxes[box_nr].iter().position(|(l, _)| l == &label) {
                boxes[box_nr].remove(index);
            }
        }
    }

    let mut result = 0;
    for (i, lbox) in boxes.iter().enumerate() {
        for (j, (_, focal_length)) in lbox.iter().enumerate() {
            result += (i + 1) * (j + 1) * *focal_length as usize
        }
    }
    result
}

#[test]
fn example() {
    let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    assert_eq!(run(input), (1320, 145));
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("inputs/2023/day15.txt").unwrap();
    assert_eq!(run(&input), (497373, 259356));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("inputs/2023/day15.txt").unwrap();
    bencher.bench(|| run(divan::black_box(&input)));
}
