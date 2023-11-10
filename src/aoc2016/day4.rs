pub fn run(input: &str) -> (i32, i32) {
    let mut real_rooms = 0;
    let mut northpole_sector_id = 0;
    input.lines().for_each(|line| {
        let mut parts = line
            .split(|c| c == '-' || c == '[' || c == ']')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();
        let checksum = parts.pop().unwrap();
        let sector_id = parts.pop().unwrap().parse::<i32>().unwrap();
        let name = parts.join("");
        let mut counts = [0; 26];
        for c in name.chars() {
            counts[c as usize - 'a' as usize] += 1;
        }
        let mut counts = counts.iter().enumerate().collect::<Vec<_>>();
        counts.sort_by_key(|(_, &c)| -c);
        let most_common = counts
            .iter()
            .take(5)
            .map(|(i, _)| (*i as u8 + 'a' as u8) as char)
            .collect::<String>();

        if most_common == checksum {
            real_rooms += sector_id;

            let mut real_name = String::new();
            for c in name.chars() {
                if c == '-' {
                    real_name.push(' ');
                } else {
                    let c = c as u8 - 'a' as u8;
                    let c = ((c as i32 + sector_id) % 26) as u8 + 'a' as u8;
                    real_name.push(c as char);
                }
            }
            if real_name == "northpoleobjectstorage" {
                northpole_sector_id = sector_id;
            }
        }
    });

    (real_rooms, northpole_sector_id)
}

#[test]
fn example() {
    let input = "aaaaa-bbb-z-y-x-123[abxyz]\n\
                 a-b-c-d-e-f-g-h-987[abcde]\n\
                 not-a-real-room-404[oarel]\n\
                 totally-real-room-200[decoy]";
    assert_eq!(run(input), (1514, 0));
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("inputs/2016/day4.txt").unwrap();
    assert_eq!(run(&input), (278221, 267));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("inputs/2016/day4.txt").unwrap();
    bencher.bench(|| run(&input));
}
