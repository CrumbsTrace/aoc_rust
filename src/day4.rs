use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/day4.txt").unwrap();
    let lines = input.lines().collect::<Vec<&str>>();
    let numbers = lines[0]
        .split(',')
        .map(|n| n.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();

    let mut bingo_cardss = lines[1..].chunks(6).map(Bingo::new).collect::<Vec<Bingo>>();

    let mut winning_score = 0;
    let mut last_winning_score = 0;
    for n in numbers {
        let cards_left = bingo_cardss.len();
        for card in &mut bingo_cardss {
            card.mark(n);
            if winning_score == 0 && card.won {
                winning_score = card.unmarked_sum() * n as u32;
            }
            if card.won && cards_left == 1 {
                last_winning_score = card.unmarked_sum() * n as u32;
            }
        }
        bingo_cardss = bingo_cardss.into_iter().filter(|c| !c.won).collect();
    }
    // println!("Part 1: {}", winning_score);
    // println!("Part 2: {}", last_winning_score);
    assert_eq!(winning_score, 89001);
    assert_eq!(last_winning_score, 7296);
}

struct Bingo {
    squares: [[u8; 5]; 5],
    row_visits: [u8; 5],
    column_visits: [u8; 5],
    visited: Vec<u8>,
    won: bool,
}

impl Bingo {
    pub fn new(bingo_chunk: &[&str]) -> Self {
        Bingo {
            squares: Bingo::create_squares(bingo_chunk),
            row_visits: [0, 0, 0, 0, 0],
            column_visits: [0, 0, 0, 0, 0],
            visited: vec![],
            won: false,
        }
    }

    pub fn mark(&mut self, n: u8) {
        for x in 0..5 {
            for y in 0..5 {
                if self.squares[x][y] == n {
                    self.row_visits[y] += 1;
                    self.column_visits[x] += 1;
                    self.visited.push(n);
                    self.won = self.row_visits[y] == 5 || self.column_visits[x] == 5;
                    break;
                }
            }
        }
    }

    pub fn unmarked_sum(&self) -> u32 {
        self.squares
            .concat()
            .into_iter()
            .filter(|c| !self.visited.contains(c))
            .map(|c| c as u32)
            .sum()
    }

    fn create_squares(bingo_chunk: &[&str]) -> [[u8; 5]; 5] {
        let mut squares = [
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
        ];

        for i in 0..5 {
            let line: Vec<u8> = bingo_chunk[i + 1]
                .split(' ')
                .into_iter()
                .filter(|c| !c.is_empty())
                .map(|c| c.parse::<u8>().unwrap())
                .collect();

            squares[i][..5].clone_from_slice(&line[..5]);
        }
        squares
    }
}
