pub fn part_one(input: &str) -> Option<usize> {
    let forest = parse_input(input);
    Some(forest.visible_trees())
}

pub fn part_two(input: &str) -> Option<u32> {
    let forest = parse_input(input);
    forest.scenery_scores().into_iter().max()
}

struct Forest {
    heights: Vec<Vec<u8>>,
    max_row: usize,
    max_col: usize,
}

fn parse_input(input: &str) -> Forest {
    let heights: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect();
    let max_row = heights.len();
    let max_col = heights.first().unwrap().len();
    Forest {
        heights,
        max_row,
        max_col,
    }
}

impl Forest {
    fn visible_trees(&self) -> usize {
        let mut visible = (self.max_row + self.max_col) * 2 - 4;
        for current_row in 1..self.max_row - 1 {
            'next_tree: for current_col in 1..self.max_col - 1 {
                let current_height = self.location_height(current_row, current_col);
                //check to the left
                if (0..current_col)
                    .into_iter()
                    .all(|check_col| current_height > self.location_height(current_row, check_col))
                {
                    visible += 1;
                    continue 'next_tree;
                }
                //check to the right
                if (current_col + 1..self.max_col)
                    .into_iter()
                    .all(|check_col| current_height > self.location_height(current_row, check_col))
                {
                    visible += 1;
                    continue 'next_tree;
                }
                //check up
                if (0..current_row)
                    .into_iter()
                    .all(|check_row| current_height > self.location_height(check_row, current_col))
                {
                    visible += 1;
                    continue 'next_tree;
                }
                //check down
                if (current_row + 1..self.max_row)
                    .into_iter()
                    .all(|check_row| current_height > self.location_height(check_row, current_col))
                {
                    visible += 1;
                    continue 'next_tree;
                }
            }
        }
        visible
    }

    fn location_height(&self, row: usize, col: usize) -> &u8 {
        &self.heights[row][col]
    }

    fn scenery_scores(&self) -> Vec<u32> {
        let mut result = vec![];
        for current_row in 1..self.max_row - 1 {
            for current_col in 1..self.max_col - 1 {
                result.push(self.scenery_score(current_row, current_col))
            }
        }
        result
    }

    fn scenery_score(&self, row: usize, col: usize) -> u32 {
        let score = self.viewing_distance_north(row, col)
            * self.viewing_distance_south(row, col)
            * self.viewing_distance_east(row, col)
            * self.viewing_distance_west(row, col);
        score
    }

    fn viewing_distance_north(&self, row: usize, col: usize) -> u32 {
        let current_height = self.location_height(row, col);
        let mut distance: u32 = 0;
        for check_row in (0..row).rev() {
            distance += 1;
            if current_height <= self.location_height(check_row, col) {
                break;
            }
        }
        distance
    }

    fn viewing_distance_south(&self, row: usize, col: usize) -> u32 {
        let current_height = self.location_height(row, col);
        let mut distance: u32 = 0;
        for check_row in row + 1..self.max_row {
            distance += 1;
            if current_height <= self.location_height(check_row, col) {
                break;
            }
        }
        distance
    }

    fn viewing_distance_east(&self, row: usize, col: usize) -> u32 {
        let current_height = self.location_height(row, col);
        let mut distance: u32 = 0;
        for check_col in col + 1..self.max_col {
            distance += 1;
            if current_height <= self.location_height(row, check_col) {
                break;
            }
        }
        distance
    }

    fn viewing_distance_west(&self, row: usize, col: usize) -> u32 {
        let current_height = self.location_height(row, col);
        let mut distance: u32 = 0;
        for check_col in (0..col).rev() {
            distance += 1;
            if current_height <= self.location_height(row, check_col) {
                break;
            }
        }
        distance
    }
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
        assert_eq!(part_two(&input), Some(8));
    }
}
