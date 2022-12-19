#![feature(test)]

use std::collections::HashSet;

type Sol = i32;

#[derive(Debug, Clone)]
pub struct Blueprint {
    ore_rob_ore_cost: Sol,
    clay_rob_ore_cost: Sol,
    obs_rob_ore_cost: Sol,
    obs_rob_clay_cost: Sol,
    geo_rob_ore_cost: Sol,
    geo_rob_obs_cost: Sol,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct SimulationState {
    minute: Sol,
    ore_robs: Sol,
    clay_robs: Sol,
    obs_robs: Sol,
    geo_robs: Sol,
    ore: Sol,
    clay: Sol,
    obs: Sol,
    geo: Sol,
}

impl SimulationState {
    pub fn default() -> SimulationState {
        SimulationState {
            minute: 0,
            ore_robs: 1,
            clay_robs: 0,
            obs_robs: 0,
            geo_robs: 0,
            ore: 0,
            clay: 0,
            obs: 0,
            geo: 0,
        }
    }

    pub fn build_ore_rob(&self, blueprint: &Blueprint) -> Option<Self> {
        if self.ore >= blueprint.ore_rob_ore_cost {
            let mut n = self.clone();
            n.ore -= blueprint.ore_rob_ore_cost;
            n.advance_factory();
            n.ore_robs += 1;
            return Some(n);
        }
        None
    }

    pub fn build_clay_rob(&self, blueprint: &Blueprint) -> Option<Self> {
        if self.ore >= blueprint.clay_rob_ore_cost {
            let mut n = self.clone();
            n.ore -= blueprint.clay_rob_ore_cost;
            n.advance_factory();
            n.clay_robs += 1;
            return Some(n);
        }
        None
    }

    pub fn build_obs_rob(&self, blueprint: &Blueprint) -> Option<Self> {
        if self.ore >= blueprint.obs_rob_ore_cost && self.clay >= blueprint.obs_rob_clay_cost {
            let mut n = self.clone();
            n.ore -= blueprint.obs_rob_ore_cost;
            n.clay -= blueprint.obs_rob_clay_cost;
            n.advance_factory();
            n.obs_robs += 1;
            return Some(n);
        }
        None
    }

    pub fn build_geo_rob(&self, blueprint: &Blueprint) -> Option<Self> {
        if self.ore >= blueprint.geo_rob_ore_cost && self.obs >= blueprint.geo_rob_obs_cost {
            let mut n = self.clone();
            n.ore -= blueprint.geo_rob_ore_cost;
            n.obs -= blueprint.geo_rob_obs_cost;
            n.advance_factory();
            n.geo_robs += 1;
            return Some(n);
        }
        None
    }

