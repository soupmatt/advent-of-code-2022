use std::{cell::RefCell, fmt::Display, ops::Add, rc::Rc};

pub fn part_one(input: &str) -> Option<usize> {
    let mut cave = Cave::build(input);
    cave.run_rounds(2022);
    Some(cave.highest_point())
}

pub fn part_two(_input: &str) -> Option<usize> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Air,
    Rock,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    col: usize,
    row: usize,
}

#[derive(Debug, Clone, PartialEq)]
struct RockShape {
    height: usize,
    width: usize,
    parts: Vec<Point>,
}

#[derive(Debug)]
struct FallingRock {
    rock_shape: RockShape,
    bottom_left: Point,
}

struct Cave {
    jets: Rc<RefCell<dyn Iterator<Item = Jet>>>,
    rock_shapes: Rc<RefCell<dyn Iterator<Item = RockShape>>>,
    columns: Rc<RefCell<[Vec<Tile>; 7]>>,
    highest_point: Rc<RefCell<usize>>,
    width: usize,
    fr: Rc<RefCell<Option<FallingRock>>>,
}

enum MoveDirection {
    Right,
    Left,
    Down,
}

impl Cave {
    fn build(input: &str) -> Cave {
        Self::new(parse_input(input))
    }

    fn new(jets: Vec<Jet>) -> Cave {
        let jets = Rc::new(RefCell::new(jets.into_iter().cycle()));
        let rock_shapes = Rc::new(RefCell::new(build_rocks().into_iter().cycle()));
        let columns = Rc::new(RefCell::new([
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ]));
        Cave {
            jets,
            rock_shapes,
            columns,
            highest_point: Rc::new(RefCell::new(0usize)),
            width: 7,
            fr: Rc::new(RefCell::new(None)),
        }
    }

    fn drop_next_rock(&mut self) {
        self.fr.replace(Some(FallingRock {
            rock_shape: self
                .rock_shapes
                .borrow_mut()
                .next()
                .as_ref()
                .unwrap()
                .clone(),
            bottom_left: Point {
                col: 2,
                row: self.highest_point.borrow().add(3),
            },
        }));
        self.increase_height(
            self.highest_point()
                + self.fr.borrow().as_ref().unwrap().rock_shape.height
                + self.fr.borrow().as_ref().unwrap().bottom_left.row,
        );

        loop {
            let next = self.jets.borrow_mut().next();
            match next {
                Some(Jet::Left) => self.handle_move(MoveDirection::Left),
                Some(Jet::Right) => self.handle_move(MoveDirection::Right),
                None => false,
            };

            if !self.handle_move(MoveDirection::Down) {
                break;
            }
        }

        self.handle_settled_rock();
    }

    fn handle_move(&mut self, move_direction: MoveDirection) -> bool {
        let new_coords: (usize, usize);
        let collision: bool;
        {
            let binding = self.fr.borrow();
            let fr = binding.as_ref().unwrap();

            new_coords = match move_direction {
                MoveDirection::Right => {
                    //check if we are up against the right wall
                    if fr.bottom_left.col + fr.rock_shape.width >= self.width {
                        return false;
                    }
                    (fr.bottom_left.col + 1, fr.bottom_left.row)
                }
                MoveDirection::Left => {
                    //check if we are up against the left wall
                    if fr.bottom_left.col == 0 {
                        return false;
                    }
                    (fr.bottom_left.col - 1, fr.bottom_left.row)
                }
                MoveDirection::Down => {
                    if fr.bottom_left.row == 0 {
                        return false;
                    }
                    (fr.bottom_left.col, fr.bottom_left.row - 1)
                }
            };
            collision = self.detect_collision(&new_coords);
        }

        let mut borrow_mut = self.fr.borrow_mut();
        if !collision {
            borrow_mut.as_mut().unwrap().bottom_left = Point {
                col: new_coords.0,
                row: new_coords.1,
            };
            true
        } else {
            false
        }
    }

    fn detect_collision(&self, (col, row): &(usize, usize)) -> bool {
        let binding = self.fr.as_ref().borrow();
        let rs = &binding.as_ref().unwrap().rock_shape;

        let columns = self.columns.borrow();
        rs.parts
            .iter()
            .any(|p| columns[p.col + col][p.row + row] == Tile::Rock)
    }

