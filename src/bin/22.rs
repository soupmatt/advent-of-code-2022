pub fn part_one(input: &str) -> Option<i32> {
    let (map, instructions) = parse_input(input);
    let result = walk_map(map, instructions, wrap_part_1);

    Some(result)
}

fn walk_map<F>(map: Vec<Vec<Tile>>, instructions: Vec<Instruction>, wrap: F) -> i32
where
    F: Fn(&[Vec<Tile>], &Coord, &Direction) -> (Coord, Direction),
{
    let start_pos = map[0].iter().position(|t| *t == Tile::Open).unwrap() as i32;
    let mut pos = Coord {
        col: start_pos,
        row: 0,
    };
    let mut dir = Direction::R;

    let ins_total = instructions.len();
    println!("Total instructions: {ins_total}");
    for (ins_count, ins) in instructions.into_iter().enumerate() {
        // println!("Handling ins {ins_count}");
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
                            let (new_pos, new_dir) = wrap(&map, &pos, &dir);

                            // if new_pos is solid stop moving
                            if map[new_pos.row as usize][new_pos.col as usize] == Tile::Solid {
                                break;
                            }

                            dir = new_dir;
                            pos = new_pos;
                        }
                    }
                }
            }
        }
    }

    1000 * (pos.row + 1) + 4 * (pos.col + 1) + (dir.score() as i32)
}

pub fn part_two(input: &str) -> Option<i32> {
    part_two_impl(input, 50, real_face_map)
}

fn real_face_map(pos: &Coord, dir: &Direction) -> (Coord, Direction) {
    let Coord { row: r, col: c } = pos;
    match (r, c, dir) {
        (0, 1, Direction::L) => (Coord { row: 2, col: 0 }, Direction::R),
        (0, 1, Direction::U) => (Coord { row: 3, col: 0 }, Direction::R),
        (0, 2, Direction::R) => (Coord { row: 2, col: 1 }, Direction::L),
        (0, 2, Direction::U) => (Coord { row: 3, col: 0 }, Direction::U),
        (0, 2, Direction::D) => (Coord { row: 1, col: 1 }, Direction::L),
        (1, 1, Direction::R) => (Coord { row: 0, col: 2 }, Direction::U),
        (1, 1, Direction::L) => (Coord { row: 2, col: 0 }, Direction::D),
        (2, 0, Direction::L) => (Coord { row: 0, col: 1 }, Direction::R),
        (2, 0, Direction::U) => (Coord { row: 1, col: 1 }, Direction::R),
        (2, 1, Direction::R) => (Coord { row: 0, col: 2 }, Direction::L),
        (2, 1, Direction::D) => (Coord { row: 3, col: 0 }, Direction::L),
        (3, 0, Direction::L) => (Coord { row: 0, col: 1 }, Direction::D),
        (3, 0, Direction::R) => (Coord { row: 2, col: 1 }, Direction::U),
        (3, 0, Direction::D) => (Coord { row: 0, col: 2 }, Direction::D),
        _ => panic!("Need Map for {pos:?} with Direction {dir:?}"),
    }
}

fn part_two_impl<F>(input: &str, side_length: i32, face_map: F) -> Option<i32>
where
    F: Fn(&Coord, &Direction) -> (Coord, Direction),
{
    let (map, instructions) = parse_input(input);

    let result = walk_map(
        map,
        instructions,
        |_map: &[Vec<Tile>], pos: &Coord, dir: &Direction| {
            wrap_part_2(pos, dir, side_length, &face_map)
        },
    );

    Some(result)
}

