use std::cmp::Ordering;

use itertools::Itertools;
use lyn::Scanner;

pub fn part_one(input: &str) -> Option<usize> {
    let chunks = input.split_whitespace().chunks(2);
    let sum = chunks
        .into_iter()
        .enumerate()
        .filter_map(|(i, mut c)| {
            let lhs = parse_input_line(c.next().unwrap()).unwrap();
            let rhs = parse_input_line(c.next().unwrap()).unwrap();
            match lhs.partial_cmp(&rhs) {
                Some(Ordering::Less) => Some(i + 1),
                Some(Ordering::Greater) => None,
                Some(Ordering::Equal) => panic!("these lines shouldn't be equal"),
                None => panic!("couldn't figure out the line ordering!"),
            }
        })
        .sum();
    Some(sum)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

fn parse_input_line(input: &str) -> Result<Item, String> {
    let mut p = Parser::new(input);
    p.parse()
}

#[derive(Debug, PartialEq)]
enum Item {
    List(Vec<Item>),
    Num(u32),
}

impl Item {
    fn cmp_lists(lhs: &[Item], rhs: &[Item]) -> Option<Ordering> {
        match (lhs, rhs) {
            (&[], &[]) => Some(Ordering::Equal),
            (&[_, ..], &[]) => Some(Ordering::Greater),
            (&[], &[_, ..]) => Some(Ordering::Less),
            ([l_head, l_tail @ ..], [r_head, r_tail @ ..]) => match l_head.partial_cmp(r_head) {
                Some(Ordering::Equal) => Self::cmp_lists(l_tail, r_tail),
                a => a,
            },
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.eq(other) {
            return Some(Ordering::Equal);
        }
        match (self, other) {
            (Item::Num(lhs), Item::Num(rhs)) => lhs.partial_cmp(rhs),
            (Item::List(lhs), Item::List(rhs)) => Item::cmp_lists(lhs, rhs),
            (Item::Num(lhs), Item::List(rhs)) => Item::cmp_lists(&[Item::Num(*lhs)], rhs),
            (Item::List(lhs), Item::Num(rhs)) => Item::cmp_lists(lhs, &[Item::Num(*rhs)]),
        }
    }
}

struct Parser {
    scanner: Scanner,
}

impl Parser {
    fn new(input: &str) -> Parser {
        Parser {
            scanner: Scanner::new(input),
        }
    }

    fn parse(&mut self) -> Result<Item, String> {
        self.parse_list()
    }

    fn parse_list(&mut self) -> Result<Item, String> {
        if self.scanner.take(&'[') {
            let mut items: Vec<Item> = vec![];
            match self.parse_items(&mut items) {
                Err(msg) => Err(msg),
                Ok(()) => {
                    if self.scanner.take(&']') {
                        Ok(Item::List(items))
                    } else {
                        Err(format!("expected list to end with ']'\n{:?}", self.scanner))
                    }
                }
            }
        } else {
            Err(format!(
                "expected list to start with '['\n{:?}",
                self.scanner
            ))
        }
    }

    fn parse_items(&mut self, items: &mut Vec<Item>) -> Result<(), String> {
        loop {
            match self.scanner.peek() {
                None => return Err(format!("expected an item\n{:?}", self.scanner)),
                Some(&'[') => match self.parse_list() {
                    Ok(list) => items.push(list),
                    Err(msg) => return Err(msg),
                },
                Some(&',') => {
                    self.scanner.pop();
                    match self.parse_items(items) {
                        Err(msg) => return Err(msg),
                        Ok(()) => break,
                    }
                }
                Some(&']') => break,
                Some(c) => {
                    if *c >= '0' && *c <= '9' {
                        match self.parse_number(0) {
                            Ok(val) => items.push(Item::Num(val)),
                            Err(msg) => return Err(msg),
                        }
                    } else {
                        return Err(format!("unexpected token {}\n{:?}", c, self.scanner));
                    }
                }
            }
        }
        Ok(())
    }

    fn parse_number(&mut self, value: u32) -> Result<u32, String> {
        let num = self.scanner.transform(|c| {
            if *c >= '0' && *c <= '9' {
                Some(c.to_string().parse::<u32>().unwrap())
            } else {
                None
            }
        });
        match num {
            Some(n) => self.parse_number(n + (value * 10)),
            None => Ok(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), None);
    }

    #[test]
    fn test_parse_input_line() {
        assert_eq!(parse_input_line("[]"), Ok(Item::List(vec![])));
        assert_eq!(
            parse_input_line("[[]]"),
            Ok(Item::List(vec![Item::List(vec![])]))
        );
        assert_eq!(
            parse_input_line("[[[]]]"),
            Ok(Item::List(vec![Item::List(vec![Item::List(vec![])])]))
        );
        assert_eq!(parse_input_line("[2]"), Ok(Item::List(vec![Item::Num(2)])));
        assert_eq!(
            parse_input_line("[2,3]"),
            Ok(Item::List(vec![Item::Num(2), Item::Num(3),]))
        );
        assert_eq!(
            parse_input_line("[2,[],3,[934,[]]]"),
            Ok(Item::List(vec![
                Item::Num(2),
                Item::List(vec![]),
                Item::Num(3),
                Item::List(vec![Item::Num(934), Item::List(vec![])]),
            ]))
        );
    }

    #[test]
    fn test_item_partial_cmp() {
        assert_eq!(
            Item::Num(2).partial_cmp(&Item::Num(2)),
            Some(Ordering::Equal)
        );
        assert_eq!(
            Item::Num(2).partial_cmp(&Item::Num(4)),
            Some(Ordering::Less)
        );
        assert_eq!(
            Item::Num(4).partial_cmp(&Item::Num(2)),
            Some(Ordering::Greater)
        );

        assert_eq!(
            Item::List(vec![]).partial_cmp(&Item::List(vec![])),
            Some(Ordering::Equal)
        );
        assert_eq!(
            Item::List(vec![Item::Num(5)]).partial_cmp(&Item::List(vec![Item::Num(5)])),
            Some(Ordering::Equal)
        );
        assert_eq!(
            Item::List(vec![]).partial_cmp(&Item::List(vec![Item::Num(5)])),
            Some(Ordering::Less)
        );
        assert_eq!(
            Item::List(vec![Item::Num(5)]).partial_cmp(&Item::List(vec![])),
            Some(Ordering::Greater)
        );

        assert_eq!(
            Item::List(vec![Item::Num(7)]).partial_cmp(&Item::List(vec![Item::Num(5)])),
            Some(Ordering::Greater)
        );
        assert_eq!(
            Item::List(vec![Item::Num(7)]).partial_cmp(&Item::List(vec![Item::Num(9)])),
            Some(Ordering::Less)
        );

        assert_eq!(
            Item::Num(5).partial_cmp(&Item::List(vec![Item::Num(5)])),
            Some(Ordering::Equal)
        );
        assert_eq!(
            Item::Num(7).partial_cmp(&Item::List(vec![Item::Num(5)])),
            Some(Ordering::Greater)
        );
        assert_eq!(
            Item::Num(7).partial_cmp(&Item::List(vec![Item::Num(9)])),
            Some(Ordering::Less)
        );

        assert_eq!(
            Item::List(vec![Item::Num(7)]).partial_cmp(&Item::Num(7)),
            Some(Ordering::Equal)
        );
        assert_eq!(
            Item::List(vec![Item::Num(7)]).partial_cmp(&Item::Num(5)),
            Some(Ordering::Greater)
        );
        assert_eq!(
            Item::List(vec![Item::Num(7)]).partial_cmp(&Item::Num(9)),
            Some(Ordering::Less)
        );
    }
}
