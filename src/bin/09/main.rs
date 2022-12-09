#![feature(test)]

use std::collections::HashSet;

type Solution = i32;

#[derive(Debug)]
pub enum Move {
    Left(i32),
    Right(i32),
    Up(i32),
    Down(i32),
}
pub type ParseOutput = Vec<Move>;
const MAIN_INPUT: &str = include_str!("main_input");

pub fn parse(file: &str) -> ParseOutput {
    file.lines()
        .map(|l| {
            let (direction, length) = l.split_once(' ').unwrap();
            match direction {
                "U" => Move::Up(length.parse().unwrap()),
                "D" => Move::Down(length.parse().unwrap()),
                "L" => Move::Left(length.parse().unwrap()),
                "R" => Move::Right(length.parse().unwrap()),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn part_1(parse_output: &ParseOutput) -> Solution {
    let mut visited_map = HashSet::with_capacity(10000);

    let mut head_x: i32 = 0;
    let mut head_y: i32 = 0;
    let mut tail_x: i32 = 0;
    let mut tail_y: i32 = 0;

    for m in parse_output {
        match m {
            Move::Down(l) => {
                for _ in 0..*l {
                    head_y += 1;
                    (tail_x, tail_y) = move_tail(head_x, head_y, tail_x, tail_y);
                    visited_map.insert((tail_x, tail_y));
                }
            }
            Move::Up(l) => {
                for _ in 0..*l {
                    head_y -= 1;
                    (tail_x, tail_y) = move_tail(head_x, head_y, tail_x, tail_y);
                    visited_map.insert((tail_x, tail_y));
                }
            }
            Move::Left(l) => {
                for _ in 0..*l {
                    head_x -= 1;
                    (tail_x, tail_y) = move_tail(head_x, head_y, tail_x, tail_y);
                    visited_map.insert((tail_x, tail_y));
                }
            }
            Move::Right(l) => {
                for _ in 0..*l {
                    head_x += 1;
                    (tail_x, tail_y) = move_tail(head_x, head_y, tail_x, tail_y);
                    visited_map.insert((tail_x, tail_y));
                }
            }
        }
    }

    visited_map.len() as i32
}

fn part_2(parse_output: &ParseOutput) -> Solution {
    let mut parts: Vec<(i32, i32)> = vec![
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
    ];
    let mut visited_map = HashSet::with_capacity(10000);

    for m in parse_output {
        match m {
            Move::Down(l) => {
                for _ in 0..*l {
                    parts[0].1 += 1;
                    move_tails(&mut parts, &mut visited_map);
                }
            }
            Move::Up(l) => {
                for _ in 0..*l {
                    parts[0].1 -= 1;
                    move_tails(&mut parts, &mut visited_map);
                }
            }
            Move::Left(l) => {
                for _ in 0..*l {
                    parts[0].0 -= 1;
                    move_tails(&mut parts, &mut visited_map);
                }
            }
            Move::Right(l) => {
                for _ in 0..*l {
                    parts[0].0 += 1;
                    move_tails(&mut parts, &mut visited_map);
                }
            }
        }
    }

    visited_map.len() as i32
}

fn move_tails(parts: &mut [(i32, i32)], visited_map: &mut HashSet<(i32, i32)>) {
    for i in 1..10 {
        (parts[i].0, parts[i].1) = move_tail(parts[i - 1].0, parts[i - 1].1, parts[i].0, parts[i].1)
    }
    visited_map.insert((parts[9].0, parts[9].1));
}

fn move_tail(head_x: i32, head_y: i32, tail_x: i32, tail_y: i32) -> (i32, i32) {
    let diff_x = head_x - tail_x;
    let diff_y = head_y - tail_y;
    match (diff_x, diff_y) {
        (-1 | 0 | 1, -1 | 0 | 1) => (tail_x, tail_y),
        _ => (tail_x + diff_x.signum(), tail_y + diff_y.signum()),
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
        assert_eq!(part_1(&parse_output), 13);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = parse(TEST_INPUT);
        assert_eq!(part_2(&parse_output), 1);
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
            assert_eq!(part_1(black_box(&parse_output)), 5513);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = parse(MAIN_INPUT);
        b.iter(|| {
            assert_eq!(part_2(black_box(&parse_output)), 2427);
        });
    }
}
