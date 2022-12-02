#![feature(test)]

type Solution = u32;
pub type ParseOutput = Vec<(u32, u32)>;
const MAIN_INPUT: &str = include_str!("main_input");
const TEST_INPUT: &str = include_str!("test_input");
const ROCK: u32 = 1;
const PAPER: u32 = 2;
const SCISSORS: u32 = 3;
const LOOSE: u32 = 1;
const DRAW: u32 = 2;
const WIN: u32 = 3;

pub fn parse(file: &str) -> ParseOutput {
    file.lines()
        .map(|m: &str| {
            let mut moves = m.chars().enumerate().filter(|(i, _c)| *i != 1);
            (
                get_move_from_char(&moves.next().unwrap().1).unwrap(),
                get_move_from_char(&moves.next().unwrap().1).unwrap(),
            )
        })
        .collect()
}

pub fn get_move_from_char(c: &char) -> Option<u32> {
    match c {
        'A' => Some(ROCK),
        'B' => Some(PAPER),
        'C' => Some(SCISSORS),
        'X' => Some(ROCK),
        'Y' => Some(PAPER),
        'Z' => Some(SCISSORS),
        _ => None,
    }
}

fn part_1(parse_output: &ParseOutput) -> Solution {
    parse_output
        .clone()
        .into_iter()
        .map(|game_match| {
            let me = game_match.1;
            let opponent = game_match.0;

            me + get_outcome_value(me, opponent)
        })
        .sum()
}

fn part_2(parse_output: &ParseOutput) -> Solution {
    let mut solution = 0;
    for game_match in parse_output {
        let opponent = game_match.0;
        let match_outcome = game_match.1;

        let me = select_appropriate_move(opponent, match_outcome);
        solution += me + get_outcome_value(me, opponent);
    }
    solution
}

fn get_outcome_value(me: u32, opponent: u32) -> u32 {
    if me == opponent {
        3
    } else if (me == ROCK && opponent == SCISSORS)
        || (me == PAPER && opponent == ROCK)
        || (me == SCISSORS && opponent == PAPER)
    {
        6
    } else {
        0
    }
}

fn select_appropriate_move(opponent: u32, match_outcome: u32) -> u32 {
    match match_outcome {
        DRAW => opponent,
        WIN => {
            if opponent == SCISSORS {
                ROCK
            } else if opponent == PAPER {
                SCISSORS
            } else {
                PAPER
            }
        }
        LOOSE => {
            if opponent == SCISSORS {
                PAPER
            } else if opponent == PAPER {
                ROCK
            } else {
                SCISSORS
            }
        }
        _ => 0,
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

    #[test]
    pub fn test_part_1() {
        let parse_output = parse(TEST_INPUT);
        assert_eq!(part_1(&parse_output), 15);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = parse(TEST_INPUT);
        assert_eq!(part_2(&parse_output), 12);
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
            assert_eq!(part_1(black_box(&parse_output)), 9759);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = parse(MAIN_INPUT);
        b.iter(|| {
            assert_eq!(part_2(black_box(&parse_output)), 12429);
        });
    }
}