    fn handle_settled_rock(&mut self) {
        {
            let fr_ref = self.fr.borrow_mut();
            let fr = fr_ref.as_ref().unwrap();

            let new_highest_point = fr.bottom_left.row + fr.rock_shape.height;
            let current_highest_point = self.highest_point();
            if new_highest_point.cmp(&current_highest_point).is_gt() {
                self.highest_point.replace(new_highest_point);
            }
            self.increase_height(self.highest_point());

            let mut columns = self.columns.borrow_mut();
            fr.rock_shape.parts.iter().for_each(|p| {
                columns[p.col + fr.bottom_left.col][p.row + fr.bottom_left.row] = Tile::Rock;
            });
        }

        self.fr.replace(None);
    }

    fn increase_height(&self, new_max_height: usize) {
        if new_max_height.cmp(&self.column_height()).is_gt() {
            let mut columns = self.columns.borrow_mut();
            columns.iter_mut().for_each(|c| {
                while c.len() < new_max_height {
                    c.push(Tile::Air)
                }
            });
        }
    }

    fn run_rounds(&mut self, num: usize) {
        for _ in 0..num {
            self.drop_next_rock();
        }
    }

    fn highest_point(&self) -> usize {
        self.highest_point.borrow().to_owned()
    }

    fn column_height(&self) -> usize {
        self.columns.borrow()[0].len()
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fr_coords: Vec<Point> = match self.fr.borrow().as_ref() {
            None => vec![],
            Some(fr) => fr
                .rock_shape
                .parts
                .iter()
                .map(|p| Point {
                    col: p.col + fr.bottom_left.col,
                    row: p.row + fr.bottom_left.row,
                })
                .collect(),
        };
        let columns = self.columns.borrow();
        for row in (0..self.column_height()).rev() {
            write!(f, "|")?;
            for col in 0..self.width {
                if fr_coords.contains(&Point { col, row }) {
                    write!(f, "@")?
                } else {
                    match columns[col][row] {
                        Tile::Air => write!(f, ".")?,
                        Tile::Rock => write!(f, "#")?,
                    }
                }
            }
            writeln!(f, "|")?;
        }
        writeln!(f, "+-------+")
    }
}

