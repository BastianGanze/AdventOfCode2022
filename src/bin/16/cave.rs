use crate::Sol;
use std::cmp::Ordering;
use std::collections::HashMap;

type Cost = Sol;
type NodeIndex = usize;
pub type ValveIndex = usize;
pub type Rate = Sol;

pub type Routes = HashMap<ValveIndex, (NodeIndex, Cost, Rate)>;
pub type RoutingPath = Vec<ValveIndex>;

#[derive(Debug, Clone)]
pub struct CaveNode {
    pub routes: Routes,
    pub paths: Vec<NodeIndex>,
    pub rate: Rate,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ValvePathItem(pub Cost, pub NodeIndex, pub NodeIndex);

impl PartialOrd<Self> for ValvePathItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ValvePathItem {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 > other.0 {
            return Ordering::Less;
        }

        if self.0 == other.0 {
            return Ordering::Equal;
        }

        return Ordering::Greater;
    }
}

fn print_route_vis(
    cave_system: &Vec<CaveNode>,
    valves: &Vec<ValveIndex>,
    i_t_map: &HashMap<usize, String>,
) {
    let mut e = 0;
    println!(
        "{}",
        valves
            .iter()
            .chain(&[0])
            .map(|v_i| {
                format!(
                    "{}: {{ name: \"{}\" }}, ",
                    i_t_map.get(v_i).unwrap(),
                    cave_system[*v_i].rate
                )
            })
            .collect::<String>()
    );
    println!(
        "{}",
        valves
            .iter()
            .chain(&[0])
            .map(|v_i| {
                cave_system[*v_i]
                    .routes
                    .iter()
                    .map(|(goal_index, (_, cost, _))| {
                        e += 1;
                        format!(
                            "edge{}: {{ source: \"{}\", target: \"{}\", label: \"{}\" }}, ",
                            e,
                            i_t_map.get(v_i).unwrap(),
                            i_t_map.get(goal_index).unwrap(),
                            cost
                        )
                    })
                    .collect::<String>()
            })
            .collect::<String>()
    );
}

fn print_graph_vis(cave_system: &Vec<CaveNode>, i_t_map: &HashMap<usize, String>) {
    let mut e = 0;
    println!(
        "{}",
        cave_system
            .iter()
            .enumerate()
            .map(|(c_i, c)| {
                format!(
                    "{}: {{ name: \"{}\" }}, ",
                    i_t_map.get(&c_i).unwrap(),
                    c.rate
                )
            })
            .collect::<String>()
    );
    println!(
        "{}",
        cave_system
            .iter()
            .enumerate()
            .map(|(c_i, c)| {
                c.routes
                    .iter()
                    .map(|(goal_index, (next_node, cost, _))| {
                        e += 1;
                        format!(
                            "edge{}: {{ source: \"{}\", target: \"{}\", label: \"{}-{}\" }}, ",
                            e,
                            i_t_map.get(&c_i).unwrap(),
                            i_t_map.get(next_node).unwrap(),
                            i_t_map.get(goal_index).unwrap(),
                            cost
                        )
                    })
                    .collect::<String>()
            })
            .collect::<String>()
    );
}
