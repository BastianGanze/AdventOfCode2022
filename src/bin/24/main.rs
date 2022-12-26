#![feature(test)]

pub mod grid;
use colored::*;

type Sol = i32;
use crate::grid::{manhattan_distance, Field, FieldType, Grid, StormMask, DOWN, LEFT, RIGHT, TOP};
use colored::Color::Black;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::io::stdin;
use std::{thread, time, usize};

pub type Point = (usize, usize);
pub type Storm = (Point, StormMask);
pub type ParseOutput = (usize, usize, Point, Point, Vec<Storm>);

#[derive(Debug, Clone)]
pub enum Dir {
    North,
    East,
    South,
    West,
}

pub type TimeNodeKey = (usize, Point);

#[derive(Debug, Clone)]
pub struct TimeNode {
    coordinate: Point,
    step: usize,
    edges: Vec<TimeNodeKey>,
}

pub type TimeGraph = HashMap<TimeNodeKey, TimeNode>;

const MAIN_INPUT: &str = include_str!("main_input");

pub fn parse(file: &str) -> ParseOutput {
    let lines = file.lines().collect::<Vec<&str>>();
    let (width, height) = (lines[0].len(), lines.len());
    let mut start = (0, 0);
    let mut end = (0, 0);
    let v: Vec<Storm> = lines
        .iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter_map(|(x, c)| match c {
                    '.' => {
                        if y == 0 {
                            start = (y, x);
                        }
                        if y == lines.len() - 1 {
                            end = (y, x);
                        }
                        None
                    }
                    '>' => Some(((y, x), RIGHT)),
                    '<' => Some(((y, x), LEFT)),
                    '^' => Some(((y, x), TOP)),
                    'v' => Some(((y, x), DOWN)),
                    _ => None,
                })
                .collect::<Vec<Storm>>()
        })
        .collect();

    (width, height, start, end, v)
}

fn part_1((width, height, (start_y, start_x), (end_y, end_x), storms_start): &ParseOutput) -> Sol {
    println!("{:?}", (TOP, RIGHT, DOWN, LEFT));

    let (possible_storm_states, cycle_count) =
        generate_possible_storm_states(width, height, start_x, end_x, storms_start);

    let mut known_fields = HashSet::new();
    let mut open_fields = BinaryHeap::<Field>::new();
    let end = (*end_y, *end_x);
    open_fields.push(Field::new((*start_y, *start_x), 0, 0));
    let mut currently_known_shortest_path = usize::MAX;
    let mut current_shortest_distance = u32::MAX;
    while !open_fields.is_empty() {
        let current_field = open_fields.pop().unwrap();
        if !known_fields.insert(current_field) {
            continue;
        }
        if end == current_field.coordinate
            && current_field.current_step < currently_known_shortest_path
        {
            currently_known_shortest_path = current_field.current_step;
            println!("new shorter path found {}", currently_known_shortest_path);
            println!("stats o: {} h: {}", open_fields.len(), known_fields.len());
        }
        if current_field.current_step > currently_known_shortest_path {
            continue;
        }
        let (field_y, field_x) = current_field.coordinate;
        let storm_state_next_step =
            &possible_storm_states[(current_field.current_step + 1) % cycle_count];

        let neighbours = storm_state_next_step.get_possible_positions(field_y, field_x);
        if manhattan_distance((field_y, field_x), end) < current_shortest_distance {
            current_shortest_distance = manhattan_distance((field_y, field_x), end);
            println!(
                "{:?} o: {} h: {}",
                current_shortest_distance,
                open_fields.len(),
                known_fields.len()
            );
        }

        for (y, x) in neighbours {
            //manhattan_distance((y, x), end)
            open_fields.push(Field::new(
                (y, x),
                current_field.cost + get_cost(&current_field, (y, x), end),
                current_field.current_step + 1,
            ));
        }
        if !storm_state_next_step.is_field_full(field_y, field_x) {
            open_fields.push(Field::new(
                (field_y, field_x),
                current_field.cost + get_cost(&current_field, (field_y, field_x), end),
                current_field.current_step + 1,
            ));
        }
    }
    currently_known_shortest_path as Sol
}

fn get_cost(_field: &Field, point: Point, end: Point) -> u32 {
    manhattan_distance(point, end)
}

fn generate_possible_storm_states(
    width: &usize,
    height: &usize,
    start_x: &usize,
    end_x: &usize,
    storms_start: &Vec<Storm>,
) -> (Vec<Grid>, usize) {
    let mut grid = generate_empty_valley(width, height, start_x, end_x);

    let (h, w) = (*height as Sol - 2, *width as Sol - 2);
    let cycle_count = lcm(h as usize, w as usize);
    let mut possible_storm_states = vec![grid; cycle_count];

    for i in 0..cycle_count {
        for ((y, x), storm_mask) in storms_start {
            let (s_y, s_x) = match *storm_mask {
                TOP => (move_and_mod_to_range(1, h, *y as Sol, -(i as Sol)), *x),
                RIGHT => (*y, move_and_mod_to_range(1, w, *x as Sol, i as Sol)),
                DOWN => (move_and_mod_to_range(1, h, *y as Sol, i as Sol), *x),
                LEFT => (*y, move_and_mod_to_range(1, w, *x as Sol, -(i as Sol))),
                _ => unreachable!(),
            };
            possible_storm_states[i].add_storm(s_y, s_x, *storm_mask);
        }
    }
    (possible_storm_states, cycle_count)
}

fn move_and_mod_to_range(start: Sol, end: Sol, pos: Sol, n: Sol) -> usize {
    let p = pos - start;
    let len = end - start + 1;
    (start + modulo(p + n, len)) as usize
}

fn modulo(s: Sol, o: Sol) -> Sol {
    ((s % o) + o) % o
}

fn generate_empty_valley(width: &usize, height: &usize, start_x: &usize, end_x: &usize) -> Grid {
    let mut grid = Grid::new((*height, *width));
    for x in 0..*width {
        if x != *start_x {
            grid.mark_field(0, x, FieldType::Rock);
        }
        if x != *end_x {
            grid.mark_field(height - 1, x, FieldType::Rock);
        }
    }
    for y in 0..*height {
        grid.mark_field(y, 0, FieldType::Rock);
        grid.mark_field(y, width - 1, FieldType::Rock);
    }
    grid
}

fn part_2(parse_output: &ParseOutput) -> Sol {
    0
}

fn gcd(a: usize, b: usize) -> usize {
    match ((a, b), (a & 1, b & 1)) {
        ((x, y), _) if x == y => y,
        ((0, x), _) | ((x, 0), _) => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
        ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1)) => {
            let (x, y) = (x.min(y), x.max(y));
            gcd((y - x) >> 1, x)
        }
        _ => unreachable!(),
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
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
        assert_eq!(part_1(&parse_output), 18);
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
