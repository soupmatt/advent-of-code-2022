use std::cmp::Ordering::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::{collections::BTreeMap, fmt::Display};

use num_integer::Integer;
use num_iter::range_inclusive;
use num_traits::{ToPrimitive, Zero};

pub struct SparseTable<N, T>
where
    T: Clone,
    N: Integer + Hash,
{
    data: BTreeMap<(N, N), T>,
    default: T,
    row_defaults: HashMap<N, T>,
    row_min: N,
    row_max: N,
    col_min: N,
    col_max: N,
}

impl<N: Integer + Hash + Copy, T: Clone> SparseTable<N, T> {
    pub fn new(default: T) -> Self {
        Self::new_with_start_point(default, (Zero::zero(), Zero::zero()))
    }

    pub fn new_with_start_point(default: T, (row, col): (N, N)) -> Self {
        let data = BTreeMap::new();
        let row_defaults = HashMap::new();
        SparseTable {
            data,
            default,
            row_defaults,
            row_min: row,
            row_max: row,
            col_min: col,
            col_max: col,
        }
    }

    pub fn insert(&mut self, (row, col): (N, N), value: T) -> Option<T> {
        self.update_min_max_vals(row, col);
        self.data.insert((row, col), value)
    }

    fn update_min_max_vals(&mut self, row: N, col: N) {
        self.row_min = row.min(self.row_min);
        self.row_max = row.max(self.row_max);
        self.col_min = col.min(self.col_min);
        self.col_max = col.max(self.col_max);
    }

    pub fn insert_only(&mut self, (row, col): (N, N), value: T) -> &T {
        self.update_min_max_vals(row, col);
        self.data.entry((row, col)).or_insert(value)
    }

    pub fn get(&self, (row, col): (N, N)) -> &T {
        self.data
            .get(&(row, col))
            .unwrap_or_else(|| self.get_default_for_row(row))
    }

    fn get_default_for_row(&self, row: N) -> &T {
        self.row_defaults.get(&row).unwrap_or(&self.default)
    }

    pub fn add_row_default(&mut self, row: N, default: T) -> Option<T> {
        self.update_min_max_vals(row, self.col_min);
        self.row_defaults.insert(row, default)
    }

    pub fn col_min(&self) -> &N {
        &self.col_min
    }

    pub fn col_max(&self) -> &N {
        &self.col_max
    }

    pub fn row_min(&self) -> &N {
        &self.row_min
    }

    pub fn row_max(&self) -> &N {
        &self.row_max
    }
}

