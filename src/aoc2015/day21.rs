use std::fs;
use divan::black_box;

const WEAPONS: [(i32, i32); 5] = [(8, 4), (10, 5), (25, 6), (40, 7), (74, 8)];
const ARMOR: [(i32, i32); 6] = [(0, 0), (13, 1), (31, 2), (53, 3), (75, 4), (102, 5)];
const RINGS: [(i32, i32, i32); 7] = [
    (0, 0, 0),
    (25, 1, 0),
    (50, 2, 0),
    (100, 3, 0),
    (20, 0, 1),
    (40, 0, 2),
    (80, 0, 3),
];

#[derive(Copy, Clone, Debug)]
struct Entity {
    hp: i32,
    damage: i32,
    armor: i32,
}

pub fn run(input: &str) -> (i32, i32) {
    let boss = parse_boss(input);
    let mut min_cost = i32::MAX;
    let mut max_cost = i32::MIN;
    for weapon in &WEAPONS {
        for armor in &ARMOR {
            for ring1 in &RINGS {
                for ring2 in &RINGS {
                    if ring1 == ring2 && ring1.0 != 0 {
                        continue;
                    }
                    let cost = weapon.0 + armor.0 + ring1.0 + ring2.0;
                    let damage = weapon.1 + ring1.1 + ring2.1;
                    let armor = armor.1 + ring1.2 + ring2.2;
                    let player = Entity {
                        hp: 100,
                        damage,
                        armor,
                    };
                    if player_wins(&player, &boss) {
                        min_cost = min_cost.min(cost);
                    } else {
                        max_cost = max_cost.max(cost);
                    }
                }
            }
        }
    }
    (min_cost, max_cost)
}

fn player_wins(player: &Entity, boss: &Entity) -> bool {
    let mut player = *player;
    let mut boss = *boss;
    loop {
        boss.hp -= (player.damage - boss.armor).max(1);
        if boss.hp <= 0 {
            return true;
        }
        player.hp -= (boss.damage - player.armor).max(1);
        if player.hp <= 0 {
            return false;
        }
    }
}

fn parse_boss(input: &str) -> Entity {
    let mut lines = input.lines();
    let hp = optimistic_parse(next_line_split(lines.next()).nth(2));
    let damage = optimistic_parse(next_line_split(lines.next()).nth(1));
    let armor = optimistic_parse(next_line_split(lines.next()).nth(1));
    Entity { hp, damage, armor }
}

fn next_line_split(line: Option<&str>) -> impl Iterator<Item = &'_ str> {
    line.unwrap().split_whitespace()
}

fn optimistic_parse(s: Option<&str>) -> i32 {
    s.unwrap().parse().unwrap()
}

#[test]
fn real_input() {
    let input = fs::read_to_string("inputs/2015/day21.txt").unwrap();
    let result = run(&input);
    assert_eq!(result, (111, 188));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = fs::read_to_string("inputs/2015/day21.txt").unwrap();
    bencher.bench(|| run(black_box(&input)));
}
