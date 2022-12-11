#![feature(test)]

use std::collections::{BTreeSet, VecDeque};

type Solution = u64;

#[derive(Debug, Clone)]
pub enum Operation {
    Add(u64),
    Multiply(u64),
    Pow,
}
#[derive(Debug, Clone)]
pub struct Monkey {
    pub items: VecDeque<u64>,
    pub operation: Operation,
    pub divisible_test: u64,
    pub true_monkey: usize,
    pub false_monkey: usize,
    pub inspections: u64,
}
pub type ParseOutput = Vec<Monkey>;
const MAIN_INPUT: &str = include_str!("main_input");

pub fn parse(file: &str) -> ParseOutput {
    file.split("\n\n")
        .map(|m| {
            let mut monkey = Monkey {
                items: VecDeque::new(),
                operation: Operation::Multiply(0),
                false_monkey: 0,
                true_monkey: 0,
                divisible_test: 0,
                inspections: 0,
            };
            for l in m.lines() {
                let parts: Vec<&str> = l.trim().split_ascii_whitespace().collect();
                match &parts[..] {
                    ["Monkey", _] => {}
                    ["Operation:", "new", "=", "old", op, num] => {
                        monkey.operation = match *op {
                            "*" => {
                                if let Ok(n) = num.parse() {
                                    Operation::Multiply(n)
                                } else {
                                    Operation::Pow
                                }
                            }
                            "+" => Operation::Add(num.parse().unwrap()),
                            _ => unreachable!(),
                        }
                    }
                    ["Test:", "divisible", "by", n] => {
                        monkey.divisible_test = n.parse().unwrap();
                    }
                    ["If", "true:", "throw", "to", "monkey", n] => {
                        monkey.true_monkey = n.parse().unwrap();
                    }
                    ["If", "false:", "throw", "to", "monkey", n] => {
                        monkey.false_monkey = n.parse().unwrap();
                    }
                    u => {
                        if u[0] == "Starting" {
                            for item in u[2..].iter() {
                                let n = item.replace(',', "").parse().unwrap();
                                monkey.items.push_back(n);
                            }
                        } else {
                            unreachable!();
                        }
                    }
                }
            }
            monkey
        })
        .collect()
}

fn part_1(parse_output: &ParseOutput) -> Solution {
    monkey_throws(parse_output.clone(), 20, |item_level| item_level / 3)
}

fn part_2(parse_output: &ParseOutput) -> Solution {
    let relief = parse_output
        .iter()
        .map(|m| m.divisible_test)
        .reduce(|acc, d| acc * d)
        .unwrap();

    monkey_throws(parse_output.clone(), 10000, |item_level| {
        item_level % relief
    })
}

fn monkey_throws<F: FnMut(u64) -> u64>(
    mut monkeys: ParseOutput,
    count: usize,
    mut calm_down: F,
) -> u64 {
    let mut throws = Vec::new();
    for _ in 0..count {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            while !monkey.items.is_empty() {
                monkey.inspections += 1;
                let stress_level = calm_down(get_stress_level(
                    monkey.items.pop_front().unwrap(),
                    &monkey.operation,
                ));
                if stress_level % monkey.divisible_test == 0 {
                    throws.push((monkey.true_monkey, stress_level));
                } else {
                    throws.push((monkey.false_monkey, stress_level))
                }
            }
            for (m_i, item) in throws.drain(..) {
                monkeys[m_i].items.push_back(item);
            }
        }
    }

    BTreeSet::from_iter(monkeys.into_iter().map(|m| m.inspections))
        .into_iter()
        .rev()
        .take(2)
        .reduce(|acc, n| acc * n)
        .unwrap()
}

fn get_stress_level(item: u64, operation: &Operation) -> u64 {
    match operation {
        Operation::Pow => item * item,
        Operation::Add(n) => item + n,
        Operation::Multiply(n) => item * n,
    }
}

fn main() {
    let parse_output = parse(MAIN_INPUT);
    println!("Solution to part 1 is {}", part_1(&parse_output));
    println!("Solution to part 2 is {}", part_2(&parse_output));
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use test::{black_box, Bencher};
    const TEST_INPUT: &str = include_str!("test_input");

    #[test]
    pub fn test_part_1() {
        let parse_output = parse(TEST_INPUT);
        assert_eq!(part_1(&parse_output), 10605);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = parse(TEST_INPUT);
        assert_eq!(part_2(&parse_output), 2713310158);
    }

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| {
            let _ = parse(MAIN_INPUT);
        });
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let parse_output = parse(MAIN_INPUT);
        b.iter(move || {
            assert_eq!(part_1(black_box(&parse_output)), 54054);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = parse(MAIN_INPUT);
        b.iter(|| {
            assert_eq!(part_2(black_box(&parse_output)), 14314925001);
        });
    }
}
