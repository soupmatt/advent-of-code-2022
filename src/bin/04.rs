use std::ops::RangeInclusive;

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(parse_ranges)
        .map(|ranges| has_full_overlap(&ranges.0, &ranges.1) as u32)
        .sum();
    Some(result)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn parse_ranges(input: &str) -> (RangeInclusive<u32>, RangeInclusive<u32>) {
    let mut itr = input.split(',').map(|range| {
        let mut range = range.split('-');
        let first: u32 = range.next().unwrap().parse().unwrap();
        let second: u32 = range.next().unwrap().parse().unwrap();
        first..=second
    });
    let first = itr.next().unwrap();
    let second = itr.next().unwrap();
    (first, second)
}

fn has_full_overlap(left: &RangeInclusive<u32>, right: &RangeInclusive<u32>) -> bool {
    contains_other(left, right) || contains_other(right, left)
}

fn contains_other(left: &RangeInclusive<u32>, right: &RangeInclusive<u32>) -> bool {
    left.start() <= right.start() && left.end() >= right.end()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_parse_ranges() {
        assert_eq!(parse_ranges("1-3,5-7"), (1..=3, 5..=7));
        assert_eq!(parse_ranges("2-64,195-4097"), (2..=64, 195..=4097));
        assert_eq!(parse_ranges("34-34,52-87"), (34..=34, 52..=87));
    }

    #[test]
    fn test_has_full_overlap() {
        assert!(has_full_overlap(&(1..=3), &(1..=7)));
        assert!(has_full_overlap(&(3..=3), &(3..=7)));
        assert!(!has_full_overlap(&(12..=32), &(45..=70)));
        assert!(!has_full_overlap(&(45..=70), &(12..=32)));
        assert!(has_full_overlap(&(45..=48), &(45..=48)));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), None);
    }
}
