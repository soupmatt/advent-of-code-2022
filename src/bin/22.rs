pub fn part_one(input: &str) -> Option<i32> {
    let (map, instructions) = parse_input(input);
    let start_pos = map[0].iter().position(|t| *t == Tile::Open).unwrap() as i32;
    let mut pos = Coord {
        col: start_pos,
        row: 0,
    };
    let mut dir = Direction::R;

    for ins in instructions {
        match ins {
            Instruction::Rotate(t) => dir = dir.turn(&t),
            Instruction::Forward(movement_amount) => {
                for _ in 0..movement_amount {
                    let Coord { row: dr, col: dc } = dir.offset();
                    let new_tile = map
                        .get((pos.row + dr) as usize)
                        .and_then(|row| row.get((pos.col + dc) as usize))
                        .unwrap_or(&Tile::None);

                    match new_tile {
                        Tile::Solid => break,
                        Tile::Open => {
                            pos = Coord {
                                row: pos.row + dr,
                                col: pos.col + dc,
                            }
                        }
                        Tile::None => {
                            let new_pos = wrap(&map, &pos, &dir);

                            // if new_pos is solid stop moving
                            if map[new_pos.row as usize][new_pos.col as usize] == Tile::Solid {
                                break;
                            }

                            pos = new_pos;
                        }
                    }
                }
            }
        }
    }

    let result = 1000 * (pos.row + 1) + 4 * (pos.col + 1) + (dir.score() as i32);

    Some(result)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn wrap(map: &[Vec<Tile>], pos: &Coord, dir: &Direction) -> Coord {
    let Coord { row: dr, col: dc } = dir.offset();
    let mut curr = pos.clone();

    // walk backwards until we find a Tile::None
    while let Some(tile) = map
        .get((curr.row - dr) as usize)
        .and_then(|row| row.get((curr.col - dc) as usize))
    {
        if *tile == Tile::None {
            break;
        }

        curr = Coord {
            row: curr.row - dr,
            col: curr.col - dc,
        }
    }

    curr
}

enum Instruction {
    Rotate(Turn),
    Forward(u8),
}

enum Turn {
    L,
    R,
}

#[derive(PartialEq)]
enum Tile {
    Open,
    Solid,
    None,
}

fn parse_input(input: &str) -> (Vec<Vec<Tile>>, Vec<Instruction>) {
    let mut instructions = Vec::new();
    let mut digits = Vec::new();

    let (grid, moves) = input.trim_end().split_once("\n\n").unwrap();
    for c in moves.chars() {
        if c.is_numeric() {
            let digit = c.to_digit(10).unwrap() as u8;
            digits.push(digit);
        } else {
            //construct a number out of digits
            let num = digits.iter().fold(0, |num, digit| num * 10 + digit);
            digits.clear();
            instructions.push(Instruction::Forward(num));

            //parse the turn
            let turn = match c {
                'L' => Turn::L,
                'R' => Turn::R,
                _ => panic!("Invalid input {c}"),
            };
            instructions.push(Instruction::Rotate(turn));
        }
    }
    //construct a number out of remaining digits
    let num = digits.iter().fold(0, |num, digit| num * 10 + digit);
    digits.clear();
    instructions.push(Instruction::Forward(num));

    let mut map = Vec::new();
    for line in grid.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            let tile = match c {
                '.' => Tile::Open,
                '#' => Tile::Solid,
                ' ' => Tile::None,
                _ => panic!("invalid input {c}"),
            };
            row.push(tile);
        }
        map.push(row);
    }

    (map, instructions)
}

#[derive(Clone)]
struct Coord {
    row: i32,
    col: i32,
}

enum Direction {
    L,
    R,
    U,
    D,
}

impl Direction {
    fn score(&self) -> usize {
        use Direction::*;
        match self {
            R => 0,
            D => 1,
            L => 2,
            U => 3,
        }
    }

    fn turn(self, turn: &Turn) -> Direction {
        use Direction::*;
        match (self, turn) {
            (L, Turn::L) => D,
            (L, Turn::R) => U,
            (R, Turn::L) => U,
            (R, Turn::R) => D,
            (U, Turn::L) => L,
            (U, Turn::R) => R,
            (D, Turn::L) => R,
            (D, Turn::R) => L,
        }
    }

    fn offset(&self) -> Coord {
        use Direction::*;
        match &self {
            L => Coord { row: 0, col: -1 },
            R => Coord { row: 0, col: 1 },
            U => Coord { row: -1, col: 0 },
            D => Coord { row: 1, col: 0 },
        }
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 22);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_one(&input), Some(6032));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_two(&input), None);
    }
}
