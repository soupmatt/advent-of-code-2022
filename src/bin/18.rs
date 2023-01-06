use std::{collections::HashSet, fmt::Display};

use itertools::Itertools;
use pathfinding::{directed::dijkstra::dijkstra_all, prelude::build_path};
use strum_macros::EnumIter;

// #[cfg(test)]
use strum::IntoEnumIterator;

pub fn part_one(input: &str) -> Option<usize> {
    count_open_sides(input)
}

pub fn part_two(input: &str) -> Option<usize> {
    count_exterior_faces(input)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

fn count_open_sides(input: &str) -> Option<usize> {
    let faces = get_open_faces(input);

    Some(faces.len())
}

fn get_open_faces(input: &str) -> HashSet<Face> {
    let cubes = get_cube_set(input);
    let mut faces = initialize_face_set(&cubes);
    faces.retain(|face| match face.adjacent_cube() {
        None => true,
        Some(cube) => !cubes.contains(&cube),
    });
    faces
}

fn get_cube_set(input: &str) -> HashSet<Cube> {
    let cubes = parse_input(input);
    let mut set: HashSet<Cube> = HashSet::new();
    set.extend(cubes.into_iter());
    set
}

fn initialize_face_set(cubes: &HashSet<Cube>) -> HashSet<Face> {
    let mut faces = HashSet::new();
    let dirs = [
        Dir::XPlus,
        Dir::XMinus,
        Dir::YPlus,
        Dir::YMinus,
        Dir::ZPlus,
        Dir::ZMinus,
    ];
    cubes
        .iter()
        .cloned()
        .cartesian_product(dirs.iter().cloned())
        .for_each(|(cube, dir)| {
            faces.insert(Face { cube, dir });
        });
    faces
}

fn count_exterior_faces(input: &str) -> Option<usize> {
    let faces = get_open_faces(input);

    let start = faces.iter().sorted().next().unwrap();
    println!("{:?}", start);

    let search = dijkstra_all(start, |face| {
        faces
            .iter()
            .cloned()
            .filter_map(|other| {
                if other.is_adjacent(face) {
                    Some((other, 1))
                } else {
                    None
                }
            })
            .collect_vec()
    });
    println!("{:?}", search);
    let bad_cube = Cube { x: 2, y: 2, z: 5 };
    Dir::iter()
        .map(|d| {
            let f = Face {
                cube: bad_cube.clone(),
                dir: d,
            };
            Face {
                cube: f.adjacent_cube().unwrap(),
                dir: d.opposite(),
            }
        })
        .for_each(|f| {
            let path = build_path(&f, &search);
            println!();
            println!("Path for {:?}", f);
            println!("{:?}", path);
        });
    Some(search.len() + 1)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
struct Cube {
    x: u8,
    y: u8,
    z: u8,
}

impl Cube {
    fn abs_diff(&self, other: &Cube) -> (u8, u8, u8) {
        let x_diff = self.x.abs_diff(other.x);
        let y_diff = self.y.abs_diff(other.y);
        let z_diff = self.z.abs_diff(other.z);
        (x_diff, y_diff, z_diff)
    }
}

impl Display for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Dim {
    X,
    Y,
    Z,
}

enum Sign {
    Plus,
    Minus,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord, EnumIter)]
enum Dir {
    XMinus,
    YMinus,
    ZMinus,
    XPlus,
    YPlus,
    ZPlus,
}

impl Dir {
    fn opposite(&self) -> Dir {
        match self {
            Self::XMinus => Self::XPlus,
            Self::XPlus => Self::XMinus,
            Self::YMinus => Self::YPlus,
            Self::YPlus => Self::YMinus,
            Self::ZMinus => Self::ZPlus,
            Self::ZPlus => Self::ZMinus,
        }
    }

    fn breakdown(&self) -> (Dim, Sign) {
        match self {
            Self::XMinus => (Dim::X, Sign::Minus),
            Self::YMinus => (Dim::Y, Sign::Minus),
            Self::ZMinus => (Dim::Z, Sign::Minus),
            Self::XPlus => (Dim::X, Sign::Plus),
            Self::YPlus => (Dim::Y, Sign::Plus),
            Self::ZPlus => (Dim::Z, Sign::Plus),
        }
    }

    #[cfg(test)]
    fn adjacent(&self) -> Vec<Dir> {
        Self::iter()
            .filter(|d| d != self && d != &self.opposite())
            .take(4)
            .collect_vec()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
struct Face {
    cube: Cube,
    dir: Dir,
}

impl Face {
    fn adjacent_cube(&self) -> Option<Cube> {
        let mut x = self.cube.x;
        let mut y = self.cube.y;
        let mut z = self.cube.z;

        match self.dir {
            Dir::XPlus => {
                x += 1;
            }
            Dir::XMinus => {
                if x == 0 {
                    return None;
                }
                x -= 1;
            }
            Dir::YPlus => {
                y += 1;
            }
            Dir::YMinus => {
                if y == 0 {
                    return None;
                }
                y -= 1;
            }
            Dir::ZPlus => {
                z += 1;
            }
            Dir::ZMinus => {
                if z == 0 {
                    return None;
                }
                z -= 1;
            }
        }

        Some(Cube { x, y, z })
    }

    fn is_adjacent(&self, other: &Face) -> bool {
        if self.dir == other.dir.opposite() {
            return false;
        }
        if self.cube == other.cube {
            self.dir != other.dir && self.dir != other.dir.opposite()
        } else {
            let abs_diff = self.cube.abs_diff(&other.cube);
            if abs_diff.0 > 1 || abs_diff.1 > 1 || abs_diff.2 > 1 {
                return false;
            }
            if self.dir == other.dir {
                match abs_diff {
                    (1, 0, 0) => self.dir != Dir::XMinus && self.dir != Dir::XPlus,
                    (0, 1, 0) => self.dir != Dir::YMinus && self.dir != Dir::YPlus,
                    (0, 0, 1) => self.dir != Dir::ZMinus && self.dir != Dir::ZPlus,
                    _ => false,
                }
            } else {
                let (self_dim, self_sign) = self.dir.breakdown();
                let (other_dim, other_sign) = other.dir.breakdown();
                match (self_dim, other_dim) {
                    (Dim::X, Dim::Y) => {
                        self.cube.z == other.cube.z
                            && match self_sign {
                                Sign::Plus => self.cube.x < other.cube.x,
                                Sign::Minus => self.cube.x > other.cube.x,
                            }
                            && match other_sign {
                                Sign::Plus => self.cube.y > other.cube.y,
                                Sign::Minus => self.cube.y < other.cube.y,
                            }
                    }
                    (Dim::X, Dim::Z) => {
                        self.cube.y == other.cube.y
                            && match self_sign {
                                Sign::Plus => self.cube.x < other.cube.x,
                                Sign::Minus => self.cube.x > other.cube.x,
                            }
                            && match other_sign {
                                Sign::Plus => self.cube.z > other.cube.z,
                                Sign::Minus => self.cube.z < other.cube.z,
                            }
                    }
                    (Dim::Y, Dim::X) => {
                        self.cube.z == other.cube.z
                            && match self_sign {
                                Sign::Plus => self.cube.y < other.cube.y,
                                Sign::Minus => self.cube.y > other.cube.y,
                            }
                            && match other_sign {
                                Sign::Plus => self.cube.x > other.cube.x,
                                Sign::Minus => self.cube.x < other.cube.x,
                            }
                    }
                    (Dim::Y, Dim::Z) => {
                        self.cube.x == other.cube.x
                            && match self_sign {
                                Sign::Plus => self.cube.y < other.cube.y,
                                Sign::Minus => self.cube.y > other.cube.y,
                            }
                            && match other_sign {
                                Sign::Plus => self.cube.z > other.cube.z,
                                Sign::Minus => self.cube.z < other.cube.z,
                            }
                    }
                    (Dim::Z, Dim::X) => {
                        self.cube.y == other.cube.y
                            && match self_sign {
                                Sign::Plus => self.cube.z < other.cube.z,
                                Sign::Minus => self.cube.z > other.cube.z,
                            }
                            && match other_sign {
                                Sign::Plus => self.cube.x > other.cube.x,
                                Sign::Minus => self.cube.x < other.cube.x,
                            }
                    }
                    (Dim::Z, Dim::Y) => {
                        self.cube.x == other.cube.x
                            && match self_sign {
                                Sign::Plus => self.cube.z < other.cube.z,
                                Sign::Minus => self.cube.z > other.cube.z,
                            }
                            && match other_sign {
                                Sign::Plus => self.cube.y > other.cube.y,
                                Sign::Minus => self.cube.y < other.cube.y,
                            }
                    }
                    _ => false,
                }
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<Cube> {
    input
        .lines()
        .map(|line| {
            let (x, y, z) = line
                .splitn(3, ',')
                .map(|s| s.parse::<u8>().unwrap())
                .collect_tuple()
                .unwrap();
            Cube { x, y, z }
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(64));
    }

    #[test]
    fn test_face_is_adjacent() {
        let mut face = Face {
            dir: Dir::XPlus,
            cube: Cube { x: 1, y: 1, z: 1 },
        };
        let mut same_cube = face.clone();

        Dir::iter().for_each(|dir| {
            face.dir = dir;
            adjacent_for_dirs(&face, &mut same_cube, dir.adjacent());
        });

        let mut same_dir = face.clone();

        Dir::iter().for_each(|dir| {
            face.dir = dir;
            same_dir.dir = dir;

            let passing = if face.dir.breakdown().0 == Dim::X {
                vec![]
            } else {
                vec![dir]
            };
            same_dir.cube.x = 0;
            same_dir.cube.y = 1;
            same_dir.cube.z = 1;
            adjacent_for_dirs(&face, &mut same_dir, passing.clone());

            same_dir.cube.x = 2;
            same_dir.cube.y = 1;
            same_dir.cube.z = 1;
            adjacent_for_dirs(&face, &mut same_dir, passing);

            let passing = if face.dir.breakdown().0 == Dim::Y {
                vec![]
            } else {
                vec![dir]
            };
            same_dir.cube.x = 1;
            same_dir.cube.y = 0;
            same_dir.cube.z = 1;
            adjacent_for_dirs(&face, &mut same_dir, passing.clone());

            same_dir.cube.x = 1;
            same_dir.cube.y = 2;
            same_dir.cube.z = 1;
            adjacent_for_dirs(&face, &mut same_dir, passing);

            let passing = if face.dir.breakdown().0 == Dim::Z {
                vec![]
            } else {
                vec![dir]
            };
            same_dir.cube.x = 1;
            same_dir.cube.y = 1;
            same_dir.cube.z = 0;
            adjacent_for_dirs(&face, &mut same_dir, passing.clone());

            same_dir.cube.x = 1;
            same_dir.cube.y = 1;
            same_dir.cube.z = 2;
            adjacent_for_dirs(&face, &mut same_dir, passing);
        });

        // check ones with 2 dims different
        Dir::iter().for_each(|d| {
            face.dir = d;
            let mut adjacent_face = face.clone();

            adjacent_face.cube.x = 2;
            adjacent_face.cube.y = 2;
            adjacent_face.cube.z = 1;
            if d == Dir::XPlus {
                adjacent_for_dirs(&face, &mut adjacent_face, vec![Dir::YMinus]);
            } else if d == Dir::YPlus {
                adjacent_for_dirs(&face, &mut adjacent_face, vec![Dir::XMinus]);
            } else {
                adjacent_for_dirs(&face, &mut adjacent_face, vec![]);
            }

            adjacent_face.cube.x = 2;
            adjacent_face.cube.y = 0;
            adjacent_face.cube.z = 1;
            if d == Dir::XPlus {
                adjacent_for_dirs(&face, &mut adjacent_face, vec![Dir::YPlus]);
            } else if d == Dir::YMinus {
                adjacent_for_dirs(&face, &mut adjacent_face, vec![Dir::XMinus]);
            } else {
                adjacent_for_dirs(&face, &mut adjacent_face, vec![]);
            }

            adjacent_face.cube.x = 2;
            adjacent_face.cube.y = 1;
            adjacent_face.cube.z = 2;
            if d == Dir::XPlus {
                adjacent_for_dirs(&face, &mut adjacent_face, vec![Dir::ZMinus]);
            } else if d == Dir::ZPlus {
                adjacent_for_dirs(&face, &mut adjacent_face, vec![Dir::XMinus]);
            } else {
                adjacent_for_dirs(&face, &mut adjacent_face, vec![]);
            }

            adjacent_face.cube.x = 2;
            adjacent_face.cube.y = 1;
            adjacent_face.cube.z = 0;
            if d == Dir::XPlus {
                adjacent_for_dirs(&face, &mut adjacent_face, vec![Dir::ZPlus]);
            } else if d == Dir::ZMinus {
                adjacent_for_dirs(&face, &mut adjacent_face, vec![Dir::XMinus]);
            } else {
                adjacent_for_dirs(&face, &mut adjacent_face, vec![]);
            }

            adjacent_face.cube.x = 0;
            adjacent_face.cube.y = 2;
            adjacent_face.cube.z = 1;
            if d == Dir::XMinus {
                adjacent_for_dirs(&face, &mut adjacent_face, vec![Dir::YMinus]);
            } else if d == Dir::YPlus {
                adjacent_for_dirs(&face, &mut adjacent_face, vec![Dir::XPlus]);
            } else {
                adjacent_for_dirs(&face, &mut adjacent_face, vec![]);
            }

            adjacent_face.cube.x = 0;
            adjacent_face.cube.y = 0;
            adjacent_face.cube.z = 1;
            if d == Dir::XMinus {
                adjacent_for_dirs(&face, &mut adjacent_face, vec![Dir::YPlus]);
            } else if d == Dir::YMinus {
                adjacent_for_dirs(&face, &mut adjacent_face, vec![Dir::XPlus]);
            } else {
                adjacent_for_dirs(&face, &mut adjacent_face, vec![]);
            }

            adjacent_face.cube.x = 0;
            adjacent_face.cube.y = 1;
            adjacent_face.cube.z = 2;
            if d == Dir::XMinus {
                adjacent_for_dirs(&face, &mut adjacent_face, vec![Dir::ZMinus]);
            } else if d == Dir::ZPlus {
                adjacent_for_dirs(&face, &mut adjacent_face, vec![Dir::XPlus]);
            } else {
                adjacent_for_dirs(&face, &mut adjacent_face, vec![]);
            }

            adjacent_face.cube.x = 0;
            adjacent_face.cube.y = 1;
            adjacent_face.cube.z = 0;
            if d == Dir::XMinus {
                adjacent_for_dirs(&face, &mut adjacent_face, vec![Dir::ZPlus]);
            } else if d == Dir::ZMinus {
                adjacent_for_dirs(&face, &mut adjacent_face, vec![Dir::XPlus]);
            } else {
                adjacent_for_dirs(&face, &mut adjacent_face, vec![]);
            }

            adjacent_face.cube.x = 1;
            adjacent_face.cube.y = 2;
            adjacent_face.cube.z = 2;
            if d == Dir::YPlus {
                adjacent_for_dirs(&face, &mut adjacent_face, vec![Dir::ZMinus]);
            } else if d == Dir::ZPlus {
                adjacent_for_dirs(&face, &mut adjacent_face, vec![Dir::YMinus]);
            } else {
                adjacent_for_dirs(&face, &mut adjacent_face, vec![]);
            }

            adjacent_face.cube.x = 1;
            adjacent_face.cube.y = 2;
            adjacent_face.cube.z = 0;
            if d == Dir::YPlus {
                adjacent_for_dirs(&face, &mut adjacent_face, vec![Dir::ZPlus]);
            } else if d == Dir::ZMinus {
                adjacent_for_dirs(&face, &mut adjacent_face, vec![Dir::YMinus]);
            } else {
                adjacent_for_dirs(&face, &mut adjacent_face, vec![]);
            }

            adjacent_face.cube.x = 1;
            adjacent_face.cube.y = 0;
            adjacent_face.cube.z = 2;
            if d == Dir::YMinus {
                adjacent_for_dirs(&face, &mut adjacent_face, vec![Dir::ZMinus]);
            } else if d == Dir::ZPlus {
                adjacent_for_dirs(&face, &mut adjacent_face, vec![Dir::YPlus]);
            } else {
                adjacent_for_dirs(&face, &mut adjacent_face, vec![]);
            }

            adjacent_face.cube.x = 1;
            adjacent_face.cube.y = 0;
            adjacent_face.cube.z = 0;
            if d == Dir::YMinus {
                adjacent_for_dirs(&face, &mut adjacent_face, vec![Dir::ZPlus]);
            } else if d == Dir::ZMinus {
                adjacent_for_dirs(&face, &mut adjacent_face, vec![Dir::YPlus]);
            } else {
                adjacent_for_dirs(&face, &mut adjacent_face, vec![]);
            }

            // check the corners
            adjacent_face.cube.x = 0;
            adjacent_face.cube.y = 0;
            adjacent_face.cube.z = 0;
            adjacent_for_dirs(&face, &mut adjacent_face, vec![]);

            adjacent_face.cube.x = 0;
            adjacent_face.cube.y = 0;
            adjacent_face.cube.z = 2;
            adjacent_for_dirs(&face, &mut adjacent_face, vec![]);

            adjacent_face.cube.x = 0;
            adjacent_face.cube.y = 2;
            adjacent_face.cube.z = 0;
            adjacent_for_dirs(&face, &mut adjacent_face, vec![]);

            adjacent_face.cube.x = 0;
            adjacent_face.cube.y = 2;
            adjacent_face.cube.z = 2;
            adjacent_for_dirs(&face, &mut adjacent_face, vec![]);

            adjacent_face.cube.x = 2;
            adjacent_face.cube.y = 0;
            adjacent_face.cube.z = 0;
            adjacent_for_dirs(&face, &mut adjacent_face, vec![]);

            adjacent_face.cube.x = 2;
            adjacent_face.cube.y = 0;
            adjacent_face.cube.z = 2;
            adjacent_for_dirs(&face, &mut adjacent_face, vec![]);

            adjacent_face.cube.x = 2;
            adjacent_face.cube.y = 2;
            adjacent_face.cube.z = 0;
            adjacent_for_dirs(&face, &mut adjacent_face, vec![]);

            adjacent_face.cube.x = 2;
            adjacent_face.cube.y = 2;
            adjacent_face.cube.z = 2;
            adjacent_for_dirs(&face, &mut adjacent_face, vec![]);
        });

        // negative tests
        let face = Face {
            cube: Cube { x: 1, y: 2, z: 2 },
            dir: Dir::ZPlus,
        };
        let other = Face {
            cube: Cube { x: 2, y: 2, z: 4 },
            dir: Dir::XMinus,
        };
        assert!(!face.is_adjacent(&other));

        let mut face = Face {
            cube: Cube { x: 3, y: 3, z: 3 },
            dir: Dir::ZPlus,
        };
        let mut other = Face {
            cube: Cube { x: 2, y: 2, z: 2 },
            dir: Dir::ZPlus,
        };
        for d in Dir::iter() {
            face.dir = d;
            adjacent_for_dirs(&face, &mut other, vec![]);
        }
    }

    fn adjacent_for_dirs(face: &Face, other: &mut Face, passing_dirs: Vec<Dir>) {
        Dir::iter()
            .map(|d| (d, passing_dirs.contains(&d)))
            .for_each(|(d, passes)| {
                other.dir = d;
                if passes {
                    assert!(
                        face.is_adjacent(other),
                        "expected {:?} to be adjacent with {:?}",
                        face.dir,
                        d
                    )
                } else {
                    assert!(
                        !face.is_adjacent(other),
                        "expected {:?} not to be adjacent with {:?}",
                        face.dir,
                        d
                    )
                }
            })
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), Some(58));
    }
}
