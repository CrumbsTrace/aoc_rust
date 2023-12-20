use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::collections::VecDeque;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Module {
    Broadcast(Vec<String>),
    FlipFlop(Vec<String>, bool),
    Conjuction(Vec<String>, FxHashMap<String, bool>),
}

impl Module {
    fn destinations(&self) -> &Vec<String> {
        match self {
            Module::Broadcast(destinations) => &destinations,
            Module::FlipFlop(destinations, _) => &destinations,
            Module::Conjuction(destinations, _) => &destinations,
        }
    }

    fn process(&mut self, input_name: &str, high_pulse: bool) -> Option<bool> {
        match self {
            Module::Broadcast(_) => Some(false),
            Module::FlipFlop(_, on) => {
                if !high_pulse {
                    *on = !*on;
                    Some(*on)
                } else {
                    None
                }
            }
            Module::Conjuction(_, inputs) => {
                let input = inputs.get_mut(input_name).unwrap();
                *input = high_pulse;
                if inputs.values().any(|v| !*v) {
                    Some(true)
                } else {
                    Some(false)
                }
            }
        }
    }
}

pub fn run(input: &str) -> (u64, u64) {
    let mut modules = parse(input);
    let ((low, high), p2) = push_button_n_times(&mut modules, 1000);
    (low * high, p2)
}

fn push_button_n_times(modules: &mut FxHashMap<String, Module>, n: u64) -> ((u64, u64), u64) {
    let mut pulse_counts = (0, 0);
    let mut track_i = None;
    let mut result = None;
    let mut i = 1;
    let mut track_inputs = FxHashMap::default();
    let mut to_track = String::default();
    if let Some((name, module)) = modules
        .iter()
        .find(|(_, module)| module.destinations().contains(&"rx".to_string()))
    {
        match module {
            Module::Conjuction(_, inputs) => {
                to_track = name.clone();
                track_inputs = inputs
                    .keys()
                    .map(|k| (k.clone(), None))
                    .collect::<FxHashMap<String, Option<u64>>>()
            }
            _ => panic!("rx destination must be a conjuction"),
        };
    }

    while (to_track != String::default() && track_i.is_none()) || result.is_none() {
        let (pulse_result, tracked_high_pulses) = push_button(modules, to_track.clone());
        pulse_counts.0 += pulse_result.0;
        pulse_counts.1 += pulse_result.1;
        if i == n {
            result = Some(pulse_counts);
        }

        for tracked_high_pulse in tracked_high_pulses {
            if let Some(input) = track_inputs.get_mut(&tracked_high_pulse) {
                if input.is_none() {
                    *input = Some(i);
                }
            }

            if track_inputs.values().all(|v| v.is_some()) {
                let lcm = track_inputs
                    .values()
                    .map(|v| v.unwrap())
                    .fold(1, |acc, v| lcm(acc, v));
                track_i = Some(lcm);
            }
        }
        i += 1;
    }
    (result.unwrap(), track_i.unwrap_or(0))
}

fn push_button(
    modules: &mut FxHashMap<String, Module>,
    to_track: String,
) -> ((u64, u64), Vec<String>) {
    let mut queue = VecDeque::new();
    let mut pulse_counts = (1, 0);
    let mut totrack_high_pulses = Vec::new();
    queue.push_back(("broadcaster".to_string(), "".to_string(), false));
    while let Some((current, input_name, high_pulse)) = queue.pop_front() {
        if let Some(module) = modules.get_mut(&current) {
            if current == to_track && high_pulse {
                totrack_high_pulses.push(input_name.clone());
            }
            if let Some(high_pulse) = module.process(&input_name, high_pulse) {
                let destinations = module.destinations();
                for destination in destinations {
                    queue.push_back((destination.clone(), current.clone(), high_pulse));
                    if high_pulse {
                        pulse_counts.1 += 1;
                    } else {
                        pulse_counts.0 += 1;
                    }
                }
            }
        }
    }
    (pulse_counts, totrack_high_pulses)
}

fn parse(input: &str) -> FxHashMap<String, Module> {
    let mut inputs_map = FxHashMap::default();
    let mut modules = input
        .lines()
        .map(|line| {
            let parts = line
                .split([' ', '-', '>', ','])
                .filter(|s| !s.is_empty())
                .collect_vec();
            let name = parts[0].to_string();
            let destinations = parts[1..].iter().map(|s| s.to_string()).collect_vec();
            for destination in destinations.clone() {
                inputs_map
                    .entry(destination)
                    .or_insert_with(Vec::new)
                    .push(name.replace("%", "").replace("&", "").to_string());
            }
            if parts[0] == "broadcaster" {
                (name, Module::Broadcast(destinations))
            } else if parts[0].starts_with('%') {
                (name.replace("%", ""), Module::FlipFlop(destinations, false))
            } else {
                (
                    name.replace("&", ""),
                    Module::Conjuction(destinations, FxHashMap::default()),
                )
            }
        })
        .collect::<FxHashMap<_, _>>();

    //Fill the inputs maps for the conjuction modules.
    for (name, inputs) in inputs_map {
        if let Some(Module::Conjuction(_, states)) = modules.get_mut(&name) {
            for input in inputs {
                states.insert(input, false);
            }
        }
    }
    modules
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[test]
fn example() {
    let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
    assert_eq!(run(input), (32000000, 0));
}

#[test]
fn example2() {
    let input = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
    assert_eq!(run(input), (11687500, 0));
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("inputs/2023/day20.txt").unwrap();
    assert_eq!(run(&input), (712543680, 238920142622879));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("inputs/2023/day20.txt").unwrap();
    bencher.bench(|| run(divan::black_box(&input)));
}
