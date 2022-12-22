#![feature(test)]
#![feature(hash_drain_filter)]

use std::borrow::Borrow;
use std::collections::HashMap;
use std::ops::Deref;

type Sol = i64;

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Add,
    Sub,
    Mult,
    Div,
}

#[derive(Debug, Clone, PartialEq)]
pub enum P2Sol {
    Unknown,
    Number(Sol),
    Operation(Box<P2Sol>, Box<P2Sol>, Operation),
}

#[derive(Debug, Clone)]
pub enum Monkey {
    Number(Sol),
    Operation(String, String, Operation),
}

pub type ParseOutput = HashMap<String, Monkey>;
const MAIN_INPUT: &str = include_str!("main_input");

pub fn parse(file: &str) -> ParseOutput {
    file.lines()
        .map(|l| {
            match l
                .replace(":", "")
                .split_ascii_whitespace()
                .collect::<Vec<&str>>()[..]
            {
                [monkey_name, op_left, op, op_right] => (
                    monkey_name.into(),
                    Monkey::Operation(
                        op_left.into(),
                        op_right.into(),
                        match op {
                            "+" => Operation::Add,
                            "-" => Operation::Sub,
                            "*" => Operation::Mult,
                            "/" => Operation::Div,
                            _ => unreachable!(),
                        },
                    ),
                ),
                [monkey_name, num] => (monkey_name.into(), Monkey::Number(num.parse().unwrap())),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn part_1(monkeys: &ParseOutput) -> Sol {
    let sorted = top_sort_monkeys(monkeys);
    let mut solution = HashMap::<String, Sol>::new();
    for (name, monkey) in sorted {
        match monkey {
            Monkey::Number(n) => {
                solution.insert(name, n);
            }
            Monkey::Operation(spring_1, spring_2, op) => {
                let sol_spring_1 = solution.get(&spring_1).unwrap();
                let sol_spring_2 = solution.get(&spring_2).unwrap();
                solution.insert(
                    name,
                    match op {
                        Operation::Add => sol_spring_1 + sol_spring_2,
                        Operation::Sub => sol_spring_1 - sol_spring_2,
                        Operation::Mult => sol_spring_1 * sol_spring_2,
                        Operation::Div => sol_spring_1 / sol_spring_2,
                    },
                );
            }
        }
    }

    *solution.get("root").unwrap()
}

fn part_2(monkeys: &ParseOutput) -> Sol {
    let sorted = top_sort_monkeys(monkeys);
    let mut solution = HashMap::<String, P2Sol>::new();

    for (name, monkey) in sorted {
        match monkey {
            Monkey::Number(n) => {
                if name == "humn" {
                    solution.insert(name, P2Sol::Unknown);
                } else {
                    solution.insert(name, P2Sol::Number(n));
                }
            }
            Monkey::Operation(spring_1, spring_2, op) => {
                let p2sol_1 = solution.remove(&spring_1).unwrap();
                let p2sol_2 = solution.remove(&spring_2).unwrap();
                if let (P2Sol::Number(n1), P2Sol::Number(n2)) = (&p2sol_1, &p2sol_2) {
                    solution.insert(
                        name,
                        P2Sol::Number(match op {
                            Operation::Add => n1 + n2,
                            Operation::Sub => n1 - n2,
                            Operation::Mult => n1 * n2,
                            Operation::Div => n1 / n2,
                        }),
                    );
                } else {
                    solution.insert(
                        name,
                        P2Sol::Operation(Box::from(p2sol_1), Box::from(p2sol_2), op),
                    );
                }
            }
        }
    }

    if let Some(P2Sol::Operation(p1, p2, _)) = solution.remove("root") {
        let mut monkey_n: Sol = 0;
        let mut unknown_o = p1.clone();
        if let P2Sol::Number(n) = p2.borrow() {
            monkey_n = *n;
        }
        if let P2Sol::Number(n) = p1.borrow() {
            monkey_n = *n;
            unknown_o = p2;
        }
        println!("{} = {}", get_fun(*unknown_o), monkey_n);
        0
    } else {
        0
    }
}

fn get_fun(unknown: P2Sol) -> String {
    match unknown {
        P2Sol::Unknown => "n".into(),
        P2Sol::Number(n) => format!("{}", n),
        P2Sol::Operation(p1, p2, op) => match op {
            Operation::Add => format!("({}+{})", get_fun(*p1), get_fun(*p2)),
            Operation::Sub => format!("({}-{})", get_fun(*p1), get_fun(*p2)),
            Operation::Mult => format!("({}*{})", get_fun(*p1), get_fun(*p2)),
            Operation::Div => format!("({}/{})", get_fun(*p1), get_fun(*p2)),
        },
    }
}

fn calculate_unknown_n(unknown: P2Sol, current_n: Sol) -> Sol {
    match unknown {
        P2Sol::Unknown => current_n,
        P2Sol::Number(_) => unreachable!(),
        P2Sol::Operation(p1, p2, op) => {
            if let P2Sol::Number(n) = *p1 {
                calculate_unknown_n(
                    *p2,
                    match op {
                        Operation::Add => current_n - n,
                        Operation::Sub => current_n + n,
                        Operation::Mult => current_n / n,
                        Operation::Div => current_n * n,
                    },
                )
            } else if let P2Sol::Number(n) = *p2 {
                calculate_unknown_n(
                    *p1,
                    match op {
                        Operation::Add => n - current_n,
                        Operation::Sub => n + current_n,
                        Operation::Mult => n / current_n,
                        Operation::Div => n * current_n,
                    },
                )
            } else {
                unreachable!()
            }
        }
    }
}

fn top_sort_monkeys(monkeys: &ParseOutput) -> Vec<(String, Monkey)> {
    let mut spring_to_source: HashMap<String, String> = HashMap::new();
    let mut springs: Vec<(String, Monkey)> = monkeys
        .iter()
        .filter_map(|(n, m)| {
            if let Monkey::Number(_) = m {
                Some((n.clone(), m.clone()))
            } else {
                None
            }
        })
        .collect();
    let mut sorted: Vec<(String, Monkey)> = Vec::new();
    for (name, m) in monkeys.iter() {
        match m {
            Monkey::Number(_) => {}
            Monkey::Operation(n1, n2, _) => {
                assert_eq!(spring_to_source.insert(n1.into(), name.into()), None);
                assert_eq!(spring_to_source.insert(n2.into(), name.into()), None);
            }
        }
    }

    while let Some(spring) = springs.pop() {
        if let Some((_, source_id)) = spring_to_source.remove_entry(&spring.0) {
            if let Some(m) = monkeys.get(&source_id) {
                if let Monkey::Operation(source_1, source_2, _) = m {
                    if !spring_to_source.contains_key(source_1)
                        && !spring_to_source.contains_key(source_2)
                    {
                        springs.push((source_id.clone(), m.clone()));
                    }
                }
            }
        }
        sorted.push(spring);
    }
    sorted
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
        assert_eq!(part_1(&parse_output), 152);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = parse(TEST_INPUT);
        assert_eq!(part_2(&parse_output), 301);
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
            assert_eq!(part_1(black_box(&parse_output)), 223971851179174);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = parse(MAIN_INPUT);
        b.iter(|| {
            assert_eq!(part_2(black_box(&parse_output)), 21276);
        });
    }
}
