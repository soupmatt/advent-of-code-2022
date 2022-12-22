#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

use regex::Regex;

pub fn part_one(input: &str) -> Option<usize> {
    let mut monkeys: Monkeys = parse_input(input);
    monkeys.find_monkey_value("root")
}

pub fn part_two(_input: &str) -> Option<usize> {
    None
}

fn parse_input(input: &str) -> Monkeys {
    lazy_static! {
        static ref CONST_MONKEY: Regex = Regex::new(r"^(\w+): (\d+)$").unwrap();
        static ref EQ_MONKEY: Regex = Regex::new(r"^(\w+): (\w+) (.) (\w+)$").unwrap();
    }

    let monkeys = input
        .lines()
        .map(|line| {
            if let Some(captures) = CONST_MONKEY.captures(line) {
                let id = captures.get(1).unwrap().as_str();
                let val: usize = captures.get(2).unwrap().as_str().parse().unwrap();
                (
                    id.to_owned(),
                    Monkey {
                        constant: Some(val),
                        operation: None,
                        params: None,
                    },
                )
            } else {
                let captures = EQ_MONKEY.captures(line).unwrap();
                let id = captures.get(1).unwrap().as_str();
                let lhs = captures.get(2).unwrap().as_str();
                let op = captures.get(3).unwrap().as_str();
                let rhs = captures.get(4).unwrap().as_str();
                (
                    id.to_owned(),
                    Monkey {
                        constant: None,
                        operation: Some(Operation::parse(op)),
                        params: Some((lhs.to_owned(), rhs.to_owned())),
                    },
                )
            }
        })
        .collect();
    Monkeys {
        monkeys,
        values: HashMap::new(),
    }
}

struct Monkeys {
    monkeys: HashMap<String, Monkey>,
    values: HashMap<String, usize>,
}

impl Monkeys {
    fn find_monkey_value(&mut self, name: &str) -> Option<usize> {
        {
            if let Some(v) = self.values.get(name) {
                return Some(*v);
            }
        }
        let params: (String, String);
        let op: Operation;
        {
            let monkey = self.monkeys.get(name).unwrap();
            if let Some(value) = monkey.constant {
                self.values.insert(name.to_owned(), value);
                Some(value)
            } else {
                params = monkey.params.as_ref().unwrap().clone();
                op = monkey.operation.unwrap();
                let (lhs_name, rhs_name) = params;
                let lhs = self.find_monkey_value(&lhs_name).unwrap();
                let rhs = self.find_monkey_value(&rhs_name).unwrap();
                let result = match op {
                    Operation::Add => lhs + rhs,
                    Operation::Sub => lhs - rhs,
                    Operation::Mul => lhs * rhs,
                    Operation::Div => lhs / rhs,
                };
                self.values.insert(name.to_owned(), result);
                Some(result)
            }
        }
    }
}

struct Monkey {
    constant: Option<usize>,
    operation: Option<Operation>,
    params: Option<(String, String)>,
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operation {
    fn parse(input: &str) -> Operation {
        match input {
            "+" => Operation::Add,
            "-" => Operation::Sub,
            "*" => Operation::Mul,
            "/" => Operation::Div,
            _ => panic!("unrecognized operation {}", input),
        }
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one(&input), Some(152));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_two(&input), None);
    }
}
