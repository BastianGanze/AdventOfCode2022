#![feature(test)]

extern crate core;

use crate::cave::{CaveNode, ValveIndex, ValvePathItem};
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::process::id;

pub mod cave;

type Sol = i32;
const COST_OF_OPENING_VALVE: Sol = 1;

pub type ParseOutput = (
    Vec<CaveNode>,
    Vec<ValveIndex>,
    HashMap<usize, String>,
    usize,
);
const MAIN_INPUT: &str = include_str!("main_input");

pub fn parse(file: &str) -> ParseOutput {
    let mut lavel_to_index_map = HashMap::new();
    let mut t_map = HashMap::new();
    let mut index_to_label_map = HashMap::new();
    let mut valves: Vec<usize> = Vec::new();
    let mut start_i = 0;
    let mut cave_system: Vec<CaveNode> = file
        .lines()
        .enumerate()
        .map(|(i, l)| {
            let (valve, tunnels) = l.split_once("; ").unwrap();
            let (isis, rr) = valve.split_once(" has ").unwrap();
            let cave_label = isis.replace("Valve ", "");
            let r = rr.replace("flow rate=", "");
            let t: Vec<String> = tunnels
                .replace("tunnels lead to valves ", "")
                .replace("tunnel leads to valve ", "")
                .split(", ")
                .map(|tt| tt.to_string())
                .collect();
            if cave_label == "AA" {
                start_i = i;
            }
            lavel_to_index_map.insert(cave_label.clone(), i);
            index_to_label_map.insert(i, cave_label);
            t_map.insert(i, t);
            let rate: Sol = r.parse().unwrap();
            if rate > 0 {
                valves.push(i);
            }

            CaveNode {
                rate,
                paths: Vec::new(),
                routes: HashMap::new(),
            }
        })
        .collect();

    for (i, cave) in cave_system.iter_mut().enumerate() {
        if let Some(t) = t_map.get(&i) {
            cave.paths = t
                .iter()
                .map(|ti| *lavel_to_index_map.get(ti).unwrap())
                .collect()
        }
    }

    for vi in &valves {
        let mut next: BinaryHeap<ValvePathItem> = BinaryHeap::new();
        let rate = cave_system[*vi].rate;
        next.push(ValvePathItem(0, *vi, *vi));
        while let Some(ValvePathItem(cost, current_i, last_i)) = next.pop() {
            let cave = &mut cave_system[current_i];
            if !cave.routes.contains_key(vi) {
                if *vi != current_i {
                    cave.routes.insert(*vi, (last_i, cost, rate));
                }
                for ti in &cave_system[current_i].paths {
                    next.push(ValvePathItem(cost + 1, *ti, current_i));
                }
            }
        }
    }

    /*print_graph_vis(&cave_system, &i_i_t_map);
    print_route_vis(&cave_system, &valves, &i_i_t_map);*/
    (cave_system, valves, index_to_label_map, start_i)
}

fn main() {
    let parse_output = parse(MAIN_INPUT);
    println!("Solution to part 1 is {}", part_1(&parse_output));
    println!("Solution to part 2 is {}", part_2(&parse_output));
}

fn part_1(parse_output: &ParseOutput) -> Sol {
    let (cave_system, valves, index_to_label_map, start_i) = &parse_output;
    let mut solution = 0;
    permutate(
        valves.iter().map(|v_i| (*v_i, 0)).collect(),
        vec![*start_i],
        cave_system,
        0,
        *start_i,
        &mut |valve_idx| {
            let mut flow_rate_at =
                calculate_flow_rate_at(cave_system, valve_idx, index_to_label_map);

            solution = solution.max(calc_total_flow(&flow_rate_at));
        },
    );

    solution
}

fn calc_total_flow(flow_rate_at: &Vec<(Sol, Sol)>) -> i32 {
    let mut current_flow = 0;
    let mut last_time = 0;
    let mut sol = 0;
    for (now, rate) in flow_rate_at {
        let delta = now - last_time;

        sol += delta * current_flow;
        current_flow += rate;
        last_time = *now;
    }
    sol += (30 - last_time) * current_flow;
    sol
}