    pub fn advance_factory(&mut self) {
        self.ore += self.ore_robs;
        self.clay += self.clay_robs;
        self.obs += self.obs_robs;
        self.geo += self.geo_robs;
    }
}

pub type ParseOutput = Vec<Blueprint>;
const MAIN_INPUT: &str = include_str!("main_input");

pub fn parse(file: &str) -> ParseOutput {
    file.lines()
        .map(|l| match l.split_ascii_whitespace().collect::<Vec<&str>>()[..] {
            ["Blueprint", _, "Each", "ore", "robot", "costs", ore_rob_ore_cost, "ore.", "Each", "clay", "robot", "costs", clay_rob_ore_cost, "ore.", "Each", "obsidian", "robot", "costs", obs_rob_ore_cost, "ore", "and", obs_rob_clay_cost, "clay.", "Each", "geode", "robot", "costs", geo_rob_ore_cost, "ore", "and", geo_rob_obs_cost, "obsidian."] => Blueprint {
                ore_rob_ore_cost: ore_rob_ore_cost.parse().unwrap(),
                clay_rob_ore_cost: clay_rob_ore_cost.parse().unwrap(),
                obs_rob_ore_cost: obs_rob_ore_cost.parse().unwrap(),
                obs_rob_clay_cost: obs_rob_clay_cost.parse().unwrap(),
                geo_rob_ore_cost: geo_rob_ore_cost.parse().unwrap(),
                geo_rob_obs_cost: geo_rob_obs_cost.parse().unwrap()
            },
            _ => unreachable!(),
        })
        .collect()
}

fn main() {
    let cubes = parse(MAIN_INPUT);
    println!("Solution to part 1 is {}", part_1(&cubes));
    println!("Solution to part 2 is {}", part_2(&cubes));
}

fn part_1(blueprints: &ParseOutput) -> Sol {
    let mut solution = 0;
    let mut visited_states = HashSet::new();

    for (i, blueprint) in blueprints.iter().enumerate() {
        visited_states.clear();
        let mut possible_simulation_states: Vec<SimulationState> = vec![SimulationState::default()];
        let mut max_geos: Sol = 0;
        let max_ore_robs = blueprint
            .ore_rob_ore_cost
            .max(blueprint.clay_rob_ore_cost)
            .max(blueprint.obs_rob_ore_cost)
            .max(blueprint.geo_rob_ore_cost);
        let max_clay_robs = blueprint.obs_rob_clay_cost;
        let max_obs_robs = blueprint.geo_rob_obs_cost;
        let mut c: u64 = 0;
        let mut max_state = SimulationState::default();
        while let Some(mut state) = possible_simulation_states.pop() {
            if !visited_states.insert(state.clone()) {
                continue;
            }
            c += 1;
            if state.minute == 24 {
                if state.geo > max_geos {
                    max_geos = state.geo;
                    max_state = state;
                }
                continue;
            }
            state.minute += 1;

            if state.ore_robs < max_ore_robs {
                if let Some(n) = state.build_ore_rob(blueprint) {
                    possible_simulation_states.push(n);
                }
            }

            if state.clay_robs < max_clay_robs {
                if let Some(n) = state.build_clay_rob(blueprint) {
                    possible_simulation_states.push(n);
                }
            }

            if state.obs_robs < max_obs_robs {
                if let Some(n) = state.build_obs_rob(blueprint) {
                    possible_simulation_states.push(n);
                }
            }

            if let Some(n) = state.build_geo_rob(blueprint) {
                possible_simulation_states.push(n);
            } else {
                state.advance_factory();
                possible_simulation_states.push(state);
            }
        }
        println!("{} {} {:?}", max_geos, c, max_state);
        solution += (i as Sol + 1) * max_geos;
    }
    solution
}

fn part_2(blueprints: &ParseOutput) -> Sol {
    let mut solution = 1;
    let mut visited_states = HashSet::new();
    for blueprint in blueprints.iter().take(3) {
        visited_states.clear();
        let mut possible_simulation_states: Vec<SimulationState> = vec![SimulationState::default()];
        let mut max_geos: Sol = 0;
        let max_ore_robs = blueprint
            .ore_rob_ore_cost
            .max(blueprint.clay_rob_ore_cost)
            .max(blueprint.obs_rob_ore_cost)
            .max(blueprint.geo_rob_ore_cost);
        let max_clay_robs = blueprint.obs_rob_clay_cost;
        let max_obs_robs = blueprint.geo_rob_obs_cost;
        let mut c: u64 = 0;
        let mut max_state = SimulationState::default();
        while let Some(mut state) = possible_simulation_states.pop() {
            if !visited_states.insert(state.clone()) {
                continue;
            }
            c += 1;
            if state.minute == 32 {
                if state.geo > max_geos {
                    max_geos = state.geo;
                    max_state = state;
                }
                continue;
            }
            state.minute += 1;

            if let Some(n) = state.build_geo_rob(blueprint) {
                possible_simulation_states.push(n);
                continue;
            }

            if state.ore_robs < max_ore_robs {
                if let Some(n) = state.build_ore_rob(blueprint) {
                    possible_simulation_states.push(n);
                }
            }

            if state.clay_robs < max_clay_robs {
                if let Some(n) = state.build_clay_rob(blueprint) {
                    possible_simulation_states.push(n);
                }
            }

            if state.obs_robs < max_obs_robs {
                if let Some(n) = state.build_obs_rob(blueprint) {
                    possible_simulation_states.push(n);
                }
            }

            state.advance_factory();
            possible_simulation_states.push(state);
        }
        println!("{} {} {:?}", max_geos, c, max_state);
        solution *= max_geos;
    }
    solution
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
        assert_eq!(part_1(&parse_output), 64);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = parse(TEST_INPUT);
        assert_eq!(part_2(&parse_output), 58);
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
            assert_eq!(part_1(black_box(&parse_output)), 1349);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = parse(MAIN_INPUT);
        b.iter(|| {
            assert_eq!(part_2(black_box(&parse_output)), 2456);
        });
    }
}
