pub fn part_one(input: &str) -> Option<u32> {
    let final_score = parse_input(input).map(|r| r.points_scored()).sum();
    Some(final_score)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = Round> + 'a {
    input.lines().map(|line| {
        let mut parts = line.split_whitespace();
        let op_play = match parts.next().unwrap() {
            "A" => Rps::Rock,
            "B" => Rps::Paper,
            "C" => Rps::Scissors,
            _ => panic!("Invalid input"),
        };
        let my_play = match parts.next().unwrap() {
            "X" => Rps::Rock,
            "Y" => Rps::Paper,
            "Z" => Rps::Scissors,
            _ => panic!("Invalid input"),
        };
        assert_eq!(parts.next(), None);
        Round {
            me: my_play,
            opponent: op_play,
        }
    })
}

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
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
}

#[derive(Debug, PartialEq, Eq)]
struct Round {
    me: Rps,
    opponent: Rps,
}

impl Round {
    fn points_scored(&self) -> u32 {
        let outcome = self.me.outcome(&self.opponent);
        self.me.points() + outcome.points()
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
    fn test_parse_input() {
        let input = advent_of_code::read_file("examples", 2);
        let result: Vec<Round> = parse_input(&input).collect();
        let expected = vec![
            Round {
                opponent: Rps::Rock,
                me: Rps::Paper,
            },
            Round {
                opponent: Rps::Paper,
                me: Rps::Rock,
            },
            Round {
                opponent: Rps::Scissors,
                me: Rps::Scissors,
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
            me: Rps::Rock,
            opponent: Rps::Rock,
        };
        assert_eq!(round.points_scored(), 4);

        let round = Round {
            me: Rps::Rock,
            opponent: Rps::Paper,
        };
        assert_eq!(round.points_scored(), 1);

        let round = Round {
            me: Rps::Rock,
            opponent: Rps::Scissors,
        };
        assert_eq!(round.points_scored(), 7);

        let round = Round {
            me: Rps::Paper,
            opponent: Rps::Rock,
        };
        assert_eq!(round.points_scored(), 8);

        let round = Round {
            me: Rps::Paper,
            opponent: Rps::Paper,
        };
        assert_eq!(round.points_scored(), 5);

        let round = Round {
            me: Rps::Paper,
            opponent: Rps::Scissors,
        };
        assert_eq!(round.points_scored(), 2);

        let round = Round {
            me: Rps::Scissors,
            opponent: Rps::Rock,
        };
        assert_eq!(round.points_scored(), 3);

        let round = Round {
            me: Rps::Scissors,
            opponent: Rps::Paper,
        };
        assert_eq!(round.points_scored(), 9);

        let round = Round {
            me: Rps::Scissors,
            opponent: Rps::Scissors,
        };
        assert_eq!(round.points_scored(), 6);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
