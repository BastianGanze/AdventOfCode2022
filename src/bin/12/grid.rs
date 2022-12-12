use std::cmp::Ordering;

pub type CostType = i8;
pub type MaskType = bool;

#[derive(Debug, Clone)]
pub struct Grid {
    pub fields: Vec<Vec<(CostType, MaskType)>>,
    size: (usize, usize),
}

pub fn manhattan_distance(p1: (usize, usize), p2: (usize, usize)) -> u32 {
    (p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)) as u32
}

impl Grid {
    pub fn new(size: (usize, usize)) -> Grid {
        Grid {
            fields: vec![vec![(0, false); size.1]; size.0],
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

    pub fn mark_field(&mut self, y: usize, x: usize) {
        self.fields[y][x].1 = true;
    }

    pub fn is_field_marked(&self, y: usize, x: usize) -> bool {
        self.fields[y][x].1
    }

    pub fn get_left(&self, y: usize, x: usize) -> Option<CostType> {
        if !self.is_field_in_bounds(y as i32, x as i32 - 1) {
            return None;
        }

        Some(self.get_field_height(y, x - 1))
    }

    pub fn get_right(&self, y: usize, x: usize) -> Option<CostType> {
        if !self.is_field_in_bounds(y as i32, x as i32 + 1) {
            return None;
        }

        Some(self.get_field_height(y, x + 1))
    }

    pub fn get_top(&self, y: usize, x: usize) -> Option<CostType> {
        if !self.is_field_in_bounds(y as i32 - 1, x as i32) {
            return None;
        }

        Some(self.get_field_height(y - 1, x))
    }

    pub fn get_bottom(&self, y: usize, x: usize) -> Option<CostType> {
        if !self.is_field_in_bounds(y as i32 + 1, x as i32) {
            return None;
        }

        Some(self.get_field_height(y + 1, x))
    }

    pub fn get_unmarked_neighbours(&self, y: usize, x: usize) -> Vec<(usize, usize, CostType)> {
        let mut neighbours = Vec::new();
        let (left_o, top_o, right_o, bottom_o) = (
            self.get_left(y, x),
            self.get_top(y, x),
            self.get_right(y, x),
            self.get_bottom(y, x),
        );
        if let Some(n) = left_o {
            if !self.is_field_marked(y, x - 1) {
                neighbours.push((y, x - 1, n));
            }
        };
        if let Some(n) = right_o {
            if !self.is_field_marked(y, x + 1) {
                neighbours.push((y, x + 1, n));
            }
        };
        if let Some(n) = top_o {
            if !self.is_field_marked(y - 1, x) {
                neighbours.push((y - 1, x, n));
            }
        };
        if let Some(n) = bottom_o {
            if !self.is_field_marked(y + 1, x) {
                neighbours.push((y + 1, x, n));
            }
        };

        neighbours
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Field {
    pub coordinate: (usize, usize),
    pub path_length: u32,
    pub cost: u32,
}

impl Field {
    pub fn new(coordinate: (usize, usize), cost: u32, path_length: u32) -> Field {
        Field {
            coordinate,
            cost,
            path_length,
        }
    }
}

impl PartialOrd<Self> for Field {
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

impl Ord for Field {
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
