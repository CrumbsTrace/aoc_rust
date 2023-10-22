use std::{cell::RefCell, collections::HashMap, fs, rc::Rc};

//This was just to play around with RefCell/Rc as an alternative. It's not faster and also broken for part 2. I'm leaving it here
#[divan::bench]
pub fn run() {
    let cave_map = get_cave_map();

    let p1 = traverse_all_routes(cave_map.get("start").unwrap(), &mut Vec::new(), false);
    let _p2 = traverse_all_routes(cave_map.get("start").unwrap(), &mut Vec::new(), true);

    // println!("Part 1: {}", p1);
    // println!("Part 2: {}", p2);
    assert_eq!(p1, 4885);
    // assert_eq!(p2, 117_095);
}

fn get_cave_map() -> HashMap<String, Rc<RefCell<Cave>>> {
    let mut caves = HashMap::new();
    fs::read_to_string("inputs/2021/day12.txt")
        .unwrap()
        .lines()
        .for_each(|line| {
            let split = line.split('-').collect::<Vec<&str>>();
            caves
                .entry(split[0].to_string())
                .or_insert_with(|| Rc::new(RefCell::new(Cave::new(split[0]))));
            caves
                .entry(split[1].to_string())
                .or_insert_with(|| Rc::new(RefCell::new(Cave::new(split[1]))));

            let cave1 = caves.get(split[0]).unwrap();
            caves
                .get(split[1])
                .unwrap()
                .borrow_mut()
                .edges
                .push(cave1.clone());

            let cave2 = caves.get(split[1]).unwrap();
            caves
                .get(split[0])
                .unwrap()
                .borrow_mut()
                .edges
                .push(cave2.clone());
        });
    caves
}

fn traverse_all_routes(
    cave: &Rc<RefCell<Cave>>,
    visited: &mut Vec<String>,
    mut allow_double_visit: bool,
) -> u32 {
    let cave = cave.borrow();
    if cave.name == "end" {
        return 1;
    }

    if cave.small {
        if allow_double_visit && visited.contains(&cave.name) {
            allow_double_visit = false;
        }

        visited.push(cave.name.clone());
    }

    let edges_to_visit: Vec<&Rc<RefCell<Cave>>> = cave
        .edges
        .iter()
        .filter(|next_position| {
            !visited.contains(&next_position.borrow().name) || allow_double_visit
        })
        .collect();

    let mut sum = 0;
    for edge in edges_to_visit {
        sum += traverse_all_routes(edge, visited, allow_double_visit);
    }

    if cave.small {
        visited.pop();
    }
    sum
}

#[derive(Debug, Clone)]
struct Cave {
    name: String,
    edges: Vec<Rc<RefCell<Cave>>>,
    small: bool,
}

impl Cave {
    fn new(name: &str) -> Cave {
        Cave {
            edges: Vec::new(),
            small: name.to_lowercase() == name,
            name: name.to_string(),
        }
    }
}
