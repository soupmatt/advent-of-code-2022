use std::{collections::HashSet, fmt::Display};

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    count_open_sides(input)
}

pub fn part_two(_input: &str) -> Option<usize> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

fn count_open_sides(input: &str) -> Option<usize> {
    let cubes = parse_input(input);
    let mut num_sides = cubes.len() * 6;
    let mut set: HashSet<Cube> = HashSet::new();
    set.extend(cubes.into_iter());

    set.iter().for_each(|cube| {
        let adjacent = possible_adjacent_cubes(cube);
        for c in adjacent {
            if set.contains(&c) {
                num_sides -= 1;
            } else {
            }
        }
    });
    Some(num_sides)
}

fn possible_adjacent_cubes(c: &Cube) -> Vec<Cube> {
    let mut result = Vec::new();
    let x = c.x;
    let y = c.y;
    let z = c.z;
    if x > 0 {
        result.push(Cube { x: x - 1, y, z });
    }
    if y > 0 {
        result.push(Cube { x, y: y - 1, z });
    }
    if z > 0 {
        result.push(Cube { x, y, z: z - 1 });
    }
    result.push(Cube { x: x + 1, y, z });
    result.push(Cube { x, y: y + 1, z });
    result.push(Cube { x, y, z: z + 1 });
    result
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Cube {
    x: u8,
    y: u8,
    z: u8,
}

impl Display for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

fn parse_input(input: &str) -> Vec<Cube> {
    input
        .lines()
        .map(|line| {
            let (x, y, z) = line
                .splitn(3, ',')
                .map(|s| s.parse::<u8>().unwrap())
                .collect_tuple()
                .unwrap();
            Cube { x, y, z }
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(64));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), None);
    }
}
