#![feature(test)]

use std::cmp::max;
use std::collections::HashMap;

type Solution = u32;

pub type ParseOutput = Vec<Vec<u32>>;
const MAIN_INPUT: &str = include_str!("main_input");

pub fn parse(file: &str) -> ParseOutput {
    file.lines()
        .map(|l| {
            l.chars()
                .into_iter()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}

fn part_1(parse_output: &ParseOutput) -> Solution {
    let mut solution = 0;
    let size_y = parse_output.len();
    let size_x = parse_output.first().unwrap().len();

    for y in 1..(size_y - 1) {
        for x in 1..(size_x - 1) {
            if los(y, x, parse_output, size_x, size_y) {
                solution += 1;
            }
        }
    }

    solution + (size_y as u32) * 2 + (size_x as u32) * 2 - 4
}

fn los(y: usize, x: usize, map: &ParseOutput, map_x: usize, map_y: usize) -> bool {
    let n = map.get(y).unwrap().get(x).unwrap();
    let mut left = x - 1;
    loop {
        let c = map.get(y).unwrap().get(left).unwrap();
        if c >= n {
            break;
        }
        if left == 0 {
            return true;
        }
        left -= 1;
    }

    let mut right = x + 1;
    loop {
        let c = map.get(y).unwrap().get(right).unwrap();
        if c >= n {
            break;
        }
        if right == map_x - 1 {
            return true;
        }
        right += 1;
    }

    let mut bottom = y - 1;
    loop {
        let c = map.get(bottom).unwrap().get(x).unwrap();
        if c >= n {
            break;
        }
        if bottom == 0 {
            return true;
        }
        bottom -= 1;
    }

    let mut top = y + 1;
    loop {
        let c = map.get(top).unwrap().get(x).unwrap();
        if c >= n {
            break;
        }
        if top == map_y - 1 {
            return true;
        }
        top += 1;
    }

    false
}

fn score(y: usize, x: usize, map: &ParseOutput, map_x: usize, map_y: usize) -> u32 {
    let n = map.get(y).unwrap().get(x).unwrap();
    let mut left = x - 1;
    let mut score_left = 0;
    loop {
        let c = map.get(y).unwrap().get(left).unwrap();
        score_left += 1;
        if c >= n {
            break;
        }
        if left == 0 {
            break;
        }
        left -= 1;
    }

    let mut right = x + 1;
    let mut score_right = 0;
    loop {
        let c = map.get(y).unwrap().get(right).unwrap();
        score_right += 1;
        if c >= n {
            break;
        }
        if right == map_x - 1 {
            break;
        }
        right += 1;
    }

    let mut bottom = y - 1;
    let mut score_bottom = 0;
    loop {
        let c = map.get(bottom).unwrap().get(x).unwrap();
        score_bottom += 1;
        if c >= n {
            break;
        }
        if bottom == 0 {
            break;
        }
        bottom -= 1;
    }

    let mut top = y + 1;
    let mut score_top = 0;
    loop {
        let c = map.get(top).unwrap().get(x).unwrap();
        score_top += 1;
        if c >= n {
            break;
        }
        if top == map_y - 1 {
            break;
        }
        top += 1;
    }

    score_top * score_bottom * score_right * score_left
}

fn part_2(parse_output: &ParseOutput) -> Solution {
    let mut solution = 0;
    let size_y = parse_output.len();
    let size_x = parse_output.first().unwrap().len();

    for y in 1..(size_y - 1) {
        for x in 1..(size_x - 1) {
            solution = max(solution, score(y, x, parse_output, size_x, size_y));
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
    const TEST_INPUT: &str = include_str!("test_input");

    #[test]
    pub fn test_part_1() {
        let parse_output = parse(TEST_INPUT);
        assert_eq!(part_1(&parse_output), 21);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = parse(TEST_INPUT);
        assert_eq!(part_2(&parse_output), 8);
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
            assert_eq!(part_1(black_box(&parse_output)), 1835);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = parse(MAIN_INPUT);
        b.iter(|| {
            assert_eq!(part_2(black_box(&parse_output)), 263670);
        });
    }
}
