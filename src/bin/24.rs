use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Display,
};

use pathfinding::prelude::fringe;

pub fn part_one(input: &str) -> Option<usize> {
    let valley = WindyValley::parse_input(input);

    let results = fringe(
        &valley,
        |v| v.possible_next_steps(),
        |v| v.minimum_cost(),
        |v| v.position == v.exit,
    );
    results.map(|(valleys, _)| valleys.len() - 1)
}

pub fn part_two(_input: &str) -> Option<usize> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 24);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Dir {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Bliz {
    pos: Pos,
    dir: Dir,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Pos(usize, usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct WindyValley {
    east_boundary: usize,
    south_boundary: usize,
    position: Pos,
    exit: Pos,
    blizzards: Vec<Bliz>,
}

impl WindyValley {
    const ENTRANCE: Pos = Pos(1, 0);

    fn parse_input(input: &str) -> WindyValley {
        let mut exit = Pos(0, 0);
        let mut blizzards = vec![];

        let mut lines = input.trim_end().lines().enumerate();
        let (_, first) = lines.next().unwrap();
        let east_boundary = first.len() - 1;
        exit.0 = east_boundary - 1;

        let mut south_boundary = 0;
        for (y, line) in lines {
            south_boundary += 1;
            for (x, c) in line.char_indices() {
                match c {
                    '^' => blizzards.push(Bliz {
                        pos: Pos(x, y),
                        dir: Dir::North,
                    }),
                    'v' => blizzards.push(Bliz {
                        pos: Pos(x, y),
                        dir: Dir::South,
                    }),
                    '<' => blizzards.push(Bliz {
                        pos: Pos(x, y),
                        dir: Dir::West,
                    }),
                    '>' => blizzards.push(Bliz {
                        pos: Pos(x, y),
                        dir: Dir::East,
                    }),
                    _ => (),
                }
            }
        }
        exit.1 = south_boundary;

        WindyValley {
            east_boundary,
            south_boundary,
            position: WindyValley::ENTRANCE,
            exit,
            blizzards,
        }
    }

    fn candidate_moves(&self) -> BTreeSet<Pos> {
        let mut result = BTreeSet::new();

        // stay put
        result.insert(self.position);

        // go north
        if self.position.1 > 1 {
            result.insert(Pos(self.position.0, self.position.1 - 1));
        }

        // go south
        if self.position.1 < self.south_boundary - 1
            || self.position == Pos(self.exit.0, self.exit.1 - 1)
        {
            result.insert(Pos(self.position.0, self.position.1 + 1));
        }

        // go west
        if self.position.0 > 1 && self.position.1 != 0 {
            result.insert(Pos(self.position.0 - 1, self.position.1));
        }

        // go east
        if self.position.0 < self.east_boundary - 1 && self.position.1 != 0 {
            result.insert(Pos(self.position.0 + 1, self.position.1));
        }

        result
    }

    fn minimum_cost(&self) -> usize {
        self.position.0.abs_diff(self.exit.0) + self.position.1.abs_diff(self.exit.1)
    }

    fn move_blizzards(&mut self, candidate_moves: &mut BTreeSet<Pos>) {
        for b in self.blizzards.iter_mut() {
            match b.dir {
                Dir::North => {
                    if b.pos.1 == 1 {
                        b.pos.1 = self.south_boundary - 1;
                    } else {
                        b.pos.1 -= 1;
                    }
                }
                Dir::South => {
                    if b.pos.1 < self.south_boundary - 1 {
                        b.pos.1 += 1;
                    } else {
                        b.pos.1 = 1;
                    }
                }
                Dir::West => {
                    if b.pos.0 == 1 {
                        b.pos.0 = self.east_boundary - 1;
                    } else {
                        b.pos.0 -= 1;
                    }
                }
                Dir::East => {
                    if b.pos.0 == self.east_boundary - 1 {
                        b.pos.0 = 1;
                    } else {
                        b.pos.0 += 1;
                    }
                }
            }
            candidate_moves.remove(&b.pos);
        }
    }

    fn possible_next_steps(&self) -> Vec<(WindyValley, usize)> {
        let mut moves = self.candidate_moves();
        if moves.is_empty() {
            return vec![];
        }

        let mut moved = self.clone();
        moved.move_blizzards(&mut moves);
        if moves.is_empty() {
            return vec![];
        }

        let mut result = vec![];
        while result.len() < moves.len() - 1 {
            result.push((moved.clone(), 1));
        }
        result.push((moved.clone(), 1));

        for (i, p) in moves.into_iter().enumerate() {
            result[i].0.position = p;
        }

        result
    }
}

impl Display for WindyValley {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // first row
        write!(f, "#")?;
        if self.position == Self::ENTRANCE {
            write!(f, "E")?;
        } else {
            write!(f, ".")?;
        }
        for _ in 2..self.east_boundary {
            write!(f, "#")?;
        }
        writeln!(f, "#")?;

        let bmap: BTreeMap<Pos, Vec<&Bliz>> =
            self.blizzards.iter().fold(BTreeMap::new(), |mut m, b| {
                m.entry(b.pos)
                    .and_modify(|v| v.push(b))
                    .or_insert_with(|| vec![b]);
                m
            });

        // main body
        for y in 1..self.south_boundary {
            write!(f, "#")?;
            for x in 1..self.east_boundary {
                let pos = Pos(x, y);
                if pos == self.position {
                    write!(f, "E")?;
                } else {
                    match bmap.get(&pos) {
                        None => write!(f, ".")?,
                        Some(v) => match v.len() {
                            0 => write!(f, ".")?,
                            1 => match v[0].dir {
                                Dir::North => write!(f, "^")?,
                                Dir::South => write!(f, "v")?,
                                Dir::East => write!(f, ">")?,
                                Dir::West => write!(f, "<")?,
                            },
                            s => write!(f, "{}", s)?,
                        },
                    }
                }
            }
            writeln!(f, "#")?;
        }

        // last row
        for x in 0..=self.east_boundary {
            if x == self.exit.0 {
                if self.position == self.exit {
                    write!(f, "E")?;
                } else {
                    write!(f, ".")?;
                }
            } else {
                write!(f, "#")?;
            }
        }
        writeln!(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_one(&input), Some(18));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_two(&input), None);
    }
}
