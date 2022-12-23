use itertools::Itertools;
use num_traits::ToPrimitive;
use std::collections::VecDeque;

pub fn part_one(input: &str) -> Option<i64> {
    let mut list = MixList::parse_input(input, 1);
    list.mix_list();
    list.get_answer()
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut list = MixList::parse_input(input, 811589153);
    for _ in 0..10 {
        list.mix_list();
    }
    list.get_answer()
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Node {
    value: i64,
    index: usize,
}

struct MixList {
    list: VecDeque<Node>,
}

impl MixList {
    fn parse_input(input: &str, decryption_key: i64) -> MixList {
        let list: VecDeque<Node> = input
            .lines()
            .enumerate()
            .map(|(index, line)| Node {
                value: line.parse::<i64>().unwrap() * decryption_key,
                index,
            })
            .collect();
        MixList { list }
    }

    fn get_answer(&self) -> Option<i64> {
        let len = self.list.len();
        let zero_index = self.list.iter().position(|v| v.value == 0).unwrap();

        let one = self.list[(zero_index + 1000) % len].value;
        let two = self.list[(zero_index + 2000) % len].value;
        let three = self.list[(zero_index + 3000) % len].value;

        Some(one + two + three)
    }

    fn len(&self) -> usize {
        self.list.len()
    }

    fn mix_list(&mut self) {
        for i in 0..self.len() {
            self.run_mix_step(i);
        }
    }

    fn run_mix_step(&mut self, i: usize) {
        let (idx, node) = self.list.iter().find_position(|n| n.index == i).unwrap();

        let idx_i64 = idx.to_i64().unwrap();
        let len = self.len().to_i64().unwrap();
        let mut new_position = (idx_i64 + node.value) % (len - 1);

        if new_position < 0 {
            new_position += len - 1;
        }

        let n = self.list.remove(idx);
        self.list
            .insert(new_position.to_usize().unwrap(), n.unwrap());
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
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), Some(1623178306));
    }
}
