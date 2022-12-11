use std::{cmp::Ordering, collections::HashSet, iter::repeat};

pub fn part_one(input: &str) -> Option<usize> {
    let directions = parse_input(input);
    let mut rope = Rope::new(2);
    for dir in directions {
        rope.make_step(dir)
    }
    let set: HashSet<Point> = HashSet::from_iter(rope.history.into_iter());
    Some(set.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let directions = parse_input(input);
    let mut rope = Rope::new(10);
    for dir in directions {
        rope.make_step(dir)
    }
    let set: HashSet<Point> = HashSet::from_iter(rope.history.into_iter());
    Some(set.len())
}

fn parse_input(input: &str) -> Vec<Direction> {
    input
        .lines()
        .map(|line| {
            if let Some((direction, count)) = line.split_once(' ') {
                let count: usize = count.parse().unwrap();
                let direction: Direction = match direction {
                    "R" => Direction::Right,
                    "L" => Direction::Left,
                    "U" => Direction::Up,
                    "D" => Direction::Down,
                    _ => panic!(
                        "something when wrong! don't understand direction {}",
                        direction
                    ),
                };
                repeat(direction).take(count)
            } else {
                panic!("something went wrong! line wouldn't split \"{}\"", line)
            }
        })
        .fold(vec![], |mut acc, i| {
            acc.extend(i);
            acc
        })
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
struct Rope {
    segments: Vec<Point>,
    history: Vec<Point>,
}

impl Rope {
    fn new(length: u8) -> Rope {
        let mut r = Rope {
            segments: vec![],
            history: vec![Point { x: 0, y: 0 }],
        };
        for _ in 0..length {
            r.segments.push(Point { x: 0, y: 0 })
        }
        r
    }

    fn head(&mut self) -> &mut Point {
        self.segments.first_mut().unwrap()
    }

    fn make_step(&mut self, dir: Direction) {
        match dir {
            Direction::Up => self.step_up(),
            Direction::Down => self.step_down(),
            Direction::Left => self.step_left(),
            Direction::Right => self.step_right(),
        }
        self.adjust_tail()
    }

    fn step_up(&mut self) {
        self.head().y += 1
    }
    fn step_down(&mut self) {
        self.head().y -= 1
    }
    fn step_left(&mut self) {
        self.head().x -= 1
    }
    fn step_right(&mut self) {
        self.head().x += 1
    }

    fn adjust_tail(&mut self) {
        let mut itr = self.segments.iter_mut();
        let mut h = itr.next().unwrap();
        let mut t = itr.next().unwrap();
        loop {
            let moved = Rope::adjust_segment(h, t);

            match itr.next() {
                Some(i) => {
                    h = t;
                    t = i;
                }
                None => {
                    if moved {
                        self.history.push(t.to_owned());
                    }
                    break;
                }
            }
        }
    }

    fn adjust_segment(head: &Point, tail: &mut Point) -> bool {
        let y_offset = head.y - tail.y;
        let x_offset = head.x - tail.x;
        if (-1..=1).contains(&x_offset) && (-1..=1).contains(&y_offset) {
            return false;
        }
        match y_offset.cmp(&0isize) {
            Ordering::Greater => tail.y += 1,
            Ordering::Less => tail.y -= 1,
            Ordering::Equal => (),
        }
        match x_offset.cmp(&0isize) {
            Ordering::Greater => tail.x += 1,
            Ordering::Less => tail.x -= 1,
            Ordering::Equal => (),
        }
        true
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_parse_input() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(
            parse_input(&input),
            vec![
                Direction::Right,
                Direction::Right,
                Direction::Right,
                Direction::Right,
                Direction::Up,
                Direction::Up,
                Direction::Up,
                Direction::Up,
                Direction::Left,
                Direction::Left,
                Direction::Left,
                Direction::Down,
                Direction::Right,
                Direction::Right,
                Direction::Right,
                Direction::Right,
                Direction::Down,
                Direction::Left,
                Direction::Left,
                Direction::Left,
                Direction::Left,
                Direction::Left,
                Direction::Right,
                Direction::Right,
            ]
        )
    }

    #[test]
    fn test_rope_new() {
        let rope = Rope::new(2);
        assert_eq!(
            rope,
            Rope {
                segments: vec![Point { x: 0, y: 0 }, Point { x: 0, y: 0 }],
                history: vec![Point { x: 0, y: 0 }],
            }
        )
    }

    #[test]
    fn test_rope_adjust_tail() {
        // from new
        let mut rope = Rope::new(2);
        rope.adjust_tail();
        assert_eq!(
            rope,
            Rope {
                segments: vec![Point { x: 0, y: 0 }, Point { x: 0, y: 0 }],
                history: vec![Point { x: 0, y: 0 }],
            }
        );

        // head on top of tail
        tst_adjust_tail(0, 0, 0, 0, vec![]);

        // head +1, 0
        tst_adjust_tail(1, 0, 0, 0, vec![]);

        // head +1, +1
        tst_adjust_tail(1, 1, 0, 0, vec![]);

        // head 0, +1
        tst_adjust_tail(0, 1, 0, 0, vec![]);

        // head -1, -1
        tst_adjust_tail(-1, -1, 0, 0, vec![]);

        // head 0, -1
        tst_adjust_tail(0, -1, 0, 0, vec![]);

        // head -1, 0
        tst_adjust_tail(-1, 0, 0, 0, vec![]);

        // head +1, -1
        tst_adjust_tail(1, -1, 0, 0, vec![]);

        // head -1, +1
        tst_adjust_tail(-1, 1, 0, 0, vec![]);

        // head 0, 2
        tst_adjust_tail(0, 2, 0, 1, vec![Point { x: 3, y: 4 }]);

        // head 0, -2
        tst_adjust_tail(0, -2, 0, -1, vec![Point { x: 3, y: 2 }]);

        // head 2, 0
        tst_adjust_tail(2, 0, 1, 0, vec![Point { x: 4, y: 3 }]);

        // head -2, 0
        tst_adjust_tail(-2, 0, -1, 0, vec![Point { x: 2, y: 3 }]);

        // head 1, 2
        tst_adjust_tail(1, 2, 1, 1, vec![Point { x: 4, y: 4 }]);

        // head 1, -2
        tst_adjust_tail(1, -2, 1, -1, vec![Point { x: 4, y: 2 }]);

        // head 2, 1
        tst_adjust_tail(2, 1, 1, 1, vec![Point { x: 4, y: 4 }]);

        // head -2, 1
        tst_adjust_tail(-2, 1, -1, 1, vec![Point { x: 2, y: 4 }]);

        // head -1, 2
        tst_adjust_tail(-1, 2, -1, 1, vec![Point { x: 2, y: 4 }]);

        // head -1, -2
        tst_adjust_tail(-1, -2, -1, -1, vec![Point { x: 2, y: 2 }]);

        // head 2, -1
        tst_adjust_tail(2, -1, 1, -1, vec![Point { x: 4, y: 2 }]);

        // head -2, -1
        tst_adjust_tail(-2, -1, -1, -1, vec![Point { x: 2, y: 2 }]);
    }

    fn tst_adjust_tail(
        head_x_offset: isize,
        head_y_offset: isize,
        tail_x_move: isize,
        tail_y_move: isize,
        a_hist: Vec<Point>,
    ) {
        let mut rope = Rope {
            segments: vec![
                Point {
                    x: 3 + head_x_offset,
                    y: 3 + head_y_offset,
                },
                Point { x: 3, y: 3 },
            ],
            history: vec![],
        };
        rope.adjust_tail();
        assert_eq!(
            rope,
            Rope {
                segments: vec![
                    Point {
                        x: 3 + head_x_offset,
                        y: 3 + head_y_offset,
                    },
                    Point {
                        x: 3 + tail_x_move,
                        y: 3 + tail_y_move,
                    }
                ],
                history: a_hist,
            }
        );
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));

        let input = indoc! {"
            R 5
            U 8
            L 8
            D 3
            R 17
            D 10
            L 25
            U 20
        "};
        assert_eq!(part_two(input), Some(36));
    }
}
