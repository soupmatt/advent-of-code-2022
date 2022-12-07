#[macro_use]
extern crate lazy_static;

use regex::Regex;

pub fn part_one(input: &str) -> Option<String> {
    let (start_state, commands) = split_input(input);
    let mut stacks = parse_initial_stacks(start_state);
    commands
        .lines()
        .map(parse_move_line)
        .for_each(|cmd| execute_move(&mut stacks, &cmd));
    let result: String = stacks.iter_mut().map(|v| v.pop().unwrap()).collect();
    Some(result)
}

fn split_input(input: &str) -> (&str, &str) {
    input.split_once("\n\n").unwrap()
}

fn parse_initial_stacks(input: &str) -> Vec<Vec<char>> {
    let mut result = vec![];
    input.lines().for_each(|line| {
        let mut chars = line.chars().fuse();
        let columns = (line.len() + 1) / 4;
        while result.len() < columns {
            result.push(vec![])
        }
        chars.next(); // skip first character
        let mut i = 0;
        loop {
            match chars.next() {
                None => break,
                Some(c) => match c {
                    ' ' => (),
                    'A'..='Z' => {
                        let v = result.get_mut(i);
                        match v {
                            None => result[i] = vec![c],
                            Some(s) => s.push(c),
                        }
                    }
                    _ => break,
                },
            }
            // consume the closing brace, the space and the opening brace
            chars.next();
            chars.next();
            chars.next();
            i += 1;
        }
    });
    result.iter_mut().for_each(|v| {
        v.reverse();
    });
    result
}

#[derive(Debug, PartialEq, Eq)]
struct Command {
    count: u32,
    from: usize,
    to: usize,
}

fn parse_move_line(input: &str) -> Command {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    }

    let caps = RE.captures(input).unwrap();

    Command {
        count: caps.get(1).unwrap().as_str().parse().unwrap(),
        from: caps.get(2).unwrap().as_str().parse().unwrap(),
        to: caps.get(3).unwrap().as_str().parse().unwrap(),
    }
}

fn execute_move(stacks: &mut [Vec<char>], cmd: &Command) {
    for _ in 0..cmd.count {
        let from = stacks.get_mut(cmd.from - 1).unwrap();
        let item = from.pop().unwrap();
        let to = stacks.get_mut(cmd.to - 1).unwrap();
        to.push(item);
    }
}

pub fn part_two(_input: &str) -> Option<String> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = &advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_parse_initial_stacks() {
        let input = &advent_of_code::read_file("examples", 5);
        let (input, _) = split_input(input);
        assert_eq!(
            parse_initial_stacks(input),
            vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]
        )
    }

    #[test]
    fn test_parse_move_line() {
        assert_eq!(
            parse_move_line("move 2 from 2 to 1"),
            Command {
                count: 2,
                from: 2,
                to: 1
            }
        );
        assert_eq!(
            parse_move_line("move 34 from 98 to 17"),
            Command {
                count: 34,
                from: 98,
                to: 17
            }
        );
    }

    #[test]
    fn test_execute_move() {
        let mut stacks = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        execute_move(
            &mut stacks,
            &Command {
                count: 1,
                from: 2,
                to: 3,
            },
        );
        assert_eq!(stacks, vec![vec!['Z', 'N'], vec!['M', 'C'], vec!['P', 'D']]);

        let mut stacks = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        execute_move(
            &mut stacks,
            &Command {
                count: 2,
                from: 1,
                to: 2,
            },
        );
        assert_eq!(
            stacks,
            vec![vec![], vec!['M', 'C', 'D', 'N', 'Z'], vec!['P']]
        );
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), None);
    }
}
