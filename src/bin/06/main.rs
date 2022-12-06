#![feature(test)]

type Solution = u32;

pub type ParseOutput = String;
const MAIN_INPUT: &str = include_str!("main_input");
const TEST_INPUT: &str = include_str!("test_input");

pub fn parse(file: &str) -> ParseOutput {
    file.into()
}

fn part_1(parse_output: &ParseOutput) -> Solution {
    let mut b: [u8; 4] = [0; 4];
    let mut bytes = parse_output.bytes().enumerate();
    b[0] = bytes.next().unwrap().1;
    b[1] = bytes.next().unwrap().1;
    b[2] = bytes.next().unwrap().1;
    b[3] = bytes.next().unwrap().1;

    for (i, byte) in bytes {
        if b[0] ^ b[1] == 0
            || b[0] ^ b[2] == 0
            || b[0] ^ b[3] == 0
            || b[1] ^ b[2] == 0
            || b[1] ^ b[3] == 0
            || b[2] ^ b[3] == 0
        {
            b[0] = b[1];
            b[1] = b[2];
            b[2] = b[3];
            b[3] = byte;
            continue;
        }

        return i as u32;
    }

    0
}

fn part_2(parse_output: &ParseOutput) -> Solution {
    let mut b: [u8; 14] = [0; 14];
    let mut bytes = parse_output.bytes().enumerate();

    for (i, byte) in (&mut bytes).take(14) {
        b[i] = byte;
    }

    'outer: for (i, byte) in bytes {
        for u in 0..14 {
            for v in (u + 1)..14 {
                if b[u] ^ b[v] == 0 {
                    for x in 0..13 {
                        b[x] = b[x + 1];
                    }
                    b[13] = byte;
                    continue 'outer;
                }
            }
        }

        return i as u32;
    }

    0
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
