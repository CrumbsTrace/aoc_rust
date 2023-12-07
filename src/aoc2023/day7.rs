use itertools::Itertools;

const RANK: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

const RANK_P2: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

#[derive(Debug, Eq)]
struct Hand {
    hand: [char; 5],
    p2_hand: [char; 5],
    hand_counts: Vec<(char, usize)>,
    p2_hand_counts: Vec<(char, usize)>,
    bid: u32,
    p2: bool,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let rank = if self.p2 { &RANK_P2 } else { &RANK };
        let same_type_cmp = || {
            for i in 0..5 {
                if self.hand[i] != other.hand[i] {
                    return rank
                        .iter()
                        .position(|&c| c == other.hand[i])
                        .unwrap()
                        .cmp(&rank.iter().position(|&c| c == self.hand[i]).unwrap());
                }
            }
            std::cmp::Ordering::Equal
        };
        let mut ours = self.hand;
        let mut theirs = other.hand;
        let mut our_counts = &self.hand_counts;
        let mut their_counts = &other.hand_counts;
        if self.p2 {
            ours = self.p2_hand;
            theirs = other.p2_hand;
            our_counts = &self.p2_hand_counts;
            their_counts = &other.p2_hand_counts;
        }

        if our_counts[0].1 > 3 || their_counts[0].1 > 3 {
            match our_counts[0].1.cmp(&their_counts[0].1) {
                std::cmp::Ordering::Equal => same_type_cmp(),
                o => o,
            }
        } else if is_full_house(ours) || is_full_house(theirs) {
            match is_full_house(ours).cmp(&is_full_house(theirs)) {
                std::cmp::Ordering::Equal => same_type_cmp(),
                o => o,
            }
        } else if our_counts[0].1 == 3 || their_counts[0].1 == 3 {
            match our_counts[0].1.cmp(&their_counts[0].1) {
                std::cmp::Ordering::Equal => same_type_cmp(),
                o => o,
            }
        } else if (our_counts[1].1 == 2) || (their_counts[1].1 == 2) {
            match (our_counts[1].1 == 2).cmp(&(their_counts[1].1 == 2)) {
                std::cmp::Ordering::Equal => same_type_cmp(),
                o => o,
            }
        } else if (our_counts[0].1 == 2) || (their_counts[0].1 == 2) {
            match (our_counts[0].1 == 2).cmp(&(their_counts[0].1 == 2)) {
                std::cmp::Ordering::Equal => same_type_cmp(),
                o => o,
            }
        } else {
            same_type_cmp()
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn run(input: &str) -> (u32, u32) {
    let mut hands = input
        .lines()
        .map(|l| {
            let (hand, bid) = l.split(' ').collect_tuple().unwrap();
            let bid = bid.parse::<u32>().unwrap();
            let hand: [char; 5] = hand.chars().collect_vec().try_into().unwrap();
            let mut hand_counts = hand.into_iter().counts().into_iter().collect_vec();
            hand_counts.sort_by_key(|(_, v)| usize::MAX - *v);
            let mut p2_hand = hand;
            update_hand(&mut p2_hand);
            let mut p2_hand_counts = p2_hand.into_iter().counts().into_iter().collect_vec();
            p2_hand_counts.sort_by_key(|(_, v)| usize::MAX - *v);
            Hand {
                hand,
                p2_hand,
                hand_counts,
                p2_hand_counts,
                bid,
                p2: false,
            }
        })
        .collect_vec();

    hands.sort();
    let p1 = hands
        .iter()
        .enumerate()
        .map(|(i, h)| h.bid * (i as u32 + 1))
        .sum();
    hands.iter_mut().for_each(|h| h.p2 = true);
    hands.sort();
    let p2 = hands
        .iter()
        .enumerate()
        .map(|(i, h)| h.bid * (i as u32 + 1))
        .sum();
    (p1, p2)
}

pub fn is_full_house(hand: [char; 5]) -> bool {
    let mut hand = hand.to_vec();
    hand.sort();
    hand[0] == hand[1] && hand[3] == hand[4] && (hand[2] == hand[1] || hand[2] == hand[3])
}

fn update_hand(ours: &mut [char; 5]) {
    let mut counts = ours
        .iter()
        .filter(|&&c| c != 'J')
        .counts()
        .into_iter()
        .collect_vec();
    counts.sort_by_key(|(_, v)| *v);
    let j = counts.last().unwrap_or(&(&'J', 0)).0;
    *ours = ours
        .iter()
        .map(|&c| if c == 'J' { *j } else { c })
        .collect_vec()
        .try_into()
        .unwrap();
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
