#[macro_use]
extern crate lazy_static;

use std::collections::BTreeMap;

use itertools::Itertools;
use pathfinding::{
    prelude::dijkstra,
    prelude::{astar, fringe},
};
use regex::Regex;

pub fn part_one(input: &str) -> Option<usize> {
    let cave = CaveSystem::parse_input(input);
    let (steps, _) = astar(
        &cave,
        |c| c.successors(),
        |c| c.potential_pressure_drop,
        |c| c.potential_pressure_drop == 0 || c.step_number == 30,
    )
    .unwrap();

    for c in steps.iter() {
        println!("Step {}", c.step_number);
        println!("Location {}", c.location);
        println!(
            "Open Valves: {:?}",
            c.valves
                .values()
                .filter(|v| v.open)
                .map(|v| format!("{}: {}", v.name, v.rate))
                .collect_vec()
        );
        println!(
            "Closed Valves: {:?}",
            c.valves
                .values()
                .filter(|v| !v.open)
                .map(|v| format!("{}: {}", v.name, v.rate))
                .collect_vec()
        );
        println!("Pressure Release rate: {}", c.pressure_drop_rate);
        println!("Pressure Released so far: {}", c.pressure_dropped_so_far);
        println!();
    }

    Some(steps.last().unwrap().total_pressure_drop())
}

pub fn part_two(_input: &str) -> Option<usize> {
    None
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Valve {
    name: String,
    rate: usize,
    open: bool,
}

type Node = String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct CaveSystem {
    valves: BTreeMap<Node, Valve>,
    tunnels: BTreeMap<Node, Vec<Node>>,
    location: Node,
    potential_pressure_drop: usize,
    pressure_drop_rate: usize,
    pressure_dropped_so_far: usize,
    step_number: usize,
}

impl CaveSystem {
    fn parse_input(input: &str) -> CaveSystem {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.*)$")
                    .unwrap();
        }

        let mut valves = BTreeMap::new();
        let mut tunnels = BTreeMap::new();

        input.lines().for_each(|line| {
            let caps = RE.captures(line).unwrap();
            let name = caps.get(1).unwrap().as_str();
            let rate: usize = caps.get(2).unwrap().as_str().parse().unwrap();
            let links = caps.get(3).unwrap().as_str().split(", ");

            if rate > 0 {
                valves.insert(
                    name.to_string(),
                    Valve {
                        name: name.to_string(),
                        rate,
                        open: false,
                    },
                );
            };

            tunnels.insert(name.to_string(), links.map(|t| t.to_string()).collect());
        });

        let potential_pressure_drop = valves.values().map(|v| v.rate).sum();

        CaveSystem {
            valves,
            tunnels,
            location: "AA".to_string(),
            potential_pressure_drop,
            pressure_drop_rate: 0,
            pressure_dropped_so_far: 0,
            step_number: 0,
        }
    }

    fn successors(&self) -> Vec<(CaveSystem, usize)> {
        let mut result = vec![];

        let mut cost_per_minute: usize = 0;
        let open_valves = self
            .valves
            .iter()
            .filter_map(|(k, v)| {
                if v.open {
                    None
                } else {
                    cost_per_minute += v.rate;
                    Some(k)
                }
            })
            .collect_vec();

        for v_name in open_valves {
            let path = dijkstra(
                &self.location,
                |l| {
                    self.tunnels
                        .get(l)
                        .unwrap()
                        .iter()
                        .map(|v| (v.to_string(), 1_usize))
                        .collect_vec()
                },
                |l| l == v_name,
            );
            if let Some((_, steps)) = path {
                let mut cave = self.clone();
                cave.advance_minutes(v_name, steps + 1);
                result.push((cave, cost_per_minute * (steps + 1)));
            }
        }

        result
    }

    fn advance_minutes(&mut self, dest: &str, steps: usize) {
        let mut step_delta = steps;
        if self.step_number + steps > 30 {
            step_delta = self.step_number + steps - 30;
            self.step_number = 30;
        } else {
            self.step_number += steps;
            self.location = dest.to_string();
        }

        self.pressure_dropped_so_far += self.pressure_drop_rate * step_delta;

        self.activate_valve(dest);
    }

    fn activate_valve(&mut self, valve_name: &str) {
        let v = self.valves.get_mut(valve_name).unwrap();
        if !v.open {
            v.open = true;
            self.potential_pressure_drop -= v.rate;
            self.pressure_drop_rate += v.rate;
        }
    }

    fn total_pressure_drop(&self) -> usize {
        let minutes_left = 30 - self.step_number;
        (self.total_potential_pressure_drop() * minutes_left) + self.pressure_dropped_so_far
    }

    fn total_potential_pressure_drop(&self) -> usize {
        self.valves.values().map(|v| v.rate).sum()
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), None);
    }
}
