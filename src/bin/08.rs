pub fn part_one(input: &str) -> Option<usize> {
    let heights = parse_input(input);
    let max_row = heights.len();
    let max_col = heights.first().unwrap().len();
    let mut visible = (max_row + max_col) * 2 - 4;
    for current_row in 1..max_row - 1 {
        'next_tree: for current_col in 1..max_col - 1 {
            let current_height = &heights[current_row][current_col];
            //check to the left
            if (0..current_col)
                .into_iter()
                .all(|check_col| current_height > &heights[current_row][check_col])
            {
                println!(
                    "{},{}, with height {} is visible from the left",
                    current_row, current_col, current_height
                );
                visible += 1;
                continue 'next_tree;
            }
            //check to the right
            if (current_col + 1..max_col)
                .into_iter()
                .all(|check_col| current_height > &heights[current_row][check_col])
            {
                println!(
                    "{},{}, with height {} is visible from the right",
                    current_row, current_col, current_height
                );
                visible += 1;
                continue 'next_tree;
            }
            //check up
            if (0..current_row)
                .into_iter()
                .all(|check_row| current_height > &heights[check_row][current_col])
            {
                println!(
                    "{},{}, with height {} is visible from the top",
                    current_row, current_col, current_height
                );
                visible += 1;
                continue 'next_tree;
            }
            //check down
            if (current_row + 1..max_row)
                .into_iter()
                .all(|check_row| current_height > &heights[check_row][current_col])
            {
                println!(
                    "{},{}, with height {} is visible from the bottom",
                    current_row, current_col, current_height
                );
                visible += 1;
                continue 'next_tree;
            }
        }
    }
    Some(visible)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), None);
    }
}