fn parse_input(input: &str) -> Vec<Jet> {
    input
        .chars()
        .filter_map(|c| match c {
            '<' => Some(Jet::Left),
            '>' => Some(Jet::Right),
            _ => None,
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Jet {
    Right,
    Left,
}

fn build_rocks() -> Vec<RockShape> {
    let dash = RockShape {
        height: 1,
        width: 4,
        parts: vec![
            Point { col: 0, row: 0 },
            Point { col: 1, row: 0 },
            Point { col: 2, row: 0 },
            Point { col: 3, row: 0 },
        ],
    };
    let plus = RockShape {
        height: 3,
        width: 3,
        parts: vec![
            Point { col: 1, row: 0 },
            Point { col: 0, row: 1 },
            Point { col: 1, row: 1 },
            Point { col: 2, row: 1 },
            Point { col: 1, row: 2 },
        ],
    };
    let el = RockShape {
        height: 3,
        width: 3,
        parts: vec![
            Point { col: 2, row: 2 },
            Point { col: 2, row: 1 },
            Point { col: 0, row: 0 },
            Point { col: 1, row: 0 },
            Point { col: 2, row: 0 },
        ],
    };
    let stick = RockShape {
        height: 4,
        width: 1,
        parts: vec![
            Point { col: 0, row: 0 },
            Point { col: 0, row: 1 },
            Point { col: 0, row: 2 },
            Point { col: 0, row: 3 },
        ],
    };
    let block = RockShape {
        height: 2,
        width: 2,
        parts: vec![
            Point { col: 0, row: 0 },
            Point { col: 1, row: 0 },
            Point { col: 0, row: 1 },
            Point { col: 1, row: 1 },
        ],
    };

    vec![dash, plus, el, stick, block]
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use lazy_static::__Deref;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_one(&input), Some(3068));
    }

    #[test]
    fn test_new() {
        let jets = vec![Jet::Left, Jet::Right];
        let cave = Cave::new(jets.clone());

        //assert starting state
        assert_eq!(cave.highest_point(), 0);
        assert_eq!(cave.width, 7);
        assert_eq!(
            cave.columns.borrow().deref(),
            &[
                Vec::<Tile>::new(),
                Vec::<Tile>::new(),
                Vec::<Tile>::new(),
                Vec::<Tile>::new(),
                Vec::<Tile>::new(),
                Vec::<Tile>::new(),
                Vec::<Tile>::new(),
            ]
        );

        // jets iterator should cycle
        let mut actual_jets = cave.jets.borrow_mut();
        assert_eq!(actual_jets.next().unwrap(), jets[0]);
        assert_eq!(actual_jets.next().unwrap(), jets[1]);
        assert_eq!(actual_jets.next().unwrap(), jets[0]);
        assert_eq!(actual_jets.next().unwrap(), jets[1]);
        assert_eq!(actual_jets.next().unwrap(), jets[0]);
        assert_eq!(actual_jets.next().unwrap(), jets[1]);

        // rock_shapes iterator should cycle
        let rock_shapes = build_rocks();
        let mut actual_rock_shapes = cave.rock_shapes.borrow_mut();
        assert_eq!(actual_rock_shapes.next().unwrap(), rock_shapes[0]);
        assert_eq!(actual_rock_shapes.next().unwrap(), rock_shapes[1]);
        assert_eq!(actual_rock_shapes.next().unwrap(), rock_shapes[2]);
        assert_eq!(actual_rock_shapes.next().unwrap(), rock_shapes[3]);
        assert_eq!(actual_rock_shapes.next().unwrap(), rock_shapes[4]);
        assert_eq!(actual_rock_shapes.next().unwrap(), rock_shapes[0]);
        assert_eq!(actual_rock_shapes.next().unwrap(), rock_shapes[1]);
        assert_eq!(actual_rock_shapes.next().unwrap(), rock_shapes[2]);
        assert_eq!(actual_rock_shapes.next().unwrap(), rock_shapes[3]);
        assert_eq!(actual_rock_shapes.next().unwrap(), rock_shapes[4]);
    }

    #[test]
    fn test_display() {
        let cave = Cave::new(vec![]);

        assert_eq!(
            format!("{}", cave),
            indoc! {"
                +-------+
            "}
        );

        cave.increase_height(4);
        assert_eq!(
            format!("{}", cave),
            indoc! {"
                |.......|
                |.......|
                |.......|
                |.......|
                +-------+
            "}
        );

        cave.columns.borrow_mut()[0][0] = Tile::Rock;
        cave.columns.borrow_mut()[1][0] = Tile::Rock;
        cave.columns.borrow_mut()[2][0] = Tile::Rock;
        cave.columns.borrow_mut()[3][0] = Tile::Rock;
        assert_eq!(
            format!("{}", cave),
            indoc! {"
                |.......|
                |.......|
                |.......|
                |####...|
                +-------+
            "}
        );

        cave.columns.borrow_mut()[3][1] = Tile::Rock;
        cave.columns.borrow_mut()[4][1] = Tile::Rock;
        cave.columns.borrow_mut()[3][2] = Tile::Rock;
        cave.columns.borrow_mut()[4][2] = Tile::Rock;
        assert_eq!(
            format!("{}", cave),
            indoc! {"
                |.......|
                |...##..|
                |...##..|
                |####...|
                +-------+
            "}
        );

        let rock_shapes = build_rocks();
        cave.increase_height(8);

        cave.fr.replace(Some(FallingRock {
            bottom_left: Point { col: 2, row: 4 },
            rock_shape: rock_shapes[0].clone(),
        }));
        assert_eq!(
            format!("{}", cave),
            indoc! {"
                |.......|
                |.......|
                |.......|
                |..@@@@.|
                |.......|
                |...##..|
                |...##..|
                |####...|
                +-------+
            "}
        );

        cave.fr.replace(Some(FallingRock {
            bottom_left: Point { col: 2, row: 4 },
            rock_shape: rock_shapes[1].clone(),
        }));
        assert_eq!(
            format!("{}", cave),
            indoc! {"
                |.......|
                |...@...|
                |..@@@..|
                |...@...|
                |.......|
                |...##..|
                |...##..|
                |####...|
                +-------+
            "}
        );

        cave.fr.replace(Some(FallingRock {
            bottom_left: Point { col: 2, row: 4 },
            rock_shape: rock_shapes[2].clone(),
        }));
        assert_eq!(
            format!("{}", cave),
            indoc! {"
                |.......|
                |....@..|
                |....@..|
                |..@@@..|
                |.......|
                |...##..|
                |...##..|
                |####...|
                +-------+
            "}
        );

        cave.fr.replace(Some(FallingRock {
            bottom_left: Point { col: 2, row: 4 },
            rock_shape: rock_shapes[3].clone(),
        }));
        assert_eq!(
            format!("{}", cave),
            indoc! {"
                |..@....|
                |..@....|
                |..@....|
                |..@....|
                |.......|
                |...##..|
                |...##..|
                |####...|
                +-------+
            "}
        );

        cave.fr.replace(Some(FallingRock {
            bottom_left: Point { col: 2, row: 4 },
            rock_shape: rock_shapes[4].clone(),
        }));
        assert_eq!(
            format!("{}", cave),
            indoc! {"
                |.......|
                |.......|
                |..@@...|
                |..@@...|
                |.......|
                |...##..|
                |...##..|
                |####...|
                +-------+
            "}
        );
    }

    #[test]
    fn test_handle_settled_rock() {
        let mut cave = Cave::new(vec![]);

        assert_eq!(cave.highest_point(), 0);
        assert_eq!(
            format!("{}", cave),
            indoc! {"
                +-------+
            "}
        );

        let rock_shapes = build_rocks();
        let dash = &rock_shapes[0];
        let plus = &rock_shapes[1];
        let el = &rock_shapes[2];
        let stick = &rock_shapes[3];
        let block = &rock_shapes[4];

        cave.fr.replace(Some(FallingRock {
            rock_shape: dash.clone(),
            bottom_left: Point { col: 1, row: 0 },
        }));
        cave.handle_settled_rock();
        assert!(cave.fr.borrow().as_ref().is_none());

        assert_eq!(cave.highest_point(), 1);
        assert_eq!(
            format!("{}", cave),
            indoc! {"
                |.####..|
                +-------+
            "}
        );

        cave.fr.replace(Some(FallingRock {
            rock_shape: stick.clone(),
            bottom_left: Point { col: 3, row: 1 },
        }));
        cave.handle_settled_rock();

        assert_eq!(cave.highest_point(), 5);
        assert_eq!(
            format!("{}", cave),
            indoc! {"
                |...#...|
                |...#...|
                |...#...|
                |...#...|
                |.####..|
                +-------+
            "}
        );

        cave.fr.replace(Some(FallingRock {
            rock_shape: plus.clone(),
            bottom_left: Point { col: 1, row: 4 },
        }));
        cave.handle_settled_rock();

        assert_eq!(cave.highest_point(), 7);
        assert_eq!(
            format!("{}", cave),
            indoc! {"
                |..#....|
                |.###...|
                |..##...|
                |...#...|
                |...#...|
                |...#...|
                |.####..|
                +-------+
            "}
        );

        cave.fr.replace(Some(FallingRock {
            rock_shape: el.clone(),
            bottom_left: Point { col: 4, row: 1 },
        }));
        cave.handle_settled_rock();

        assert_eq!(cave.highest_point(), 7);
        assert_eq!(
            format!("{}", cave),
            indoc! {"
                |..#....|
                |.###...|
                |..##...|
                |...#..#|
                |...#..#|
                |...####|
                |.####..|
                +-------+
            "}
        );

        cave.fr.replace(Some(FallingRock {
            rock_shape: block.clone(),
            bottom_left: Point { col: 5, row: 4 },
        }));
        cave.handle_settled_rock();

        assert_eq!(cave.highest_point(), 7);
        assert_eq!(
            format!("{}", cave),
            indoc! {"
                |..#....|
                |.###.##|
                |..##.##|
                |...#..#|
                |...#..#|
                |...####|
                |.####..|
                +-------+
            "}
        );
    }

    #[test]
    fn test_handle_move() {
        let mut cave = Cave::new(vec![]);
        let rock_shapes = build_rocks();
        let block = &rock_shapes[4];

        cave.increase_height(3);
        cave.fr.replace(Some(FallingRock {
            rock_shape: block.clone(),
            bottom_left: Point { col: 1, row: 1 },
        }));
        assert_eq!(
            format!("{}", cave),
            indoc! {"
                |.@@....|
                |.@@....|
                |.......|
                +-------+
            "}
        );

        assert!(cave.handle_move(MoveDirection::Left));
        assert_eq!(
            format!("{}", cave),
            indoc! {"
                |@@.....|
                |@@.....|
                |.......|
                +-------+
            "}
        );

        assert!(!cave.handle_move(MoveDirection::Left));
        assert_eq!(
            format!("{}", cave),
            indoc! {"
                |@@.....|
                |@@.....|
                |.......|
                +-------+
            "}
        );

        cave.fr.replace(Some(FallingRock {
            rock_shape: block.clone(),
            bottom_left: Point { col: 4, row: 1 },
        }));
        assert_eq!(
            format!("{}", cave),
            indoc! {"
                |....@@.|
                |....@@.|
                |.......|
                +-------+
            "}
        );

        assert!(cave.handle_move(MoveDirection::Right));
        assert_eq!(
            format!("{}", cave),
            indoc! {"
                |.....@@|
                |.....@@|
                |.......|
                +-------+
            "}
        );

        assert!(!cave.handle_move(MoveDirection::Right));
        assert_eq!(
            format!("{}", cave),
            indoc! {"
                |.....@@|
                |.....@@|
                |.......|
                +-------+
            "}
        );

        cave.fr.replace(Some(FallingRock {
            rock_shape: block.clone(),
            bottom_left: Point { col: 2, row: 1 },
        }));
        assert_eq!(
            format!("{}", cave),
            indoc! {"
                |..@@...|
                |..@@...|
                |.......|
                +-------+
            "}
        );

        assert!(cave.handle_move(MoveDirection::Down));
        assert_eq!(
            format!("{}", cave),
            indoc! {"
                |.......|
                |..@@...|
                |..@@...|
                +-------+
            "}
        );

        assert!(!cave.handle_move(MoveDirection::Down));
        assert_eq!(
            format!("{}", cave),
            indoc! {"
                |.......|
                |..@@...|
                |..@@...|
                +-------+
            "}
        );
    }

    #[test]
    fn test_handle_move_with_collisions() {
        let mut cave = Cave::new(vec![]);
        let rock_shapes = build_rocks();
        let block = &rock_shapes[4];

        cave.increase_height(3);
        cave.fr.replace(Some(FallingRock {
            rock_shape: block.clone(),
            bottom_left: Point { col: 2, row: 1 },
        }));
        cave.columns.borrow_mut()[0][1] = Tile::Rock;
        cave.columns.borrow_mut()[2][0] = Tile::Rock;
        assert_eq!(
            format!("{}", cave),
            indoc! {"
                |..@@...|
                |#.@@...|
                |..#....|
                +-------+
            "}
        );

        assert!(cave.handle_move(MoveDirection::Left));
        assert_eq!(
            format!("{}", cave),
            indoc! {"
                |.@@....|
                |#@@....|
                |..#....|
                +-------+
            "}
        );

        assert!(!cave.handle_move(MoveDirection::Left));
        assert_eq!(
            format!("{}", cave),
            indoc! {"
                |.@@....|
                |#@@....|
                |..#....|
                +-------+
            "}
        );

        cave.fr.replace(Some(FallingRock {
            rock_shape: block.clone(),
            bottom_left: Point { col: 3, row: 1 },
        }));
        cave.columns.borrow_mut()[6][2] = Tile::Rock;
        assert_eq!(
            format!("{}", cave),
            indoc! {"
                |...@@.#|
                |#..@@..|
                |..#....|
                +-------+
            "}
        );

        assert!(cave.handle_move(MoveDirection::Right));
        assert_eq!(
            format!("{}", cave),
            indoc! {"
                |....@@#|
                |#...@@.|
                |..#....|
                +-------+
            "}
        );

        assert!(!cave.handle_move(MoveDirection::Right));
        assert_eq!(
            format!("{}", cave),
            indoc! {"
                |....@@#|
                |#...@@.|
                |..#....|
                +-------+
            "}
        );

        cave.fr.replace(Some(FallingRock {
            rock_shape: block.clone(),
            bottom_left: Point { col: 2, row: 2 },
        }));
        cave.increase_height(4);
        assert_eq!(
            format!("{}", cave),
            indoc! {"
                |..@@...|
                |..@@..#|
                |#......|
                |..#....|
                +-------+
            "}
        );

        assert!(cave.handle_move(MoveDirection::Down));
        assert_eq!(
            format!("{}", cave),
            indoc! {"
                |.......|
                |..@@..#|
                |#.@@...|
                |..#....|
                +-------+
            "}
        );

        assert!(!cave.handle_move(MoveDirection::Down));
        assert_eq!(
            format!("{}", cave),
            indoc! {"
                |.......|
                |..@@..#|
                |#.@@...|
                |..#....|
                +-------+
            "}
        );
    }

    #[test]
    fn test_drop_next_rock() {
        let mut cave = Cave::new(vec![]);

        assert_eq!(
            format!("{}", cave),
            indoc! {"
                +-------+
            "}
        );
        assert_eq!(cave.highest_point(), 0);

        cave.drop_next_rock();
        assert_eq!(
            format!("{}", cave),
            indoc! {"
                |.......|
                |.......|
                |.......|
                |..####.|
                +-------+
            "}
        );
        assert_eq!(cave.highest_point(), 1);

        cave.drop_next_rock();
        assert_eq!(cave.highest_point(), 4);
        assert_eq!(
            format!("{}", cave),
            indoc! {"
                |.......|
                |.......|
                |.......|
                |.......|
                |...#...|
                |..###..|
                |...#...|
                |..####.|
                +-------+
            "}
        );
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(&input), None);
    }
}
