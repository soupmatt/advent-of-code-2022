use num_traits::ToPrimitive;

pub fn part_one(input: &str) -> Option<String> {
    let sum = input.lines().map(|l| snafu_to_int(l).unwrap()).sum();
    let result = int_to_snafu(sum);
    Some(result)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn snafu_to_int(input: &str) -> Option<i64> {
    let mut result = 0;
    let chars = input.chars().rev().enumerate();
    for (i, c) in chars {
        let factor = 5_i64.pow(i.to_u32().unwrap());
        let amount = match c {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => return None,
        };
        result += amount * factor;
    }
    Some(result)
}

fn int_to_snafu(input: i64) -> String {
    let (len, mut fives, mut twos) = snafu_len_impl(input);
    let mut chars: Vec<char> = vec![];
    let input_i64: i64 = input.to_i64().unwrap();
    let mut builder: i64 = 0;
    for digit in (1..=len).into_iter().rev() {
        let next = input_i64 - builder;
        let check: i64 = (next + twos) / (fives / 5) - 2;

        let c = match check {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => panic!("something went wrong"),
        };
        chars.push(c);
        builder += fives / 5 * check;
        fives /= 5;
        twos -= 2 * fives;
    }
    chars.iter().collect()
}

fn snafu_len_impl(input: i64) -> (u32, i64, i64) {
    let mut len: u32 = 1;
    let mut fives: i64 = 5;
    let mut twos: i64 = 2;
    while fives - twos <= input {
        len += 1;
        twos += 2 * fives;
        fives *= 5;
    }
    (len, fives, twos)
}

#[cfg(test)]
fn snafu_len(input: i64) -> u32 {
    snafu_len_impl(input).0
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 25);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_one(&input), Some("2=-1=0".to_string()));
    }

    #[test]
    fn test_snafu_to_int() {
        assert_eq!(snafu_to_int("0"), Some(0));
        assert_eq!(snafu_to_int("1"), Some(1));
        assert_eq!(snafu_to_int("2"), Some(2));
        assert_eq!(snafu_to_int("1="), Some(3));
        assert_eq!(snafu_to_int("1-"), Some(4));
        assert_eq!(snafu_to_int("10"), Some(5));
        assert_eq!(snafu_to_int("12"), Some(7));
        assert_eq!(snafu_to_int("22"), Some(12));
        assert_eq!(snafu_to_int("1=="), Some(13));
        assert_eq!(snafu_to_int("222"), Some(62));
        assert_eq!(snafu_to_int("1==="), Some(63));
        assert_eq!(snafu_to_int("1=-0-2"), Some(1747));
        assert_eq!(snafu_to_int("12111"), Some(906));
        assert_eq!(snafu_to_int("2=0="), Some(198));
        assert_eq!(snafu_to_int("21"), Some(11));
        assert_eq!(snafu_to_int("2=01"), Some(201));
        assert_eq!(snafu_to_int("111"), Some(31));
        assert_eq!(snafu_to_int("20012"), Some(1257));
        assert_eq!(snafu_to_int("112"), Some(32));
        assert_eq!(snafu_to_int("1=-1="), Some(353));
        assert_eq!(snafu_to_int("1-12"), Some(107));
        assert_eq!(snafu_to_int("122"), Some(37));
    }

    #[test]
    fn test_snafu_len() {
        assert_eq!(snafu_len(1), 1);
        assert_eq!(snafu_len(2), 1);
        assert_eq!(snafu_len(3), 2);
        assert_eq!(snafu_len(4), 2);
        assert_eq!(snafu_len(5), 2);
        assert_eq!(snafu_len(12), 2);
        assert_eq!(snafu_len(13), 3);
        assert_eq!(snafu_len(62), 3);
        assert_eq!(snafu_len(63), 4);
        assert_eq!(snafu_len(353), 5);
    }

    #[test]
    fn test_int_to_snafu() {
        assert_eq!(int_to_snafu(1), "1");
        assert_eq!(int_to_snafu(2), "2");
        assert_eq!(int_to_snafu(3), "1=");
        assert_eq!(int_to_snafu(4), "1-");
        assert_eq!(int_to_snafu(5), "10");
        assert_eq!(int_to_snafu(6), "11");
        assert_eq!(int_to_snafu(7), "12");
        assert_eq!(int_to_snafu(8), "2=");
        assert_eq!(int_to_snafu(9), "2-");
        assert_eq!(int_to_snafu(10), "20");
        assert_eq!(int_to_snafu(15), "1=0");
        assert_eq!(int_to_snafu(20), "1-0");
        assert_eq!(int_to_snafu(2022), "1=11-2");
        assert_eq!(int_to_snafu(12345), "1-0---0");
        assert_eq!(int_to_snafu(314159265), "1121-1110-1=0");
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_two(&input), None);
    }
}
