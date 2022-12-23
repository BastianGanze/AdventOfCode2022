#![feature(test)]

use crate::cycle::Cycle;
use colored::*;

pub mod cycle;

type Sol = i32;
use std::io::stdin;
use std::{thread, time};

pub type Walls = Vec<Sol>;
pub type Line = (Sol, Sol, Walls);
pub type Row = Line;
pub type Column = Line;
pub type Grid = (Vec<Row>, Vec<Column>, Sol, Sol);
pub type ParseOutput = (Grid, Vec<Move>);
#[derive(Debug, Clone)]
pub enum Move {
    Steps(Sol),
    Turn(Sol),
}
#[derive(Debug, Clone)]
pub enum Dir {
    Top(Sol),
    Left(Sol),
    Right(Sol),
    Down(Sol),
}

const MAIN_INPUT: &str = include_str!("main_input");

pub fn parse(file: &str) -> ParseOutput {
    let (gg, mm) = file.split_once("\n\n").unwrap();
    let lines = gg
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let grid_max_x = lines.iter().map(|l| l.len()).max().unwrap() - 1;
    let (mut rows, mut columns) = (Vec::<Row>::new(), Vec::<Column>::new());
    for (_, chars) in lines.iter().enumerate() {
        let (mut started, mut start, mut end, mut walls) = (false, 0, 0, Vec::<Sol>::new());
        for (x, c) in chars.iter().enumerate() {
            if *c == '#' {
                walls.push(x as Sol)
            }
            if (*c == '.' || *c == '#') && !started {
                started = true;
                start = x;
            }
            end = x;
        }
        rows.push((start as Sol, end as Sol, walls));
    }

    for x in 0..=grid_max_x {
        let (mut started, mut start, mut end, mut walls) = (false, 0, 0, Vec::<Sol>::new());
        for (y, l) in lines.iter().filter(|l| x < l.len()).enumerate() {
            let c = l[x];
            if c == '#' {
                walls.push(y as Sol)
            }
            if (c == '.' || c == '#') && !started {
                started = true;
                start = y;
            }
            end = y;
        }
        columns.push((start as Sol, end as Sol, walls));
    }

    (
        (rows, columns, grid_max_x as Sol, (lines.len() - 1) as Sol),
        mm.replace('L', ",L,")
            .replace('R', ",R,")
            .trim()
            .split(',')
            .map(|m| match m {
                "L" => Move::Turn(-1),
                "R" => Move::Turn(1),
                n => Move::Steps(n.parse().unwrap()),
            })
            .collect(),
    )
}

fn part_1(parse_output: &ParseOutput) -> Sol {
    let ((rows, columns, _, _), moves) = parse_output;
    let mut dir: Cycle<Dir, 4> =
        Cycle::new([Dir::Right(1), Dir::Down(1), Dir::Left(-1), Dir::Top(-1)]);
    let (mut y, mut x) = (0, rows[0].0);

    for m in moves {
        match m {
            Move::Steps(n) => match dir.current() {
                Dir::Left(sign) | Dir::Right(sign) => x = calc_new_pos(rows, y, x, n, sign),
                Dir::Top(sign) | Dir::Down(sign) => y = calc_new_pos(columns, x, y, n, sign),
            },
            Move::Turn(t) => dir.turn(*t),
        }
    }
    calc_solution(dir.current(), y, x)
}

fn calc_new_pos(line: &[Line], i: Sol, pos: Sol, n: &Sol, sign: &Sol) -> Sol {
    let (start, end, walls) = &line[i as usize];
    let real_n = move_amount(pos, *sign, *n, walls, *start, *end);
    move_and_mod_to_range(*start, *end, pos, real_n)
}

fn calc_solution(dir: &Dir, y: Sol, x: Sol) -> i32 {
    (y + 1) * 1000
        + (x + 1) * 4
        + match dir {
            Dir::Top(_) => 3,
            Dir::Left(_) => 2,
            Dir::Right(_) => 0,
            Dir::Down(_) => 1,
        }
}

fn move_amount(pos: Sol, dir: Sol, n: Sol, walls: &Vec<Sol>, start: Sol, end: Sol) -> Sol {
    if walls.is_empty() {
        return n * dir;
    }

    let d = distance_to_wall(pos, dir, walls, start, end);
    if d.abs() < n {
        d
    } else {
        n * dir
    }
}

fn distance_to_wall(pos: Sol, dir: Sol, walls: &Vec<Sol>, start: Sol, end: Sol) -> Sol {
    let len = end - start + 1;
    match dir {
        1 => {
            let w = if let Some(w) = walls.iter().find(|p| **p > pos) {
                *w - start
            } else {
                walls[0] - start
            };
            let p = pos - start;
            if w < p {
                (len - p) + w - 1
            } else {
                w - p - 1
            }
        }
        -1 => {
            let w = if let Some(w) = walls.iter().rev().find(|p| **p < pos) {
                *w - start
            } else {
                walls[walls.len() - 1] - start
            };
            let p = pos - start;
            if w > p {
                -((len - w) + p) + 1
            } else {
                w - p + 1
            }
        }
        _ => unreachable!(),
    }
}

fn move_and_mod_to_range(start: Sol, end: Sol, pos: Sol, n: Sol) -> Sol {
    let p = pos - start;
    let len = end - start + 1;
    start + modulo(p + n, len)
}

fn modulo(s: Sol, o: Sol) -> Sol {
    ((s % o) + o) % o
}

fn part_2(parse_output: &ParseOutput) -> Sol {
    0
}

fn print_level(
    ((rows, _, max_x, max_y), _): &ParseOutput,
    current_position: &(Sol, Sol),
    n: (&Dir, Sol, Sol),
    (player, wall, empty): &(ColoredString, ColoredString, ColoredString),
) {
    print!("{esc}c", esc = 27 as char);
    thread::sleep(time::Duration::from_millis(400));
    for y in 0..=*max_y {
        if current_position.0.abs_diff(y) > 20 {
            continue;
        }
        for x in 0..=*max_x {
            if (y, x) == *current_position {
                print!("{}", player);
                continue;
            }
            let (start, end, walls) = &rows[y as usize];
            if x >= *start && x <= *end {
                if walls.contains(&x) {
                    print!("{}", wall);
                } else {
                    print!("{}", empty);
                }
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!("{:?}", n);
    let mut s = String::new();
    stdin().read_line(&mut s);
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
        assert_eq!(part_1(&parse_output), 6032);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = parse(TEST_INPUT);
        assert_eq!(part_2(&parse_output), 5031);
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
            assert_eq!(part_1(black_box(&parse_output)), 1484);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = parse(MAIN_INPUT);
        b.iter(|| {
            assert_eq!(part_2(black_box(&parse_output)), 55);
        });
    }
}
