#![feature(test)]

type Solution = i64;

const MAX_SEARCH_P2: Solution = 4_000_000;
const Y_P1: Solution = 2_000_000;

pub type ParseOutput = Vec<(Solution, Solution, Solution, Solution, Solution)>;
const MAIN_INPUT: &str = include_str!("main_input");

pub fn parse(file: &str) -> ParseOutput {
    file.lines()
        .map(|l| {
            let (sss, bbb) = l.split_once(": ").unwrap();
            let ss = sss.replace("Sensor at x=", "").replace(" y=", "");
            let bb = bbb
                .replace("closest beacon is at x=", "")
                .replace(" y=", "");
            let s = ss.split_once(',').unwrap();
            let b = bb.split_once(',').unwrap();
            let (sx, sy) = (s.0.parse().unwrap(), s.1.parse().unwrap());
            let (bx, by) = (b.0.parse().unwrap(), b.1.parse().unwrap());
            (sy, sx, by, bx, manhattan_distance((sy, sx), (by, bx)))
        })
        .collect()
}

fn main() {
    let parse_output = parse(MAIN_INPUT);
    println!("Solution to part 1 is {}", part_1(&parse_output, Y_P1));
    println!(
        "Solution to part 2 is {}",
        part_2(&parse_output, MAX_SEARCH_P2)
    );
}

fn part_1(beacons_and_signals: &ParseOutput, y: Solution) -> Solution {
    let mut ranges = Vec::new();
    clear_and_push_sorted_ranges(y, beacons_and_signals, &mut ranges);

    let mut beacons: Vec<Solution> = beacons_and_signals
        .iter()
        .filter(|r| r.2 == y)
        .map(|r| r.2)
        .collect();
    beacons.sort();
    beacons.dedup();

    count_impossible(&mut ranges) - beacons.len() as Solution
}

fn part_2(parse_output: &ParseOutput, max: Solution) -> Solution {
    let beacons_and_signals = parse_output;
    let mut ranges = Vec::new();
    let mut x = 0;
    let mut y = 0;
    for i in 0..max {
        y = i;
        clear_and_push_sorted_ranges(y, beacons_and_signals, &mut ranges);

        if let Some(free_x) = get_free_x(&mut ranges) {
            x = free_x;
            break;
        }
    }

    (MAX_SEARCH_P2 * x) + y
}

fn count_impossible(ranges: &mut [(Solution, Solution)]) -> Solution {
    let r_min = ranges.iter().map(|r| r.0).min().unwrap();
    let r_max = ranges.iter().map(|r| r.1).max().unwrap();
    let mut solution = r_max - r_min;
    if get_free_x(ranges).is_some() {
        solution -= 1;
    }
    solution
}

fn get_free_x(ranges: &mut [(Solution, Solution)]) -> Option<Solution> {
    let mut p = ranges[0].1;
    for r in ranges.iter().skip(1) {
        if r.0 <= p {
            p = r.1.max(p);
        } else {
            return Some(p);
        }
    }

    None
}

fn clear_and_push_sorted_ranges(
    y: Solution,
    beacons_and_signals: &ParseOutput,
    ranges: &mut Vec<(Solution, Solution)>,
) {
    ranges.clear();
    for beacon_and_signal in beacons_and_signals {
        let (sy, sx, _, _, s_b_distance) = beacon_and_signal;
        let row_signal = s_b_distance - (sy.abs_diff(y) as Solution);
        if row_signal < 0 {
            continue;
        }
        let signal_range = (sx - row_signal, sx + row_signal + 1);
        ranges.push(signal_range);
    }
    ranges.sort();
}

pub fn manhattan_distance(p1: (Solution, Solution), p2: (Solution, Solution)) -> Solution {
    (p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)) as Solution
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use test::{black_box, Bencher};
    const TEST_INPUT: &str = include_str!("test_input");
    const MAX_SEARCH_P2_TEST: Solution = 20;
    const Y_P1_TEST: Solution = 10;

    #[test]
    pub fn test_part_1() {
        let parse_output = parse(TEST_INPUT);
        assert_eq!(part_1(&parse_output, Y_P1_TEST), 26);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = parse(TEST_INPUT);
        assert_eq!(part_2(&parse_output, MAX_SEARCH_P2_TEST), 56000011);
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
            assert_eq!(part_1(black_box(&parse_output), Y_P1), 5564017);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = parse(MAIN_INPUT);
        b.iter(|| {
            assert_eq!(
                part_2(black_box(&parse_output), MAX_SEARCH_P2),
                11558423398893
            );
        });
    }
}
