use std::{
    fmt::Display,
    fmt::Write,
    ops::{Add, RangeInclusive},
};

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    let mut cave = Cave::no_floor(input);
    println!("{}", cave);

    while cave.drop_sand().is_some() {
        println!("{}", cave);
    }

    Some(cave.sand_count)
}

pub fn part_two(_input: &str) -> Option<usize> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

struct Cave {
    data: Vec<Vec<Tile>>,
    sand_count: usize,
    col_offset: usize,
    col_count: usize,
    row_count: usize,
}

#[derive(Debug, PartialEq)]
enum MoveErr {
    Blocked,
    Abyss,
}

impl Cave {
    fn no_floor(input: &str) -> Cave {
        let rock_formations = Cave::parse_input(input);
        let col_bounds = rock_formations
            .iter()
            .flatten()
            .map(|p| p.0)
            .minmax()
            .into_option()
            .unwrap();
        let row_max = rock_formations.iter().flatten().map(|p| p.1).max().unwrap();

        let col_count = col_bounds.1 + 3 - col_bounds.0;
        // println!("num_cols: {}", num_cols);
        let col_offset = col_bounds.0 - 1;
        // println!("col_offset: {}", col_offset);
        let row_count = row_max + 1;
        // println!("num_rows: {}", num_rows);

        let mut data = vec![vec![Tile::Air; col_count]; row_count];

        for f in rock_formations {
            let mut pts = f.iter();

            let mut p = pts.next().unwrap();
            for n in pts {
                // println!();
                // println!("{:?} -> {:?}", p, n);
                if n.1 == p.1 {
                    for c in make_range(p.0, n.0) {
                        // println!("add ({}, {})", p.1, c - col_offset);
                        data[p.1][c - col_offset] = Tile::Rock;
                    }
                } else if n.0 == p.0 {
                    for r in make_range(p.1, n.1) {
                        // println!("add ({}, {})", r, p.0 - col_offset);
                        data[r][p.0 - col_offset] = Tile::Rock;
                    }
                } else {
                    panic!("something went wrong!")
                }
                p = n;
            }
        }

        let mut cave = Cave::from_vector_data(data);
        cave.col_offset = col_offset;
        cave
    }

    fn from_vector_data(data: Vec<Vec<Tile>>) -> Cave {
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

        Cave {
            sand_count,
            col_offset: 0,
            row_count: data.len(),
            col_count: data.first().unwrap().len(),
            data,
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
        let start_col = 500 - self.col_offset;
        let sand = (start_col, start_row);

        match self.sand_move(sand) {
            Ok((new_col, new_row)) => {
                self.data[new_row][new_col] = Tile::Sand;
                self.sand_count += 1;
                Some(self.sand_count)
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

    fn handle_sand_move(&self, row: usize, col: usize) -> Result<(usize, usize), MoveErr> {
        if row >= self.row_count || col >= self.col_count {
            return Err(MoveErr::Abyss);
        }
        match &self.data[row][col] {
            Tile::Rock | Tile::Sand => Err(MoveErr::Blocked),
            Tile::Air => Ok((col, row)),
        }
    }

    fn sand_move_down(&self, (col, row): (usize, usize)) -> Result<(usize, usize), MoveErr> {
        self.handle_sand_move(row + 1, col)
    }

    fn sand_move_left(&self, (col, row): (usize, usize)) -> Result<(usize, usize), MoveErr> {
        if col == 0 {
            return Err(MoveErr::Abyss);
        }
        self.handle_sand_move(row + 1, col - 1)
    }

    fn sand_move_right(&self, (col, row): (usize, usize)) -> Result<(usize, usize), MoveErr> {
        self.handle_sand_move(row + 1, col + 1)
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
        let result = self
            .data
            .iter()
            .map(|row| {
                row.iter()
                    .map(|t| match t {
                        Tile::Air => ".",
                        Tile::Rock => "#",
                        Tile::Sand => "o",
                    })
                    .fold(String::from(""), |acc, item| acc.add(item))
            })
            .enumerate()
            .fold("".to_string(), |mut acc, (row_num, item)| {
                write!(acc, "\n{} {}", row_num, item).unwrap();
                acc
            });
        writeln!(f, "{}", result)
    }
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Air,
    Rock,
    Sand,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_cave_sand_move_down() {
        let cave = Cave::from_vector_data(vec![
            vec![Tile::Air; 3],
            vec![Tile::Air; 3],
            vec![Tile::Air, Tile::Rock, Tile::Rock],
        ]);

        assert_eq!(cave.sand_move_down((0, 0)), Ok((0, 1)));
        assert_eq!(cave.sand_move_down((1, 0)), Ok((1, 1)));
        assert_eq!(cave.sand_move_down((1, 1)), Err(MoveErr::Blocked));
        assert_eq!(cave.sand_move_down((0, 2)), Err(MoveErr::Abyss));
    }

    #[test]
    fn test_cave_sand_move_left() {
        let cave = Cave::from_vector_data(vec![
            vec![Tile::Air; 3],
            vec![Tile::Air, Tile::Sand, Tile::Air],
            vec![Tile::Rock; 3],
        ]);

        assert_eq!(cave.sand_move_left((1, 0)), Ok((0, 1)));
        assert_eq!(cave.sand_move_left((2, 0)), Err(MoveErr::Blocked));
        assert_eq!(cave.sand_move_left((1, 1)), Err(MoveErr::Blocked));
    }

    #[test]
    fn test_cave_sand_move_right() {
        let cave = Cave::from_vector_data(vec![
            vec![Tile::Air; 3],
            vec![Tile::Air, Tile::Sand, Tile::Air],
            vec![Tile::Rock; 3],
        ]);

        assert_eq!(cave.sand_move_right((1, 0)), Ok((2, 1)));
        assert_eq!(cave.sand_move_right((0, 0)), Err(MoveErr::Blocked));
        assert_eq!(cave.sand_move_right((1, 1)), Err(MoveErr::Blocked));
    }

    #[test]
    fn test_cave_sand_move() {
        let cave = Cave::from_vector_data(vec![
            vec![Tile::Air; 5],
            vec![Tile::Rock, Tile::Air, Tile::Air, Tile::Air, Tile::Air],
            vec![Tile::Rock; 5],
        ]);

        assert_eq!(cave.sand_move((0, 0)), Err(MoveErr::Abyss));
        assert_eq!(cave.sand_move((1, 0)), Ok((1, 1)));
        assert_eq!(cave.sand_move((2, 0)), Ok((2, 1)));
        assert_eq!(cave.sand_move((3, 0)), Ok((3, 1)));
        assert_eq!(cave.sand_move((4, 0)), Err(MoveErr::Abyss));

        let cave = Cave::from_vector_data(vec![
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
        ]);
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
        assert_eq!(part_two(&input), None);
    }
}
