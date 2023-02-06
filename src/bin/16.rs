#[macro_use]
extern crate lazy_static;

use std::{cell::RefCell, collections::BTreeMap};

use pathfinding::{
    prelude::astar,
    prelude::{dijkstra, dijkstra_all},
};
use regex::Regex;

pub fn part_one(input: &str) -> Option<usize> {
    let (cave, state) = CaveSystem::parse_input(input);
    let max_pressure_drop = cave.max_pressure_drop_rate;
    let (steps, cost) = answer_part_1(cave, state);

    println!("Max Possible Flow Rate {max_pressure_drop}");

    for c in steps.iter() {
        println!("Minute {}", c.minute);
        println!("Location {}", c.location);
        println!("Open Valves: {:?}", c.open_valves);
        println!("Pressure Release rate: {}", c.current_flow_rate);
        println!("Pressure Released so far: {}", c.released_pressure);
        println!("Cost: {}", c.cost);
        println!();
    }
    println!("Total Cost: {cost}");
    println!(
        "Max Possible Pressure Release: {}",
        max_pressure_drop * CaveSystem::NUM_MINUTES
    );

    Some(steps.last().unwrap().released_pressure)
}

fn answer_part_1(cave: CaveSystem, state: CaveState) -> (Vec<CaveState>, usize) {
    let cave = RefCell::new(cave);
    astar(
        &state,
        |s| cave.borrow_mut().state_successors(s),
        |s| cave.borrow().huristic_cost(s),
        |s| s.minute >= CaveSystem::NUM_MINUTES,
    )
    .unwrap()
}

pub fn part_two(_input: &str) -> Option<usize> {
    None
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Valve {
    name: String,
    rate: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct CaveState {
    minute: usize,
    location: Node,
    open_valves: Vec<Node>,
    released_pressure: usize,
    current_flow_rate: usize,
    cost: usize,
}

type Node = String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct CaveSystem {
    valves: Vec<Valve>,
    tunnel_costs: BTreeMap<(Node, Node), usize>,
    tunnels: BTreeMap<Node, Vec<Node>>,
    location: Node,
    max_pressure_drop_rate: usize,
    pressure_drop_rate: usize,
    pressure_dropped_so_far: usize,
    step_number: usize,
    estimated_cost: usize,
}

impl CaveSystem {
    const NUM_MINUTES: usize = 30;

    fn parse_input(input: &str) -> (CaveSystem, CaveState) {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.*)$")
                    .unwrap();
        }

        let mut valves = Vec::new();
        let mut tunnels: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let mut tunnel_costs = BTreeMap::new();

        input.lines().for_each(|line| {
            let caps = RE.captures(line).unwrap();
            let name = caps.get(1).unwrap().as_str().to_owned();
            let rate: usize = caps.get(2).unwrap().as_str().parse().unwrap();
            let links = caps.get(3).unwrap().as_str().split(", ");

            tunnels.insert(name.to_string(), links.map(|t| t.to_string()).collect());

            if rate > 0 {
                valves.push(Valve { name, rate });
            };
        });

        // calculate all path from AA as that is where we start
        let aa = "AA".to_string();
        valves
            .iter()
            .filter_map(|v| {
                if v.rate > 0 {
                    Some(v.name.clone())
                } else {
                    None
                }
            })
            .for_each(|dest| {
                let k = (aa.clone(), dest.clone());
                let (_, cost) = dijkstra(
                    &aa,
                    |a| {
                        tunnels
                            .get(a)
                            .unwrap()
                            .iter()
                            .map(|t| (t.to_owned(), 1_usize))
                    },
                    |a| *a == dest,
                )
                .unwrap();
                tunnel_costs.insert(k, cost);
            });

        let unrealized_pressure_drop = valves.iter().map(|v| v.rate).sum();

        (
            CaveSystem {
                valves,
                tunnels,
                tunnel_costs,
                location: "AA".to_string(),
                max_pressure_drop_rate: unrealized_pressure_drop,
                pressure_drop_rate: 0,
                pressure_dropped_so_far: 0,
                step_number: 0,
                estimated_cost: 0,
            },
            CaveState {
                current_flow_rate: 0,
                location: "AA".to_string(),
                minute: 0,
                open_valves: vec![],
                released_pressure: 0,
                cost: 0,
            },
        )
    }

    fn valve_travel_cost(&mut self, start: &str, dest: &str) -> usize {
        if start == dest {
            panic!("Something went wrong!");
        }
        match self.tunnel_costs.get(&(start.to_owned(), dest.to_owned())) {
            Some(cost) => cost.to_owned(),
            None => {
                let paths = dijkstra_all(&start, |a| {
                    self.tunnels
                        .get(*a)
                        .unwrap()
                        .iter()
                        .map(|t| (t.as_str(), 1_usize))
                });
                let (_, result) = paths.get(dest).unwrap();

                self.valves.iter().filter(|v| v.rate > 0).for_each(|v| {
                    if let Some(get) = paths.get(v.name.as_str()) {
                        self.tunnel_costs
                            .insert((start.to_owned(), v.name.to_owned()), get.1);
                    }
                });

                result.to_owned()
            }
        }
    }

    fn state_successors(&mut self, state: &CaveState) -> Vec<(CaveState, usize)> {
        let mut result = vec![];
        if state.minute < Self::NUM_MINUTES {
            for v in self.valves.clone().iter() {
                if !state.open_valves.contains(&v.name) {
                    let s = self.advance_state(state, v);
                    result.push(s);
                }
            }
            if result.is_empty() {
                let mut new_state = state.clone();
                new_state.minute = CaveSystem::NUM_MINUTES;
                new_state.cost = 0;
                let remaining_minutes = new_state.minute - state.minute;
                new_state.cost =
                    (self.max_pressure_drop_rate - new_state.current_flow_rate) * remaining_minutes;
                new_state.released_pressure += new_state.current_flow_rate * (remaining_minutes);
                let cost = new_state.cost;
                result.push((new_state, cost));
            }
        }
        result
    }

    fn advance_state(&mut self, state: &CaveState, valve: &Valve) -> (CaveState, usize) {
        let valve_cost = self.valve_travel_cost(&state.location, &valve.name) + 1;

        let mut new_state = state.clone();

        new_state.minute = state.minute + valve_cost;
        new_state.location = valve.name.to_owned();

        if new_state.minute >= Self::NUM_MINUTES {
            new_state.minute = Self::NUM_MINUTES;
        } else {
            new_state.open_valves.push(valve.name.to_owned());
            new_state.current_flow_rate = state.current_flow_rate + valve.rate;
        }
        let release_minutes = new_state.minute - state.minute;
        let cost = (self.max_pressure_drop_rate - state.current_flow_rate) * release_minutes;
        new_state.cost = cost;
        new_state.released_pressure += state.current_flow_rate * release_minutes;
        (new_state, cost)
    }

    fn huristic_cost(&self, state: &CaveState) -> usize {
        let remaining_minutes = Self::NUM_MINUTES - state.minute;
        (self.max_pressure_drop_rate - state.current_flow_rate) * remaining_minutes / 5
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
