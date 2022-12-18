#![feature(test)]

use std::collections::HashSet;

type Sol = i32;

pub type Cube = (Sol, Sol, Sol);
pub type ParseOutput = HashSet<Cube>;
const MAIN_INPUT: &str = include_str!("main_input");

pub fn parse(file: &str) -> ParseOutput {
    file.lines()
        .map(|l| match l.split(',').collect::<Vec<&str>>()[..] {
            [z, y, x] => (z.parse().unwrap(), y.parse().unwrap(), x.parse().unwrap()),
            _ => unreachable!(),
        })
        .collect()
}

fn main() {
    let cubes = parse(MAIN_INPUT);
    println!("Solution to part 1 is {}", part_1(&cubes));
    println!("Solution to part 2 is {}", part_2(&cubes));
}

fn part_1(cubes: &ParseOutput) -> Sol {
    get_surface_area(cubes)
}

fn get_surface_area(cubes: &ParseOutput) -> Sol {
    let mut sol = 0;
    for cube in cubes {
        sol += Sol::from(!cubes.contains(&(cube.0 - 1, cube.1, cube.2)));
        sol += Sol::from(!cubes.contains(&(cube.0 + 1, cube.1, cube.2)));
        sol += Sol::from(!cubes.contains(&(cube.0, cube.1 - 1, cube.2)));
        sol += Sol::from(!cubes.contains(&(cube.0, cube.1 + 1, cube.2)));
        sol += Sol::from(!cubes.contains(&(cube.0, cube.1, cube.2 - 1)));
        sol += Sol::from(!cubes.contains(&(cube.0, cube.1, cube.2 + 1)));
    }
    sol
}

fn part_2(cubes: &ParseOutput) -> Sol {
    let min_cube = cubes.iter().fold(
        (Sol::MAX, Sol::MAX, Sol::MAX),
        |(a_z, a_y, a_x), (z, y, x)| (a_z.min(*z), a_y.min(*y), a_x.min(*x)),
    );
    let max_cube = cubes.iter().fold(
        (Sol::MIN, Sol::MIN, Sol::MIN),
        |(a_z, a_y, a_x), (z, y, x)| (a_z.max(*z), a_y.max(*y), a_x.max(*x)),
    );

    let air = get_air(cubes, &min_cube, &max_cube);
    let mut holes = HashSet::new();
    for z in min_cube.0..=max_cube.0 {
        for y in min_cube.1..=max_cube.1 {
            for x in min_cube.2..=max_cube.2 {
                if !air.contains(&(z, y, x)) && !cubes.contains(&(z, y, x)) {
                    holes.insert((z, y, x));
                }
            }
        }
    }

    let surface_all = get_surface_area(cubes);
    let surface_holes = get_surface_area(&holes);

    surface_all - surface_holes
}

fn get_air(cubes: &ParseOutput, min_cube: &Cube, max_cube: &Cube) -> HashSet<Cube> {
    let mut air_cubes = HashSet::new();
    let current_path = &mut vec![(min_cube.0 - 1, min_cube.1, min_cube.2)];
    while let Some((z, y, x)) = current_path.pop() {
        if !(z >= min_cube.0 - 1
            && z <= max_cube.0 + 1
            && y >= min_cube.1 - 1
            && y <= max_cube.1 + 1
            && x >= min_cube.2 - 1
            && x <= max_cube.2 + 1)
        {
            continue;
        }
        if cubes.contains(&(z, y, x)) {
            continue;
        }
        if !air_cubes.insert((z, y, x)) {
            continue;
        }

        current_path.push((z - 1, y, x));
        current_path.push((z + 1, y, x));
        current_path.push((z, y - 1, x));
        current_path.push((z, y + 1, x));
        current_path.push((z, y, x - 1));
        current_path.push((z, y, x + 1));
    }
    air_cubes
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
            assert_eq!(part_1(black_box(&parse_output)), 4320);
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
