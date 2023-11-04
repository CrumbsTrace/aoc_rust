use std::fs;
use divan::black_box;

#[derive(Debug)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

pub fn run(input: &str) -> (i32, i32) {
    let ingredients = parse_ingredients(input);
    let mut best_score = 0;
    let mut best_calorie_score = 0;
    for amounts in ingredient_combinations(&ingredients, 100) {
        let mut capacity = 0;
        let mut durability = 0;
        let mut flavor = 0;
        let mut texture = 0;
        let mut calories = 0;
        for (i, ingredient) in ingredients.iter().enumerate() {
            capacity += amounts[i] * ingredient.capacity;
            durability += amounts[i] * ingredient.durability;
            flavor += amounts[i] * ingredient.flavor;
            texture += amounts[i] * ingredient.texture;
            calories += amounts[i] * ingredient.calories;
        }
        if capacity <= 0 || durability <= 0 || flavor <= 0 || texture <= 0 {
            continue;
        }
        let score = capacity * durability * flavor * texture;
        best_score = best_score.max(score);
        if calories == 500 {
            best_calorie_score = best_calorie_score.max(score);
        }
    }
    (best_score, best_calorie_score)
}

fn ingredient_combinations(ingredients: &[Ingredient], amount: i32) -> Vec<Vec<i32>> {
    if ingredients.len() == 1 {
        return vec![vec![amount]];
    }
    let mut result = Vec::new();
    for i in 0..amount {
        let mut sub_result = ingredient_combinations(&ingredients[1..], amount - i);
        for sub in sub_result.iter_mut() {
            sub.insert(0, i);
        }
        result.append(&mut sub_result);
    }
    result
}

fn parse_ingredients(input: &str) -> Vec<Ingredient> {
    let mut result = Vec::new();
    for line in input.lines() {
        let mut words = line.split_whitespace();
        let capacity = words.nth(2).unwrap().trim_end_matches(',').parse::<i32>().unwrap();
        let durability = words.nth(1).unwrap().trim_end_matches(',').parse::<i32>().unwrap();
        let flavor = words.nth(1).unwrap().trim_end_matches(',').parse::<i32>().unwrap();
        let texture = words.nth(1).unwrap().trim_end_matches(',').parse::<i32>().unwrap();
        let calories = words.nth(1).unwrap().parse::<i32>().unwrap();
        result.push(Ingredient {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        });
    }
    result
}

#[test]
fn example() {
    let input = r"Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";
    assert_eq!(run(input), (62842880, 57600000));
}

#[test]
fn real_input() {
    let input = fs::read_to_string("inputs/2015/day15.txt").unwrap();
    let result = run(&input);
    assert_eq!(result, (18965440, 15862900))
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = fs::read_to_string("inputs/2015/day15.txt").unwrap();
    bencher.bench(|| run(black_box(&input)));
}

