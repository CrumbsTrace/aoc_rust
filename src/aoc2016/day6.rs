pub fn run(input: &str) -> (String, String) {
    let mut letter_frequencies = [[0; 26]; 8];
    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            letter_frequencies[i][c as usize - 'a' as usize] += 1;
        }
    }
    let mut p1 = String::new();
    let mut p2 = String::new();
    for freqs in letter_frequencies.iter() {
        p1.push(
            (b'a'
                + freqs
                    .iter()
                    .enumerate()
                    .max_by_key(|(_, &freq)| freq)
                    .unwrap()
                    .0 as u8) as char,
        );
        p2.push(
            (b'a'
                + freqs
                    .iter()
                    .enumerate()
                    .filter(|(_, &freq)| freq > 0)
                    .min_by_key(|(_, &freq)| freq)
                    .unwrap_or((0, &0))
                    .0 as u8) as char,
        );
    }
    (p1, p2)
}

#[test]
fn test() {
    let input = "eedadn\n\
                 drvtee\n\
                 eandsr\n\
                 raavrd\n\
                 atevrs\n\
                 tsrnev\n\
                 sdttsa\n\
                 rasrtv\n\
                 nssdts\n\
                 ntnada\n\
                 svetve\n\
                 tesnvt\n\
                 vntsnd\n\
                 vrdear\n\
                 dvrsen\n\
                 enarar";
    assert_eq!(run(input), ("easterzz".to_owned(), "adventaa".to_owned()));
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("inputs/2016/day6.txt").unwrap();
    assert_eq!(run(&input), ("tkspfjcc".to_owned(), "xrlmbypn".to_owned()));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("inputs/2016/day6.txt").unwrap();
    bencher.bench(|| run(&input));
}
