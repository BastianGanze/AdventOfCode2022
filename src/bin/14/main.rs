#![feature(test)]

pub mod grid;

use crate::grid::FieldType;
use grid::Grid;
use std::cmp::{max, min};
use std::fmt::format;
use std::usize;

type Solution = u32;

type RockLines = Vec<Vec<(usize, usize)>>;

pub type ParseOutput = Grid;
const MAIN_INPUT: &str = include_str!("main_input");

pub fn parse(file: &str) -> ParseOutput {
    let mut min_c = (usize::MAX, usize::MAX);
    let mut max_c = (0, 0);

    let rock_lines: RockLines = file
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|r| {
                    let (x_s, y_s) = r.split_once(',').unwrap();
                    let (y, x) = (y_s.parse().unwrap(), x_s.parse().unwrap());
                    min_c = (min(min_c.0, y), min(min_c.1, x));
                    max_c = (max(max_c.0, y), max(max_c.1, x));
                    (y, x)
                })
                .collect()
        })
        .collect();

    let mut grid = Grid::new((max_c.0 + 4, max_c.1 + 500));
    for rock_line in rock_lines {
        for m in rock_line[..].windows(2) {
            match m {
                &[(y0, x0), (y1, x1)] => {
                    let (min_y, min_x) = ((min(y0, y1)), (min(x0, x1)));
                    let (max_y, max_x) = ((max(y0, y1)), (max(x0, x1)));
                    for y in min_y..max_y + 1 {
                        for x in min_x..max_x + 1 {
                            grid.mark_field(y, x, FieldType::Rock);
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    grid
}

fn part_1(parse_output: &ParseOutput) -> Solution {
    let mut solution = 0;
    let mut grid = parse_output.clone();
    let grid_max_y = grid.get_size().0;
    'outer: loop {
        let mut sand_corn = (0, 500);
        solution += 1;
        loop {
            let possible_positions = grid.get_sandcorn_positions(sand_corn.0, sand_corn.1);
            if possible_positions.is_empty() && sand_corn == (0, 500) {
                break 'outer;
            }
            if possible_positions.is_empty() {
                grid.mark_field(sand_corn.0, sand_corn.1, FieldType::Sand);
                break;
            }
            sand_corn = (possible_positions[0].0, possible_positions[0].1);
            if sand_corn.0 + 2 > grid_max_y {
                break 'outer;
            }
        }
    }
    solution - 1
}

fn part_2(parse_output: &ParseOutput) -> Solution {
    let mut solution = 0;
    let mut grid = parse_output.clone();
    let floor = grid.get_size().0 - 2;
    for x in 0..grid.get_size().1 {
        grid.mark_field(floor, x, FieldType::Rock);
    }
    'outer: loop {
        let mut sand_corn = (0, 500);
        solution += 1;
        loop {
            let possible_positions = grid.get_sandcorn_positions(sand_corn.0, sand_corn.1);
            if possible_positions.is_empty() {
                grid.mark_field(sand_corn.0, sand_corn.1, FieldType::Sand);
                if sand_corn == (0, 500) {
                    break 'outer;
                }
                break;
            }
            sand_corn = (possible_positions[0].0, possible_positions[0].1);
        }
    }
    /*println!(
        "{}",
        grid.fields
            .iter()
            .map(|f| format!(
                "{}\n",
                f.iter()
                    .skip(0)
                    .map(|g| match g.1 {
                        FieldType::Air => ' ',
                        FieldType::Rock => '#',
                        FieldType::Sand => 'o',
                    })
                    .collect::<String>()
            ))
            .collect::<String>()
    );*/
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
        assert_eq!(part_1(&parse_output), 24);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = parse(TEST_INPUT);
        assert_eq!(part_2(&parse_output), 93);
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
            assert_eq!(part_1(black_box(&parse_output)), 832);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = parse(MAIN_INPUT);
        b.iter(|| {
            assert_eq!(part_2(black_box(&parse_output)), 27601);
        });
    }
}