fn calculate_flow_rate_at(
    cave_system: &Vec<CaveNode>,
    valve_idx: &Vec<usize>,
    _index_to_label_map: &HashMap<usize, String>,
) -> Vec<(Sol, Sol)> {
    let mut c_i = valve_idx[0];
    let mut v_i = 1;
    let mut minute = 0;
    let mut flow_rate_added_at: Vec<(Sol, Sol)> = Vec::new();
    while let Some((_, cost, rate)) = cave_system[c_i].routes.get(&valve_idx[v_i]) {
        let delta = cost + COST_OF_OPENING_VALVE;
        minute += delta;
        flow_rate_added_at.push((minute, *rate));
        c_i = valve_idx[v_i];
        v_i += 1;
        if v_i >= valve_idx.len() {
            break;
        }
    }

    flow_rate_added_at
}

pub fn permutate<F: FnMut(&Vec<usize>)>(
    idx: Vec<(usize, Sol)>,
    nice: Vec<usize>,
    cave_system: &Vec<CaveNode>,
    current_cost: Sol,
    current_i: usize,
    cb: &mut F,
) {
    let new_idx: Vec<(usize, Sol)> = idx
        .iter()
        .filter_map(|(v_i, _)| {
            if let Some((_, cost_to_next_node, _)) = cave_system[current_i].routes.get(v_i) {
                let new_cost = current_cost + cost_to_next_node + COST_OF_OPENING_VALVE;
                if new_cost <= 30 {
                    return Some((*v_i, new_cost));
                }
            }
            None
        })
        .collect();

    if new_idx.is_empty() {
        cb(&nice);
        return;
    }

    for (v_i, new_cost) in new_idx.iter() {
        permutate(
            new_idx.clone(),
            nice.iter().chain(&[*v_i]).cloned().collect(),
            cave_system,
            *new_cost,
            *v_i,
            cb,
        );
    }
}

fn part_2(parse_output: &ParseOutput) -> Sol {
    let (cave_system, valves, index_to_label_map, start_i) = &parse_output;
    let mut solution = 0;
    let mut c = 0;
    let mut state = HashSet::new();

    permutation_4(
        valves.clone(),
        &mut [0; 27],
        cave_system,
        0,
        0,
        *start_i,
        *start_i,
        &mut state,
        &mut |flow| {
            c += 1;
            if c % 1_000_000 == 0 {
                println!("{:?}", c);
                println!("{:?}, {}", flow, solution);
            }

            let mut current_flow = 0;
            let mut sol = 0;
            for f in flow {
                sol += current_flow;
                current_flow += f;
            }
            solution = solution.max(sol);
        },
    );

    solution
}

type TimeLine = [Sol; 27];

pub fn permutation_4<F: FnMut(&TimeLine)>(
    left: Vec<usize>,
    timeline: &mut TimeLine,
    cave_system: &Vec<CaveNode>,
    current_you_cost: Sol,
    current_elephant_cost: Sol,
    current_you_i: usize,
    current_elephant_i: usize,
    state_hash: &mut HashSet<TimeLine>,
    cb: &mut F,
) {
    if !state_hash.insert(*timeline) {
        return;
    }

    let mut both_end = true;
    for v_i in &left {
        let new_left: Vec<usize> = left
            .iter()
            .filter_map(|l| if l != v_i { Some(*l) } else { None })
            .collect();

        let next_you_cost = current_you_cost + route_cost(cave_system, current_you_i, v_i);
        if next_you_cost <= 26 {
            both_end = false;
            timeline[next_you_cost as usize] += cave_system[*v_i].rate;
            permutation_4(
                new_left.clone(),
                timeline,
                cave_system,
                next_you_cost,
                current_elephant_cost,
                *v_i,
                current_elephant_i,
                state_hash,
                cb,
            );
            timeline[next_you_cost as usize] -= cave_system[*v_i].rate;
        }

        let next_elephant_cost =
            current_elephant_cost + route_cost(cave_system, current_elephant_i, v_i);
        if next_elephant_cost <= 26 {
            both_end = false;
            timeline[next_elephant_cost as usize] += cave_system[*v_i].rate;
            permutation_4(
                new_left,
                timeline,
                cave_system,
                current_you_cost,
                next_elephant_cost,
                current_you_i,
                *v_i,
                state_hash,
                cb,
            );
            timeline[next_elephant_cost as usize] -= cave_system[*v_i].rate;
        }
    }

    if both_end {
        cb(timeline);
    }
}

