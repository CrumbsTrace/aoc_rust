use std::collections::HashMap;
use std::fs;
use divan::black_box;

enum Instruction {
    Hlf(char),
    Tpl(char),
    Inc(char),
    Jmp(i32),
    Jie(char, i32),
    Jio(char, i32),
}

pub fn run(input: &str) -> (i32, i32) {
    let instructions = parse_instructions(input);
    let mut registers = HashMap::new();
    execute(&instructions, &mut registers);
    let part1 = *registers.get(&'b').unwrap_or(&0);
    let mut registers = HashMap::new();
    registers.insert('a', 1);
    execute(&instructions, &mut registers);
    let part2 = *registers.get(&'b').unwrap_or(&0);
    (part1, part2)
}

fn execute(instructions: &[Instruction], registers: &mut HashMap<char, i32>) {
    let mut pc = 0;
    while pc < instructions.len() {
        match instructions[pc] {
            Instruction::Hlf(register) => {
                *registers.entry(register).or_insert(0) /= 2;
                pc += 1;
            }
            Instruction::Tpl(register) => {
                *registers.entry(register).or_insert(0) *= 3;
                pc += 1;
            }
            Instruction::Inc(register) => {
                *registers.entry(register).or_insert(0) += 1;
                pc += 1;
            }
            Instruction::Jmp(offset) => {
                pc = (pc as i32 + offset) as usize;
            }
            Instruction::Jie(register, offset) => {
                if registers.get(&register).unwrap_or(&0) % 2 == 0 {
                    pc = (pc as i32 + offset) as usize;
                } else {
                    pc += 1;
                }
            }
            Instruction::Jio(register, offset) => {
                if registers.get(&register).unwrap_or(&0) == &1 {
                    pc = (pc as i32 + offset) as usize;
                } else {
                    pc += 1;
                }
            }
        }
    }
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let mut words = line.split_whitespace();
            let instruction = words.next().unwrap();
            match instruction {
                "hlf" => Instruction::Hlf(parse_register(words.next().unwrap())),
                "tpl" => Instruction::Tpl(parse_register(words.next().unwrap())),
                "inc" => Instruction::Inc(parse_register(words.next().unwrap())),
                "jmp" => Instruction::Jmp(parse_offset(words.next().unwrap())),
                "jie" => {
                    let register = parse_register(words.next().unwrap());
                    let offset = parse_offset(words.next().unwrap());
                    Instruction::Jie(register, offset)
                }
                "jio" => {
                    let register = parse_register(words.next().unwrap());
                    let offset = parse_offset(words.next().unwrap());
                    Instruction::Jio(register, offset)
                }
                _ => panic!("Unknown instruction: {}", instruction),
            }
        })
        .collect()
}

fn parse_register(register: &str) -> char {
    register.chars().next().unwrap()
}

fn parse_offset(offset: &str) -> i32 {
    offset.parse().unwrap()
}

#[test]
fn real_input() {
    let input = fs::read_to_string("inputs/2015/day23.txt").unwrap();
    let result = run(&input);
    assert_eq!(result, (255, 334));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = fs::read_to_string("inputs/2015/day23.txt").unwrap();
    bencher.bench(|| run(black_box(&input)));
}
