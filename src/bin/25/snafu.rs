use crate::Sol;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::iter::Sum;
use std::ops::Add;
use std::vec::Splice;

const SNAFU_BASE: Sol = 5;

#[derive(Debug, Clone)]
pub struct SNAFU {
    base_10: Sol,
}

impl SNAFU {
    pub fn new(base_10: Sol) -> SNAFU {
        SNAFU { base_10 }
    }

    pub fn add(&mut self, other: Sol) {
        self.base_10 += other;
    }

    pub fn as_string(&self) -> String {
        let max_n_pow = get_pow(self.base_10, SNAFU_BASE);
        let mut vec = Vec::<char>::new();

        let mut pow = if self.base_10 > SNAFU_BASE.pow(max_n_pow + 1) as Sol / 2 {
            max_n_pow + 1
        } else {
            max_n_pow
        };
        let mut rest = self.base_10;
        println!("doing {:?}", (rest, pow));
        loop {
            if let Some(s) = match rest {
                -2 => Some('='),
                -1 => Some('-'),
                0 => Some('0'),
                1 => Some('1'),
                2 => Some('2'),
                _ => None,
            } {
                if pow > 0 {
                    vec.push('0');
                    pow -= 1;
                    continue;
                }
                vec.push(s);
                break;
            }
            if pow == 0 {
                break;
            }
            let c_pow_u_1 = SNAFU_BASE.pow(pow) as Sol;
            let c_pow_u_2 = c_pow_u_1 * 2;
            let c_pow_n_1 = SNAFU_BASE.pow(pow - 1) as Sol;
            let c_pow_n_2 = c_pow_n_1 * 2;

            if rest <= 0 {
                if rest.abs() <= c_pow_u_1 / 2 {
                    vec.push('0');
                } else if rest.abs() > (c_pow_u_1 + c_pow_u_2) / 2 {
                    rest += c_pow_u_2;
                    vec.push('=');
                } else {
                    vec.push('-');
                    rest += c_pow_u_1;
                }
            } else if rest <= c_pow_u_1 / 2 {
                vec.push('0');
            } else if rest > (c_pow_u_1 + c_pow_u_2) / 2 {
                vec.push('2');
                rest -= c_pow_u_2;
            } else {
                vec.push('1');
                rest -= c_pow_u_1;
            }
            pow -= 1;
        }
        vec.into_iter().collect::<String>()
    }
}

impl From<Sol> for SNAFU {
    fn from(value: Sol) -> Self {
        todo!()
    }
}

impl From<&str> for SNAFU {
    fn from(s: &str) -> Self {
        let mut base_10 = 0;
        for (i, c) in s.chars().enumerate() {
            let pos = (s.len() - 1 - i) as u32;
            let num_at_pos = SNAFU_BASE.pow(pos);
            base_10 += match c {
                '2' => num_at_pos * 2,
                '1' => num_at_pos,
                '0' => 0,
                '-' => -num_at_pos,
                '=' => -num_at_pos * 2,
                n => unreachable!("{}", n),
            }
        }
        SNAFU::new(base_10)
    }
}

impl From<SNAFU> for Sol {
    fn from(value: SNAFU) -> Self {
        value.base_10
    }
}

impl Display for SNAFU {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

impl Sum for SNAFU {
    fn sum<I: Iterator<Item = SNAFU>>(iter: I) -> SNAFU {
        iter.fold(SNAFU::new(0), |mut acc, s| {
            acc.add(s.base_10);
            acc
        })
    }
}

fn get_pow(num: Sol, base: Sol) -> u32 {
    let mut n = num;
    let mut c = 0;
    while n > 0 {
        n /= base;
        c += 1;
    }
    c - 1
}
