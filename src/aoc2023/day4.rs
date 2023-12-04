pub fn run(input: &str) -> (u32, u32) {
    let card_matches = input
        .lines()
        .map(|line| {
            let mut cards = line.split([':', '|']).map(|l| l.split_whitespace());
            let winning_cards = cards.nth(1).unwrap().collect::<Vec<_>>();
            cards.next().unwrap().filter(|card| winning_cards.contains(card)).count() as u32
        })
        .collect::<Vec<_>>();

    let p1 = p1_points(&card_matches);
    let p2 = p2_card_total(&card_matches);
    (p1, p2)
}

fn p1_points(matches_per_card: &[u32]) -> u32 {
    matches_per_card
        .iter()
        .filter(|m| **m > 0)
        .fold(0, |acc, matches| acc + 2u32.pow(*matches - 1))
}

fn p2_card_total(matches_per_card: &[u32]) -> u32 {
    let mut number_of_copies_by_card = vec![1; matches_per_card.len()];
    for i in 0..matches_per_card.len() {
        let matches = matches_per_card[i];
        for j in 1..=matches as usize {
            number_of_copies_by_card[i + j] += number_of_copies_by_card[i];
        }
    }
    number_of_copies_by_card.iter().sum()
}

#[test]
fn example() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    assert_eq!(run(input), (13, 30));
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("inputs/2023/day4.txt").unwrap();
    assert_eq!(run(&input), (32001, 5037841));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("inputs/2023/day4.txt").unwrap();
    bencher.bench(|| run(divan::black_box(&input)));
}
