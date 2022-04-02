use std::{collections::HashMap, fs};

pub fn run() {
    let mut caves = HashMap::new();

    fs::read_to_string("inputs/day12.txt")
        .unwrap()
        .lines()
        .for_each(|line| {
            let split = line.split('-').collect::<Vec<&str>>();
            if !caves.contains_key(split[0]) {
                caves.insert(split[0].to_string(), Cave::new(split[0]));
            }
            if !caves.contains_key(split[1]) {
                caves.insert(split[1].to_string(), Cave::new(split[1]));
            }

            caves
                .get_mut(split[0])
                .unwrap()
                .add_edge(split[1].to_string());
            caves
                .get_mut(split[1])
                .unwrap()
                .add_edge(split[0].to_string());
        });

    let p1 = traverse_all_routes(&caves, "start", &mut Vec::new(), false);
    let p2 = traverse_all_routes(&caves, "start", &mut Vec::new(), true);

    // println!("Part 1: {}", p1);
    // println!("Part 2: {}", p2);
    assert_eq!(p1, 4885);
    assert_eq!(p2, 117_095);
}

fn traverse_all_routes(
    caves: &HashMap<String, Cave>,
    current_position: &str,
    visited: &mut Vec<String>,
    mut allow_double_visit: bool,
) -> u32 {
    if current_position == "end" {
        return 1;
    }
    let cave = caves.get(current_position).unwrap();

    if cave.small {
        if allow_double_visit && visited.contains(&current_position.to_string()) {
            allow_double_visit = false;
        }

        visited.push(current_position.to_string());
    }

    let edges_to_visit: Vec<&String> = cave
        .edges
        .iter()
        .filter(|next_position| !visited.contains(next_position) || allow_double_visit)
        .collect();

    let mut sum = 0;
    for edge in edges_to_visit {
        sum += traverse_all_routes(caves, edge, visited, allow_double_visit);
    }

    if cave.small {
        visited.pop();
    }
    sum
}

struct Cave {
    edges: Vec<String>,
    small: bool,
}

impl Cave {
    fn new(name: &str) -> Cave {
        Cave {
            edges: Vec::new(),
            small: name.to_lowercase() == name,
        }
    }

    fn add_edge(&mut self, name: String) {
        if name != "start" {
            self.edges.push(name);
        }
    }
}
