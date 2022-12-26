#![feature(test)]

use colored::*;

type Sol = i64;
use crate::snafu::SNAFU;
use std::io::stdin;
use std::{thread, time};

pub mod snafu;

pub type ParseOutput = Vec<SNAFU>;

const MAIN_INPUT: &str = include_str!("main_input");

pub fn parse(file: &str) -> ParseOutput {
    file.lines().map(SNAFU::from).collect()
}

fn part_1(parse_output: &ParseOutput) -> String {
    parse_output.iter().cloned().sum::<SNAFU>().as_string()
}

fn main() {
    let parse_output = parse(MAIN_INPUT);
    println!("Solution to part 1 is {}", part_1(&parse_output));
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
        assert_eq!(part_1(&parse_output), "2=-1=0");
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
            assert_eq!(part_1(black_box(&parse_output)), "2011-=2=-1020-1===-1");
        });
    }
}
