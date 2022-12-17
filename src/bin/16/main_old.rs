#![feature(test)]

use std::cell::{Ref, RefCell};
use std::cmp::{max, Ord, Ordering};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::env::current_dir;
use std::ops::Deref;
use std::rc::Rc;

type Sol = u64;

#[derive(Debug, Clone)]
pub struct CaveNode {
    paths: Vec<usize>,
    sign: ElectricSign,
    valve: Valve,
}

#[derive(Debug, Clone)]
pub struct Valve {
    rate: Sol,
    open: bool,
}

type Cost = Sol;

#[derive(Debug, Clone)]
pub struct ElectricSign {
    valve_directions: Vec<(usize, Cost, usize)>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct ElectricSignItem {
    cost: Sol,
    current_i: usize,
    last_i: usize,
}

impl PartialOrd<Self> for ElectricSignItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.cost > other.cost {
            return Some(Ordering::Less);
        }

        if self.cost == other.cost {
            return Some(Ordering::Equal);
        }

        return Some(Ordering::Greater);
    }
}

impl Ord for ElectricSignItem {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.cost > other.cost {
            return Ordering::Less;
        }

        if self.cost == other.cost {
            return Ordering::Equal;
        }

        return Ordering::Greater;
    }
}

pub type ParseOutput = (Vec<CaveNode>, Sol);
const MAIN_INPUT: &str = include_str!("main_input");

pub fn parse(file: &str) -> ParseOutput {
    let mut i_map = HashMap::new();
    let mut t_map = HashMap::new();
    let mut valves: Vec<usize> = Vec::new();
    let mut number_of_valves = 0;
    let mut cave_system: Vec<CaveNode> = file
        .lines()
        .enumerate()
        .map(|(i, l)| {
            let (valve, tunnels) = l.split_once("; ").unwrap();
            let (isis, rr) = valve.split_once(" has ").unwrap();
            let is = isis.replace("Valve ", "");
            let r = rr.replace("flow rate=", "");
            let t: Vec<String> = tunnels
                .replace("tunnels lead to valves ", "")
                .replace("tunnel leads to valve ", "")
                .split(", ")
                .map(|tt| tt.to_string())
                .collect();
            i_map.insert(is, i);
            t_map.insert(i, t);
            let rate: Sol = r.parse().unwrap();
            if rate > 0 {
                valves.push(i);
                number_of_valves += 1;
            }

            CaveNode {
                valve: Valve { open: false, rate },
                sign: ElectricSign {
                    valve_directions: Vec::new(),
                },
                paths: Vec::new(),
            }
        })
        .collect();

    for (i, cave) in cave_system.iter_mut().enumerate() {
        if let Some(t) = t_map.get(&i) {
            cave.paths = t.iter().map(|ti| *i_map.get(ti).unwrap()).collect()
        }
    }

    for vi in valves {
        let valve = cave_system[vi].valve.clone();
        let mut visited = vec![false; cave_system.len()];
        let mut next = BinaryHeap::new();
        next.push(ElectricSignItem {
            current_i: vi,
            last_i: vi,
            cost: 0,
        });
        while let Some(e) = next.pop() {
            let cave = &mut cave_system[e.current_i];
            if !visited[e.current_i] {
                visited[e.current_i] = true;
                if e.last_i != e.current_i {
                    cave.sign
                        .valve_directions
                        .push((e.last_i, e.cost, e.current_i));
                }

                for ti in &cave_system[e.current_i].paths {
                    next.push(ElectricSignItem {
                        last_i: e.current_i,
                        current_i: *ti,
                        cost: e.cost + 1,
                    })
                }
            }
        }
    }

    (cave_system, number_of_valves)
}

fn main() {
    let parse_output = parse(MAIN_INPUT);
    println!("Solution to part 1 is {}", part_1(&parse_output));
    println!("Solution to part 2 is {}", part_2(&parse_output));
}

fn part_1(parse_output: &ParseOutput) -> Sol {
    let mut next = BinaryHeap::new();
    let (mut cave_system, number_of_valves) = parse_output.clone();
    next.push((0, 30, 0, 0, number_of_valves));
    let mut solution = 0;
    while let Some(next_best) = next.pop() {
        let (i, minutes_left, total_flow, flow_rate, open_valves) = next_best;
        let cave_node = &mut cave_system[i];
        let mut valve = &mut cave_node.valve;
        let new_total_flow = total_flow + flow_rate;
        println!("{:?}", (i, open_valves, minutes_left));
        if open_valves == 0 {
            solution = max(solution, total_flow + (flow_rate * minutes_left));
            continue;
        }
        solution = max(solution, new_total_flow);
        if minutes_left == 0 {
            continue;
        }
        if valve.rate == 0 || valve.open {
            for (i, _, d_i) in &cave_node.sign.valve_directions {
                if !o_v_r.open {
                    next.push((*i, minutes_left - 1, new_total_flow, flow_rate, open_valves))
                }
            }
        } else {
            if minutes_left == 1 {
                solution = max(solution, new_total_flow + flow_rate);
                continue;
            }

            valve.open = true;
            for (i, _, o_v_r) in &cave_node.sign.valve_directions {
                if !o_v_r.borrow().open {
                    next.push((
                        *i,
                        minutes_left - 2,
                        new_total_flow,
                        flow_rate + valve.rate,
                        open_valves - 1,
                    ))
                }
            }
        }
    }
    solution
}

fn part_2(parse_output: &ParseOutput) -> Sol {
    0
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
        assert_eq!(part_1(&parse_output), 26);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = parse(TEST_INPUT);
        assert_eq!(part_2(&parse_output), 56000011);
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
            assert_eq!(part_1(black_box(&parse_output)), 5564017);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = parse(MAIN_INPUT);
        b.iter(|| {
            assert_eq!(part_2(black_box(&parse_output)), 11558423398893);
        });
    }
}