pub fn permutation_3<F: FnMut(&Vec<usize>, &Vec<usize>)>(
    left: Vec<usize>,
    you: Vec<usize>,
    elephant: Vec<usize>,
    cave_system: &Vec<CaveNode>,
    current_you_cost: Sol,
    current_elephant_cost: Sol,
    cb: &mut F,
) {
    let mut both_end = true;
    for v_i in &left {
        let new_left: Vec<usize> = left
            .iter()
            .filter_map(|l| if l != v_i { Some(*l) } else { None })
            .collect();

        let next_you_cost = current_you_cost + route_cost(cave_system, you[you.len() - 1], v_i);
        if next_you_cost <= 26 {
            both_end = false;
            permutation_3(
                new_left.clone(),
                you.iter().chain(&[*v_i]).copied().collect(),
                elephant.clone(),
                cave_system,
                next_you_cost,
                current_elephant_cost,
                cb,
            );
        }

        let next_elephant_cost =
            current_elephant_cost + route_cost(cave_system, elephant[elephant.len() - 1], v_i);
        if next_elephant_cost <= 26 {
            both_end = false;
            permutation_3(
                new_left,
                you.clone(),
                elephant.iter().chain(&[*v_i]).copied().collect(),
                cave_system,
                current_you_cost,
                next_elephant_cost,
                cb,
            );
        }
    }

    if both_end {
        cb(&you, &elephant);
    }
}

pub fn route_cost(cave_system: &Vec<CaveNode>, c_i: usize, v_i: &usize) -> Sol {
    cave_system[c_i].routes.get(v_i).unwrap().1 + COST_OF_OPENING_VALVE
}

fn calculate_flow_rate_at_hash(
    hash_map: &mut [Sol; 27],
    cave_system: &Vec<CaveNode>,
    valve_idx: &Vec<usize>,
    _index_to_label_map: &HashMap<usize, String>,
) {
    let mut c_i = valve_idx[0];
    let mut v_i = 1;
    let mut minute = 0;
    while let Some((_, cost, rate)) = cave_system[c_i].routes.get(&valve_idx[v_i]) {
        let delta = cost + COST_OF_OPENING_VALVE;
        minute += delta;
        hash_map[minute as usize] += *rate;
        c_i = valve_idx[v_i];
        v_i += 1;
        if v_i >= valve_idx.len() {
            break;
        }
    }
}

fn new_left(
    own_vi: usize,
    other_vi: usize,
    left: &Vec<(usize, Sol)>,
    cave_system: &Vec<CaveNode>,
    current_cost: Sol,
    current_i: usize,
) -> Vec<(usize, Sol)> {
    left.iter()
        .filter_map(|(v_i, _)| {
            if *v_i == other_vi || *v_i == own_vi {
                return None;
            }

            if let Some((_, cost_to_next_node, _)) = cave_system[current_i].routes.get(v_i) {
                let new_cost = current_cost + cost_to_next_node + COST_OF_OPENING_VALVE;
                if new_cost <= 26 {
                    return Some((*v_i, new_cost));
                }
            }
            None
        })
        .collect()
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
        assert_eq!(part_1(&parse_output), 1651);
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
            assert_eq!(part_1(black_box(&parse_output)), 1720);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = parse(MAIN_INPUT);
        b.iter(|| {
            assert_eq!(part_2(black_box(&parse_output)), 3);
        });
    }
}
