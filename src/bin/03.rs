use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|line| {
            let (left, right) = string_split(line);
            let common_char = find_common_char_from_2(left, right);
            char_priority(&common_char)
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let mut total = 0;

    loop {
        let mut group: Vec<&str> = Vec::new();
        for _ in 0..3 {
            match lines.next() {
                Some(line) => group.push(line),
                None => break,
            }
        }
        let priority = match group.len() {
            0 => break,
            3 => {
                let common_char = find_common_char(group.iter());
                char_priority(&common_char)
            }
            _ => panic!("something went wrong with the input"),
        };
        total += priority;
    }

    Some(total)
}

fn string_split(input: &str) -> (&str, &str) {
    let length = input.len();
    let left = &input[..length / 2];
    let right = &input[length / 2..];
    (left, right)
}

fn find_common_char_from_2(left: &str, right: &str) -> char {
    let left_hash: HashSet<char> = HashSet::from_iter(left.chars());
    let right_hash = HashSet::from_iter(right.chars());
    *left_hash.intersection(&right_hash).next().unwrap()
}

fn find_common_char<'a>(input: impl Iterator<Item = &'a &'a str> + 'a) -> char {
    *input
        .map(|line| line.chars().collect::<HashSet<_>>())
        .reduce(|acc, e| acc.intersection(&e).cloned().collect())
        .unwrap()
        .iter()
        .next()
        .unwrap()
}

fn char_priority(input: &char) -> u32 {
    let num = (*input) as u32;
    match num {
        97..=122 => num - 96,
        65..=90 => num - 38,
        _ => 0,
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_string_split() {
        assert_eq!(string_split("abcdef"), ("abc", "def"));
        assert_eq!(
            string_split("02934203975-093lksdjfklajrnarf"),
            ("02934203975-093", "lksdjfklajrnarf")
        );
    }

    #[test]
    fn test_find_common_char_from_2() {
        assert_eq!(find_common_char_from_2("abc", "aef"), 'a');
        assert_eq!(find_common_char_from_2("ZseFgD", "kSNfDi"), 'D');
    }

    #[test]
    fn test_find_common_char() {
        assert_eq!(find_common_char(vec!("abc", "aef", "aeg").iter()), 'a');
        assert_eq!(
            find_common_char(vec!("ZseFgD", "kSNfDi", "kSNfDj").iter()),
            'D'
        );
    }

    #[test]
    fn test_char_priority() {
        assert_eq!(char_priority(&'a'), 1);
        assert_eq!(char_priority(&'z'), 26);
        assert_eq!(char_priority(&'A'), 27);
        assert_eq!(char_priority(&'Z'), 52);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
