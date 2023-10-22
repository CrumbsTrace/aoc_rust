use rustc_hash::FxHashMap;
use std::fs;

#[divan::bench] 
pub fn run() {
    let mut caves = FxHashMap::default();

    fs::read_to_string("inputs/2021/day12.txt")
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

    let p1 = traverse_all_routes(&caves, "start", &mut Vec::with_capacity(20), false);
    let p2 = traverse_all_routes(&caves, "start", &mut Vec::with_capacity(20), true);

    // println!("Part 1: {}", p1);
    // println!("Part 2: {}", p2);
    assert_eq!(p1, 4885);
    assert_eq!(p2, 117_095);
}

fn traverse_all_routes<'a>(
    caves: &'a FxHashMap<String, Cave>,
    current_position: &'a str,
    visited: &mut Vec<&'a str>,
    mut allow_double_visit: bool,
) -> u32 {
    if current_position == "end" {
        return 1;
    }
    let cave = caves.get(current_position).unwrap();

    if cave.small {
        let has_been_visited = visited.contains(&current_position);
        if has_been_visited {
            if allow_double_visit {
                allow_double_visit = false;
            } else {
                return 0;
            }
        }

        visited.push(current_position);
    }

    let mut sum = 0;
    for edge in cave.edges.iter() {
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
