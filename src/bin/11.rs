#[macro_use]
extern crate lazy_static;

use std::{
    cell::{Ref, RefCell},
    collections::{BTreeMap, VecDeque},
    rc::Rc,
};

use regex::Regex;

pub fn part_one(input: &str) -> Option<usize> {
    let monkeys = parse_input(input);

    for _ in 0..20 {
        for m in monkeys.values() {
            while m.as_ref().borrow_mut().inspect_item().is_some() {}
        }
    }

    let mut sorted = monkeys
        .values()
        .map(|rc| {
            let y = rc.as_ref().borrow();
            y
        })
        .collect::<Vec<Ref<Monkey>>>();
    sorted.sort_by(|a, b| b.inspect_count.cmp(&a.inspect_count));

    let first = sorted[0].inspect_count;
    let second = sorted[1].inspect_count;
    Some(first * second)
}

pub fn part_two(_input: &str) -> Option<usize> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[derive(Debug)]
enum WorryChangeParam {
    Old,
    Val(usize),
}

#[derive(Debug)]
enum WorryChangeOp {
    Add(WorryChangeParam),
    Mult(WorryChangeParam),
}

#[derive(Debug)]
struct Monkey {
    id: usize,
    inspect_count: usize,
    items: VecDeque<usize>,
    worry_change: WorryChangeOp,
    test_mod: usize,
    true_dest: Option<Rc<RefCell<Monkey>>>,
    false_dest: Option<Rc<RefCell<Monkey>>>,
    test_true: usize,
    test_false: usize,
}

impl Monkey {
    fn inspect_item(&mut self) -> Option<bool> {
        self.items.pop_front().map(|orig_item| {
            self.inspect_count += 1;
            let item_after_inspect = match &self.worry_change {
                WorryChangeOp::Add(v) => match v {
                    WorryChangeParam::Old => orig_item + orig_item,
                    WorryChangeParam::Val(v) => orig_item + v,
                },
                WorryChangeOp::Mult(v) => match v {
                    WorryChangeParam::Old => orig_item * orig_item,
                    WorryChangeParam::Val(val) => orig_item * val,
                },
            };
            let item = item_after_inspect / 3;
            if item % self.test_mod == 0 {
                self.true_dest
                    .as_ref()
                    .unwrap()
                    .borrow_mut()
                    .items
                    .push_back(item);
                true
            } else {
                self.false_dest
                    .as_ref()
                    .unwrap()
                    .borrow_mut()
                    .items
                    .push_back(item);
                false
            }
        })
    }
}

fn parse_input(input: &str) -> BTreeMap<usize, Rc<RefCell<Monkey>>> {
    lazy_static! {
        static ref LINE1: Regex = Regex::new(r"^Monkey (\d+):$").unwrap();
        static ref LINE2: Regex = Regex::new(r"^Starting items: (.*)$").unwrap();
        static ref LINE3: Regex = Regex::new(r"^Operation: new = old (.) (\w+)$").unwrap();
        static ref LINE4: Regex = Regex::new(r"^Test: divisible by (\d+)$").unwrap();
        static ref LINE5: Regex = Regex::new(r"^If true: throw to monkey (\d+)$").unwrap();
        static ref LINE6: Regex = Regex::new(r"^If false: throw to monkey (\d+)$").unwrap();
    }

    let chunks = input.split("\n\n");
    let monkeys = chunks
        .map(|chunk| {
            let mut lines = chunk.lines().map(|s| s.trim());

            // line 1
            let line1 = lines.next().unwrap();
            let l1c = LINE1.captures(line1).unwrap();
            let id: usize = l1c.get(1).unwrap().as_str().parse().unwrap();

            // line 2
            let line2 = lines.next().unwrap();
            let l2c = LINE2.captures(line2).unwrap();
            let items = l2c
                .get(1)
                .unwrap()
                .as_str()
                .split(", ")
                .map(|s| s.parse().unwrap())
                .collect();

            // line 3
            let line3 = lines.next().unwrap();
            let l3c = LINE3.captures(line3).unwrap();
            let worry_op = l3c.get(1).unwrap().as_str();
            let worry_val_str = l3c.get(2).unwrap().as_str();
            let worry_val = if worry_val_str == "old" {
                WorryChangeParam::Old
            } else {
                let worry_val_num = worry_val_str.parse().unwrap();
                WorryChangeParam::Val(worry_val_num)
            };
            let worry_change = match worry_op {
                "*" => WorryChangeOp::Mult(worry_val),
                "+" => WorryChangeOp::Add(worry_val),
                _ => panic!("something went wrong"),
            };

            // line 4
            let line4 = lines.next().unwrap();
            let l4c = LINE4.captures(line4).unwrap();
            let test_mod = l4c.get(1).unwrap().as_str().parse().unwrap();

            // line 5
            let line5 = lines.next().unwrap();
            let l5c = LINE5.captures(line5).unwrap();
            let test_true: usize = l5c.get(1).unwrap().as_str().parse().unwrap();

            // line 6
            let line6 = lines.next().unwrap();
            let l6c = LINE6.captures(line6).unwrap();
            let test_false: usize = l6c.get(1).unwrap().as_str().parse().unwrap();

            Monkey {
                id,
                inspect_count: 0,
                items,
                worry_change,
                test_mod,
                true_dest: None,
                false_dest: None,
                test_true,
                test_false,
            }
        })
        .fold(BTreeMap::new(), |mut h, m| {
            h.insert(m.id, Rc::new(RefCell::new(m)));
            h
        });

    for m in monkeys.values() {
        let mut m = m.as_ref().borrow_mut();
        m.true_dest = monkeys.get(&m.test_true).cloned();
        m.false_dest = monkeys.get(&m.test_false).cloned();
    }

    monkeys
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), None);
    }
}
