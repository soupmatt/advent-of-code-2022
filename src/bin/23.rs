use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use num_traits::ToPrimitive;
use strum::{EnumCount, EnumIter, IntoEnumIterator};

pub fn part_one(input: &str) -> Option<usize> {
    let mut crater = Crater::parse_input(input);
    for _ in 0..10 {
        crater.run_round();
    }
    Some(crater.part_one_answer())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut crater = Crater::parse_input(input);
    let mut count = 1;
    while crater.run_round() > 0 {
        count += 1;
    }
    Some(count)
}

#[derive(EnumIter, EnumCount, Clone, Copy, Debug)]
enum Dir {
    North,
    South,
    West,
    East,
}

impl Dir {
    fn dir_rotator() -> DirRotator {
        let dirs = Dir::iter().collect();
        DirRotator { dirs }
    }
}

struct DirRotator {
    dirs: VecDeque<Dir>,
}

impl Iterator for DirRotator {
    type Item = Vec<Dir>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.dirs.iter().cloned().collect_vec();
        let first = self.dirs.pop_front().unwrap();
        self.dirs.push_back(first);
        Some(result)
    }
}

type Elf = (isize, isize);

struct Crater {
    positions: HashSet<Elf>,
    proposed_moves: HashMap<Elf, Vec<Elf>>,
    dir_iter: DirRotator,
}

impl Crater {
    fn parse_input(input: &str) -> Crater {
        let positions: HashSet<Elf> = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| match c {
                    '#' => Some((x.to_isize().unwrap(), y.to_isize().unwrap())),
                    _ => None,
                })
            })
            .collect();

        let dir_iter = Dir::dir_rotator();
        Crater {
            positions,
            proposed_moves: HashMap::new(),
            dir_iter,
        }
    }

    fn run_round(&mut self) -> usize {
        self.propose_moves();
        self.execute_moves()
    }

    fn propose_moves(&mut self) {
        let dirs = self.dir_iter.next().unwrap();
        let moves = self
            .positions
            .iter()
            .filter_map(|e| self.propose_move(e, &dirs))
            .collect_vec();

        moves.iter().for_each(|(current, proposed)| {
            self.proposed_moves
                .entry(*proposed)
                .and_modify(|v| v.push(*current))
                .or_insert_with(|| vec![*current]);
        });
    }

    fn execute_moves(&mut self) -> usize {
        let mut num_moves = 0;
        self.proposed_moves.iter_mut().for_each(|(dest, elves)| {
            if elves.len() == 1 {
                num_moves += 1;
                let from = &elves[0];
                self.positions.remove(from);
                self.positions.insert(*dest);
            }
            elves.clear();
        });
        num_moves
    }

    fn propose_move(&self, &(x, y): &Elf, dirs: &[Dir]) -> Option<(Elf, Elf)> {
        let nw = self.positions.contains(&(x - 1, y - 1));
        let n = self.positions.contains(&(x, y - 1));
        let ne = self.positions.contains(&(x + 1, y - 1));
        let e = self.positions.contains(&(x + 1, y));
        let se = self.positions.contains(&(x + 1, y + 1));
        let s = self.positions.contains(&(x, y + 1));
        let sw = self.positions.contains(&(x - 1, y + 1));
        let w = self.positions.contains(&(x - 1, y));

        if !(nw || n || ne || e || se || s || sw || w) {
            return None;
        }
        dirs.iter().find_map(|dir| match dir {
            Dir::North => {
                if !(ne || n || nw) {
                    Some(((x, y), (x, y - 1)))
                } else {
                    None
                }
            }
            Dir::South => {
                if !(se || s || sw) {
                    Some(((x, y), (x, y + 1)))
                } else {
                    None
                }
            }
            Dir::West => {
                if !(nw || w || sw) {
                    Some(((x, y), (x - 1, y)))
                } else {
                    None
                }
            }
            Dir::East => {
                if !(ne || e || se) {
                    Some(((x, y), (x + 1, y)))
                } else {
                    None
                }
            }
        })
    }

    fn part_one_answer(&self) -> usize {
        let (xmin, xmax, ymin, ymax) = self.dim_ranges();
        let xdim = xmax.abs_diff(xmin) + 1;
        let ydim = ymax.abs_diff(ymin) + 1;
        (xdim * ydim) - self.positions.len()
    }

    fn dim_ranges(&self) -> (isize, isize, isize, isize) {
        let mut i = self.positions.iter();
        let (x, y) = *i.next().unwrap();
        i.fold((x, x, y, y), |(xmin, xmax, ymin, ymax), (x, y)| {
            (xmin.min(*x), xmax.max(*x), ymin.min(*y), ymax.max(*y))
        })
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 23);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_one(&input), Some(110));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_two(&input), Some(20));
    }
}
