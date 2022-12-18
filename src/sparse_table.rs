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
        self.row_max = row.max(self.row_max);
        self.col_min = col.min(self.col_min);
        self.col_max = col.max(self.col_max);
        self.data.insert((row, col), value)
    }

    pub fn get(&self, (row, col): (N, N)) -> &T {
        self.data
            .get(&(row, col))
            .unwrap_or_else(|| self.get_default_for_row(row))
    }

    pub fn get_default_for_row(&self, row: N) -> &T {
        self.row_defaults.get(&row).unwrap_or(&self.default)
    }

    pub fn add_row_default(&mut self, row: N, default: T) -> Option<T> {
        self.row_max = row.max(self.row_max);
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
        let row_num_width = self.row_max.to_string().len();
        // header
        let col_num_length: u32 = self.col_max.to_string().len().to_u32().unwrap();
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
            write!(f, "{:row_num_width$} ", r)?;
            for c in range_inclusive(self.col_min, self.col_max) {
                let res = match cell {
                    Some((k, v)) => match (r, c).cmp(k) {
                        Less => write!(f, "{}", self.get_default_for_row(r)),
                        Equal => {
                            let result = write!(f, "{}", v);
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
        let mut table = SparseTable::new(0);

        assert_eq!(*table.get((0, 0)), 0);
        assert_eq!(*table.get((27, 4399)), 0);

        table.insert((3, 5), 7);
        table.insert((0, 2), 2);
        table.insert((1, 3), 3);
        table.insert((4, 2), 9);

        assert_eq!(*table.get((3, 5)), 7);
        assert_eq!(*table.get((0, 2)), 2);
        assert_eq!(*table.get((1, 3)), 3);
        assert_eq!(*table.get((4, 2)), 9);

        assert_eq!(*table.get((0, 0)), 0);
        assert_eq!(*table.get((2, 1)), 0);

        assert_eq!(*table.get((3, 4)), 0);
        assert_eq!(table.insert((3, 4), 4), None);
        assert_eq!(*table.get((3, 4)), 4);
        assert_eq!(table.insert((3, 4), 6), Some(4));
        assert_eq!(*table.get((3, 4)), 6);
    }

    #[test]
    fn test_sparse_table_display() {
        let table: SparseTable<i32, i32> = SparseTable::new(0);

        assert_eq!(
            format!("{}", table),
            indoc! { "
              0
            0 0
            "
            }
        );
    }

    #[test]
    fn test_sparse_table_display_medium() {
        let mut table = SparseTable::new(0);

        table.insert((3, 5), 7);
        table.insert((0, 2), 2);
        table.insert((1, 3), 3);
        table.insert((4, 2), 9);
        table.insert((3, 4), 4);

        assert_eq!(
            format!("{}", table),
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
        let mut table = SparseTable::new(0);

        table.insert((13, 5), 7);
        table.insert((0, 2), 2);
        table.insert((19, 17), 3);
        table.insert((8, 19), 9);
        table.insert((3, 4), 4);

        assert_eq!(
            format!("{}", table),
            indoc! { "
                         1    1   1
               0    5    0    5   9
             0 00200000000000000000
             1 00000000000000000000
             2 00000000000000000000
             3 00004000000000000000
             4 00000000000000000000
             5 00000000000000000000
             6 00000000000000000000
             7 00000000000000000000
             8 00000000000000000009
             9 00000000000000000000
            10 00000000000000000000
            11 00000000000000000000
            12 00000000000000000000
            13 00000700000000000000
            14 00000000000000000000
            15 00000000000000000000
            16 00000000000000000000
            17 00000000000000000000
            18 00000000000000000000
            19 00000000000000000300
            "
            }
        );
    }

    #[test]
    fn test_sparse_table_with_row_default() {
        let mut table = SparseTable::new(0);
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
            format!("{}", table),
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
