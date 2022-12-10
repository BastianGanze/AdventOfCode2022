#![feature(test)]
#![feature(drain_filter)]

type Solution = i32;
#[derive(Debug, Clone)]
pub enum Instruction {
    Noop,
    Add(i32),
}
pub type ParseOutput = Vec<(usize, Instruction)>;
const MAIN_INPUT: &str = include_str!("main_input");

pub fn parse(file: &str) -> ParseOutput {
    let mut cycle = 0;
    file.lines()
        .map(|l| {
            if let Some((_, length)) = l.split_once(' ') {
                cycle += 2;
                (cycle, Instruction::Add(length.parse().unwrap()))
            } else {
                cycle += 1;
                (cycle, Instruction::Noop)
            }
        })
        .collect()
}

fn part_1(parse_output: &ParseOutput) -> Solution {
    let mut solution = 0;
    let cycle_distance = 40;
    let mut check_cycle = 19;
    let mut current_signal = 1;
    let mut instructions: Vec<(usize, Instruction)> =
        parse_output.clone().into_iter().rev().collect();
    let cycles = parse_output.last().unwrap().0;
    let mut current_instruction: (usize, Instruction) = instructions.pop().unwrap();

    for cycle in 1..cycles {
        let (exec_cycle, instruction) = &current_instruction;
        if *exec_cycle == cycle {
            match instruction {
                Instruction::Add(l) => current_signal += l,
                Instruction::Noop => {}
            }
            current_instruction = instructions.pop().unwrap_or((240, Instruction::Noop));
        }
        if cycle == check_cycle {
            check_cycle += cycle_distance;
            solution += (cycle + 1) as i32 * current_signal;
        }
    }

    solution
}

fn part_2(parse_output: &ParseOutput) -> String {
    let mut solution: Vec<char> = Vec::new();
    solution.push('\n');
    let width = 40;
    let mut current_signal = 1;
    let mut instructions: Vec<(usize, Instruction)> =
        parse_output.clone().into_iter().rev().collect();
    let cycles = parse_output.last().unwrap().0;
    let mut current_instruction: (usize, Instruction) = instructions.pop().unwrap();

    for cycle in 0..cycles {
        let (exec_cycle, instruction) = &current_instruction;
        if *exec_cycle == cycle {
            match instruction {
                Instruction::Add(l) => current_signal += l,
                Instruction::Noop => {}
            }
            current_instruction = instructions.pop().unwrap_or((240, Instruction::Noop));
        }
        let pixel = cycle % 40;

        match current_signal - pixel as i32 {
            -1 | 0 | 1 => {
                solution.push('#');
            }
            _ => solution.push('.'),
        }
        if pixel + 1 == width {
            solution.push('\n');
        }
    }

    solution.into_iter().collect()
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
        assert_eq!(part_1(&parse_output), 13140);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = parse(TEST_INPUT);
        let test_2 = "\n\
##..##..##..##..##..##..##..##..##..##..\n\
###...###...###...###...###...###...###.\n\
####....####....####....####....####....\n\
#####.....#####.....#####.....#####.....\n\
######......######......######......####\n\
#######.......#######.......#######.....\n";
        assert_eq!(part_2(&parse_output), test_2);
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
            assert_eq!(part_1(black_box(&parse_output)), 15360);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = parse(MAIN_INPUT);
        let solution_2 = "\n\
###..#..#.#....#..#...##..##..####..##..\n\
#..#.#..#.#....#..#....#.#..#....#.#..#.\n\
#..#.####.#....####....#.#......#..#..#.\n\
###..#..#.#....#..#....#.#.##..#...####.\n\
#....#..#.#....#..#.#..#.#..#.#....#..#.\n\
#....#..#.####.#..#..##...###.####.#..#.\n";

        b.iter(|| {
            assert_eq!(part_2(black_box(&parse_output)), solution_2);
        });
    }
}