fn wrap_part_2<F>(pos: &Coord, dir: &Direction, side_length: i32, face_map: F) -> (Coord, Direction)
where
    F: Fn(&Coord, &Direction) -> (Coord, Direction),
{
    println!("Starting wrap at {pos:?} {dir:?}");

    let current_face = pos.current_face(side_length);
    println!("Current face {current_face:?}");

    //figure out which face we switched to
    let (new_face, new_dir) = face_map(&current_face, dir);
    println!("New face {new_face:?} with direction {new_dir:?}");

    let offset = match dir {
        Direction::R | Direction::L => pos.row % side_length,
        Direction::U | Direction::D => pos.col % side_length,
    };

    let test_row = match (dir, new_dir) {
        (_, Direction::D) => new_face.row * side_length,
        (_, Direction::U) => (new_face.row + 1) * side_length - 1,
        (Direction::R, Direction::L)
        | (Direction::L, Direction::R)
        | (Direction::D, Direction::L)
        | (Direction::U, Direction::R) => (new_face.row * side_length) + offset,
        _ => panic!("Don't know how to calc row for {dir:?} -> {new_dir:?}"),
    };

    let test_col = match (dir, new_dir) {
        (_, Direction::R) => new_face.col * side_length,
        (_, Direction::L) => (new_face.col + 1) * side_length - 1,
        (Direction::L, Direction::D)
        | (Direction::U, Direction::U)
        | (Direction::D, Direction::D) => (new_face.col * side_length) + offset,
        (Direction::R, Direction::U) | (Direction::R, Direction::D) => {
            new_face.col * side_length + (side_length - offset - 1)
        }
        (Direction::D, Direction::U) => (new_face.col * side_length) + (side_length - offset - 1),
        _ => panic!("Don't know how to calc col for {dir:?} -> {new_dir:?}"),
    };

    // calculate new position
    // let new_pos: Coord = match (dir, new_dir) {
    //     (Direction::L, Direction::D) => {
    //         let offset = pos.row % side_length;
    //         Coord {
    //             row: new_face.row * side_length,
    //             col: (new_face.col * side_length) + offset,
    //         }
    //     }
    //     (Direction::L, Direction::R) => {
    //         let offset = pos.row % side_length;
    //         Coord {
    //             row: (new_face.row * side_length) + offset,
    //             col: new_face.col * side_length,
    //         }
    //     }
    //     (Direction::R, Direction::L) => {
    //         let offset = pos.row % side_length;
    //         Coord {
    //             row: (new_face.row * side_length) + offset,
    //             col: (new_face.col + 1) * side_length - 1,
    //         }
    //     }
    //     (Direction::R, Direction::U) => {
    //         let offset = pos.row % side_length;
    //         Coord {
    //             row: (new_face.row + 1) * side_length - 1,
    //             col: new_face.col * side_length + (side_length - offset - 1),
    //         }
    //     }
    //     (Direction::R, Direction::D) => {
    //         let offset = pos.row % side_length;
    //         Coord {
    //             row: new_face.row * side_length,
    //             col: new_face.col * side_length + (side_length - offset - 1),
    //         }
    //     }
    //     (Direction::U, Direction::R) => {
    //         let offset = pos.col % side_length;
    //         Coord {
    //             row: (new_face.row * side_length) + offset,
    //             col: new_face.col * side_length,
    //         }
    //     }
    //     (Direction::U, Direction::U) => {
    //         let offset = pos.col % side_length;
    //         Coord {
    //             row: (new_face.row + 1) * side_length - 1,
    //             col: (new_face.col * side_length) + offset,
    //         }
    //     }
    //     (Direction::D, Direction::L) => {
    //         let offset = pos.col % side_length;
    //         Coord {
    //             row: (new_face.row * side_length) + offset,
    //             col: (new_face.col + 1) * side_length - 1,
    //         }
    //     }
    //     (Direction::D, Direction::U) => {
    //         let offset = pos.col % side_length;
    //         Coord {
    //             row: (new_face.row + 1) * side_length - 1,
    //             col: (new_face.col * side_length) + (side_length - offset - 1),
    //         }
    //     }
    //     (Direction::D, Direction::D) => {
    //         let offset = pos.col % side_length;
    //         Coord {
    //             row: new_face.row * side_length,
    //             col: (new_face.col * side_length) + offset,
    //         }
    //     }
    //     _ => panic!("Don't know how to handle change from {dir:?} to {new_dir:?}"),
    // };

    let new_pos = Coord {
        row: test_row,
        col: test_col,
    };
    println!("Wrapped from {pos:?} {dir:?} to {new_pos:?} {new_dir:?}");

    (new_pos, new_dir)
}

fn wrap_part_1(map: &[Vec<Tile>], pos: &Coord, dir: &Direction) -> (Coord, Direction) {
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

    (curr, *dir)
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

#[derive(Debug, Clone, PartialEq)]
struct Coord {
    row: i32,
    col: i32,
}

impl Coord {
    fn current_face(&self, side_length: i32) -> Coord {
        Coord {
            row: self.row / side_length,
            col: self.col / side_length,
        }
    }
}

#[derive(Debug, Clone, Copy)]
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

    fn test_face_map(pos: &Coord, dir: &Direction) -> (Coord, Direction) {
        let Coord { row: r, col: c } = pos;
        match (r, c, dir) {
            (1, 2, Direction::R) => (Coord { row: 2, col: 3 }, Direction::D),
            (2, 2, Direction::D) => (Coord { row: 1, col: 0 }, Direction::U),
            (2, 3, Direction::D) => (Coord { row: 1, col: 0 }, Direction::R),
            (1, 1, Direction::U) => (Coord { row: 0, col: 2 }, Direction::R),
            _ => panic!("Need Map for {pos:?} with Direction {dir:?}"),
        }
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_two_impl(&input, 4, test_face_map), Some(5031));
    }
}
