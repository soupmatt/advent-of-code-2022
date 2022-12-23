use std::{cell::RefCell, cmp::Ordering::*, rc::Rc};

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<i32> {
    let mut list = MixList::parse_input(input);
    list.mix_list();
    list.part_one_answer()
}

pub fn part_two(_input: &str) -> Option<i32> {
    None
}

struct Node {
    value: i32,
    index: usize,
}

struct MixList {
    orig_list: Vec<Rc<RefCell<Node>>>,
    list: Vec<Rc<RefCell<Node>>>,
}

impl MixList {
    fn parse_input(input: &str) -> MixList {
        let orig_list = input
            .lines()
            .enumerate()
            .map(|(index, line)| {
                Rc::new(RefCell::new(Node {
                    value: line.parse().unwrap(),
                    index,
                }))
            })
            .collect_vec();
        let list = orig_list.clone();
        MixList { orig_list, list }
    }

    fn part_one_answer(&self) -> Option<i32> {
        let len = self.list.len();
        let zero_index = self
            .list
            .iter()
            .position(|v| v.borrow().value == 0)
            .unwrap();

        let one = self.list[(zero_index + 1000) % len].as_ref().borrow().value;
        let two = self.list[(zero_index + 2000) % len].as_ref().borrow().value;
        let three = self.list[(zero_index + 3000) % len].as_ref().borrow().value;

        Some(one + two + three)
    }

    fn len(&self) -> usize {
        self.list.len()
    }

    fn mix_list(&mut self) {
        for i in 0..self.len() {
            let val: i32;
            let node_rc = Rc::clone(&self.orig_list[i]);
            {
                let node = node_rc.borrow();
                val = node.value;
            }
            match val.cmp(&0) {
                Greater => {
                    for _ in 0..val {
                        self.swap_right(node_rc.clone());
                    }
                }
                Less => {
                    for _ in val..0 {
                        self.swap_left(node_rc.clone());
                    }
                }
                Equal => (),
            }
        }
    }

    fn swap_right(&mut self, node: Rc<RefCell<Node>>) {
        let node_idx = node.borrow().index;
        let right_idx = self.increase_idx(node_idx);
        let right = self.list[right_idx].clone();
        self.list.swap(node_idx, right_idx);
        node.borrow_mut().index = right_idx;
        right.borrow_mut().index = node_idx;
    }

    fn swap_left(&mut self, node: Rc<RefCell<Node>>) {
        let node_idx = node.borrow().index;
        let left_idx = self.decrease_idx(node_idx);
        let left = self.list[left_idx].clone();
        self.list.swap(node_idx, left_idx);
        node.borrow_mut().index = left_idx;
        left.borrow_mut().index = node_idx;
    }

    fn increase_idx(&self, idx: usize) -> usize {
        (idx + 1) % self.len()
    }

    fn decrease_idx(&self, idx: usize) -> usize {
        if idx == 0 {
            self.len() - 1
        } else {
            idx - 1
        }
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_mix_list() {
        let input = advent_of_code::read_file("examples", 20);
        let mut list = MixList::parse_input(&input);
        list.mix_list();
        let actual = list.list.iter().map(|n| n.borrow().value).collect_vec();
        assert_eq!(actual, vec![-2, 1, 2, -3, 4, 0, 3])
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), None);
    }
}
