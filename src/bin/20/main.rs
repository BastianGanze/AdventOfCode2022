#![feature(test)]

use std::collections::HashSet;

type Sol = i64;

pub type ParseOutput = Vec<Sol>;
const DECRYPTION_KEY: Sol = 811589153;
const MAIN_INPUT: &str = include_str!("main_input");

pub fn parse(file: &str) -> ParseOutput {
    file.lines().map(|l| l.parse().unwrap()).collect()
}

fn main() {
    let parse_output = parse(MAIN_INPUT);
    println!("Solution to part 1 is {}", part_1(&parse_output));
    println!("Solution to part 2 is {}", part_2(&parse_output));
}

fn part_1(parse_output: &ParseOutput) -> Sol {
    let mut mixed_numbers = parse_output.clone();
    let n_len = mixed_numbers.len() as Sol;
    let mut o_num_i_to_mix_num_i: Vec<Sol> = (0..n_len).collect();
    let mut mix_num_i_to_o_num_i: Vec<Sol> = (0..n_len).collect();
    mix_numbers(
        &mut mixed_numbers,
        &mut o_num_i_to_mix_num_i,
        &mut mix_num_i_to_o_num_i,
    );
    extract_solution(&mixed_numbers)
}

fn part_2(parse_output: &ParseOutput) -> Sol {
    let mut mixed_numbers: Vec<Sol> = parse_output.iter().map(|n| n * DECRYPTION_KEY).collect();
    let n_len = mixed_numbers.len() as Sol;
    let mut o_num_i_to_mix_num_i: Vec<Sol> = (0..n_len).collect();
    let mut mix_num_i_to_o_num_i: Vec<Sol> = (0..n_len).collect();
    for _ in 0..10 {
        mix_numbers(
            &mut mixed_numbers,
            &mut o_num_i_to_mix_num_i,
            &mut mix_num_i_to_o_num_i,
        );
    }
    extract_solution(&mixed_numbers)
}

fn extract_solution(mixed_numbers: &Vec<Sol>) -> Sol {
    let index_0 = mixed_numbers
        .iter()
        .enumerate()
        .find_map(|(i, n)| if *n == 0 { Some(i) } else { None })
        .unwrap();
    mixed_numbers[(index_0 + 1000) % mixed_numbers.len()]
        + mixed_numbers[(index_0 + 2000) % mixed_numbers.len()]
        + mixed_numbers[(index_0 + 3000) % mixed_numbers.len()]
}

fn mix_numbers(
    mixed_numbers: &mut Vec<Sol>,
    o_num_i_to_mix_num_i: &mut Vec<Sol>,
    mix_num_i_to_o_num_i: &mut [Sol],
) {
    let n_len = mixed_numbers.len() as Sol;
    for original_number_index in 0..n_len {
        let mix_number_i = get_i_d(o_num_i_to_mix_num_i, original_number_index);

        let moves_to_make = get_modulo_moves(mixed_numbers[mix_number_i], n_len);
        let move_dir = moves_to_make.signum();
        for i in 0..moves_to_make.abs() {
            let mixed_number_i_1 = modulo(mix_number_i as Sol + (i * move_dir), n_len);
            let mixed_number_i_2 = modulo(mixed_number_i_1 + move_dir, n_len);
            mixed_numbers.swap(mixed_number_i_1 as usize, mixed_number_i_2 as usize);
            let org_number_i_1 = mix_num_i_to_o_num_i[modulo(mixed_number_i_1, n_len) as usize];
            let org_number_i_2 = mix_num_i_to_o_num_i[modulo(mixed_number_i_2, n_len) as usize];
            o_num_i_to_mix_num_i.swap(org_number_i_1 as usize, org_number_i_2 as usize);
            mix_num_i_to_o_num_i.swap(mixed_number_i_1 as usize, mixed_number_i_2 as usize);
        }
    }
}

fn get_modulo_moves(moves: Sol, n_len: Sol) -> Sol {
    // wrap moves around until we are within the range of one cycle
    // -1 because the number doesn't pass itself
    moves - ((moves / (n_len - 1)) * (n_len - 1))
}

fn get_i_d(idx: &[Sol], i: Sol) -> usize {
    idx[i as usize] as usize
}

fn modulo(s: Sol, o: Sol) -> Sol {
    ((s % o) + o) % o
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
        assert_eq!(part_1(&parse_output), 3);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = parse(TEST_INPUT);
        assert_eq!(part_2(&parse_output), 1623178306);
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
            assert_eq!(part_1(black_box(&parse_output)), 4914);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = parse(MAIN_INPUT);
        b.iter(|| {
            assert_eq!(part_2(black_box(&parse_output)), 7973051839072);
        });
    }
}
