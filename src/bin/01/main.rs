#![feature(test)]

type Solution = i32;
pub type ParseOutput = Vec<i32>;
const MAIN_INPUT: &str = include_str!("main_input");
const TEST_INPUT: &str = include_str!("test_input");

pub fn parse(file: &str) -> ParseOutput {
    file.split("\n\n")
        .map(|package| {
            package
                .lines()
                .fold(0, |acc, item| acc + item.parse::<i32>().unwrap())
        })
        .collect()
}

fn part_1(parse_output: &ParseOutput) -> Solution {
    parse_output.clone().into_iter().max().unwrap()
}

fn part_2(parse_output: &ParseOutput) -> Solution {
    let mut solution_vec = parse_output.clone();
    solution_vec.sort();

    solution_vec.iter().rev().take(3).sum()
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
        assert_eq!(part_1(&parse_output), 24000);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = parse(TEST_INPUT);
        assert_eq!(part_2(&parse_output), 45000);
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
            assert_eq!(part_1(black_box(&parse_output)), 68442);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = parse(MAIN_INPUT);
        b.iter(|| {
            assert_eq!(part_2(black_box(&parse_output)), 204837);
        });
    }
}
