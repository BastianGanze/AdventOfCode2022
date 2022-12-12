#![feature(test)]

pub mod grid;

use crate::grid::{manhattan_distance, CostType, Field, Grid};
use std::cmp::min;
use std::collections::BinaryHeap;

type Solution = u32;
type ParseOutput = (Grid, (usize, usize), (usize, usize));

const MAIN_INPUT: &str = include_str!("main_input");

pub fn parse(file: &str) -> ParseOutput {
    let mut grid = Grid::new((file.lines().count(), file.lines().next().unwrap().len()));
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (y, l) in file.lines().enumerate() {
        for (x, character) in l.bytes().enumerate() {
            grid.set_field_height(
                y,
                x,
                match character {
                    83 => {
                        start = (y, x);
                        0
                    }
                    69 => {
                        end = (y, x);
                        25
                    }
                    c => c as CostType - 97,
                },
            );
        }
    }

    (grid, start, end)
}

fn part_1(parse_output: &ParseOutput) -> Solution {
    calc_fastest_path(parse_output.clone())
}

fn part_2(parse_output: &ParseOutput) -> Solution {
    let (grid, _, end_coords) = parse_output.clone();
    let size = grid.get_size();
    let mut fastest = u32::MAX;

    for y in 0..size.0 {
        for x in 0..size.1 {
            if grid.get_field_height(y, x) == 0 {
                let path_len = calc_fastest_path((grid.clone(), (y, x), end_coords));
                if path_len == 0 {
                    continue;
                }
                fastest = min(fastest, path_len);
            }
        }
    }
    fastest
}

fn calc_fastest_path(out: ParseOutput) -> Solution {
    let (mut grid, start_coord, end_coords) = out;
    let mut open_fields = BinaryHeap::<Field>::new();
    let start = Field::new(start_coord, 0, 0);
    grid.mark_field(start_coord.0, start_coord.1);

    open_fields.push(start);

    'outer: while !open_fields.is_empty() {
        let current_field = open_fields.pop().unwrap();
        let neighbours =
            grid.get_unmarked_neighbours(current_field.coordinate.0, current_field.coordinate.1);
        for (y, x, height) in neighbours {
            let field_cost = calc_cost(
                &(
                    current_field.coordinate,
                    grid.get_field_height(current_field.coordinate.0, current_field.coordinate.1),
                ),
                (y, x, height),
                &end_coords,
            );
            if field_cost < u32::MAX {
                if (y, x) == end_coords {
                    return current_field.path_length + 1;
                }
                grid.mark_field(y, x);
                open_fields.push(Field::new(
                    (y, x),
                    current_field.cost + field_cost,
                    current_field.path_length + 1,
                ));
            }
        }
    }

    0
}

fn calc_cost(
    field: &((usize, usize), CostType),
    neighbour: (usize, usize, CostType),
    end: &(usize, usize),
) -> u32 {
    if (neighbour.2 - field.1) > 1 {
        return u32::MAX;
    }

    manhattan_distance(field.0, *end)
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
        assert_eq!(part_1(&parse_output), 31);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = parse(TEST_INPUT);
        assert_eq!(part_2(&parse_output), 29);
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
            assert_eq!(part_1(black_box(&parse_output)), 330);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = parse(MAIN_INPUT);
        b.iter(|| {
            assert_eq!(part_2(black_box(&parse_output)), 321);
        });
    }
}
