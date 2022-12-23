#![feature(test)]

use crate::cycle::Cycle;
use colored::*;

pub mod cycle;

type Sol = i32;
use std::collections::{HashMap, HashSet};
use std::io::stdin;
use std::{thread, time};

pub type Elf = (Sol, Sol);
pub type ParseOutput = HashSet<Elf>;

#[derive(Debug, Clone)]
pub enum Dir {
    North,
    East,
    South,
    West,
}

const MAIN_INPUT: &str = include_str!("main_input");

pub fn parse(file: &str) -> ParseOutput {
    file.lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    if c == '#' {
                        Some((y as Sol, x as Sol))
                    } else {
                        None
                    }
                })
                .collect::<Vec<Elf>>()
        })
        .collect()
}

fn part_1(parse_output: &ParseOutput) -> Sol {
    let mut elves = parse_output.clone();
    let dirs = Cycle::new([Dir::North, Dir::South, Dir::West, Dir::East]);
    let mut proposals: HashMap<(Sol, Sol), (Elf, Sol)> = HashMap::new();

    for i in 0..10 {
        move_elves(&mut elves, &dirs, i, &mut proposals);
    }

    let (y_min, x_min, y_max, x_max) = min_max_elves(&elves);
    (x_max - x_min + 1) * (y_max - y_min + 1) - elves.len() as Sol
}

fn part_2(parse_output: &ParseOutput) -> Sol {
    let mut elves = parse_output.clone();
    let dirs = Cycle::new([Dir::North, Dir::South, Dir::West, Dir::East]);
    let mut proposals: HashMap<(Sol, Sol), (Elf, Sol)> = HashMap::new();

    let mut i = 0;
    while move_elves(&mut elves, &dirs, i, &mut proposals) {
        i += 1;
    }
    i + 1
}

fn move_elves(
    elves: &mut HashSet<Elf>,
    dirs: &Cycle<Dir, 4>,
    dir_start_i: Sol,
    proposals: &mut HashMap<(Sol, Sol), (Elf, Sol)>,
) -> bool {
    for (y, x) in elves.iter() {
        if !has_neighbours(elves, y, x) {
            continue;
        }
        for i in dir_start_i..dir_start_i + 4 {
            if match dirs.get(i) {
                Dir::North => propose(elves, proposals, *y, *x, y - 1, *x, false),
                Dir::East => propose(elves, proposals, *y, *x, *y, x + 1, true),
                Dir::South => propose(elves, proposals, *y, *x, y + 1, *x, false),
                Dir::West => propose(elves, proposals, *y, *x, *y, x - 1, true),
            } {
                break;
            }
        }
    }
    if proposals.is_empty() {
        return false;
    }
    for ((y, x), (elf, count)) in proposals.drain() {
        if count > 1 {
            continue;
        }
        if elves.remove(&elf) {
            elves.insert((y, x));
        }
    }
    true
}

fn has_neighbours(elves: &HashSet<Elf>, y: &Sol, x: &Sol) -> bool {
    elves.contains(&(y - 1, x - 1))
        || elves.contains(&(y - 1, *x))
        || elves.contains(&(y - 1, x + 1))
        || elves.contains(&(*y, x - 1))
        || elves.contains(&(*y, x + 1))
        || elves.contains(&(y + 1, x - 1))
        || elves.contains(&(y + 1, *x))
        || elves.contains(&(y + 1, x + 1))
}

fn min_max_elves(elves: &HashSet<Elf>) -> (Sol, Sol, Sol, Sol) {
    let ((y_min, x_min), (y_max, x_max)) = elves.iter().fold(
        ((Sol::MAX, Sol::MAX), (Sol::MIN, Sol::MIN)),
        |((y_min, x_min), (y_max, x_max)), (y, x)| {
            (
                (y_min.min(*y), x_min.min(*x)),
                (y_max.max(*y), x_max.max(*x)),
            )
        },
    );
    (y_min, x_min, y_max, x_max)
}

fn propose(
    elves: &HashSet<Elf>,
    proposals: &mut HashMap<(Sol, Sol), (Elf, Sol)>,
    y: Sol,
    x: Sol,
    n_y: Sol,
    n_x: Sol,
    horizontal: bool,
) -> bool {
    let h = Sol::from(horizontal);
    let v = Sol::from(!horizontal);
    if !elves.contains(&(n_y - h, n_x - v))
        && !elves.contains(&(n_y, n_x))
        && !elves.contains(&(n_y + h, n_x + v))
    {
        proposals.entry((n_y, n_x)).or_insert(((y, x), 0)).1 += 1;
        return true;
    }
    false
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
        assert_eq!(part_1(&parse_output), 110);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = parse(TEST_INPUT);
        assert_eq!(part_2(&parse_output), 20);
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
            assert_eq!(part_1(black_box(&parse_output)), 4158);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = parse(MAIN_INPUT);
        b.iter(|| {
            assert_eq!(part_2(black_box(&parse_output)), 1014);
        });
    }
}

/*fn print_level(elves: &ParseOutput, (y_min, x_min, y_max, x_max): &(Sol, Sol, Sol, Sol)) {
    //print!("{esc}c", esc = 27 as char);
    thread::sleep(time::Duration::from_millis(400));
    for y in *y_min..=*y_max {
        for x in *x_min..=*x_max {
            if elves.contains(&(y, x)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
    let mut s = String::new();
    stdin().read_line(&mut s);
}*/
