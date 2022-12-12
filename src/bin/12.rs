use std::{
    collections::BTreeMap,
    fmt::{Debug, Display},
};

use pathfinding::prelude::dijkstra;

pub fn part_one(input: &str) -> Option<u32> {
    let (start, goal, edges) = parse_input(input);

    let path = dijkstra(
        &start,
        |p| {
            if let Some(neighbors) = edges.get(p) {
                neighbors.iter().map(|p| (p.clone(), 1u32))
            } else {
                panic!(
                    "Something went wrong trying to find the neighbors of {:?}",
                    p
                )
            }
        },
        |p| *p == goal,
    );
    path.map(|(_, cost)| cost)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn parse_input(input: &str) -> (Pos, Pos, BTreeMap<Pos, Vec<Pos>>) {
    let mut start = Pos(0, 0);
    let mut goal = Pos(0, 0);
    let mut edges = BTreeMap::new();

    let chars: Vec<Vec<u8>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    'S' => {
                        start = Pos(x, y);
                        b'a'
                    }
                    'E' => {
                        goal = Pos(x, y);
                        b'z'
                    }
                    _ => c as u8,
                })
                .collect()
        })
        .collect();

    let y_max = chars.len() - 1;
    let x_max = chars.first().unwrap().len() - 1;

    let mut add_to_map = |p: &Pos, c: u8| {
        let Pos(x, y) = *p;
        let mut neighbors = vec![];
        if x > 0 && chars[y][x - 1] <= c + 1 {
            neighbors.push(Pos(x - 1, y))
        };
        if x < x_max && chars[y][x + 1] <= c + 1 {
            neighbors.push(Pos(x + 1, y))
        };
        if y > 0 && chars[y - 1][x] <= c + 1 {
            neighbors.push(Pos(x, y - 1))
        };
        if y < y_max && chars[y + 1][x] <= c + 1 {
            neighbors.push(Pos(x, y + 1))
        };
        edges.insert(p.clone(), neighbors);
    };

    chars.iter().enumerate().for_each(|(y, row)| {
        row.iter()
            .enumerate()
            .for_each(|(x, c)| add_to_map(&Pos(x, y), *c))
    });

    (start, goal, edges)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), None);
    }
}
