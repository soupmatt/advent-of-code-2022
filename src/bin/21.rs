#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

use regex::Regex;

pub fn part_one(input: &str) -> Option<usize> {
    let mut monkeys: Monkeys = parse_input(input);
    monkeys.find_monkey_value("root")
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut monkeys: Monkeys = parse_input(input);
    make_part_two_mods(&mut monkeys);

    let root = monkeys.monkeys.get("root").unwrap();
    let (lhs_name, rhs_name) = root.params.as_ref().unwrap();
    let lhs = monkeys.find_monkey_expression(lhs_name);
    let rhs = monkeys.find_monkey_expression(rhs_name);

    if let Expression::Const(val) = lhs {
        Some(solve_for_variable(val, &rhs))
    } else if let Expression::Const(val) = rhs {
        Some(solve_for_variable(val, &lhs))
    } else {
        None
    }
}

fn handle_variable(result: usize, expr: &Expression) -> usize {
    match expr {
        Expression::Var(_) => result,
        _ => solve_for_variable(result, expr),
    }
}

fn solve_for_variable(result: usize, expr: &Expression) -> usize {
    if let Expression::Op(op, lhs, rhs) = expr {
        match (lhs.as_ref(), rhs.as_ref()) {
            (Expression::Const(val), exp) => {
                // do something
                match op {
                    Operation::Add => {
                        // result = val + exp
                        // result - val = exp
                        let new_result = result - val;
                        handle_variable(new_result, exp)
                    }
                    Operation::Sub => {
                        // result = val - exp
                        // result + exp = val
                        // exp = val - result
                        let new_result = val - result;
                        handle_variable(new_result, exp)
                    }
                    Operation::Mul => {
                        // result = val * exp
                        // result / val = exp
                        let new_result = result / val;
                        handle_variable(new_result, exp)
                    }
                    Operation::Div => {
                        // result = val / exp
                        // result * exp = val
                        // exp = val / result
                        let new_result = val / result;
                        handle_variable(new_result, exp)
                    }
                }
            }
            (exp, Expression::Const(val)) => {
                // do something
                match op {
                    Operation::Add => {
                        // result = exp + val
                        // result - val = exp
                        let new_result = result - val;
                        handle_variable(new_result, exp)
                    }
                    Operation::Sub => {
                        // result = exp - val
                        // result + val = exp
                        let new_result = result + val;
                        handle_variable(new_result, exp)
                    }
                    Operation::Mul => {
                        // result = exp * val
                        // result / val = exp
                        let new_result = result / val;
                        handle_variable(new_result, exp)
                    }
                    Operation::Div => {
                        // result = exp / val
                        // result * val = exp
                        let new_result = result * val;
                        handle_variable(new_result, exp)
                    }
                }
            }
            _ => panic!("something went wrong"),
        }
    } else {
        panic!("something went wrong");
    }
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
                        variable: None,
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
                        variable: None,
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

fn make_part_two_mods(monkeys: &mut Monkeys) {
    monkeys.monkeys.entry("humn".to_string()).and_modify(|m| {
        m.constant = None;
        m.operation = None;
        m.params = None;
        m.variable = Some("humn".to_string());
    });
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

    fn find_monkey_expression(&self, name: &str) -> Expression {
        let monkey = self.monkeys.get(name).unwrap();
        if let Some(value) = monkey.constant {
            return Expression::Const(value);
        }
        if let Some(var) = monkey.variable.as_ref() {
            return Expression::Var(var.clone());
        }
        let (lhs_name, rhs_name) = monkey.params.as_ref().unwrap();
        let lhs = self.find_monkey_expression(lhs_name);
        let rhs = self.find_monkey_expression(rhs_name);
        let op = monkey.operation.unwrap();
        if let (Expression::Const(l_val), Expression::Const(r_val)) = (lhs.clone(), rhs.clone()) {
            match op {
                Operation::Add => Expression::Const(l_val + r_val),
                Operation::Sub => Expression::Const(l_val - r_val),
                Operation::Mul => Expression::Const(l_val * r_val),
                Operation::Div => Expression::Const(l_val / r_val),
            }
        } else {
            Expression::Op(op, Box::new(lhs), Box::new(rhs))
        }
    }
}

struct Monkey {
    variable: Option<String>,
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

#[derive(Debug, Clone)]
enum Expression {
    Const(usize),
    Op(Operation, Box<Expression>, Box<Expression>),
    Var(String),
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
        assert_eq!(part_two(&input), Some(301));
    }
}
