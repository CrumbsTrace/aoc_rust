use itertools::Itertools;

const RANK: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

const RANK_P2: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

#[derive(Debug, Ord, Eq)]
struct Hand {
    hand: [char; 5],
    bid: u32,
    p2: bool,
}

pub fn n_of_kind(hand: [char; 5]) -> usize {
    *hand.iter().counts().values().max().unwrap()
}

pub fn is_full_house(hand: [char; 5]) -> bool {
    let mut hand = hand.to_vec();
    hand.sort();
    hand[0] == hand[1] && hand[3] == hand[4] && (hand[2] == hand[1] || hand[2] == hand[3])
}

pub fn is_two_pair(hand: [char; 5]) -> bool {
    hand
        .iter()
        .counts()
        .values()
        .filter(|v| v == &&2)
        .count()
        == 2
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let mut same_type_cmp = None;
        let rank = if self.p2 {
            &RANK_P2
        } else {
            &RANK
        };
        for i in 0..5 {
            if self.hand[i] != other.hand[i] {
                same_type_cmp = rank
                    .iter()
                    .position(|&c| c == other.hand[i])
                    .unwrap()
                    .partial_cmp(&rank.iter().position(|&c| c == self.hand[i]).unwrap());
                break;
            }
        }

        let mut ours = self.hand.clone();
        let mut theirs = other.hand.clone();
        if self.p2 {
            update_hand(&mut ours);
            update_hand(&mut theirs);
        }

        let self_n = n_of_kind(ours);
        let other_n = n_of_kind(theirs);
        if self_n > 3 || other_n > 3 {
            match self_n.cmp(&other_n) {
                std::cmp::Ordering::Equal => return same_type_cmp,
                o => return Some(o),
            }
        }

        let self_is_full_house = is_full_house(ours);
        let other_is_full_house = is_full_house(theirs);
        if self_is_full_house || other_is_full_house {
            match self_is_full_house.cmp(&other_is_full_house) {
                std::cmp::Ordering::Equal => return same_type_cmp,
                o => return Some(o),
            }
        }

        if self_n == 3 || other_n == 3 {
            match self_n.cmp(&other_n) {
                std::cmp::Ordering::Equal => return same_type_cmp,
                o => return Some(o),
            }
        }

        let self_two_pair = is_two_pair(ours);
        let other_two_pair = is_two_pair(theirs);
        if self_two_pair || other_two_pair {
            match self_two_pair.cmp(&other_two_pair) {
                std::cmp::Ordering::Equal => return same_type_cmp,
                o => return Some(o),
            }
        }

        let self_is_pair = self_n == 2;
        let other_is_pair = other_n == 2;
        if self_is_pair || other_is_pair {
            match self_is_pair.cmp(&other_is_pair) {
                std::cmp::Ordering::Equal => return same_type_cmp,
                o => return Some(o),
            }
        }

        same_type_cmp
    }
}

fn update_hand(ours: &mut [char; 5]) {
    let mut counts = ours.iter().filter(| &&c| c != 'J').counts().into_iter().collect_vec();
    counts.sort_by_key(|(_, v)| *v);
    let j = counts.last().unwrap_or(&(&'J', 0)).0;
    *ours = ours
        .iter()
        .map(|&c| if c == 'J' { *j } else { c })
        .collect_vec()
        .try_into()
        .unwrap();
}

pub fn run(input: &str) -> (u32, u32) {
    let mut hands = input
        .lines()
        .map(|l| {
            let (hand, bid) = l.split(" ").collect_tuple().unwrap();
            let bid = bid.parse::<u32>().unwrap();
            Hand {
                hand: hand.chars().collect_vec().try_into().unwrap(),
                bid,
                p2: false,
            }
        })
        .collect_vec();

    hands.sort();
    let p1 = hands.iter().enumerate().map(|(i, h)| h.bid * (i as u32 + 1)).sum();

    hands.iter_mut().for_each(|h| h.p2 = true);
    hands.sort();
    let p2 = hands.iter().enumerate().map(|(i, h)| h.bid * (i as u32 + 1)).sum();
    (p1, p2)
}

#[test]
fn example() {
    let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    assert_eq!(run(input), (6440, 5905));
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("inputs/2023/day7.txt").unwrap();
    assert_eq!(run(&input), (251927063, 255632664));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("inputs/2023/day7.txt").unwrap();
    bencher.bench(|| run(divan::black_box(&input)));
}
