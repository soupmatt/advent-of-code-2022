use std::{fmt::Display, ops::RangeInclusive};

use advent_of_code::sparse_table::SparseTable;
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    let mut cave = Cave::new(input, false);

    while cave.drop_sand().is_some() {}

    Some(cave.sand_count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut cave = Cave::new(input, true);

    while cave.drop_sand().is_some() {}
    println!("{cave}");

    Some(cave.sand_count)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

struct Cave {
    table: SparseTable<usize, Tile>,
    sand_count: usize,
    with_floor: bool,
}

#[derive(Debug, PartialEq)]
enum MoveErr {
    Blocked,
    Abyss,
}

impl Cave {
    fn new(input: &str, with_floor: bool) -> Cave {
        let rock_formations = Cave::parse_input(input);

        let mut table = SparseTable::new_with_start_point(Tile::Air, (0, 500));

        for f in rock_formations {
            let mut pts = f.into_iter();

            let (mut p_col, mut p_row) = pts.next().unwrap();
            for (n_col, n_row) in pts {
                if n_row == p_row {
                    for c in make_range(p_col, n_col) {
                        table.insert((p_row, c), Tile::Rock);
                    }
                } else if n_col == p_col {
                    for r in make_range(p_row, n_row) {
                        table.insert((r, p_col), Tile::Rock);
                    }
                } else {
                    panic!("something went wrong!")
                }
                (p_col, p_row) = (n_col, n_row);
            }
        }

        if with_floor {
            let floor_row = table.row_max() + 2;

            table.add_row_default(floor_row, Tile::Rock);
        }

        Cave {
            table,
            sand_count: 0,
            with_floor,
        }
    }

    fn parse_input(input: &str) -> Vec<Vec<(usize, usize)>> {
        input
            .lines()
            .map(|line| {
                line.split(" -> ")
                    .map(|coord| {
                        let c = coord.split_once(',').unwrap();
                        (c.0.parse().unwrap(), c.1.parse().unwrap())
                    })
                    .collect_vec()
            })
            .collect_vec()
    }

    fn drop_sand(&mut self) -> Option<usize> {
        let start_row = 0;
        let start_col = 500;
        let sand = (start_col, start_row);

        match self.sand_move(sand) {
            Ok((new_col, new_row)) => {
                self.table.insert((new_row, new_col), Tile::Sand);
                self.sand_count += 1;
                if (start_col, start_row) == (new_col, new_row) {
                    None
                } else {
                    Some(self.sand_count)
                }
            }
            Err(_) => None,
        }
    }

    fn sand_move(&self, sand: (usize, usize)) -> Result<(usize, usize), MoveErr> {
        let new = match self.sand_move_down(sand) {
            Ok(a) => Ok(a),
            Err(MoveErr::Abyss) => Err(MoveErr::Abyss),
            Err(MoveErr::Blocked) => match self.sand_move_left(sand) {
                Ok(b) => Ok(b),
                Err(MoveErr::Abyss) => Err(MoveErr::Abyss),
                Err(MoveErr::Blocked) => match self.sand_move_right(sand) {
                    Ok(c) => Ok(c),
                    Err(m) => Err(m),
                },
            },
        };
        match new {
            Ok(a) => self.sand_move(a),
            Err(MoveErr::Blocked) => Ok(sand),
            Err(MoveErr::Abyss) => Err(MoveErr::Abyss),
        }
    }

    fn handle_sand_move(&self, col: usize, row: usize) -> Result<(usize, usize), MoveErr> {
        if !self.with_floor
            && (row > *self.table.row_max()
                || col > *self.table.col_max()
                || col < *self.table.col_min())
        {
            return Err(MoveErr::Abyss);
        }

        match &self.table.get((row, col)) {
            Tile::Rock | Tile::Sand => Err(MoveErr::Blocked),
            Tile::Air => Ok((col, row)),
        }
    }

    fn sand_move_down(&self, (col, row): (usize, usize)) -> Result<(usize, usize), MoveErr> {
        self.handle_sand_move(col, row + 1)
    }

    fn sand_move_left(&self, (col, row): (usize, usize)) -> Result<(usize, usize), MoveErr> {
        if col == 0 {
            if self.with_floor {
                panic!("something went wrong and we are moving into column -1!")
            } else {
                return Err(MoveErr::Abyss);
            }
        }
        self.handle_sand_move(col - 1, row + 1)
    }

    fn sand_move_right(&self, (col, row): (usize, usize)) -> Result<(usize, usize), MoveErr> {
        self.handle_sand_move(col + 1, row + 1)
    }
}

fn make_range(lhs: usize, rhs: usize) -> RangeInclusive<usize> {
    if lhs > rhs {
        rhs..=lhs
    } else {
        lhs..=rhs
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Sand Count: {}\n{}", self.sand_count, self.table)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Air,
    Rock,
    Sand,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Air => write!(f, "."),
            Self::Rock => write!(f, "#"),
            Self::Sand => write!(f, "o"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cave_from_vector_data(data: Vec<Vec<Tile>>, with_floor: bool) -> Cave {
        let floor_row = data.len() + 2;
        let sand_count = data
            .iter()
            .map(|row| {
                row.iter()
                    .filter_map(|t| match t {
                        Tile::Sand => Some(1),
                        _ => None,
                    })
                    .count()
            })
            .sum();

        let mut table = SparseTable::new(Tile::Air);

        data.iter().enumerate().for_each(|(r, row)| {
            row.iter().enumerate().for_each(|(c, v)| {
                if *v != Tile::Air {
                    table.insert((r, c), *v);
                }
            })
        });

        if with_floor {
            table.add_row_default(floor_row, Tile::Rock);
        }

        Cave {
            sand_count,
            table,
            with_floor,
        }
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_cave_sand_move_down() {
        let cave = cave_from_vector_data(
            vec![
                vec![Tile::Air; 3],
                vec![Tile::Air; 3],
                vec![Tile::Air, Tile::Rock, Tile::Rock],
            ],
            false,
        );

        assert_eq!(cave.sand_move_down((0, 0)), Ok((0, 1)));
        assert_eq!(cave.sand_move_down((1, 0)), Ok((1, 1)));
        assert_eq!(cave.sand_move_down((1, 1)), Err(MoveErr::Blocked));
        assert_eq!(cave.sand_move_down((0, 2)), Err(MoveErr::Abyss));
    }

    #[test]
    fn test_cave_sand_move_left() {
        let cave = cave_from_vector_data(
            vec![
                vec![Tile::Air; 3],
                vec![Tile::Air, Tile::Sand, Tile::Air],
                vec![Tile::Rock; 3],
            ],
            false,
        );

        assert_eq!(cave.sand_move_left((1, 0)), Ok((0, 1)));
        assert_eq!(cave.sand_move_left((2, 0)), Err(MoveErr::Blocked));
        assert_eq!(cave.sand_move_left((1, 1)), Err(MoveErr::Blocked));
    }

    #[test]
    fn test_cave_sand_move_right() {
        let cave = cave_from_vector_data(
            vec![
                vec![Tile::Air; 3],
                vec![Tile::Air, Tile::Sand, Tile::Air],
                vec![Tile::Rock; 3],
            ],
            false,
        );

        assert_eq!(cave.sand_move_right((1, 0)), Ok((2, 1)));
        assert_eq!(cave.sand_move_right((0, 0)), Err(MoveErr::Blocked));
        assert_eq!(cave.sand_move_right((1, 1)), Err(MoveErr::Blocked));
    }

    #[test]
    fn test_cave_sand_move() {
        let cave = cave_from_vector_data(
            vec![
                vec![Tile::Air; 5],
                vec![Tile::Rock, Tile::Air, Tile::Air, Tile::Air, Tile::Air],
                vec![Tile::Rock; 5],
            ],
            false,
        );

        assert_eq!(cave.sand_move((0, 0)), Err(MoveErr::Abyss));
        assert_eq!(cave.sand_move((1, 0)), Ok((1, 1)));
        assert_eq!(cave.sand_move((2, 0)), Ok((2, 1)));
        assert_eq!(cave.sand_move((3, 0)), Ok((3, 1)));
        assert_eq!(cave.sand_move((4, 0)), Err(MoveErr::Abyss));

        let cave = cave_from_vector_data(
            vec![
                vec![Tile::Air; 6],
                vec![
                    Tile::Rock,
                    Tile::Air,
                    Tile::Sand,
                    Tile::Sand,
                    Tile::Air,
                    Tile::Sand,
                ],
                vec![Tile::Rock; 6],
            ],
            false,
        );
        assert_eq!(cave.sand_move((0, 0)), Err(MoveErr::Abyss));
        assert_eq!(cave.sand_move((1, 0)), Ok((1, 1)));
        assert_eq!(cave.sand_move((2, 0)), Ok((1, 1)));
        assert_eq!(cave.sand_move((3, 0)), Ok((4, 1)));
        assert_eq!(cave.sand_move((4, 0)), Ok((4, 1)));
        assert_eq!(cave.sand_move((5, 0)), Ok((4, 1)));
    }

    #[test]
    fn test_cave_sand_move_with_floor() {
        let cave = cave_from_vector_data(
            vec![
                vec![],
                vec![
                    Tile::Air,
                    Tile::Air,
                    Tile::Rock,
                    Tile::Air,
                    Tile::Air,
                    Tile::Air,
                    Tile::Air,
                ],
                vec![
                    Tile::Air,
                    Tile::Air,
                    Tile::Rock,
                    Tile::Rock,
                    Tile::Rock,
                    Tile::Rock,
                ],
            ],
            true,
        );

        assert_eq!(cave.sand_move((1, 0)), Ok((1, 4)));
        assert_eq!(cave.sand_move((2, 0)), Ok((1, 4)));
        assert_eq!(cave.sand_move((3, 0)), Ok((3, 1)));
        assert_eq!(cave.sand_move((4, 0)), Ok((4, 1)));
        assert_eq!(cave.sand_move((5, 0)), Ok((6, 4)));

        let cave = cave_from_vector_data(
            vec![
                vec![Tile::Air; 6],
                vec![
                    Tile::Rock,
                    Tile::Air,
                    Tile::Sand,
                    Tile::Sand,
                    Tile::Air,
                    Tile::Sand,
                ],
                vec![Tile::Rock; 6],
            ],
            false,
        );
        assert_eq!(cave.sand_move((0, 0)), Err(MoveErr::Abyss));
        assert_eq!(cave.sand_move((1, 0)), Ok((1, 1)));
        assert_eq!(cave.sand_move((2, 0)), Ok((1, 1)));
        assert_eq!(cave.sand_move((3, 0)), Ok((4, 1)));
        assert_eq!(cave.sand_move((4, 0)), Ok((4, 1)));
        assert_eq!(cave.sand_move((5, 0)), Ok((4, 1)));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
