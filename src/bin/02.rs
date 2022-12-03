pub fn part_one(input: &str) -> Option<u32> {
    let final_score = parse_input_part_one(input).map(|r| r.points_scored()).sum();
    Some(final_score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let final_score = parse_input_part_two(input).map(|r| r.points_scored()).sum();
    Some(final_score)
}

fn parse_input_part_one(input: &str) -> impl Iterator<Item = Round> + '_ {
    parse_input(input, |left, right| {
        let op_play = match left {
            "A" => Rps::Rock,
            "B" => Rps::Paper,
            "C" => Rps::Scissors,
            _ => panic!("Invalid input"),
        };
        let my_play = match right {
            "X" => Rps::Rock,
            "Y" => Rps::Paper,
            "Z" => Rps::Scissors,
            _ => panic!("Invalid input"),
        };
        Round {
            opponent: op_play,
            info: Info::Play(my_play),
        }
    })
}

fn parse_input_part_two(input: &str) -> impl Iterator<Item = Round> + '_ {
    parse_input(input, |left, right| {
        let op_play = match left {
            "A" => Rps::Rock,
            "B" => Rps::Paper,
            "C" => Rps::Scissors,
            _ => panic!("Invalid input"),
        };
        let my_outcome = match right {
            "X" => Outcome::Loss,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Invalid input"),
        };
        Round {
            opponent: op_play,
            info: Info::Outcome(my_outcome),
        }
    })
}

