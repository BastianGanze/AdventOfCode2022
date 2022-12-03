#![feature(test)]

use std::collections::HashSet;

type Solution = u32;

pub type Rucksack = Vec<u32>;
pub type ParseOutput = Vec<Rucksack>;
const MAIN_INPUT: &str = include_str!("main_input");
const TEST_INPUT: &str = include_str!("test_input");

pub fn parse(file: &str) -> ParseOutput {
    file.lines()
        .map(|rucksack: &str| rucksack.chars().map(match_chars_to_nums).collect())
        .collect()
}

fn match_chars_to_nums(c: char) -> u32 {
    if c.is_uppercase() {
        c as u32 - 38
    } else {
        c as u32 - 96
    }
}

fn part_1(parse_output: &ParseOutput) -> Solution {
    let mut solution = 0;
    let mut chars_in_map: HashSet<u32> = HashSet::new();
    'outer: for rucksack in parse_output {
        let compartment_1_len = rucksack.len() / 2;
        let rucksack_iter = rucksack.iter().enumerate();
        for (i, item) in rucksack_iter {
            if i < compartment_1_len {
                chars_in_map.insert(*item);
                continue;
            }
            if chars_in_map.contains(item) {
                solution += item;
                chars_in_map.clear();
                continue 'outer;
            }
        }
    }
    solution
}

fn part_2(parse_output: &ParseOutput) -> Solution {
    let mut solution = 0;
    let mut items_in_first_ruck: HashSet<u32> = HashSet::new();
    let mut items_in_second_ruck: HashSet<u32> = HashSet::new();
    'outer: for r in parse_output.chunks(3) {
        for item in &r[0] {
            items_in_first_ruck.insert(*item);
        }
        for item in &r[1] {
            items_in_second_ruck.insert(*item);
        }
        for item in &r[2] {
            if items_in_first_ruck.contains(item) && items_in_second_ruck.contains(item) {
                solution += item;
                items_in_first_ruck.clear();
                items_in_second_ruck.clear();
                continue 'outer;
            }
        }
    }
    solution
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
        assert_eq!(part_1(&parse_output), 157);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = parse(TEST_INPUT);
        assert_eq!(part_2(&parse_output), 70);
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
            assert_eq!(part_1(black_box(&parse_output)), 7716);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = parse(MAIN_INPUT);
        b.iter(|| {
            assert_eq!(part_2(black_box(&parse_output)), 2973);
        });
    }
}
