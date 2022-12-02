pub fn part_one(input: &str) -> Option<u32> {
    let counts = parse_input(input);
    counts.iter().max().copied()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut counts = parse_input(input);
    counts.sort();
    counts.reverse();
    let counts = &counts[0..3];

    let result: u32 = counts.iter().sum();
    Some(result)
}

fn parse_input(input: &str) -> Vec<u32> {
    let mut counts: Vec<u32> = Vec::new();

    let lines = input.lines();

    let mut total: u32 = 0;
    for line in lines {
        if line.is_empty() {
            counts.push(total);
            total = 0
        } else {
            let cals: u32 = line.parse().expect("couldn't parse line");
            total += cals
        }
    }
    if total > 0 {
        counts.push(total)
    }
    counts
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
