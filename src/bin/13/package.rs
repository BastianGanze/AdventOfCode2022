use std::cmp::{min, Ordering};

#[derive(Debug, Clone)]
pub enum Package {
    Number(u32),
    List(Vec<Package>),
}

impl Eq for Package {}

impl PartialEq<Self> for Package {
    fn eq(&self, other: &Self) -> bool {
        Ordering::Equal == get_order(self, other)
    }
}

impl PartialOrd<Self> for Package {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(get_order(self, other))
    }
}

impl Ord for Package {
    fn cmp(&self, other: &Self) -> Ordering {
        get_order(self, other)
    }
}

fn get_order(p1: &Package, p2: &Package) -> Ordering {
    match (p1, p2) {
        (Package::Number(n1), Package::Number(n2)) => n1.cmp(n2),
        (Package::List(_), Package::Number(n)) => {
            get_order(p1, &Package::List(vec![Package::Number(*n)]))
        }
        (Package::Number(n), Package::List(_)) => {
            get_order(&Package::List(vec![Package::Number(*n)]), p2)
        }
        (Package::List(l1), Package::List(l2)) => {
            let n = min(l1.len(), l2.len());
            for i in 0..n {
                match get_order(&l1[i], &l2[i]) {
                    Ordering::Less => return Ordering::Less,
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Equal => {
                        continue;
                    }
                }
            }
            l1.len().cmp(&l2.len())
        }
    }
}

impl Package {
    pub fn new(p: String) -> Package {
        if &p[0..1] == "[" {
            let mut depth = 0;
            let new_p: String = p
                .chars()
                .map(|c| {
                    match c {
                        ']' => depth -= 1,
                        '[' => depth += 1,
                        _ => {}
                    }
                    if c == ',' && depth == 1 {
                        '|'
                    } else {
                        c
                    }
                })
                .collect();
            Package::List(
                new_p[1..new_p.len() - 1]
                    .split('|')
                    .filter_map(|package| {
                        if !package.is_empty() {
                            Some(Package::new(package.into()))
                        } else {
                            None
                        }
                    })
                    .collect(),
            )
        } else {
            Package::Number(p.parse::<u32>().unwrap())
        }
    }
}
