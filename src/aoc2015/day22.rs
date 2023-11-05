use std::fs;
use divan::black_box;

const MAGIC_MISSILE_COST: i32 = 53;
const DRAIN_COST: i32 = 73;
const SHIELD_COST: i32 = 113;
const POISON_COST: i32 = 173;
const RECHARGE_COST: i32 = 229;

#[derive(Copy, Clone, Debug)]
struct EffectRemainingTurns {
    shield: i32,
    poison: i32,
    recharge: i32,
}

#[derive(Copy, Clone, Debug)]
struct Entity {
    hp: i32,
    damage: i32,
}

#[derive(Copy, Clone, Debug)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn cost(&self) -> i32 {
        match self {
            Spell::MagicMissile => MAGIC_MISSILE_COST,
            Spell::Drain => DRAIN_COST,
            Spell::Shield => SHIELD_COST,
            Spell::Poison => POISON_COST,
            Spell::Recharge => RECHARGE_COST,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct GameState {
    player: Entity,
    boss: Entity,
    mana: i32,
    spent_mana: i32,
    effect_remaining_turns: EffectRemainingTurns,
    hard_mode: bool,
}

impl GameState {
    fn cast_spell(&mut self, spell: Spell) -> bool {
        if self.mana < spell.cost() {
            return false;
        }
        if self.hard_mode {
            self.player.hp -= 1;
            if self.player.hp <= 0 {
                return false;
            }
        }

        self.mana -= spell.cost();
        self.spent_mana += spell.cost();
        match spell {
            Spell::MagicMissile => {
                self.boss.hp -= 4;
            }
            Spell::Drain => {
                self.boss.hp -= 2;
                self.player.hp += 2;
            }
            Spell::Shield if self.effect_remaining_turns.shield == 0 => {
                self.effect_remaining_turns.shield = 6;
            }
            Spell::Poison if self.effect_remaining_turns.poison == 0 => {
                self.effect_remaining_turns.poison = 6;
            }
            Spell::Recharge if self.effect_remaining_turns.recharge == 0 => {
                self.effect_remaining_turns.recharge = 5;
            }
            _ => return false,
        }
        self.apply_effects();
        if self.boss.hp <= 0 {
            return true;
        }
        self.player.hp -= self.boss.damage - self.player_armor();
        self.apply_effects();
        self.player.hp > 0
    }

    fn apply_effects(&mut self) {
        if self.effect_remaining_turns.shield > 0 {
            self.effect_remaining_turns.shield -= 1;
        }
        if self.effect_remaining_turns.poison > 0 {
            self.boss.hp -= 3;
            self.effect_remaining_turns.poison -= 1;
        }
        if self.effect_remaining_turns.recharge > 0 {
            self.mana += 101;
            self.effect_remaining_turns.recharge -= 1;
        }
    }

    fn player_armor(&self) -> i32 {
        if self.effect_remaining_turns.shield > 0 {
            7
        } else {
            0
        }
    }
}

pub fn run(input: &str) -> (i32, i32) {
    let boss = parse_boss(input);
    let player = Entity { hp: 50, damage: 0 };

    let mut initial_state = GameState {
        player,
        boss,
        mana: 500,
        effect_remaining_turns: EffectRemainingTurns {
            shield: 0,
            poison: 0,
            recharge: 0,
        },
        spent_mana: 0,
        hard_mode: false,
    };
    let p1 = play_game(&initial_state);
    initial_state.hard_mode = true;
    let p2 = play_game(&initial_state);
    (p1, p2)
}

fn play_game(initial_state: &GameState) -> i32 {
    let mut min_cost = i32::MAX;
    let mut states = vec![*initial_state];
    while !states.is_empty() {
        let mut new_states = Vec::new();
        for state in states {
            if state.player.hp <= 0 {
                continue;
            }
            if state.boss.hp <= 0 {
                min_cost = min_cost.min(state.spent_mana);
                continue;
            }
            if state.spent_mana > min_cost {
                continue;
            }
            for spell in &[Spell::MagicMissile, Spell::Drain, Spell::Shield, Spell::Poison, Spell::Recharge] {
                let mut new_state = state;
                if !new_state.cast_spell(*spell) {
                    continue;
                }
                new_states.push(new_state);
            }
        }
        states = new_states;
    }
    min_cost
}

fn parse_boss(input: &str) -> Entity {
    let mut lines = input.lines();
    let hp = optimistic_parse(next_line_split(lines.next()).nth(2));
    let damage = optimistic_parse(next_line_split(lines.next()).nth(1));
    Entity { hp, damage }
}

fn next_line_split(line: Option<&str>) -> impl Iterator<Item = &'_ str> {
    line.unwrap().split_whitespace()
}

fn optimistic_parse(s: Option<&str>) -> i32 {
    s.unwrap().parse().unwrap()
}

#[test]
fn real_input() {
    let input = fs::read_to_string("inputs/2015/day22.txt").unwrap();
    let result = run(&input);
    assert_eq!(result, (1269, 1309));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = fs::read_to_string("inputs/2015/day22.txt").unwrap();
    bencher.bench(|| run(black_box(&input)));
}