fn parse_input<'a, F>(input: &'a str, f: F) -> impl Iterator<Item = Round> + 'a
where
    F: Fn(&str, &str) -> Round + 'a,
{
    input.lines().map(move |line| {
        let mut parts = line.split_whitespace();
        let round = f(parts.next().unwrap(), parts.next().unwrap());
        assert_eq!(parts.next(), None);
        round
    })
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

impl Rps {
    fn outcome(&self, other: &Rps) -> Outcome {
        match (self, other) {
            (Rps::Rock, Rps::Scissors) => Outcome::Win,
            (Rps::Paper, Rps::Rock) => Outcome::Win,
            (Rps::Scissors, Rps::Paper) => Outcome::Win,
            (Rps::Rock, Rps::Rock) => Outcome::Draw,
            (Rps::Paper, Rps::Paper) => Outcome::Draw,
            (Rps::Scissors, Rps::Scissors) => Outcome::Draw,
            (Rps::Rock, Rps::Paper) => Outcome::Loss,
            (Rps::Paper, Rps::Scissors) => Outcome::Loss,
            (Rps::Scissors, Rps::Rock) => Outcome::Loss,
        }
    }

    fn points(&self) -> u32 {
        match self {
            Rps::Rock => 1,
            Rps::Paper => 2,
            Rps::Scissors => 3,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    fn points(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }

    fn against(&self, op_play: &Rps) -> Rps {
        match self {
            Outcome::Draw => match op_play {
                Rps::Rock => Rps::Rock,
                Rps::Paper => Rps::Paper,
                Rps::Scissors => Rps::Scissors,
            },
            Outcome::Loss => match op_play {
                Rps::Rock => Rps::Scissors,
                Rps::Paper => Rps::Rock,
                Rps::Scissors => Rps::Paper,
            },
            Outcome::Win => match op_play {
                Rps::Rock => Rps::Paper,
                Rps::Paper => Rps::Scissors,
                Rps::Scissors => Rps::Rock,
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Info {
    Play(Rps),
    Outcome(Outcome),
}

#[derive(Debug, PartialEq, Eq)]
struct Round {
    opponent: Rps,
    info: Info,
}

impl Round {
    fn outcome(&self) -> Outcome {
        match self.info {
            Info::Outcome(o) => o,
            Info::Play(p) => p.outcome(&self.opponent),
        }
    }

    fn my_play(&self) -> Rps {
        match self.info {
            Info::Outcome(o) => o.against(&self.opponent),
            Info::Play(p) => p,
        }
    }

    fn points_scored(&self) -> u32 {
        self.my_play().points() + self.outcome().points()
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_one_single_inputs() {
        assert_eq!(part_one("A X"), Some(4));
        assert_eq!(part_one("A Y"), Some(8));
        assert_eq!(part_one("A Z"), Some(3));
        assert_eq!(part_one("B X"), Some(1));
        assert_eq!(part_one("B Y"), Some(5));
        assert_eq!(part_one("B Z"), Some(9));
        assert_eq!(part_one("C X"), Some(7));
        assert_eq!(part_one("C Y"), Some(2));
        assert_eq!(part_one("C Z"), Some(6));
    }

    #[test]
    fn test_part_one_custom_input() {
        let input = "B Y
        C Z
        B Y
        C Z
        B X
        C Z
        C Z";
        assert_eq!(part_one(input), Some(35));
    }

    #[test]
    fn test_parse_input_part_1() {
        let input = advent_of_code::read_file("examples", 2);
        let result: Vec<Round> = parse_input_part_one(&input).collect();
        let expected = vec![
            Round {
                opponent: Rps::Rock,
                info: Info::Play(Rps::Paper),
            },
            Round {
                opponent: Rps::Paper,
                info: Info::Play(Rps::Rock),
            },
            Round {
                opponent: Rps::Scissors,
                info: Info::Play(Rps::Scissors),
            },
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_rps_outcome() {
        assert_eq!(Rps::Rock.outcome(&Rps::Rock), Outcome::Draw);
        assert_eq!(Rps::Rock.outcome(&Rps::Paper), Outcome::Loss);
        assert_eq!(Rps::Rock.outcome(&Rps::Scissors), Outcome::Win);
        assert_eq!(Rps::Paper.outcome(&Rps::Rock), Outcome::Win);
        assert_eq!(Rps::Paper.outcome(&Rps::Paper), Outcome::Draw);
        assert_eq!(Rps::Paper.outcome(&Rps::Scissors), Outcome::Loss);
        assert_eq!(Rps::Scissors.outcome(&Rps::Rock), Outcome::Loss);
        assert_eq!(Rps::Scissors.outcome(&Rps::Paper), Outcome::Win);
        assert_eq!(Rps::Scissors.outcome(&Rps::Scissors), Outcome::Draw);
    }

    #[test]
    fn test_round_points_scored() {
        let round = Round {
            info: Info::Play(Rps::Rock),
            opponent: Rps::Rock,
        };
        assert_eq!(round.points_scored(), 4);

        let round = Round {
            info: Info::Play(Rps::Rock),
            opponent: Rps::Paper,
        };
        assert_eq!(round.points_scored(), 1);

        let round = Round {
            info: Info::Play(Rps::Rock),
            opponent: Rps::Scissors,
        };
        assert_eq!(round.points_scored(), 7);

        let round = Round {
            info: Info::Play(Rps::Paper),
            opponent: Rps::Rock,
        };
        assert_eq!(round.points_scored(), 8);

        let round = Round {
            info: Info::Play(Rps::Paper),
            opponent: Rps::Paper,
        };
        assert_eq!(round.points_scored(), 5);

        let round = Round {
            info: Info::Play(Rps::Paper),
            opponent: Rps::Scissors,
        };
        assert_eq!(round.points_scored(), 2);

        let round = Round {
            info: Info::Play(Rps::Scissors),
            opponent: Rps::Rock,
        };
        assert_eq!(round.points_scored(), 3);

        let round = Round {
            info: Info::Play(Rps::Scissors),
            opponent: Rps::Paper,
        };
        assert_eq!(round.points_scored(), 9);

        let round = Round {
            info: Info::Play(Rps::Scissors),
            opponent: Rps::Scissors,
        };
        assert_eq!(round.points_scored(), 6);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
