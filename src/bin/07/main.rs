#![feature(test)]

use std::cmp::min;

type Solution = u64;

#[derive(Debug, Clone)]
pub struct FileSystemEntry {
    is_dir: bool,
    file_name: String,
    children: Vec<usize>,
    parent: usize,
    size: u64,
}

impl FileSystemEntry {
    pub fn new(is_dir: bool, name: String, size: u64, parent: usize) -> FileSystemEntry {
        FileSystemEntry {
            is_dir,
            file_name: name,
            children: Vec::new(),
            parent,
            size,
        }
    }
}
pub type ParseOutput = Vec<FileSystemEntry>;
const MAIN_INPUT: &str = include_str!("main_input");
const TEST_INPUT: &str = include_str!("test_input");
const MAX_SIZE_P1: u64 = 100000;

const MAX_SPACE_P2: u64 = 70000000;
const MIN_REQUIRED_SPACE_P2: u64 = 30000000;

pub fn parse(file: &str) -> ParseOutput {
    let mut filesystem: Vec<FileSystemEntry> = Vec::new();
    let mut current_dir_i: usize = 0;

    for l in file.lines() {
        let mut command_elements = l.split(' ');
        let first = command_elements.next().unwrap();
        let second = command_elements.next().unwrap();

        match first {
            "$" => match second {
                "cd" => {
                    let third = command_elements.next().unwrap();
                    match third {
                        ".." => {
                            if let Some(current_dir) = filesystem.get_mut(current_dir_i) {
                                current_dir_i = current_dir.parent;
                            }
                        }
                        dir => {
                            let file_system_i = filesystem.len();
                            if let Some(current_dir) = filesystem.get_mut(current_dir_i) {
                                current_dir.children.push(file_system_i);
                            }

                            filesystem.push(FileSystemEntry::new(
                                true,
                                dir.into(),
                                0,
                                current_dir_i,
                            ));
                            current_dir_i = filesystem.len() - 1;
                        }
                    }
                }
                "ls" => {}
                _ => unreachable!(),
            },
            "dir" => {}
            size => {
                let file_size = size.parse().unwrap();
                let file_system_i = filesystem.len();

                let current_dir = filesystem.get_mut(current_dir_i).unwrap();
                current_dir.children.push(file_system_i);

                update_parent_file_sizes(&mut filesystem, current_dir_i, file_size);

                filesystem.push(FileSystemEntry::new(
                    false,
                    second.into(),
                    file_size,
                    current_dir_i,
                ));
            }
        }
    }

    filesystem
}

fn update_parent_file_sizes(
    filesystem: &mut Vec<FileSystemEntry>,
    current_dir_i: usize,
    size: u64,
) {
    let mut cd_i = current_dir_i;
    while cd_i != 0 {
        let current_dir = filesystem.get_mut(cd_i).unwrap();
        current_dir.size += size;
        cd_i = current_dir.parent;
    }
    let current_dir = filesystem.get_mut(cd_i).unwrap();
    current_dir.size += size;
}

fn part_1(parse_output: &ParseOutput) -> Solution {
    parse_output
        .iter()
        .filter(|f| f.is_dir && f.size < MAX_SIZE_P1)
        .map(|f| f.size)
        .sum()
}

fn part_2(parse_output: &ParseOutput) -> Solution {
    let unused_space = MAX_SPACE_P2 - parse_output.first().unwrap().size;
    let mut solution = MAX_SPACE_P2;

    for f in parse_output.iter().filter(|f| f.is_dir) {
        if unused_space + f.size >= MIN_REQUIRED_SPACE_P2 {
            solution = min(solution, f.size);
        }
    }

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

    #[test]
    pub fn test_part_1() {
        let parse_output = parse(TEST_INPUT);
        assert_eq!(part_1(&parse_output), 7);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = parse(TEST_INPUT);
        assert_eq!(part_2(&parse_output), 19);
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
            assert_eq!(part_1(black_box(&parse_output)), 1542);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = parse(MAIN_INPUT);
        b.iter(|| {
            assert_eq!(part_2(black_box(&parse_output)), 3153);
        });
    }
}
