use std::fs;
use std::collections::HashMap;
use itertools::Itertools;
use divan::black_box;

pub fn run(input: &str) -> (i32, i32) {
    let lines = input.lines().collect::<Vec<&str>>();
    let distance_graph = build_graph(&lines);
    let cities = distance_graph.keys().map(|(from, _)| from.clone()).unique().collect::<Vec<String>>();
    get_minmax_path(&cities, &distance_graph)
}

fn get_minmax_path(cities: &[String], distance_graph: &HashMap<(String, String), i32>) -> (i32, i32) {
    let mut shortest_path = i32::MAX;
    let mut longest_path = 0;
    for permutation in cities.iter().permutations(cities.len()) {
        let mut path = 0;
        for i in 0..permutation.len() - 1 {
            path += distance_graph.get(&(permutation[i].clone(), permutation[i + 1].clone())).unwrap();
        }
        shortest_path = shortest_path.min(path);
        longest_path = longest_path.max(path);
    }
    (shortest_path, longest_path)

}

fn build_graph(lines: &Vec<&str>) -> HashMap<(String, String), i32> {
    let mut graph = HashMap::new();
    for line in lines {
        let mut line = line.split(" ");
        let from = line.next().unwrap();
        let to = line.nth(1).unwrap();
        let distance = line.nth(1).unwrap().parse::<i32>().unwrap();
        graph.insert((from.to_string(), to.to_string()), distance);
        graph.insert((to.to_string(), from.to_string()), distance);
    }
    graph
}

#[test]
fn example() {
    let input = "London to Dublin = 464\n\
                 London to Belfast = 518\n\
                 Dublin to Belfast = 141";

    assert_eq!(run(input), (605, 982))
}

#[test]
fn real_input() {
    let input = fs::read_to_string("inputs/2015/day9.txt").unwrap();
    let result = run(&input);
    assert_eq!(result, (117, 909));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = fs::read_to_string("inputs/2015/day9.txt").unwrap();
    bencher.bench(|| run(black_box(&input)));
}
