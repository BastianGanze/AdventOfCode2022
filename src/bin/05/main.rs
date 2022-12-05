#![feature(test)]

use std::collections::VecDeque;

type Solution = String;

pub const NUM_STACKS: usize = 9;
pub type ParseOutput = (Vec<VecDeque<char>>, Vec<(usize, usize, usize)>);
const MAIN_INPUT: &str = include_str!("main_input");
const TEST_INPUT: &str = include_str!("test_input");

pub fn parse(file: &str) -> ParseOutput {
    let mut containers = Vec::new();
    let mut container_moves = Vec::new();

    let (start_configuration, container_moves_unparsed) = file.split_once("\n\n").unwrap();
    for _i in 0..NUM_STACKS {
        containers.push(VecDeque::new());
    }

    for configuration in start_configuration.lines() {
        for (i, character) in configuration.chars().enumerate() {
            let stack_number = i / 4;
            if character.is_alphabetic() {
                containers
                    .get_mut(stack_number)
                    .unwrap()
                    .push_back(character);
            }
        }
    }

    for container_move_unparsed in container_moves_unparsed.lines() {
        let mut c = container_move_unparsed.split(' ');
        container_moves.push((
            c.nth(1).unwrap().parse::<usize>().unwrap() - 1,
            c.nth(1).unwrap().parse::<usize>().unwrap() - 1,
            c.nth(1).unwrap().parse::<usize>().unwrap() - 1,
        ))
    }

    (containers, container_moves)
}

fn part_1(parse_output: &ParseOutput) -> Solution {
    let (mut stacks, container_moves) = parse_output.clone();

    for container_move in container_moves {
        let (container_count, from, to) = container_move;
        for i in 0..container_count + 1 {
            let item = stacks.get_mut(from).unwrap().pop_front().unwrap();
            stacks.get_mut(to).unwrap().push_front(item);
        }
    }

    stacks
        .into_iter()
        .map(|s| {
            if let Some(first) = s.front() {
                *first
            } else {
                ' '
            }
        })
        .filter(|c| *c != ' ')
        .collect()
}

fn part_2(parse_output: &ParseOutput) -> Solution {
    let (mut stacks, container_moves) = parse_output.clone();

    let mut tmp_vec = Vec::new();
    for container_move in container_moves {
        let (container_count, from, to) = container_move;
        for _ in 0..container_count + 1 {
            tmp_vec.push(stacks.get_mut(from).unwrap().pop_front().unwrap());
        }
        for _ in 0..container_count + 1 {
            stacks
                .get_mut(to)
                .unwrap()
                .push_front(tmp_vec.pop().unwrap());
        }
    }

    stacks
        .into_iter()
        .map(|s| {
            if let Some(first) = s.front() {
                *first
            } else {
                ' '
            }
        })
        .filter(|c| *c != ' ')
        .collect()
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

    #[test]
    pub fn test_part_1() {
        let parse_output = parse(TEST_INPUT);
        assert_eq!(part_1(&parse_output), "CMZ");
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = parse(TEST_INPUT);
        assert_eq!(part_2(&parse_output), "MCD");
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
            assert_eq!(part_1(black_box(&parse_output)), "VRWBSFZWM");
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = parse(MAIN_INPUT);
        b.iter(|| {
            assert_eq!(part_2(black_box(&parse_output)), "RBTWJWMCF");
        });
    }
}
