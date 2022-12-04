#![feature(test)]

use std::ops::Range;

type Solution = u32;

pub type ParseOutput = Vec<(Range<u32>, Range<u32>)>;
const MAIN_INPUT: &str = include_str!("main_input");
const TEST_INPUT: &str = include_str!("test_input");

pub fn parse(file: &str) -> ParseOutput {
    file.lines()
        .map(|pair: &str| {
            let (first, second) = pair.split_once(',').unwrap();
            let (s1, s2) = first.split_once('-').unwrap();
            let (e1, e2) = second.split_once('-').unwrap();

            (
                s1.parse::<u32>().unwrap()..s2.parse::<u32>().unwrap(),
                e1.parse::<u32>().unwrap()..e2.parse::<u32>().unwrap(),
            )
        })
        .collect()
}

fn part_1(parse_output: &ParseOutput) -> Solution {
    let mut solution = 0;
    for r in parse_output {
        if (r.0.start >= r.1.start && r.0.end <= r.1.end)
            || (r.1.start >= r.0.start && r.1.end <= r.0.end)
        {
            solution += 1;
        }
    }
    solution
}

fn part_2(parse_output: &ParseOutput) -> Solution {
    let mut solution = 0;
    for r in parse_output {
        if !(r.1.end < r.0.start || r.1.start > r.0.end) {
            solution += 1;
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
        assert_eq!(part_1(&parse_output), 2);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = parse(TEST_INPUT);
        assert_eq!(part_2(&parse_output), 4);
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
            assert_eq!(part_1(black_box(&parse_output)), 580);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = parse(MAIN_INPUT);
        b.iter(|| {
            assert_eq!(part_2(black_box(&parse_output)), 895);
        });
    }
}
