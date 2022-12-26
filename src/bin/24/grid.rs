use crate::Storm;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

pub type CostType = i8;
pub type StormMask = u8;

pub const TOP: StormMask = 0b1000;
pub const RIGHT: StormMask = 0b0100;
pub const LEFT: StormMask = 0b0010;
pub const DOWN: StormMask = 0b0001;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FieldType {
    Rock,
    Valley(StormMask),
}
pub type MaskType = bool;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Grid {
    pub fields: Vec<Vec<(CostType, FieldType)>>,
    size: (usize, usize),
}

pub fn manhattan_distance(p1: (usize, usize), p2: (usize, usize)) -> u32 {
    (p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)) as u32
}

impl Grid {
    pub fn new(size: (usize, usize)) -> Grid {
        Grid {
            fields: vec![vec![(0, FieldType::Valley(0)); size.1]; size.0],
            size,
        }
    }

    pub fn get_size(&self) -> (usize, usize) {
        self.size.clone()
    }

    pub(self) fn is_field_in_bounds(&self, y: i32, x: i32) -> bool {
        y >= 0 && x >= 0 && y < self.size.0 as i32 && x < self.size.1 as i32
    }

    pub fn get_field_height(&self, y: usize, x: usize) -> CostType {
        self.fields[y][x].0
    }

    pub fn set_field_height(&mut self, y: usize, x: usize, cost: CostType) {
        if !self.is_field_in_bounds(y as i32, x as i32) {
            return;
        }

        self.fields[y][x].0 = cost;
    }

    pub fn mark_field(&mut self, y: usize, x: usize, field_type: FieldType) {
        self.fields[y][x].1 = field_type;
    }

    pub fn add_storm(&mut self, y: usize, x: usize, storm_mask: StormMask) {
        if let FieldType::Valley(ref mut n) = self.fields[y][x].1 {
            *n |= storm_mask;
        }
    }

    pub fn is_field_full(&self, y: usize, x: usize) -> bool {
        match self.fields[y][x].1 {
            FieldType::Rock => true,
            FieldType::Valley(storm) => storm > 0,
        }
    }

    pub fn get_field_type(&self, y: usize, x: usize) -> &FieldType {
        &self.fields[y][x].1
    }

    pub fn get_possible_positions(&self, y: usize, x: usize) -> Vec<(usize, usize)> {
        let mut neighbours = Vec::new();
        if self.is_field_in_bounds(y as i32 - 1, x as i32) && !self.is_field_full(y - 1, x) {
            neighbours.push((y - 1, x));
        }

        if self.is_field_in_bounds(y as i32 + 1, x as i32) && !self.is_field_full(y + 1, x) {
            neighbours.push((y + 1, x));
        }
        if self.is_field_in_bounds(y as i32, x as i32 - 1) && !self.is_field_full(y, x - 1) {
            neighbours.push((y, x - 1));
        }
        if self.is_field_in_bounds(y as i32, x as i32 + 1) && !self.is_field_full(y, x + 1) {
            neighbours.push((y, x + 1));
        }
        neighbours
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = Vec::new();
        for y in 0..self.size.0 {
            for x in 0..self.size.1 {
                match self.get_field_type(y, x) {
                    FieldType::Rock => {
                        s.push('#');
                    }
                    FieldType::Valley(n) => {
                        s.push(match n {
                            &TOP => '^',
                            &RIGHT => '>',
                            &DOWN => 'v',
                            &LEFT => '<',
                            0 => '.',
                            n => std::char::from_digit(n.count_ones(), 10).unwrap(),
                        });
                    }
                }
            }
            s.push('\n');
        }
        write!(f, "{}", s.iter().collect::<String>())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Field {
    pub coordinate: (usize, usize),
    pub current_step: usize,
    pub cost: u32,
}

impl Field {
    pub fn new(coordinate: (usize, usize), cost: u32, current_step: usize) -> Field {
        Field {
            coordinate,
            cost,
            current_step,
        }
    }
}

impl PartialOrd<Self> for Field {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Field {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.cost > other.cost {
            return Ordering::Less;
        }

        if self.cost == other.cost {
            return Ordering::Equal;
        }

        Ordering::Greater
    }
}
