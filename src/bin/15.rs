use std::collections::BTreeSet;

use itertools::Itertools;
use lazy_static::lazy_static;
use num_traits::ToPrimitive;
use regex::Regex;

pub fn part_one(input: &str) -> Option<usize> {
    not_beacon_count(input, 2_000_000)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

fn not_beacon_count(input: &str, row: isize) -> Option<usize> {
    let mut beacons_on_row: BTreeSet<isize> = BTreeSet::new();
    let mut sensors_on_row: BTreeSet<isize> = BTreeSet::new();
    let mut columns: BTreeSet<isize> = BTreeSet::new();

    parse_input(input).for_each(|(sx, sy, bx, by)| {
        let dist = manhatten_distance(sx, sy, bx, by).to_isize().unwrap();
        if sy == row {
            sensors_on_row.insert(sx);
        }
        if by == row {
            beacons_on_row.insert(bx);
        }
        if sy >= (row - dist) && sy <= (row + dist) {
            let height_diff = row.abs_diff(sy).to_isize().unwrap();
            let width_diff = dist - height_diff;
            let range = (sx - width_diff)..=(sx + width_diff);
            for i in range {
                columns.insert(i);
            }
        }
    });
    sensors_on_row.iter().for_each(|s| {
        columns.remove(s);
    });
    beacons_on_row.iter().for_each(|s| {
        columns.remove(s);
    });
    Some(columns.len())
}

fn manhatten_distance(x1: isize, y1: isize, x2: isize, y2: isize) -> u32 {
    (x1.abs_diff(x2) + y1.abs_diff(y2)).to_u32().unwrap()
}

fn parse_input(input: &str) -> impl Iterator<Item = (isize, isize, isize, isize)> + '_ {
    lazy_static! {
        static ref INPUT: Regex = Regex::new(
            r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$"
        )
        .unwrap();
    }

    input.lines().map(|line| {
        INPUT
            .captures(line)
            .unwrap()
            .iter()
            .skip(1)
            .map(|d| d.unwrap().as_str().parse::<isize>().unwrap())
            .collect_tuple()
            .unwrap()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(not_beacon_count(&input, 10), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), None);
    }
}
