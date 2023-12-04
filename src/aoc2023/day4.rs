pub fn run(input: &str) -> (u32, u32) {
    let wins_per_card = input
        .lines()
        .map(|line| {
            let mut cards = line.split([':', '|']).map(|l| l.split_whitespace());
            let winning_cards = cards.nth(1).unwrap().collect::<Vec<_>>();
            cards.next().unwrap().filter(|card| winning_cards.contains(card)).count() as u32
        })
        .collect::<Vec<_>>();

    let mut number_of_copies_by_card = vec![1; wins_per_card.len()];
    let mut points = 0;
    for i in 0..wins_per_card.len() {
        let wins = wins_per_card[i];
        if wins > 0 {
            points += 2u32.pow(wins - 1);
            update_copy_counts(wins, &mut number_of_copies_by_card, i);
        }
    }
    (points, number_of_copies_by_card.iter().sum())
}

fn update_copy_counts(wins: u32, number_of_copies_by_card: &mut Vec<u32>, i: usize) {
    for j in 1..=wins as usize {
        number_of_copies_by_card[i + j] += number_of_copies_by_card[i];
    }
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