impl<N, T> Display for SparseTable<N, T>
where
    N: Integer + Hash + ToString + Display + Clone + ToPrimitive + Copy,
    T: Clone + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let row_min_num_width = self.row_min.to_string().len();
        let row_max_num_width = self.row_max.to_string().len();
        let row_num_width = row_min_num_width.max(row_max_num_width);
        // header
        let col_max_num_length: u32 = self.col_max.to_string().len().to_u32().unwrap();
        let col_min_num_length: u32 = self.col_min.to_string().len().to_u32().unwrap();
        let col_num_length: u32 = col_min_num_length.max(col_max_num_length);
        let row_start = self.col_min.to_isize().unwrap();
        let row_stop = self.col_max.to_isize().unwrap();
        for col_digit in (0..col_num_length).rev() {
            for _ in 0..=row_num_width {
                write!(f, " ")?
            }
            let magnitude = 10isize.pow(col_digit);
            for i in range_inclusive(row_start, row_stop) {
                if i == row_start || i == row_stop || i % 5 == Zero::zero() {
                    if i.abs() >= magnitude || (i == 0 && magnitude == 1) {
                        write!(f, "{}", (i / magnitude).to_string().chars().last().unwrap())?
                    } else {
                        write!(f, " ")?
                    }
                } else {
                    write!(f, " ")?
                }
            }
            writeln!(f)?;
        }

        // main body
        let mut data_iter = self.data.iter();
        let mut cell = data_iter.next();
        for r in range_inclusive(self.row_min, self.row_max) {
            write!(f, "{r:row_num_width$} ")?;
            for c in range_inclusive(self.col_min, self.col_max) {
                let res = match cell {
                    Some((k, v)) => match (r, c).cmp(k) {
                        Less => write!(f, "{}", self.get_default_for_row(r)),
                        Equal => {
                            let result = write!(f, "{v}");
                            cell = data_iter.next();
                            result
                        }
                        Greater => panic!("Something went wrong!"),
                    },
                    None => write!(f, "{}", self.get_default_for_row(r)),
                };
                res?
            }
            writeln!(f)?
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_sparse_table() {
        let mut table: SparseTable<i32, i32> = SparseTable::new(0);

        assert_eq!(*table.get((0, 0)), 0);
        assert_eq!(*table.get((27, 4399)), 0);

        assert_eq!(*table.row_min(), 0);
        assert_eq!(*table.row_max(), 0);
        assert_eq!(*table.col_min(), 0);
        assert_eq!(*table.col_max(), 0);

        table.insert((3, 5), 7);
        table.insert((0, 2), 2);
        table.insert((1, 3), 3);
        table.insert((4, 2), 9);

        assert_eq!(*table.get((3, 5)), 7);
        assert_eq!(*table.get((0, 2)), 2);
        assert_eq!(*table.get((1, 3)), 3);
        assert_eq!(*table.get((4, 2)), 9);

        assert_eq!(*table.row_min(), 0);
        assert_eq!(*table.row_max(), 4);
        assert_eq!(*table.col_min(), 0);
        assert_eq!(*table.col_max(), 5);

        assert_eq!(*table.get((0, 0)), 0);
        assert_eq!(*table.get((2, 1)), 0);

        assert_eq!(*table.get((3, 4)), 0);
        assert_eq!(table.insert((3, 4), 4), None);
        assert_eq!(*table.get((3, 4)), 4);
        assert_eq!(table.insert((3, 4), 6), Some(4));
        assert_eq!(*table.get((3, 4)), 6);

        assert_eq!(table.insert_only((3, 4), 7), &6);
        assert_eq!(*table.get((3, 4)), 6);
        assert_eq!(table.insert_only((3, 6), 7), &7);
        assert_eq!(*table.get((3, 6)), 7);

        assert_eq!(*table.row_min(), 0);
        assert_eq!(*table.row_max(), 4);
        assert_eq!(*table.col_min(), 0);
        assert_eq!(*table.col_max(), 6);
    }

    #[test]
    fn test_sparse_table_display() {
        let table: SparseTable<i32, i32> = SparseTable::new(0);

        assert_eq!(
            format!("{table}"),
            indoc! { "
              0
            0 0
            "
            }
        );
    }

    #[test]
    fn test_sparse_table_display_medium() {
        let mut table: SparseTable<i32, i32> = SparseTable::new(0);

        table.insert((3, 5), 7);
        table.insert((0, 2), 2);
        table.insert((1, 3), 3);
        table.insert((4, 2), 9);
        table.insert((3, 4), 4);

        assert_eq!(
            format!("{table}"),
            indoc! { "
              0    5
            0 002000
            1 000300
            2 000000
            3 000047
            4 009000
            "
            }
        );
    }

    #[test]
    fn test_sparse_table_display_large() {
        let mut table: SparseTable<i32, i32> = SparseTable::new(0);

        table.insert((13, 5), 7);
        table.insert((0, 2), 2);
        table.insert((-7, 19), 3);
        table.insert((-9, 8), 9);
        table.insert((3, 4), 4);
        table.insert((3, -2), 1);

        let actual = format!("{table}");
        let expected = indoc! { "
                           1    1   1
               2 0    5    0    5   9
            -9 0000000000900000000000
            -8 0000000000000000000000
            -7 0000000000000000000003
            -6 0000000000000000000000
            -5 0000000000000000000000
            -4 0000000000000000000000
            -3 0000000000000000000000
            -2 0000000000000000000000
            -1 0000000000000000000000
             0 0000200000000000000000
             1 0000000000000000000000
             2 0000000000000000000000
             3 1000004000000000000000
             4 0000000000000000000000
             5 0000000000000000000000
             6 0000000000000000000000
             7 0000000000000000000000
             8 0000000000000000000000
             9 0000000000000000000000
            10 0000000000000000000000
            11 0000000000000000000000
            12 0000000000000000000000
            13 0000000700000000000000
            "
        };

        let act_vec: Vec<&str> = actual.lines().collect();
        let exp_vec: Vec<&str> = expected.lines().collect();

        assert_eq!(act_vec.len(), exp_vec.len());

        for i in 0..act_vec.len() {
            assert_eq!(act_vec[i], exp_vec[i]);
        }

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_sparse_table_with_row_default() {
        let mut table: SparseTable<i32, i32> = SparseTable::new(0);
        assert_eq!(*table.row_max(), 0);
        assert_eq!(*table.get((3, 0)), 0);

        table.add_row_default(3, 9);
        assert_eq!(*table.row_max(), 3);
        assert_eq!(*table.get((3, 0)), 9);

        table.insert((0, 2), 1);
        table.insert((3, 1), 2);
        assert_eq!(*table.get((3, 0)), 9);
        assert_eq!(*table.get((3, 1)), 2);
        assert_eq!(*table.get((3, 2)), 9);

        assert_eq!(
            format!("{table}"),
            indoc! {"
              0 2
            0 001
            1 000
            2 000
            3 929
        "}
        )
    }
}
