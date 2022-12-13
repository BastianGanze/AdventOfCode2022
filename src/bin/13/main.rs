#![feature(test)]

pub mod package;
use package::Package;

type Solution = u32;

pub type ParseOutput = Vec<(Package, Package)>;
const MAIN_INPUT: &str = include_str!("main_input");

pub fn parse(file: &str) -> ParseOutput {
    file.split("\n\n")
        .map(|l| {
            let (p1, p2) = l.split_once('\n').unwrap();
            (
                Package::new(p1.trim().into()),
                Package::new(p2.trim().into()),
            )
        })
        .collect()
}

fn part_1(parse_output: &ParseOutput) -> Solution {
    parse_output
        .iter()
        .enumerate()
        .map(|(i, (p1, p2))| if p1 < p2 { i as u32 + 1 } else { 0 })
        .sum()
}

fn part_2(parse_output: &ParseOutput) -> Solution {
    let mut packages =
        parse_output
            .iter()
            .fold(Vec::with_capacity(parse_output.len() * 2), |mut acc, p| {
                let (p1, p2) = p;
                acc.push(p1);
                acc.push(p2);
                acc
            });
    let (d1, d2) = (Package::new("[[2]]".into()), Package::new("[[6]]".into()));
    packages.push(&d1);
    packages.push(&d2);
    packages.sort();
    packages.iter().enumerate().fold(1, |acc, (i, p)| {
        if p == &&d1 || p == &&d2 {
            acc * (i as u32 + 1)
        } else {
            acc
        }
    })
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
        assert_eq!(part_2(&parse_output), 140);
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
            assert_eq!(part_1(black_box(&parse_output)), 5340);
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
